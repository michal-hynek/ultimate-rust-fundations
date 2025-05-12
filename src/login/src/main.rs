use authentication::{login, read_line};

fn main() {
    let mut tries = 0;
    let max_tries = 3;

    loop {
        println!("Username: ");
        let username = read_line();
        println!("Password: ");
        let password = read_line();

        if login(&username, &password) {
            println!("Welcome");
            break;
        } else {
            println!("Incorrect credentials");
            tries += 1;

            if tries >= max_tries {
                println!("Too many login attempts");
                break;
            }
        }
    }
}
