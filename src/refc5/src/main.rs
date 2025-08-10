use std::{cell::RefCell, rc::Rc};

struct MyData {
    data: RefCell<String>,
}

impl MyData {
    fn new(s: &str) -> Self {
        Self {
            data: RefCell::new(s.to_string()),
        }
    }
}

fn move_data(x: Rc<MyData>) {
    let mut data = x.data.borrow_mut();
    data.push_str(" World");
}

fn main() {
    let my_shared = Rc::new(MyData::new("Hello"));
    move_data(my_shared.clone());
    let data = my_shared.data.borrow();
    println!("{data}");
}
