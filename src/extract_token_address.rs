use regex::Regex;

pub fn extract_solana_addresses(text: &str) -> (Vec<String>) {
    let pattern = Regex::new(r"\b[a-zA-Z0-9]{44}\b").unwrap();

    let matches: Vec<String> = pattern.find_iter(text).map(|m| m.as_str().to_string()).collect();

    matches
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_solana_addresses() {
        let input = "
            DijafwofOIJIFJWOIIWFIWOFJFJWF
            fewfkwwepfwkweff
            dqwkdqdqe21r-3kkrr09kr290k90dsad
            frhb8l7y9qq41qzxyltc2nw8an1rjfllxrf2x9rwllmo
            dpqwdwqodqdqw
            wd
        ";

        let expected_addresses = vec![
            "frhb8l7y9qq41qzxyltc2nw8an1rjfllxrf2x9rwllmo".to_string(),
        ];

        let result = extract_solana_addresses(input);
        assert_eq!(result, expected_addresses);
    }

    #[test]
    fn test_no_addresses() {
        let input = "
            This is a test with no valid Solana addresses!
        ";

        let expected_addresses: Vec<String> = vec![];

        let result = extract_solana_addresses(input);
        assert_eq!(result, expected_addresses);
    }

    #[test]
    fn test_multiple_addresses() {
        let input = "
            frhb8l7y9qq41qzxyltc2nw8an1rjfllxrf2x9rwllmo
            frhb8l7y9qq41qzxyltc2nw8an1rjfllxrf2x9rwllmo
        ";

        let expected_addresses = vec![
            "frhb8l7y9qq41qzxyltc2nw8an1rjfllxrf2x9rwllmo".to_string(),
            "frhb8l7y9qq41qzxyltc2nw8an1rjfllxrf2x9rwllmo".to_string(),
        ];

        let result = extract_solana_addresses(input);
        assert_eq!(result, expected_addresses);
    }
}
