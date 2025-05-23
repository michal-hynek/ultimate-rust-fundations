use std::thread;

fn main() {
    const N_THREADS: usize = 8;

    let to_add: Vec<i32> = (0..5000).collect();
    let chunks = to_add.chunks(to_add.len() / N_THREADS);

    let sum = thread::scope(|s| {
        let mut thread_handles = Vec::new();

        for chunk in chunks {
            let handle = s.spawn(|| {
                chunk.iter().sum::<i32>()
            });

            thread_handles.push(handle);
        }

        let mut sum = 0;
        for handle in thread_handles {
            sum += handle.join().unwrap();
        }

        sum
    });

    println!("Sum is {sum}");
}
