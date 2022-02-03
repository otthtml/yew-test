#[cfg(test)]
mod tests {
    use super::super::login::check_credentials;

    #[test]
    fn it_works() {
        let username = "valid_username";
        let password = "valid_password";
        let result = check_credentials(username, password);
        assert_eq!(result, true);
    }
}