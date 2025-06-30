use rayon::ThreadPoolBuilder;

fn test() {
    println!("Hello from test");
}

fn main() {
    let pool = ThreadPoolBuilder::new()
        .num_threads(4)
        .build()
        .unwrap();

    pool.scope(|scope| {
        scope.spawn_broadcast(|_scope, context| {
            println!("Hello from broadcast thread {}", context.index());
        });
    });

    pool.join(test, test);
}
