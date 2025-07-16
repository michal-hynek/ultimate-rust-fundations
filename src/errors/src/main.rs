use std::path::Path;

fn main() {
    let my_file = Path::new("invalid.txt");

    match std::fs::read_to_string(my_file) {
        Ok(content) => println!("{content}"),
        Err(e) => {
            match e.kind() {
                std::io::ErrorKind::NotFound => println!("File {my_file:?} not found"),
                _ => println!("Error when reading {my_file:?} - {e:?}"),
            }
        },
    }
}
