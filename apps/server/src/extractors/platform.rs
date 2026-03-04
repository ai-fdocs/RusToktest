use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};

use rustok_core::PlatformInfo;

/// Extractor for platform information from User-Agent header
///
/// This extractor parses the User-Agent header and provides structured information
/// about the client device, operating system, and browser.
///
/// # Example
///
/// ```ignore
/// use axum::extract::State;
/// use rustok_core::PlatformInfo;
///
/// async fn handler(
///     State(ctx): State<AppContext>,
///     platform: PlatformInfo,
/// ) -> Result<String, StatusCode> {
///     if platform.is_mobile() {
///         Ok("Mobile device detected".to_string())
///     } else {
///         Ok("Desktop device detected".to_string())
///     }
/// }
/// ```
#[derive(Debug, Clone)]
pub struct PlatformInfo(pub rustok_core::PlatformInfo);

impl PlatformInfo {
    /// Get the underlying platform info
    pub fn info(&self) -> &rustok_core::PlatformInfo {
        &self.0
    }

    /// Check if this is a mobile device
    pub fn is_mobile(&self) -> bool {
        self.0.is_mobile()
    }

    /// Check if this is a desktop device
    pub fn is_desktop(&self) -> bool {
        self.0.is_desktop()
    }

    /// Check if this is a tablet
    pub fn is_tablet(&self) -> bool {
        self.0.is_tablet()
    }

    /// Check if this is a bot
    pub fn is_bot(&self) -> bool {
        self.0.is_bot
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for PlatformInfo
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let user_agent = parts
            .headers
            .get("user-agent")
            .and_then(|value| value.to_str().ok())
            .unwrap_or("");

        let platform_info = rustok_core::parse_user_agent(user_agent);

        Ok(Self(platform_info))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use http::request::Builder;

    fn build_request_with_user_agent(user_agent: &str) -> Parts {
        let request = Builder::new()
            .header("user-agent", user_agent)
            .body(())
            .unwrap();

        let (parts, _) = request.into_parts();
        parts
    }

    #[tokio::test]
    async fn test_platform_info_extractor_chrome() {
        let ua = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36";
        let parts = build_request_with_user_agent(ua);

        let platform = PlatformInfo::from_request_parts(&mut parts.clone(), &())
            .await
            .unwrap();

        assert_eq!(platform.0.browser.to_string(), "chrome");
        assert_eq!(platform.0.os.to_string(), "windows");
        assert!(platform.is_desktop());
        assert!(!platform.is_mobile());
    }

    #[tokio::test]
    async fn test_platform_info_extractor_mobile() {
        let ua = "Mozilla/5.0 (Linux; Android 10; SM-G973F) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.77 Mobile Safari/537.36";
        let parts = build_request_with_user_agent(ua);

        let platform = PlatformInfo::from_request_parts(&mut parts.clone(), &())
            .await
            .unwrap();

        assert!(platform.is_mobile());
        assert!(!platform.is_desktop());
    }

    #[tokio::test]
    async fn test_platform_info_extractor_bot() {
        let ua = "Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)";
        let parts = build_request_with_user_agent(ua);

        let platform = PlatformInfo::from_request_parts(&mut parts.clone(), &())
            .await
            .unwrap();

        assert!(platform.is_bot());
    }

    #[tokio::test]
    async fn test_platform_info_extractor_no_header() {
        let request = Builder::new().body(()).unwrap();
        let (mut parts, _) = request.into_parts();

        let platform = PlatformInfo::from_request_parts(&mut parts, &())
            .await
            .unwrap();

        assert_eq!(platform.0.device_type.to_string(), "unknown");
        assert_eq!(platform.0.browser.to_string(), "unknown");
        assert!(!platform.is_bot());
    }
}
