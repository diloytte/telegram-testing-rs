use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct UserData {
    usernames_to_track: Vec<String>,
}

pub fn get_special_usernames(file_path: &str) -> Vec<String> {
    let file_content = fs::read_to_string(file_path).expect("Failed to read file");
    let data: UserData = serde_json::from_str(&file_content).expect("Failed to parse JSON");
    data.usernames_to_track
}



#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_get_special_usernames() {
        let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");

        let test_data = r#"
        {
            "usernames_to_track": ["alice", "bob", "charlie"]
        }"#;

        temp_file.write_all(test_data.as_bytes()).expect("Failed to write test data");

        let file_path = temp_file.path().to_str().unwrap();

        let result = get_special_usernames(file_path);

        let expected = vec!["alice".to_string(), "bob".to_string(), "charlie".to_string()];

        assert_eq!(result, expected);
    }
}
