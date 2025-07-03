use futures::{executor::block_on, future::join_all};

async fn say_hello() {
    println!("Hello");

    let future = second_function();
    future.await;

    futures::join!(second_function(), goodbye());

    let n = double(2).await;
    println!("{n}");

    let futures = vec![double(1), double(2), double(3)];
    let results = join_all(futures).await;
    println!("{results:#?}");
}

async fn second_function() {
    println!("Hello again");
}

async fn goodbye() {
    println!("Goodbye");
}

async fn double(n: i32) -> i32 {
    n*2
}

fn main() {
    block_on(say_hello());
}
