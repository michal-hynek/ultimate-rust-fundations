async fn hello() {
    println!("Hello Tokio 2");
}

//#[tokio::main(flavor = "current_thread")]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    hello().await;

    Ok(())
}
