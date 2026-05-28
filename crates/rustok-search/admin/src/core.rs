pub fn parse_csv(value: &str) -> Vec<String> {
    value
        .split(',')
        .map(str::trim)
        .filter(|segment| !segment.is_empty())
        .map(ToOwned::to_owned)
        .collect()
}

pub fn optional_text(value: &str) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

pub fn facet_display_name(raw_name: &str) -> String {
    raw_name.replace('_', " ")
}

pub fn facet_bucket_label(value: &str, count: u64) -> String {
    format!("{} ({})", value, count)
}

pub fn snippet_or_fallback(snippet: Option<String>, fallback: &str) -> String {
    snippet.unwrap_or_else(|| fallback.to_string())
}

pub fn score_label(score: f64) -> String {
    format!("score {:.3}", score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_csv_trims_and_skips_empty_segments() {
        assert_eq!(
            parse_csv(" products, blog ,, pages "),
            vec![
                "products".to_string(),
                "blog".to_string(),
                "pages".to_string()
            ]
        );
    }

    #[test]
    fn optional_text_returns_none_for_blank() {
        assert_eq!(
            optional_text(
                "   
	"
            ),
            None
        );
    }

    #[test]
    fn optional_text_returns_trimmed_value() {
        assert_eq!(optional_text("  abc  "), Some("abc".to_string()));
    }

    #[test]
    fn formatting_helpers_are_stable() {
        assert_eq!(facet_display_name("source_module"), "source module");
        assert_eq!(facet_bucket_label("product", 42), "product (42)");
        assert_eq!(score_label(0.12345), "score 0.123");
        assert_eq!(
            snippet_or_fallback(None, "fallback"),
            "fallback".to_string()
        );
        assert_eq!(
            error_with_context("load failed", "timeout"),
            "load failed: timeout"
        );
    }

    #[test]
    fn analytics_formatting_helpers_are_stable() {
        assert_eq!(format_days(14), "14d");
        assert_eq!(format_percent_fraction(0.1234), "12.3%");
        assert_eq!(format_milliseconds(12.34), "12.3 ms");
        assert_eq!(format_decimal_1(7.89), "7.9");
        assert_eq!(format_seconds(42), "42s");
        assert_eq!(
            document_source_path("doc-1", "catalog", "product"),
            "doc-1 / catalog / product"
        );
    }

    #[test]
    fn relevance_editor_json_helpers_are_stable() {
        let config = serde_json::json!({
            "ranking_profiles": {
                "search_preview": "freshness",
                "admin_global_search": "operator"
            },
            "filter_presets": {
                "search_preview": [
                    {"key": "published", "label": "Published"}
                ]
            }
        });

        assert_eq!(
            extract_ranking_profile_value(&config, "search_preview"),
            "freshness"
        );
        assert_eq!(
            extract_ranking_profile_value(&config, "storefront_search"),
            "balanced"
        );
        assert_eq!(
            extract_ranking_profile_value(&serde_json::json!({}), "admin_global_search"),
            "exact"
        );
        assert_eq!(
            extract_surface_presets_json(&config, "search_preview"),
            "[\n  {\n    \"key\": \"published\",\n    \"label\": \"Published\"\n  }\n]"
        );
        assert_eq!(extract_surface_presets_json(&config, "missing"), "[]");
        assert_eq!(pretty_json_string("{\"a\":1}"), "{\n  \"a\": 1\n}");
        assert_eq!(pretty_json_string("not-json"), "not-json");
    }
}

pub fn entity_source_label(entity_type: &str, source_module: &str) -> String {
    format!("{} | {}", entity_type, source_module)
}

pub fn source_entity_status_label(source_module: &str, entity_type: &str, status: &str) -> String {
    format!("{}/{} ({})", source_module, entity_type, status)
}

pub fn error_with_context(context: &str, error: &str) -> String {
    format!("{}: {}", context, error)
}

pub fn pretty_json_string(value: &str) -> String {
    parse_json_for_editor(value)
        .and_then(|json| serde_json::to_string_pretty(&json).ok())
        .unwrap_or_else(|| value.to_string())
}

pub fn parse_json_for_editor(value: &str) -> Option<serde_json::Value> {
    serde_json::from_str(value).ok()
}

pub fn extract_ranking_profile_value(config: &serde_json::Value, surface: &str) -> String {
    config
        .get("ranking_profiles")
        .and_then(|value| value.get(surface))
        .and_then(serde_json::Value::as_str)
        .unwrap_or(match surface {
            "admin_global_search" => "exact",
            _ => "balanced",
        })
        .to_string()
}

pub fn extract_surface_presets_json(config: &serde_json::Value, surface: &str) -> String {
    config
        .get("filter_presets")
        .and_then(|value| value.get(surface))
        .and_then(|value| serde_json::to_string_pretty(value).ok())
        .unwrap_or_else(|| "[]".to_string())
}

pub fn format_days(days: u32) -> String {
    format!("{}d", days)
}

pub fn format_percent_fraction(value: f64) -> String {
    format!("{:.1}%", value * 100.0)
}

pub fn format_milliseconds(value: f64) -> String {
    format!("{:.1} ms", value)
}

pub fn format_decimal_1(value: f64) -> String {
    format!("{:.1}", value)
}

pub fn format_seconds(seconds: u64) -> String {
    format!("{}s", seconds)
}

pub fn document_source_path(document_id: &str, source_module: &str, entity_type: &str) -> String {
    format!("{} / {} / {}", document_id, source_module, entity_type)
}
