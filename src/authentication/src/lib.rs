use std::{collections::HashMap};
use std::fs;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub password: String,
    pub role: LoginRole,
}

impl User {
    fn new(username: &str, password: &str, role: LoginRole) -> Self {
        Self {
            username: username.to_lowercase(),
            password: hash_password(&password),
            role,
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum LoginAction {
    Granted(LoginRole),
    Denied,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum LoginRole {
    Admin,
    User,
}

pub fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("failed to read from stdin");

    input.trim().to_string()
}

fn hash_password(password: &str) -> String {
    use sha2::{Sha512, Digest};

    let mut hasher = Sha512::new();
    hasher.update(password);

    format!("{:X}", hasher.finalize())
}

pub fn get_users() -> HashMap<String, User> {
    if fs::exists("users.json").unwrap() {
        let users_json = fs::read_to_string("users.json").unwrap();
        let users: HashMap<String, User> = serde_json::from_str(&users_json).unwrap();

        users
    } else {
        let users = get_default_users();
        let users_json = serde_json::to_string_pretty(&users).unwrap();
        fs::write("users.json", users_json).unwrap();

        users
    }
}

fn get_default_users() -> HashMap<String, User> {
    let mut users = HashMap::new();

    users.insert(
        "admin".to_string(),
        User::new("admin", "password", LoginRole::Admin),
    );
    users.insert(
        "bob".to_string(),
        User::new("bob", "password", LoginRole::User),
    );

    users
}

fn get_admin_usernames() -> Vec<String> {
    get_users().into_iter()
        .filter(|u| u.1.role == LoginRole::Admin)
        .map(|u| u.1.username)
        .collect()
}

pub fn login(username: &str, password: &str) -> Option<LoginAction> {
    let username = username.to_lowercase();

    if let Some(user) = get_users().get(&username) {
        if user.password == hash_password(password) {
            Some(LoginAction::Granted(user.role.clone()))
        } else {
            Some(LoginAction::Denied)
        }
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn login_returns_granted_for_correct_credentials() {
        let login_action = login("admin", "password").unwrap();
        assert_eq!(login_action, LoginAction::Granted(LoginRole::Admin));
    }

    #[test]
    fn login_username_check_is_case_insensitive() {
        let login_action = login("BOB", "password").unwrap();
        assert_eq!(login_action, LoginAction::Granted(LoginRole::User));
    }

    #[test]
    fn login_returns_denied_if_password_is_incorrect() {
        let login_action = login("admin", "password2").unwrap();
        assert_eq!(login_action, LoginAction::Denied);
    }

    #[test]
    fn login_returns_none_if_username_does_not_exist() {
        let login_action_option = login("admin2", "password");
        assert_eq!(login_action_option, None);
    }
}
