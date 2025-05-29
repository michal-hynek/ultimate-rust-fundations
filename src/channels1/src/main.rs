use std::sync::mpsc;

enum Command {
    SayHello,
    Quit,
}

fn main() {
    let (tx, rx) = mpsc::channel::<Command>();

    let thread_handle = std::thread::spawn(move || {
        while let Ok(command) = rx.recv() {
            match command {
                Command::SayHello => println!("Hello"),
                Command::Quit => {
                    println!("Quitting");
                    break;
                }
            }
        }
    });

    println!("Sending SayHello 10x");
    (0..10).for_each(|_| tx.send(Command::SayHello).unwrap());

    println!("Sending Quit");
    tx.send(Command::Quit).unwrap();

    thread_handle.join().unwrap();
}
