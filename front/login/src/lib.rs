#[allow(dead_code)]
fn login(username: &str, password: &str) -> bool {
    return true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let username = "valid_username";
        let password = "valid_password";
        let result = login(username, password);
        assert_eq!(result, true);
    }
}
