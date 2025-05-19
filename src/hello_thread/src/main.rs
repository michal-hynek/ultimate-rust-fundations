fn hello_thread() {
    println!("Hello from thread");

}

fn main() {
    println!("Hello from main");

    let thread_handler = std::thread::spawn(hello_thread);
    thread_handler.join().unwrap();
}
