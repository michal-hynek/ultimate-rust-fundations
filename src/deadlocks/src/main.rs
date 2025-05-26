use std::sync::Mutex;

fn main() {
    // deadlock - the second lock() call will wait until the previous lock goes out of scope
    /*let my_shared = Mutex::new(0);
    let lock = my_shared.lock().unwrap();
    let lock = my_shared.lock().unwrap();*/

    // no deadllock becase the inner lock goes out of scope before the next lock
    /*let my_shared = Mutex::new(0);
    {
        let lock = my_shared.lock().unwrap();
    }

    let lock = my_shared.lock().unwrap();*/

    // no deadlock becase the first lock variable is explicitly dropped
    let my_shared = Mutex::new(0);
    let lock = my_shared.lock().unwrap();
    std::mem::drop(lock);
    let _lock = my_shared.lock().unwrap();

    let my_shared = Mutex::new(0);
    let _lock = my_shared.lock().unwrap();

    if let Ok(_lock) = my_shared.try_lock() {
        println!("got the lock");
    } else {
        println!("getting the lock failed");
    }
}
