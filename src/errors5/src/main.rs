use std::path::Path;

use serde::Deserialize;

use thiserror::Error;

#[derive(Debug, Error)]
enum UsersError {
    #[error("No users found")]
    NoUsers,
    #[error("Too many users")]
    TooManyUsers,
}

#[derive(Deserialize, Debug)]
struct User {
    username: String,
}

fn maybe_read_a_file(path: &Path) -> anyhow::Result<String> {
    let contents = std::fs::read_to_string(path)?;
    Ok(contents)
}

fn load_users(path: &Path) -> Result<Vec<User>, UsersError> {
    let raw_text = maybe_read_a_file(path)
        .map_err(|_| UsersError::NoUsers)?;
    let users = serde_json::from_str(&raw_text)
        .map_err(|_| UsersError::NoUsers)?;
    Ok(users)
}

fn main() -> Result<(), UsersError> {
    let usernames = load_users(Path::new("users.json"))?
        .into_iter()
        .map(|u| u.username)
        .collect::<Vec<_>>();

    println!("Users: {usernames:?}");

    Ok(())
}
