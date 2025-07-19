use std::time::Instant;

async fn async_count_lines(filename: &str) -> anyhow::Result<usize> {
    use tokio::io::AsyncBufReadExt;
    use tokio::io::BufReader;
    use tokio::fs::File;

    let now = Instant::now();
    let mut line_count = 0;

    println!("Reading {filename}");
    let file = File::open(filename).await?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    while let Some(line) = lines.next_line().await? {
        if !line.trim().is_empty() {
            line_count += 1;
        }
    }

    println!("Read {} lines in {:.3} seconds", line_count, now.elapsed().as_secs_f32());
    Ok(line_count)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let filename = "../warandpeace.txt";

    let now = Instant::now();
    let (count1, count2) = tokio::join!(
        async_count_lines(filename),
        async_count_lines(filename),
    );

    println!("Total lines {}", count1? + count2?);
    println!("In {:.3} seconds", now.elapsed().as_secs_f32());

    Ok(())
}
