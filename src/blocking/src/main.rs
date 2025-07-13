use std::time::Duration;

async fn hello_delay(task: u64, time: u64) {
    println!("Task {task} has started");
    // blocks tokio runtime thread - should use tokio::time::sleep or token::spawn_blocking instead
    std::thread::sleep(Duration::from_millis(time));
    println!("Task {task} has finished");
}

#[tokio::main]
async fn main() {
    tokio::join!(
        hello_delay(1, 500),
        hello_delay(2, 1000),
        hello_delay(3, 1500),
    );
}
