#[derive(PartialEq, Debug)]
pub enum LoginAction {
    Granted(LoginRole),
    Denied,
}

#[derive(PartialEq, Debug)]
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

pub fn login(username: &str, password: &str) -> Option<LoginAction> {
    let username = username.to_lowercase();

    if username != "admin" && username != "bob" {
        return None;
    }

    if username == "admin" && password == "password" {
        Some(LoginAction::Granted(LoginRole::Admin))
    } else if username == "bob" && password == "password" {
        Some(LoginAction::Granted(LoginRole::User))
    } else {
        Some(LoginAction::Denied)
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
