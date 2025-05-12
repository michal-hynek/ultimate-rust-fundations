fn read_line() -> String {
    let mut input= String::new();
    std::io::stdin().read_line(&mut input).expect("failed to read user input");

    input.trim().to_string()
}

fn main() {
    let input = read_line();
    println!("User typed [{input}]");
}
