use crate::model::{CategoryDetail, CategoryDraft, ReplyListItem, TopicDetail, TopicDraft};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ForumAdminFormError {
    CategoryRequired,
    TopicRequired,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CategoryFormSnapshot {
    pub editing_id: Option<String>,
    pub locale: String,
    pub name: String,
    pub slug: String,
    pub description: String,
    pub icon: String,
    pub color: String,
    pub position: i32,
    pub moderated: bool,
}

impl CategoryFormSnapshot {
    pub fn blank(default_locale: impl Into<String>) -> Self {
        Self {
            editing_id: None,
            locale: default_locale.into(),
            name: String::new(),
            slug: String::new(),
            description: String::new(),
            icon: String::new(),
            color: String::new(),
            position: 0,
            moderated: false,
        }
    }

    pub fn from_detail(category: &CategoryDetail) -> Self {
        Self {
            editing_id: Some(category.id.clone()),
            locale: category.locale.clone(),
            name: category.name.clone(),
            slug: category.slug.clone(),
            description: category.description.clone().unwrap_or_default(),
            icon: category.icon.clone().unwrap_or_default(),
            color: category.color.clone().unwrap_or_default(),
            position: category.position,
            moderated: category.moderated,
        }
    }

    pub fn to_draft(&self) -> Result<CategoryDraft, ForumAdminFormError> {
        let draft = CategoryDraft {
            locale: self.locale.clone(),
            name: self.name.trim().to_string(),
            slug: self.slug.trim().to_string(),
            description: self.description.trim().to_string(),
            icon: self.icon.trim().to_string(),
            color: self.color.trim().to_string(),
            position: self.position,
            moderated: self.moderated,
        };
        if draft.name.is_empty() || draft.slug.is_empty() {
            return Err(ForumAdminFormError::CategoryRequired);
        }
        Ok(draft)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TopicFormSnapshot {
    pub editing_id: Option<String>,
    pub locale: String,
    pub category_id: String,
    pub title: String,
    pub slug: String,
    pub body: String,
    pub body_format: String,
    pub tags_raw: String,
}

impl TopicFormSnapshot {
    pub fn blank(default_locale: impl Into<String>) -> Self {
        Self {
            editing_id: None,
            locale: default_locale.into(),
            category_id: String::new(),
            title: String::new(),
            slug: String::new(),
            body: String::new(),
            body_format: "markdown".to_string(),
            tags_raw: String::new(),
        }
    }

    pub fn from_detail(topic: &TopicDetail) -> Self {
        Self {
            editing_id: Some(topic.id.clone()),
            locale: topic.locale.clone(),
            category_id: topic.category_id.clone(),
            title: topic.title.clone(),
            slug: topic.slug.clone(),
            body: topic.body.clone(),
            body_format: topic.body_format.clone(),
            tags_raw: topic.tags.join(", "),
        }
    }

    pub fn to_draft(&self) -> Result<TopicDraft, ForumAdminFormError> {
        let draft = TopicDraft {
            locale: self.locale.clone(),
            category_id: self.category_id.trim().to_string(),
            title: self.title.trim().to_string(),
            slug: self.slug.trim().to_string(),
            body: self.body.trim().to_string(),
            body_format: self.body_format.trim().to_string(),
            tags: parse_tags(self.tags_raw.as_str()),
        };
        if draft.category_id.is_empty() || draft.title.is_empty() || draft.body.is_empty() {
            return Err(ForumAdminFormError::TopicRequired);
        }
        Ok(draft)
    }
}

pub fn topic_category_filter(category_id: String) -> Option<String> {
    let trimmed = category_id.trim();
    (!trimmed.is_empty()).then(|| trimmed.to_string())
}

pub fn parse_tags(raw: &str) -> Vec<String> {
    raw.split(',')
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToString::to_string)
        .collect()
}

pub fn format_count(value: usize) -> String {
    value.to_string()
}

pub fn topic_status_class(status: &str) -> &'static str {
    match status.to_ascii_lowercase().as_str() {
        "published" | "active" | "open" => "success",
        "draft" | "pending" => "warning",
        "archived" | "closed" => "muted",
        _ => "default",
    }
}

pub fn reply_count_label(replies: Option<Result<Vec<ReplyListItem>, String>>) -> usize {
    match replies {
        Some(Ok(items)) => items.len(),
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trims_topic_category_filter() {
        assert_eq!(
            topic_category_filter("  category-1  ".to_string()),
            Some("category-1".to_string())
        );
        assert_eq!(topic_category_filter("   ".to_string()), None);
    }

    #[test]
    fn parses_comma_separated_tags_without_empty_values() {
        assert_eq!(
            parse_tags(" rust, forum ,, ffa "),
            vec!["rust", "forum", "ffa"]
        );
    }

    #[test]
    fn maps_topic_status_to_stable_class_keys() {
        assert_eq!(topic_status_class("PUBLISHED"), "success");
        assert_eq!(topic_status_class("pending"), "warning");
        assert_eq!(topic_status_class("closed"), "muted");
        assert_eq!(topic_status_class("other"), "default");
    }

    #[test]
    fn builds_category_form_snapshot_and_trimmed_draft() {
        let snapshot = CategoryFormSnapshot {
            editing_id: Some("category-1".to_string()),
            locale: "ru".to_string(),
            name: "  Общение  ".to_string(),
            slug: "  community  ".to_string(),
            description: "  Описание  ".to_string(),
            icon: "  chat  ".to_string(),
            color: "  #fff  ".to_string(),
            position: 3,
            moderated: true,
        };
        let draft = snapshot.to_draft().expect("valid category draft");
        assert_eq!(draft.name, "Общение");
        assert_eq!(draft.slug, "community");
        assert_eq!(draft.description, "Описание");
        assert_eq!(draft.icon, "chat");
        assert_eq!(draft.color, "#fff");
        assert_eq!(draft.position, 3);
        assert!(draft.moderated);
    }

    #[test]
    fn rejects_category_snapshot_without_required_fields() {
        let snapshot = CategoryFormSnapshot {
            name: " ".to_string(),
            slug: "category".to_string(),
            ..CategoryFormSnapshot::blank("en")
        };
        assert_eq!(
            snapshot.to_draft().unwrap_err(),
            ForumAdminFormError::CategoryRequired
        );
    }

    #[test]
    fn builds_topic_form_snapshot_and_trimmed_draft() {
        let snapshot = TopicFormSnapshot {
            editing_id: None,
            locale: "en".to_string(),
            category_id: "  cat-1  ".to_string(),
            title: "  Welcome  ".to_string(),
            slug: "  welcome  ".to_string(),
            body: "  Body  ".to_string(),
            body_format: "  markdown  ".to_string(),
            tags_raw: " rust, forum ,, ffa ".to_string(),
        };
        let draft = snapshot.to_draft().expect("valid topic draft");
        assert_eq!(draft.category_id, "cat-1");
        assert_eq!(draft.title, "Welcome");
        assert_eq!(draft.slug, "welcome");
        assert_eq!(draft.body, "Body");
        assert_eq!(draft.body_format, "markdown");
        assert_eq!(draft.tags, vec!["rust", "forum", "ffa"]);
    }

    #[test]
    fn rejects_topic_snapshot_without_required_fields() {
        let snapshot = TopicFormSnapshot {
            category_id: "cat-1".to_string(),
            title: " ".to_string(),
            body: "Body".to_string(),
            ..TopicFormSnapshot::blank("en")
        };
        assert_eq!(
            snapshot.to_draft().unwrap_err(),
            ForumAdminFormError::TopicRequired
        );
    }
}
