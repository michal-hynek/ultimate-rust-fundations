use std::{sync::RwLock, time::Duration};
use once_cell::sync::Lazy;

static USERS: Lazy<RwLock<Vec<String>>> = Lazy::new(|| RwLock::new(build_users()));

fn build_users() -> Vec<String> {
    vec!["Alice".to_string(), "Bob".to_string()]
}

fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("error when reading from stdin");
    input.trim().to_string()
}

fn main() {
    std::thread::spawn(|| {
        loop {
            println!("Current users {:?}", USERS.read().unwrap());
            std::thread::sleep(Duration::from_secs(3));
        }
    });

    loop {
        println!("Enter new user (q to quit)");

        let input = read_line();

        if input == "q" {
            break;
        }

        let mut lock = USERS.write().unwrap();
        lock.push(input);
    }
}
