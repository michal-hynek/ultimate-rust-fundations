fn hello_thread(i: u32) {
    println!("Hello from {}", i);
}

fn main() {
    const NTHREADS: u32 = 5;

    println!("Hello from main");

    let mut thread_handles = Vec::with_capacity(NTHREADS as usize);
    for i in 0..NTHREADS {
        let handle = std::thread::spawn(move || hello_thread(i));
        thread_handles.push(handle);
    }

    for handle in thread_handles {
        handle.join().unwrap();
    }
}
