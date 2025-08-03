struct MyStruct {
    n: i32,
}

impl MyStruct {
    fn new(n: i32) -> Self {
        println!("Constructing MyStruct{n}");
        Self { n }
    }
}

impl Drop for MyStruct {
    fn drop(&mut self) {
        println!("Dropping {}", self.n);
    }
}

fn move_me(x: MyStruct) {
    println!("moved {}", x.n);
}

fn main() {
    let _x = MyStruct::new(1);

    {
        let _y = MyStruct::new(2);
        println!("Scope ends");
    }

    let z = MyStruct::new(3);
    move_me(z);
    println!("move_me() returned");

    println!("Ending the main function");
}
