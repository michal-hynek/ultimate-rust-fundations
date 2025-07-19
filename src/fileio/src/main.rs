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

fn main() {
    let now = Instant::now();
    let mut lines_count = 0;

    if let Ok(lines) = read_lines("../warandpeace.txt") {
        lines.for_each(|line| {
            if let Ok(line) = line {
                if !line.trim().is_empty() {
                    lines_count += 1;
                }
            }
        });
    }

    println!("Read {lines_count} in {:.3} seconds", now.elapsed().as_secs_f32());
}
