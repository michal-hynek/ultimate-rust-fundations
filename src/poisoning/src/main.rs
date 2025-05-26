use std::sync::Mutex;

static MY_SHARED: Mutex<u32> = Mutex::new(3);

fn poisoner() {
    let mut lock = MY_SHARED.lock().unwrap();
    *lock += 1;
    panic!("Poisoner crashed");
}

fn main() {
    let thread_handle = std::thread::spawn(poisoner);
    println!("trying to join the thread: {:?}", thread_handle.join());

    let lock = MY_SHARED.lock();
    println!("trying to acquire the lock {lock:?}");
    std::mem::drop(lock);

    let recovered_data= MY_SHARED.lock().unwrap_or_else(|poisoned| {
        println!("Mutex was poisoned, recovering data...");
        poisoned.into_inner()
    });

    println!("{recovered_data}");
}
