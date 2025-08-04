struct MyStruct {
    n: i32,
}

impl MyStruct {
    fn new(n: i32) -> Self {
        println!("Construct MyStruct {n}");
        Self { n }
    }
}

impl Drop for MyStruct {
    fn drop(&mut self) {
        println!("Dropping MyStruct {}", self.n);
    }
}

struct HasDropables {
    id: i32,
    x: MyStruct,
}

impl HasDropables {
    fn new(id: i32) -> Self {
        HasDropables{ id, x: MyStruct::new(id+100) }
    }
}

fn main() {
    let _x = HasDropables::new(1);
    println!("Ending the main function");
}
