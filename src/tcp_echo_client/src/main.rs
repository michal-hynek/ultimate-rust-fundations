use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpStream};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8123").await?;
    println!("Connected to the server");

    // send a message to the server
    stream.write_all(b"Hello World!").await?;

    // read the response
    let mut buf = vec![0; 1024];
    let bytes_read = stream.read(&mut buf).await?;
    println!("Response: {}", String::from_utf8_lossy(&buf[..bytes_read]));

    Ok(())
}
