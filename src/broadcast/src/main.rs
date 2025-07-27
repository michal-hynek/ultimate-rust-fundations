#[tokio::main]
async fn main() {
    let receivers = 16;
    let (tx, mut rx) = tokio::sync::broadcast::channel::<String>(receivers);

    for n in 0..receivers {
        let mut messages = tx.subscribe();

        tokio::spawn(async move {
            while let Ok(msg) = messages.recv().await {
                println!("{n}: {msg}");
            }
        });
    }

    tx.send("hello".to_string()).unwrap();
    while let Ok(msg) = rx.recv().await {
        println!("main: {msg}");
    }
}
