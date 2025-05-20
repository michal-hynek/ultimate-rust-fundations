use std::thread;

fn main() {
    const N_THREADS: usize = 8;

    let to_add: Vec<i32> = (0..5000).collect();
    let chunks = to_add.chunks(N_THREADS);
    let mut thread_handles = Vec::with_capacity(chunks.len());

    for chunk in chunks {
        let chunk = chunk.to_owned();
        let handle = thread::spawn(move || {
            chunk.iter().sum::<i32>()
        });

        thread_handles.push(handle);
    }

    let mut sum = 0;
    for handle in thread_handles {
        sum += handle.join().unwrap();
    }

    println!("Sum is {sum}");
}
