# 🍽️ Served - The Async Data Processing Pipeline

## 🚀 Overview
Welcome to **Served** – a sorta-fast, **asynchronous data processing pipeline** in Rust! 🦀✨ This pipeline allows you to seamlessly chain services, share global state, and process data efficiently using Rust's async capabilities.

## 🎯 Features
- ⚡ **Asynchronous Execution:** Built with `tokio` for lightning-fast concurrent processing.
- 🔗 **Service Pipeline:** Execute services in sequence with dynamic dispatch.
- 🌍 **Global Context Sharing:** Uses `Arc<RwLock<T>>` to maintain shared state across services.
- ❌ **Error Handling:** Gracefully captures service failures and prevents cascading errors.
- 🔀 **Flexible Output Processing:** Supports dynamic output types via the `ProcessingOutput` trait.

## 📦 Installation
To get started, make sure you have Rust installed, then clone the repository and run:

```sh
cargo build
```

## 🎮 Usage
Run the Served pipeline with:

```sh
cargo run
```

## 🏗️ Architecture
### **1️⃣ Service Context**
A shared state object (`ServiceContext`) that maintains global data for all services.

### **2️⃣ BasicService Trait**
Defines a contract that all services must implement. Each service processes an input and returns a result asynchronously.

```rust
pub trait BasicService: Send + Sync {
    fn call(&self, input: ProcessingResult, context: Arc<ServiceContext>) -> Pin<Box<dyn Future<Output = ProcessingResult> + Send>>;
}
```

### **3️⃣ Service Processor**
Manages and executes services in sequence. It can be initialized using:

```rust
let processor = ServiceProcessor::build(context, vec![ExampleService]);
```

### **4️⃣ Example Implementation**
An example service and output type:

```rust
pub struct ExampleService;

impl BasicService for ExampleService {
    fn call(&self, input: ProcessingResult, context: Arc<ServiceContext>) -> Pin<Box<dyn Future<Output = ProcessingResult> + Send>> {
        Box::pin(async move {
            let state = context.shared_state.read().await.clone();
            println!("Executing ExampleService with state: {}", state);
            ProcessingResult::Output(input)
        })
    }
}
```

## 📚 Dependencies
- 🚀 `tokio` - For async execution.
- 🛠️ `async-trait` - Enables async functions in traits.

## 🌟 Future Improvements
- 🏎️ **Parallel Execution:** Add support for concurrent service execution.
- 🛑 **Custom Error Handling:** More detailed error reporting and recovery.
- 📊 **Logging & Metrics:** Integrate structured logging and monitoring.

## 📜 License
This project is licensed under the MIT License. ⚖️

🎉 **Enjoy using Served! Your data has never been so deliciously processed! 🍽️🔥**


