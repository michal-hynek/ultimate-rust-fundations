async fn hello() {
    println!("Hello Tokio");
}

async fn ticker() {
    for i in 0..10 {
        println!("tick {i}");
    }
}
#[tokio::main]
async fn main() {
    tokio::spawn(ticker());

    hello().await;
}
