pub fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("failed to read from stdin");

    input.trim().to_string()
}

pub fn login(username: &str, password: &str) -> bool {
    username.to_lowercase() == "admin" && password == "password"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn login_returns_true_for_correct_credentials() {
        assert!(login("admin", "password"));
    }

    #[test]
    fn login_username_check_is_case_insensitive() {
        assert!(login("ADMIN", "password"));
    }

    #[test]
    fn login_returns_false_if_username_is_incorrect() {
        assert!(!login("admin2", "password"));
    }

    #[test]
    fn login_returns_false_if_password_is_incorrect() {
        assert!(!login("admin", "password2"));
    }

    #[test]
    fn login_returns_false_if_username_and_password_are_incorrect() {
        assert!(!login("admin2", "password2"));
    }
}
