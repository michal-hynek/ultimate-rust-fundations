use tokio::runtime;

async fn hello() {
    println!("Hello Tokio");
}

fn main() -> anyhow::Result<()> {
    let runtime = runtime::Builder::new_multi_thread()
        .enable_all()
        .worker_threads(2)
        .build()?;

    runtime.block_on(hello());

    Ok(())
}
