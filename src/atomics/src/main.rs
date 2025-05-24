use std::sync::atomic::AtomicI32;

static COUNTER: AtomicI32 = AtomicI32::new(0);

fn main() {
    let thread_count = 1000;
    let mut thread_handles = Vec::with_capacity(thread_count);

    for _ in 0..thread_count {
        let handle = std::thread::spawn(|| {
            for _ in 0..1_1000 {
                COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            }
        });

        thread_handles.push(handle);
    }

    thread_handles.into_iter().for_each(|handle| handle.join().unwrap());

    println!("{}", COUNTER.load(std::sync::atomic::Ordering::Relaxed));
}
