use std::time::Instant;

use rayon::prelude::*;

fn is_prime(n: u32) -> bool {
    n > 1 && (2..=n/2).into_par_iter().all(|i| n % i != 0)
}

fn main() {
    let numbers: Vec<u32> = (0..1_000).collect();
    let now = Instant::now();

    let mut primes = numbers.into_par_iter()
        .filter(|n| is_prime(*n))
        .collect::<Vec<_>>();

    let elapsed = now.elapsed().as_secs_f64();

    println!("Found {} primes in {elapsed} seconds", primes.len());

    primes.par_sort_unstable();
    println!("{primes:#?}");
}
