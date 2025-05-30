use std::sync::mpsc;

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Command<Job> {
    Run(Job),
    Quit,
}

fn hi_there() {
    println!("Hi there!");
}

fn main() {
    let (tx, rx) = mpsc::channel::<Command<Job>>();

    let thread_handle = std::thread::spawn(move || {
        while let Ok(command) = rx.recv() {
            match command {
                Command::Run(job) => job(),
                Command::Quit => break,
            }
        }
    });

    let job1 = || println!("Hello from closure");
    let job2 = || {
        (0..10).for_each(|i| println!("{i}"));
    };

    tx.send(Command::Run(Box::new(job1))).unwrap();
    tx.send(Command::Run(Box::new(job2))).unwrap();
    tx.send(Command::Run(Box::new(hi_there))).unwrap();
    tx.send(Command::Quit).unwrap();

    thread_handle.join().unwrap();
}
