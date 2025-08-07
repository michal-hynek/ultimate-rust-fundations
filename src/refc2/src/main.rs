use std::sync::Arc;

#[derive(Debug)]
struct Droppable(i32);

impl Droppable {
    fn new(n: i32) -> Self {
        println!("Constructing {n}");
        Self(n)
    }
}

impl Drop for Droppable {
    fn drop(&mut self) {
        println!("Dropping {}", self.0);
    }
}

fn move_me(x: Arc<Droppable>) {
    println!("Moved {}", x.0);
}

fn main() {
    let my_shared = Arc::new(Droppable::new(1));

    let mut threads = Vec::with_capacity(10);
    for i in 0..10 {
        let my_clone = my_shared.clone();

        let thread = std::thread::spawn(move || {
            move_me(my_clone);
        });

        threads.push(thread);
    }

    for handle in threads {
        handle.join().unwrap();
    }

    println!("{my_shared:?}");
    println!("Application exit");
}