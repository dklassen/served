use std::any::Any;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::RwLock;

// Type alias for the dyn-compatible Future
pub type Output<T> = Pin<Box<dyn Future<Output = T> + Send>>;

pub struct ServiceContext {
    data: Arc<RwLock<HashMap<String, Arc<dyn Any + Send + Sync>>>>,
}

impl Default for ServiceContext {
    fn default() -> Self {
        Self::new()
    }
}

impl ServiceContext {
    pub fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn insert<T: Any + Send + Sync>(&self, key: String, value: T) {
        let mut data = self.data.write().await;
        data.insert(key, Arc::new(value));
    }

    pub async fn get<T: Any + Send + Sync>(&self, key: &str) -> Option<Arc<T>> {
        let data = self.data.read().await;
        data.get(key)?
            .clone()
            .downcast::<T>()
            .ok()
    }
}

pub trait BasicService: Send + Sync {
    type Input: Send + Sync;
    type Output: Send + Sync;
    type Error: Error + Send + Sync;

    fn call(
        &self,
        input: Self::Input,
        context: Arc<ServiceContext>,
    ) -> Output<Result<Self::Output, Self::Error>>;
}

#[derive(Debug)]
pub struct ServiceError(String);

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for ServiceError {}

pub trait ServiceChainExecutor: Send + Sync + 'static {
    type Input;
    type Output;
    type Error: Error + Send + Sync;

    fn execute(
        &self,
        input: Self::Input,
        context: Arc<ServiceContext>,
    ) -> Output<Result<Self::Output, Self::Error>>;
}

impl<S> ServiceChainExecutor for S
where
    S: BasicService + 'static,
{
    type Input = S::Input;
    type Output = S::Output;
    type Error = S::Error;

    fn execute(
        &self,
        input: Self::Input,
        context: Arc<ServiceContext>,
    ) -> Output<Result<Self::Output, Self::Error>> {
        self.call(input, context)
    }
}

impl<Head, Tail> ServiceChainExecutor for (Head, Tail)
where
    Head: BasicService + Clone + 'static,
    Tail: ServiceChainExecutor<Input = Head::Output> + Clone + 'static,
    Tail::Error: From<Head::Error>,
{
    type Input = Head::Input;
    type Output = Tail::Output;
    type Error = Tail::Error;

    fn execute(
        &self,
        input: Self::Input,
        context: Arc<ServiceContext>,
    ) -> Output<Result<Self::Output, Self::Error>> {
        let head = self.0.clone();
        let tail = self.1.clone();
        Box::pin(async move {
            let intermediate = head.call(input, Arc::clone(&context)).await?;
            tail.execute(intermediate, context).await
        })
    }
}

pub struct ServiceProcessor<Chain: ServiceChainExecutor> {
    chain: Chain,
    context: Arc<ServiceContext>,
}

impl<Chain: ServiceChainExecutor> ServiceProcessor<Chain> {
    pub fn new(chain: Chain, context: ServiceContext) -> Self {
        Self {
            chain,
            context: Arc::new(context),
        }
    }

    pub async fn execute(&self, input: Chain::Input) -> Result<Chain::Output, Chain::Error> {
        self.chain.execute(input, Arc::clone(&self.context)).await
    }
}
