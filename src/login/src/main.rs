use authentication::{login, read_line, LoginAction};

fn main() {
    let mut tries = 0;
    let max_tries = 3;

    loop {
        println!("Username: ");
        let username = read_line();
        println!("Password: ");
        let password = read_line();

        match login(&username, &password) {
            Some(LoginAction::Granted(_)) => {
                println!("Welcome");
                break;
            },

            Some(LoginAction::Denied) => {
                println!("Incorrect password");
            },

            None => {
                println!("Incorrect username");
            }
        }

        tries += 1;

        if tries >= max_tries {
            println!("Too many login attempts");
            break;
        }
    }
}
