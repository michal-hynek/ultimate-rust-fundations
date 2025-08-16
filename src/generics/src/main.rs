use std::fmt::Debug;

fn just_print_it<T: ToString + Debug>(x: T) {
    println!("{}", x.to_string());
}

fn main() {
    just_print_it(5);
    just_print_it("Hello, world!");
}
