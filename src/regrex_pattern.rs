use regex::Regex;

pub fn validate_and_create_regex(pattern: &str) -> Result<Regex, String> {
    Regex::new(pattern).map_err(|e| format!("Invalid regex pattern: {}", e))
}