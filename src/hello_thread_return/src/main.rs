fn do_math(i: u32) -> u32 {
    let mut n = i+1;

    for _ in 0..10 {
        n *= 2
    }

    n
}

fn main() {
    const NTHREADS: u32 = 5;

    println!("Hello from main");

    let mut thread_handles = Vec::with_capacity(NTHREADS as usize);
    for i in 0..NTHREADS {
        let handle = std::thread::spawn(move || do_math(i));
        thread_handles.push(handle);
    }

    for handle in thread_handles {
        let result= handle.join().unwrap();
        println!("{result}");
    }
}
