use std::path::Path;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct User {
    username: String,
}

fn maybe_read_a_file(path: &Path) -> anyhow::Result<String> {
    let contents = std::fs::read_to_string(path)?;
    Ok(contents)
}

fn load_users(path: &Path) -> anyhow::Result<Vec<User>> {
    let raw_text = maybe_read_a_file(path)?;
    let users = serde_json::from_str(&raw_text)?;
    Ok(users)
}

fn main() -> anyhow::Result<()> {
    let usernames = load_users(Path::new("users.json"))?
        .into_iter()
        .map(|u| u.username)
        .collect::<Vec<_>>();

    println!("Users: {usernames:?}");

    Ok(())
}
