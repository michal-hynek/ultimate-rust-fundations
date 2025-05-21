use std::thread;

fn my_thread() {
    println!("Hello from {}", thread::current().name().unwrap());
}

fn main() {
    let thread_handle = thread::Builder::new()
        .name("thread 1".to_string())
        .stack_size(size_of::<usize>() * 4)
        .spawn(my_thread)
        .unwrap();

    thread_handle.join().unwrap();
}
