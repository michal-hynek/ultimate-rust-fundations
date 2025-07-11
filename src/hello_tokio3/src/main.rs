async fn hello() -> u32 {
    println!("Hello Tokio");
    3
}

async fn hello2() -> u32 {
    println!("Hello Tokio 2");
    4
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let (x, y) = tokio::join!(hello(), hello2());
    println!("hello() = {}, hello2() = {}", x, y);
}
