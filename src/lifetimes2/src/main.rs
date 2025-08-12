struct Cat(String);

impl Cat {
    fn feed(&mut self) {
        self.0 = format!("{} (purring)", self.0);
    }
}

struct CatFeeder<'a> {
    cat: &'a mut Cat,
}

impl<'a> CatFeeder<'a> {
    fn feed(&mut self) {
        self.cat.feed();
    }
}

fn main() {
    let mut feeders = Vec::new();
    let mut cats = vec![
        Cat("Frodo".to_string()),
        Cat("Bilbo".to_string()),
        Cat("Pippin".to_string()),
    ];

    for cat in cats.iter_mut() {
        feeders.push(CatFeeder{ cat });
    }

    feeders.iter_mut().for_each(|f| f.feed());
}
