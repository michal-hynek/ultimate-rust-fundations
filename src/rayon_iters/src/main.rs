use rayon::prelude::*;

fn main() {
    let numbers = (0..1_000_000).collect::<Vec<u64>>();
    let sum = numbers.par_iter().sum::<u64>();
    println!("The sum is {sum}");
}
