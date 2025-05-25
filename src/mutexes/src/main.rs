use std::thread;
use std::sync::Mutex;

static NUMBERS: Mutex<Vec<u32>> = Mutex::new(Vec::new());

fn main() {
    let mut thread_handles = Vec::with_capacity(10);

    for _ in 0..10 {
        let handle = thread::spawn(|| {
            let mut lock = NUMBERS.lock().unwrap();
            lock.push(1);
        });

        thread_handles.push(handle);
    }

    thread_handles.into_iter().for_each(|handle| handle.join().unwrap());

    let lock = NUMBERS.lock().unwrap();
    println!("{:#?}", lock);
}
