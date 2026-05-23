pub fn slugify(value: &str) -> String {
    value
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() {
                ch.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect::<String>()
        .split('-')
        .filter(|segment| !segment.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

pub fn parse_channel_slugs(value: &str) -> Vec<String> {
    let mut items = value
        .split(',')
        .map(|item| item.trim().to_ascii_lowercase())
        .filter(|item| !item.is_empty())
        .collect::<Vec<_>>();
    items.sort();
    items.dedup();
    items
}

pub fn error_with_context(context: &str, error: &str) -> String {
    format!("{}: {}", context, error)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slugify_normalizes_ascii_words() {
        assert_eq!(slugify("Hello, Rustok Pages!"), "hello-rustok-pages");
    }

    #[test]
    fn parse_channel_slugs_trims_sorts_and_deduplicates() {
        assert_eq!(
            parse_channel_slugs(" web, mobile-app,WEB, , mobile-app "),
            vec!["mobile-app".to_string(), "web".to_string()]
        );
    }

    #[test]
    fn error_with_context_formats_consistently() {
        assert_eq!(
            error_with_context("Failed to save page", "timeout"),
            "Failed to save page: timeout"
        );
    }
}
