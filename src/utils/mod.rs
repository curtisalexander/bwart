pub mod google;

pub fn get_command_from_query_string(query_string: &str) -> &str {
    if query_string.contains(' ') {
        let index_of_whitespace = query_string.find(' ').unwrap_or(0);
        return &query_string[..index_of_whitespace];
    }

    &query_string
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_command_from_query_string_no_whitespace() {
        // Test with command only
        let actual = get_command_from_query_string("gh");
        let expected = "gh";
        assert_eq!(actual, expected);
    }
    #[test]
    fn test_get_command_from_query_string_with_whitespace() {
        // Test with command only
        let actual = get_command_from_query_string("gh @curtisalexander");
        let expected = "gh";
        assert_eq!(actual, expected);
    }
}
