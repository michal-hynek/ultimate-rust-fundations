static mut COUNTER: i32 = 0;

fn main() {
    let mut thread_handles = Vec::with_capacity(1000);

    for _ in 0..1000 {
        let handle = std::thread::spawn(|| {
            for _ in 0..1_100 {
                unsafe {
                    COUNTER += 1;
                }
            }
        });

        thread_handles.push(handle)
    }

    thread_handles.into_iter().for_each(|handle| handle.join().unwrap());

    unsafe {
        let result = COUNTER;
        println!("{result}");
    }
}
