use std::fmt::Debug;

trait Animal: Debug {
    fn speak(&self);
}

#[derive(Debug)]
struct Cat;

impl Animal for Cat {
    fn speak(&self) {
        println!("Meow");
    }
}

#[derive(Debug)]
struct Dog;

impl Animal for Dog {
    fn speak(&self) {
        println!("Woof");
    }
}

fn speak_twice(animal: &impl Animal) {
    animal.speak();
    animal.speak();
}

fn get_animal() -> impl Animal {
    Cat
}

fn main() {
    let cat = Cat;
    cat.speak();
    speak_twice(&cat);

    let dog = Dog;
    let _animals: Vec<Box<dyn Animal>> = vec![Box::new(cat), Box::new(dog)];
}
