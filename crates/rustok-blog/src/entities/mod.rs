//! Blog module entities
//!
//! The blog module is a wrapper module that uses content module tables.
//! No separate database entities are needed.
//!
//! Entity references are provided through the content module:
//! - `rustok_content::entities::node` - for blog posts
//! - `rustok_content::entities::body` - for post body content
//! - `rustok_content::entities::node_translation` - for localized titles/slugs

// Re-export content entities for convenience
pub use rustok_content::entities::{Body, Node, NodeTranslation};
