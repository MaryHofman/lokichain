use std::collections::HashMap;
use thiserror::Error;


fn format_error_map(errors: &HashMap<String, String>) -> String {
    let mut entries = errors.iter().collect::<Vec<_>>();
    entries.sort_by(|a, b| a.0.cmp(b.0));

    let formatted_entries: Vec<String> = entries
        .iter()
        .map(|(key, value)| format!("{}: {}", key, value))
        .collect();

    format!("[{}]", formatted_entries.join(", "))
}

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("Invalid data: {}", format_error_map(.0))]
    InvalidData(HashMap<String, String>),
    #[error("Not found: {0}")]
    NotFound(String),
    #[error(transparent)]
    Other(#[from] anyhow::Error)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_error_map() {
        let mut map = HashMap::new();
        map.insert("field1".to_string(), "error1".to_string());
        map.insert("field2".to_string(), "error2".to_string());

        let formatted = format_error_map(&map);
        assert_eq!(formatted, "[field1: error1, field2: error2]");
    }
}
