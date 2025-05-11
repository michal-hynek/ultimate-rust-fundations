fn greet(name: String) {
    println!("Hello {name}");
}

fn greet2(name: String) -> String {
    println!("Hello {name}");
    name
}

fn greet3(name: &str) {
    println!("Hello {name}");
}

fn greet4(name: &mut String) {
    *name = format!("Hello {name}");
    println!("{name}");
}

fn main() {
    let name = String::from("Alice");
    greet(name.clone());
    greet(name);

    let mut name = String::from("Bob");
    name = greet2(name);
    greet2(name);

    let name = String::from("Eve");
    greet3(&name);
    greet3(&name);

    let mut name = String::from("Dave");
    greet4(&mut name);
    println!("{name}");
}
