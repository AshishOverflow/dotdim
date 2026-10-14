/// Masks a secret value according to dotdim rules:
/// - Empty string -> returns empty string
/// - Length < 6 -> replaced with matching number of `*` characters
/// - Length >= 6 -> first 3 chars + 8 `*` chars + last 2 chars
pub fn mask_value(val: &str) -> String {
    let chars: Vec<char> = val.chars().collect();
    let len = chars.len();

    if len == 0 {
        String::new()
    } else if len < 6 {
        "*".repeat(len)
    } else {
        let prefix: String = chars[..3].iter().collect();
        let suffix: String = chars[len - 2..].iter().collect();
        format!("{}********{}", prefix, suffix)
    }
}

/// Safely strips matching surrounding quotes (' or ") if present.
pub fn strip_surrounding_quotes(val: &str) -> &str {
    if (val.starts_with('"') && val.ends_with('"') && val.len() >= 2)
        || (val.starts_with('\'') && val.ends_with('\'') && val.len() >= 2)
    {
        &val[1..val.len() - 1]
    } else {
        val
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mask_empty() {
        assert_eq!(mask_value(""), "");
    }

    #[test]
    fn test_mask_short_values() {
        assert_eq!(mask_value("a"), "*");
        assert_eq!(mask_value("ab"), "**");
        assert_eq!(mask_value("abc"), "***");
        assert_eq!(mask_value("abcd"), "****");
        assert_eq!(mask_value("abcde"), "*****");
    }

    #[test]
    fn test_mask_exact_six() {
        assert_eq!(mask_value("123456"), "123********56");
    }

    #[test]
    fn test_mask_long_value() {
        // e.g., sk_live_994b
        assert_eq!(mask_value("sk_live_994b"), "sk_********4b");
        assert_eq!(mask_value("password123"), "pas********23");
    }

    #[test]
    fn test_mask_unicode() {
        assert_eq!(mask_value("🦀🦀"), "**");
        assert_eq!(mask_value("🦀🔑⭐🌟🔥🎉"), "🦀🔑⭐********🔥🎉");
    }

    #[test]
    fn test_strip_quotes() {
        assert_eq!(strip_surrounding_quotes("\"hello\""), "hello");
        assert_eq!(strip_surrounding_quotes("'secret_token'"), "secret_token");
        assert_eq!(strip_surrounding_quotes("\"\""), "");
        assert_eq!(strip_surrounding_quotes("''"), "");
        assert_eq!(strip_surrounding_quotes("\"mismatched'"), "\"mismatched'");
        assert_eq!(strip_surrounding_quotes("no_quotes"), "no_quotes");
        assert_eq!(strip_surrounding_quotes("\"quote inside \" value\""), "quote inside \" value");
    }
}
