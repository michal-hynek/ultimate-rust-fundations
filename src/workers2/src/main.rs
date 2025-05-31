use std::{collections::VecDeque, sync::Mutex, time::Duration};

use once_cell::sync::Lazy;


static WORK_QUEUE: Lazy<Mutex<VecDeque<String>>> = Lazy::new(|| Mutex::new(VecDeque::new()));

fn main() {
    let cpu_count = num_cpus::get();
    let mut thread_handles = Vec::with_capacity(cpu_count);
    let mut broadcast = Vec::with_capacity(cpu_count);

    println!("CPU count is {cpu_count}");

    for cpu in 0..cpu_count {
        let (tx, rx) = std::sync::mpsc::channel::<()>();
        broadcast.push(tx);

        let handle = std::thread::spawn(move || {
            while rx.recv().is_ok() {
                let mut lock = WORK_QUEUE.lock().unwrap();

                if let Some(work) = lock.pop_front() {
                    std::mem::drop(lock);
                    println!("CPU {cpu} got {work}");
                    std::thread::sleep(Duration::from_secs(2));
                    println!("CPU {cpu} finished");
                } else {
                    println!("CPU {cpu} found no work");
                }
            }
        });

        thread_handles.push(handle);
    }

    loop {
        let sent = {
            let mut lock = WORK_QUEUE.lock().unwrap();
            let queue_len = lock.len();

            println!("Queue length {queue_len}");

            if queue_len < 5 {
                println!("Adding work to queue");
                lock.push_back("Hello".to_string());
                true
            } else {
                false
            }
        };

        if sent {
            println!("Broadcasting queue update");
            broadcast.iter().for_each(|tx| tx.send(()).unwrap());
        }

        std::thread::sleep(Duration::from_secs(1));
    }
}
