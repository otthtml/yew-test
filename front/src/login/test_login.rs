#[cfg(test)]
mod tests {
    use super::super::login::_check_credentials;

    #[test]
    fn it_works() {
        let username = "valid_username";
        let password = "valid_password";
        let result = _check_credentials(username, password);
        assert_eq!(result, true);
    }
}