use std::path::Path;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct User {
    username: String,
}

type GenericResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn load_users(path: &Path) -> GenericResult<Vec<User>> {
    let raw_text = std::fs::read_to_string(path)?;
    let users = serde_json::from_str(&raw_text)?;

    Ok(users)
}

fn main() -> GenericResult<()> {
    let path = Path::new("users.json");
    let usernames = load_users(path)?
        .into_iter()
        .map(|u| u.username)
        .collect::<Vec<_>>();

    println!("{usernames:#?}");

    Ok(())
}
