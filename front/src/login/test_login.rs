#[cfg(test)]
mod tests {
    use super::super::login::_get_token;

    #[test]
    fn ensure_get_token_sends_request() {
        let username = "valid_username";
        let password = "valid_password";
        let result = _get_token(username, password);
        assert_eq!(result, true);
    }
}