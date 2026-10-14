use colored::Colorize;
use crate::mask::{mask_value, strip_surrounding_quotes};

#[derive(Debug, PartialEq, Eq)]
pub enum ParsedLine {
    Empty(String),
    Comment(String),
    KeyValue {
        key: String,
        masked_value: String,
    },
    Malformed {
        raw: String,
        reason: String,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub struct LintError {
    pub line_number: usize,
    pub raw: String,
    pub reason: String,
}

/// Parses a single line from an environment file.
pub fn parse_line(raw_line: &str, line_number: usize) -> (ParsedLine, Option<LintError>) {
    // 1. Whitespace preservation: preserve completely empty or whitespace-only lines untouched
    if raw_line.trim().is_empty() {
        return (ParsedLine::Empty(raw_line.to_string()), None);
    }

    // 2. Comment bypass: lines starting with '#' (including indented comments)
    if raw_line.trim_start().starts_with('#') {
        return (ParsedLine::Comment(raw_line.to_string()), None);
    }

    // 3. First-match splitting on '='
    match raw_line.split_once('=') {
        Some((raw_key, raw_val)) => {
            // Check for malformed keys (e.g. empty key or spaces in keys)
            if raw_key.is_empty() {
                let err = LintError {
                    line_number,
                    raw: raw_line.to_string(),
                    reason: "Key cannot be empty before '='".to_string(),
                };
                return (
                    ParsedLine::Malformed {
                        raw: raw_line.to_string(),
                        reason: err.reason.clone(),
                    },
                    Some(err),
                );
            }

            if raw_key.contains(char::is_whitespace) {
                let err = LintError {
                    line_number,
                    raw: raw_line.to_string(),
                    reason: format!("Key '{}' contains illegal spaces", raw_key),
                };
                return (
                    ParsedLine::Malformed {
                        raw: raw_line.to_string(),
                        reason: err.reason.clone(),
                    },
                    Some(err),
                );
            }

            // 4. Quote stripping and smart masking
            let stripped_val = strip_surrounding_quotes(raw_val.trim());
            let masked = mask_value(stripped_val);

            (
                ParsedLine::KeyValue {
                    key: raw_key.to_string(),
                    masked_value: masked,
                },
                None,
            )
        }
        None => {
            // Missing '=' delimiter in a non-empty, non-comment line
            let err = LintError {
                line_number,
                raw: raw_line.to_string(),
                reason: "Missing '=' delimiter".to_string(),
            };
            (
                ParsedLine::Malformed {
                    raw: raw_line.to_string(),
                    reason: err.reason.clone(),
                },
                Some(err),
            )
        }
    }
}

/// Renders a parsed line into styled terminal output using the syntax theme:
/// - Cyan for keys
/// - White for the `=` delimiter
/// - Yellow for the masked values
/// - Dim Gray for comments
/// - Empty lines untouched
pub fn render_line(parsed: &ParsedLine) -> String {
    match parsed {
        ParsedLine::Empty(raw) => raw.clone(),
        ParsedLine::Comment(raw) => raw.bright_black().to_string(),
        ParsedLine::KeyValue { key, masked_value } => {
            format!(
                "{}{}{}",
                key.cyan(),
                "=".white(),
                masked_value.yellow()
            )
        }
        ParsedLine::Malformed { raw, .. } => raw.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_empty_line() {
        let (parsed, err) = parse_line("", 1);
        assert_eq!(parsed, ParsedLine::Empty("".to_string()));
        assert!(err.is_none());

        let (parsed, err) = parse_line("   ", 2);
        assert_eq!(parsed, ParsedLine::Empty("   ".to_string()));
        assert!(err.is_none());
    }

    #[test]
    fn test_parse_comments() {
        let (parsed, err) = parse_line("# Database configuration", 1);
        assert_eq!(parsed, ParsedLine::Comment("# Database configuration".to_string()));
        assert!(err.is_none());

        let (parsed, err) = parse_line("  # Indented comment", 2);
        assert_eq!(parsed, ParsedLine::Comment("  # Indented comment".to_string()));
        assert!(err.is_none());
    }

    #[test]
    fn test_parse_key_value() {
        let (parsed, err) = parse_line("PORT=3000", 1);
        assert_eq!(
            parsed,
            ParsedLine::KeyValue {
                key: "PORT".to_string(),
                masked_value: "****".to_string(),
            }
        );
        assert!(err.is_none());
    }

    #[test]
    fn test_parse_inline_equals() {
        let (parsed, err) = parse_line("DATABASE_URL=postgres://user:p=ss@localhost/db?ssl=true", 1);
        assert_eq!(
            parsed,
            ParsedLine::KeyValue {
                key: "DATABASE_URL".to_string(),
                masked_value: "pos********ue".to_string(),
            }
        );
        assert!(err.is_none());
    }

    #[test]
    fn test_parse_with_quotes() {
        let (parsed, err) = parse_line("API_SECRET=\"sk_live_994b\"", 1);
        assert_eq!(
            parsed,
            ParsedLine::KeyValue {
                key: "API_SECRET".to_string(),
                masked_value: "sk_********4b".to_string(),
            }
        );
        assert!(err.is_none());
    }

    #[test]
    fn test_parse_malformed_missing_equals() {
        let (parsed, err) = parse_line("JUST_A_STRING", 5);
        assert!(matches!(parsed, ParsedLine::Malformed { .. }));
        assert!(err.is_some());
        assert_eq!(err.unwrap().reason, "Missing '=' delimiter");
    }

    #[test]
    fn test_parse_malformed_spaces_in_key() {
        let (parsed, err) = parse_line("INVALID KEY=value", 6);
        assert!(matches!(parsed, ParsedLine::Malformed { .. }));
        assert!(err.is_some());
        assert!(err.unwrap().reason.contains("contains illegal spaces"));
    }
}
