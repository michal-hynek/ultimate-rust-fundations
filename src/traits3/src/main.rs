use std::any::Any;

struct Tortoise;

trait Animal {
    fn speak(&self);
}

impl Animal for Tortoise {
    fn speak(&self) {
        println!("What noise does a tortoise make anyway?");
    }
}

trait DowncastableAnimal {
    fn as_any(&self) -> &dyn Any;
}

impl DowncastableAnimal for Tortoise {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

fn main() {
    let animals: Vec<Box<dyn DowncastableAnimal>> = vec![Box::new(Tortoise)];

    for animal in animals.iter() {
        if let Some(animal) = animal.as_any().downcast_ref::<Tortoise>() {
            println!("We have access to the tortoise");
            animal.speak();
        }
    }
}