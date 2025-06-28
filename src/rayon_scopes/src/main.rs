fn main() {
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(4)
        .build()
        .unwrap();

    pool.spawn(|| println!("Hello from pool thread"));

    pool.scope(|scope| {
        for i in 0..20 {
            scope.spawn(move |_| println!("Hello from scoped thread {i}"));
        }
    });

    println!("Hello from the main thread")
}
