pub struct User {
    pub username: String,
    pub password: String,
    pub role: LoginRole,
}

impl User {
    fn new(username: &str, password: &str, role: LoginRole) -> Self {
        Self {
            username: username.to_lowercase(),
            password: password.to_string(),
            role,
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum LoginAction {
    Granted(LoginRole),
    Denied,
}

#[derive(PartialEq, Debug, Clone)]
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

fn get_users() -> Vec<User> {
    vec![
        User::new("admin", "password", LoginRole::Admin),
        User::new("bob", "password", LoginRole::User),
    ]
}

fn get_admin_usernames() -> Vec<String> {
    get_users().into_iter()
        .filter(|u| u.role == LoginRole::Admin)
        .map(|u| u.username)
        .collect()
}

pub fn login(username: &str, password: &str) -> Option<LoginAction> {
    let username = username.to_lowercase();

    if let Some(user) = get_users().iter().find(|u| u.username == username) {
        if user.password == password {
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
