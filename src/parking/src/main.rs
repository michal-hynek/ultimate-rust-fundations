fn parkable_thread(n: u32) {
    loop {
        println!("parking thread {n}");
        std::thread::park();
        println!("thread {n} unparked");
    }
}

fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    let threads = (0..10).map(|n| {
        std::thread::spawn(move || parkable_thread(n))
    }).collect::<Vec<_>>();

    loop {
        println!("Thread to unpark (q to quit)");
        let input = read_line();

        if input == "q" {
            break;
        }

        if let Ok(n) = input.parse::<usize>() {
            if n < 10 {
                threads[n].thread().unpark();
            }
        }
    }
}
