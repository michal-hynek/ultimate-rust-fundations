use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

// Taken from https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/read_lines.html
// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

async fn count_lines(filename: &str) -> anyhow::Result<u32> {
    let now = Instant::now();
    let mut lines_count = 0;

    println!("Reading {filename}");
    let lines = read_lines(filename)?;
    lines.for_each(|line| {
        if let Ok(line) = line {
            if !line.trim().is_empty() {
                lines_count += 1;
            }
        }
    });

    println!("Read {lines_count} in {:.3} seconds", now.elapsed().as_secs_f32());
    Ok(lines_count)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let filename = "../warandpeace.txt";

    let now = Instant::now();
    let (count1, count2) = tokio::join!(
        count_lines(filename),
        count_lines(filename),
    );

    println!("Total lines {}", count1? + count2?);
    println!("In {:.3} seconds", now.elapsed().as_secs_f32());

    Ok(())
}
