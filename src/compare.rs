use std::collections::HashSet;
use std::io::{BufRead, Result as IoResult};

/// Extracts all valid environment variable keys defined in a reader.
pub fn extract_keys<R: BufRead>(reader: R) -> IoResult<Vec<String>> {
    let mut keys = Vec::new();
    let mut seen = HashSet::new();

    for line_res in reader.lines() {
        let line = line_res?;
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        if let Some((key, _)) = line.split_once('=') {
            let key = key.trim().to_string();
            if !key.is_empty() && !key.contains(char::is_whitespace) && seen.insert(key.clone()) {
                keys.push(key);
            }
        }
    }

    Ok(keys)
}

/// Identifies which keys from the template are missing in the local keys collection.
pub fn find_missing_keys(template_keys: &[String], local_keys: &HashSet<String>) -> Vec<String> {
    template_keys
        .iter()
        .filter(|k| !local_keys.contains(*k))
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_extract_keys() {
        let sample = "\
# Template file
PORT=8080
DATABASE_URL=
# Another comment
SECRET_KEY=sample

EMPTY_VAL=
";
        let cursor = Cursor::new(sample);
        let keys = extract_keys(cursor).unwrap();
        assert_eq!(keys, vec!["PORT", "DATABASE_URL", "SECRET_KEY", "EMPTY_VAL"]);
    }

    #[test]
    fn test_find_missing_keys() {
        let template_keys = vec![
            "PORT".to_string(),
            "DATABASE_URL".to_string(),
            "SECRET_KEY".to_string(),
        ];

        let mut local_keys = HashSet::new();
        local_keys.insert("PORT".to_string());
        local_keys.insert("SECRET_KEY".to_string());

        let missing = find_missing_keys(&template_keys, &local_keys);
        assert_eq!(missing, vec!["DATABASE_URL".to_string()]);
    }
}
