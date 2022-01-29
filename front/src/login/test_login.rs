#[cfg(test)]
mod tests {
    use super::super::login::login;

    #[test]
    fn it_works() {
        let username = "valid_username";
        let password = "valid_password";
        let result = login(username, password);
        assert_eq!(result, true);
    }
}