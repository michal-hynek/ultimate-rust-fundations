//use std::sync::Mutex;
use tokio::sync::Mutex;
use once_cell::sync::Lazy;

static COUNTER: Lazy<Mutex<u32>> = Lazy::new(|| Mutex::new(0));

async fn add_one(n: u32) -> u32 {
    n + 1
}

async fn increment() {
    // using standard mutex can lock the Tokio runtime and result in deadlock
    //let mut counter = COUNTER.lock().unwrap();
    let mut counter = COUNTER.lock().await;
    *counter = add_one(*counter).await;
}

#[tokio::main]
async fn main() {
    tokio::join!(increment(), increment(), increment());
    //println!("COUNTER = {}", COUNTER.lock().unwrap());
    println!("COUNTER = {}", COUNTER.lock().await);
}
