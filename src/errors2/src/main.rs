use std::path::Path;

fn maybe_read_a_file(path: &Path) -> Result<String, std::io::Error> {
    std::fs::read_to_string(path)
}

fn file_to_upper_case(path: &Path) -> Result<String, std::io::Error> {
    let contents = maybe_read_a_file(path)?;
    Ok(contents.to_uppercase())
}

fn main() {
    let path = Path::new("my_file.txt");
    if let Ok(contents) = file_to_upper_case(path) {
        println!("Contents: {contents}");
    } else {
        println!("Error reading {path:?}");
    }
}
