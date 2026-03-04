//! Platform Detection Module
//!
//! Provides user agent parsing and platform detection capabilities.
//! Extracts device type, operating system, browser, and other client information.

use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fmt;

use once_cell::sync::Lazy;

/// Device type classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DeviceType {
    Desktop,
    Mobile,
    Tablet,
    SmartTV,
    Console,
    Wearable,
    Bot,
    Unknown,
}

impl fmt::Display for DeviceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DeviceType::Desktop => write!(f, "desktop"),
            DeviceType::Mobile => write!(f, "mobile"),
            DeviceType::Tablet => write!(f, "tablet"),
            DeviceType::SmartTV => write!(f, "smarttv"),
            DeviceType::Console => write!(f, "console"),
            DeviceType::Wearable => write!(f, "wearable"),
            DeviceType::Bot => write!(f, "bot"),
            DeviceType::Unknown => write!(f, "unknown"),
        }
    }
}

/// Operating system classification
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OperatingSystem {
    Windows,
    MacOS,
    Linux,
    Android,
    iOS,
    Other(String),
    Unknown,
}

impl fmt::Display for OperatingSystem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OperatingSystem::Windows => write!(f, "windows"),
            OperatingSystem::MacOS => write!(f, "macos"),
            OperatingSystem::Linux => write!(f, "linux"),
            OperatingSystem::Android => write!(f, "android"),
            OperatingSystem::iOS => write!(f, "ios"),
            OperatingSystem::Other(name) => write!(f, "{}", name.to_lowercase()),
            OperatingSystem::Unknown => write!(f, "unknown"),
        }
    }
}

/// Browser classification
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Browser {
    Chrome,
    Firefox,
    Safari,
    Edge,
    Opera,
    InternetExplorer,
    Other(String),
    Unknown,
}

impl fmt::Display for Browser {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Browser::Chrome => write!(f, "chrome"),
            Browser::Firefox => write!(f, "firefox"),
            Browser::Safari => write!(f, "safari"),
            Browser::Edge => write!(f, "edge"),
            Browser::Opera => write!(f, "opera"),
            Browser::InternetExplorer => write!(f, "ie"),
            Browser::Other(name) => write!(f, "{}", name.to_lowercase()),
            Browser::Unknown => write!(f, "unknown"),
        }
    }
}

/// Parsed platform information from user agent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformInfo {
    /// Original user agent string
    pub user_agent: String,
    /// Detected device type
    pub device_type: DeviceType,
    /// Detected operating system
    pub os: OperatingSystem,
    /// OS version if available
    pub os_version: Option<String>,
    /// Detected browser
    pub browser: Browser,
    /// Browser version if available
    pub browser_version: Option<String>,
    /// Whether this is a bot/crawler
    pub is_bot: bool,
}

impl PlatformInfo {
    /// Create a new PlatformInfo with minimal info
    pub fn new(user_agent: String) -> Self {
        Self {
            user_agent,
            device_type: DeviceType::Unknown,
            os: OperatingSystem::Unknown,
            os_version: None,
            browser: Browser::Unknown,
            browser_version: None,
            is_bot: false,
        }
    }

    /// Check if this is a mobile device
    pub fn is_mobile(&self) -> bool {
        self.device_type == DeviceType::Mobile
    }

    /// Check if this is a desktop device
    pub fn is_desktop(&self) -> bool {
        self.device_type == DeviceType::Desktop
    }

    /// Check if this is a tablet
    pub fn is_tablet(&self) -> bool {
        self.device_type == DeviceType::Tablet
    }
}

impl fmt::Display for PlatformInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}/{}/{}/{}",
            self.device_type, self.os, self.browser,
            self.browser_version.as_deref().unwrap_or("unknown")
        )
    }
}

/// Bot/Crawler detection patterns
static BOT_PATTERNS: Lazy<Vec<Regex>> = Lazy::new(|| {
    vec![
        Regex::new(r"(?i)googlebot").unwrap(),
        Regex::new(r"(?i)bingbot").unwrap(),
        Regex::new(r"(?i)slurp").unwrap(),
        Regex::new(r"(?i)duckduckbot").unwrap(),
        Regex::new(r"(?i)baiduspider").unwrap(),
        Regex::new(r"(?i)yandexbot").unwrap(),
        Regex::new(r"(?i)sogou").unwrap(),
        Regex::new(r"(?i)exabot").unwrap(),
        Regex::new(r"(?i)facebot").unwrap(),
        Regex::new(r"(?i)facebookexternalhit").unwrap(),
        Regex::new(r"(?i)twitterbot").unwrap(),
        Regex::new(r"(?i)linkedinbot").unwrap(),
        Regex::new(r"(?i)whatsapp").unwrap(),
        Regex::new(r"(?i)telegrambot").unwrap(),
        Regex::new(r"(?i)applebot").unwrap(),
        Regex::new(r"(?i)semrushbot").unwrap(),
        Regex::new(r"(?i)ahrefsbot").unwrap(),
        Regex::new(r"(?i)mj12bot").unwrap(),
        Regex::new(r"(?i)dotbot").unwrap(),
        Regex::new(r"(?i)crawler").unwrap(),
        Regex::new(r"(?i)spider").unwrap(),
        Regex::new(r"(?i)scraper").unwrap(),
        Regex::new(r"(?i)bot").unwrap(),
    ]
});

/// Mobile device patterns
static MOBILE_PATTERNS: Lazy<Vec<Regex>> = Lazy::new(|| {
    vec![
        Regex::new(r"(?i)mobile").unwrap(),
        Regex::new(r"(?i)android").unwrap(),
        Regex::new(r"(?i)iphone").unwrap(),
        Regex::new(r"(?i)ipod").unwrap(),
        Regex::new(r"(?i)blackberry").unwrap(),
        Regex::new(r"(?i)opera mini").unwrap(),
        Regex::new(r"(?i)windows phone").unwrap(),
        Regex::new(r"(?i)iemobile").unwrap(),
    ]
});

/// Tablet patterns (checked before mobile)
static TABLET_PATTERNS: Lazy<Vec<Regex>> = Lazy::new(|| {
    vec![
        Regex::new(r"(?i)ipad").unwrap(),
        Regex::new(r"(?i)tablet").unwrap(),
        Regex::new(r"(?i)kindle").unwrap(),
        Regex::new(r"(?i)silk").unwrap(),
        Regex::new(r"(?i)playbook").unwrap(),
    ]
});

/// Desktop-specific patterns
static DESKTOP_PATTERNS: Lazy<Vec<Regex>> = Lazy::new(|| {
    vec![
        Regex::new(r"(?i)windows nt").unwrap(),
        Regex::new(r"(?i)macintosh|mac os x").unwrap(),
        Regex::new(r"(?i)linux").unwrap(),
        Regex::new(r"(?i)x11").unwrap(),
    ]
});

/// Smart TV patterns
static SMARTTV_PATTERNS: Lazy<Vec<Regex>> = Lazy::new(|| {
    vec![
        Regex::new(r"(?i)smart-tv").unwrap(),
        Regex::new(r"(?i)smarttv").unwrap(),
        Regex::new(r"(?i)appletv").unwrap(),
        Regex::new(r"(?i)roku").unwrap(),
        Regex::new(r"(?i)chromecast").unwrap(),
        Regex::new(r"(?i)google-tv").unwrap(),
        Regex::new(r"(?i)firetv").unwrap(),
        Regex::new(r"(?i)webos").unwrap(),
    ]
});

/// Gaming console patterns
static CONSOLE_PATTERNS: Lazy<Vec<Regex>> = Lazy::new(|| {
    vec![
        Regex::new(r"(?i)playstation").unwrap(),
        Regex::new(r"(?i)ps4|ps5").unwrap(),
        Regex::new(r"(?i)xbox").unwrap(),
        Regex::new(r"(?i)nintendo").unwrap(),
        Regex::new(r"(?i)wii").unwrap(),
        Regex::new(r"(?i)switch").unwrap(),
    ]
});

/// Wearable device patterns
static WEARABLE_PATTERNS: Lazy<Vec<Regex>> = Lazy::new(|| {
    vec![
        Regex::new(r"(?i)watch").unwrap(),
        Regex::new(r"(?i)wearable").unwrap(),
        Regex::new(r"(?i)fitbit").unwrap(),
        Regex::new(r"(?i)garmin").unwrap(),
    ]
});

/// Operating system patterns with version extraction
static OS_PATTERNS: Lazy<Vec<(Regex, fn(&str) -> (OperatingSystem, Option<String>))>> = Lazy::new(|| {
    vec![
        (
            Regex::new(r"(?i)Windows NT (?P<version>[\d.]+)").unwrap(),
            |caps: &str| {
                let version = extract_named_group(caps, "version");
                (OperatingSystem::Windows, version.map(normalize_windows_version))
            }
        ),
        (
            Regex::new(r"(?i)Windows (?P<version>[\d.]+|XP|Vista|7|8|10|11)").unwrap(),
            |caps: &str| {
                let version = extract_named_group(caps, "version");
                (OperatingSystem::Windows, version.map(normalize_windows_version))
            }
        ),
        (
            Regex::new(r"(?i)Mac OS X (?P<version>[\d_.]+)").unwrap(),
            |caps: &str| {
                let version = extract_named_group(caps, "version").map(|v| v.replace('_', "."));
                (OperatingSystem::MacOS, version)
            }
        ),
        (
            Regex::new(r"(?i)macOS (?P<version>[\d.]+)").unwrap(),
            |caps: &str| {
                let version = extract_named_group(caps, "version");
                (OperatingSystem::MacOS, version)
            }
        ),
        (
            Regex::new(r"(?i)Android (?P<version>[\d.]+)").unwrap(),
            |caps: &str| {
                let version = extract_named_group(caps, "version");
                (OperatingSystem::Android, version)
            }
        ),
        (
            Regex::new(r"(?i)iPhone OS (?P<version>[\d_]+)").unwrap(),
            |caps: &str| {
                let version = extract_named_group(caps, "version").map(|v| v.replace('_', "."));
                (OperatingSystem::iOS, version)
            }
        ),
        (
            Regex::new(r"(?i)iPad OS (?P<version>[\d_]+)").unwrap(),
            |caps: &str| {
                let version = extract_named_group(caps, "version").map(|v| v.replace('_', "."));
                (OperatingSystem::iOS, version)
            }
        ),
        (
            Regex::new(r"(?i)Linux").unwrap(),
            |_| (OperatingSystem::Linux, None)
        ),
        (
            Regex::new(r"(?i)Ubuntu[/ ](?P<version>[\d.]+)").unwrap(),
            |caps: &str| {
                let version = extract_named_group(caps, "version");
                (OperatingSystem::Other("Ubuntu".to_string()), version)
            }
        ),
        (
            Regex::new(r"(?i)Fedora[/ ](?P<version>[\d.]+)").unwrap(),
            |caps: &str| {
                let version = extract_named_group(caps, "version");
                (OperatingSystem::Other("Fedora".to_string()), version)
            }
        ),
        (
            Regex::new(r"(?i)Debian[/ ](?P<version>[\d.]+)").unwrap(),
            |caps: &str| {
                let version = extract_named_group(caps, "version");
                (OperatingSystem::Other("Debian".to_string()), version)
            }
        ),
    ]
});

/// Browser patterns with version extraction
static BROWSER_PATTERNS: Lazy<Vec<(Regex, fn(&str) -> (Browser, Option<String>))>> = Lazy::new(|| {
    vec![
        (
            Regex::new(r"(?i)Edg/(?P<version>[\d.]+)").unwrap(),
            |caps: &str| {
                let version = extract_named_group(caps, "version");
                (Browser::Edge, version)
            }
        ),
        (
            Regex::new(r"(?i)Edge/(?P<version>[\d.]+)").unwrap(),
            |caps: &str| {
                let version = extract_named_group(caps, "version");
                (Browser::Edge, version)
            }
        ),
        (
            Regex::new(r"(?i)Chrome/(?P<version>[\d.]+)").unwrap(),
            |caps: &str| {
                let version = extract_named_group(caps, "version");
                (Browser::Chrome, version)
            }
        ),
        (
            Regex::new(r"(?i)CriOS/(?P<version>[\d.]+)").unwrap(),
            |caps: &str| {
                let version = extract_named_group(caps, "version");
                (Browser::Chrome, version)
            }
        ),
        (
            Regex::new(r"(?i)Firefox/(?P<version>[\d.]+)").unwrap(),
            |caps: &str| {
                let version = extract_named_group(caps, "version");
                (Browser::Firefox, version)
            }
        ),
        (
            Regex::new(r"(?i)FxiOS/(?P<version>[\d.]+)").unwrap(),
            |caps: &str| {
                let version = extract_named_group(caps, "version");
                (Browser::Firefox, version)
            }
        ),
        (
            Regex::new(r"(?i)Safari/(?P<version>[\d.]+)").unwrap(),
            |caps: &str| {
                let version = extract_named_group(caps, "version");
                (Browser::Safari, version)
            }
        ),
        (
            Regex::new(r"(?i)Opera/(?P<version>[\d.]+)").unwrap(),
            |caps: &str| {
                let version = extract_named_group(caps, "version");
                (Browser::Opera, version)
            }
        ),
        (
            Regex::new(r"(?i)OPR/(?P<version>[\d.]+)").unwrap(),
            |caps: &str| {
                let version = extract_named_group(caps, "version");
                (Browser::Opera, version)
            }
        ),
        (
            Regex::new(r"(?i)MSIE (?P<version>[\d.]+)").unwrap(),
            |caps: &str| {
                let version = extract_named_group(caps, "version");
                (Browser::InternetExplorer, version)
            }
        ),
        (
            Regex::new(r"(?i)Trident/(?P<version>[\d.]+)").unwrap(),
            |caps: &str| {
                let version = extract_named_group(caps, "version");
                (Browser::InternetExplorer, version)
            }
        ),
    ]
});

/// Platform detector
#[derive(Debug, Clone, Default)]
pub struct PlatformDetector;

impl PlatformDetector {
    /// Create a new platform detector
    pub fn new() -> Self {
        Self
    }

    /// Detect platform information from user agent string
    pub fn detect(&self, user_agent: &str) -> PlatformInfo {
        let mut info = PlatformInfo::new(user_agent.to_string());

        info.is_bot = self.detect_bot(user_agent);
        info.device_type = self.detect_device_type(user_agent);

        // Only detect OS and browser for non-bots
        if !info.is_bot {
            let (os, os_version) = self.detect_os(user_agent);
            info.os = os;
            info.os_version = os_version;

            let (browser, browser_version) = self.detect_browser(user_agent);
            info.browser = browser;
            info.browser_version = browser_version;
        }

        info
    }

    /// Detect if user agent is a bot/crawler
    fn detect_bot(&self, user_agent: &str) -> bool {
        BOT_PATTERNS.iter().any(|pattern| pattern.is_match(user_agent))
    }

    /// Detect device type
    fn detect_device_type(&self, user_agent: &str) -> DeviceType {
        // Check patterns in priority order

        if CONSOLE_PATTERNS.iter().any(|p| p.is_match(user_agent)) {
            return DeviceType::Console;
        }

        if WEARABLE_PATTERNS.iter().any(|p| p.is_match(user_agent)) {
            return DeviceType::Wearable;
        }

        if SMARTTV_PATTERNS.iter().any(|p| p.is_match(user_agent)) {
            return DeviceType::SmartTV;
        }

        if TABLET_PATTERNS.iter().any(|p| p.is_match(user_agent)) {
            return DeviceType::Tablet;
        }

        if MOBILE_PATTERNS.iter().any(|p| p.is_match(user_agent)) {
            return DeviceType::Mobile;
        }

        if DESKTOP_PATTERNS.iter().any(|p| p.is_match(user_agent)) {
            return DeviceType::Desktop;
        }

        DeviceType::Unknown
    }

    /// Detect operating system and version
    fn detect_os(&self, user_agent: &str) -> (OperatingSystem, Option<String>) {
        for (pattern, extractor) in OS_PATTERNS.iter() {
            if let Some(caps) = pattern.captures(user_agent) {
                if let Some(full_match) = caps.get(0) {
                    return extractor(full_match.as_str());
                }
            }
        }

        // Fallback: try to identify OS from browser/UA patterns
        if user_agent.contains("Android") {
            return (OperatingSystem::Android, None);
        }
        if user_agent.contains("iPhone") || user_agent.contains("iPad") || user_agent.contains("iPod") {
            return (OperatingSystem::iOS, None);
        }
        if user_agent.contains("Windows") {
            return (OperatingSystem::Windows, None);
        }
        if user_agent.contains("Macintosh") || user_agent.contains("Mac OS") || user_agent.contains("macOS") {
            return (OperatingSystem::MacOS, None);
        }
        if user_agent.contains("Linux") {
            return (OperatingSystem::Linux, None);
        }

        (OperatingSystem::Unknown, None)
    }

    /// Detect browser and version
    fn detect_browser(&self, user_agent: &str) -> (Browser, Option<String>) {
        for (pattern, extractor) in BROWSER_PATTERNS.iter() {
            if let Some(caps) = pattern.captures(user_agent) {
                if let Some(full_match) = caps.get(0) {
                    return extractor(full_match.as_str());
                }
            }
        }

        // Fallback: try basic detection
        if user_agent.contains("Edg") || user_agent.contains("Edge") {
            return (Browser::Edge, None);
        }
        if user_agent.contains("Chrome") && !user_agent.contains("Edg") && !user_agent.contains("OPR") {
            return (Browser::Chrome, None);
        }
        if user_agent.contains("Firefox") || user_agent.contains("FxiOS") {
            return (Browser::Firefox, None);
        }
        if user_agent.contains("Safari") && !user_agent.contains("Chrome") {
            return (Browser::Safari, None);
        }
        if user_agent.contains("Opera") || user_agent.contains("OPR") {
            return (Browser::Opera, None);
        }
        if user_agent.contains("MSIE") || user_agent.contains("Trident") {
            return (Browser::InternetExplorer, None);
        }

        (Browser::Unknown, None)
    }
}

impl Default for PlatformInfo {
    fn default() -> Self {
        Self::new(String::new())
    }
}

/// Helper function to extract named group from regex capture
fn extract_named_group(input: &str, group_name: &str) -> Option<String> {
    let pattern = Regex::new(&format!(r"(?P<{}>[\d._]+)", group_name)).ok()?;
    pattern.captures(input).map(|caps| caps[group_name].to_string())
}

/// Normalize Windows version numbers
fn normalize_windows_version(version: &str) -> String {
    match version {
        "10.0" => "10".to_string(),
        "11.0" => "11".to_string(),
        "6.3" => "8.1".to_string(),
        "6.2" => "8".to_string(),
        "6.1" => "7".to_string(),
        "6.0" => "Vista".to_string(),
        "5.1" => "XP".to_string(),
        v => v.to_string(),
    }
}

/// Parse user agent string and return platform information
///
/// # Example
///
/// ```
/// use rustok_core::platform_detection::parse_user_agent;
///
/// let ua = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36";
/// let info = parse_user_agent(ua);
///
/// assert_eq!(info.device_type.to_string(), "desktop");
/// assert_eq!(info.os.to_string(), "windows");
/// assert_eq!(info.browser.to_string(), "chrome");
/// ```
pub fn parse_user_agent(user_agent: &str) -> PlatformInfo {
    let detector = PlatformDetector::new();
    detector.detect(user_agent)
}

/// Quick check if user agent is a bot
///
/// This is faster than full parsing when you only need to know if it's a bot.
pub fn is_bot(user_agent: &str) -> bool {
    BOT_PATTERNS.iter().any(|pattern| pattern.is_match(user_agent))
}

/// Quick check if user agent is mobile
pub fn is_mobile(user_agent: &str) -> bool {
    MOBILE_PATTERNS.iter().any(|pattern| pattern.is_match(user_agent))
        && !TABLET_PATTERNS.iter().any(|pattern| pattern.is_match(user_agent))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_desktop_chrome_windows() {
        let ua = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36";
        let info = parse_user_agent(ua);

        assert_eq!(info.device_type, DeviceType::Desktop);
        assert_eq!(info.os, OperatingSystem::Windows);
        assert_eq!(info.browser, Browser::Chrome);
        assert!(!info.is_bot);
        assert!(!info.is_mobile());
        assert!(info.is_desktop());
    }

    #[test]
    fn test_mobile_android_chrome() {
        let ua = "Mozilla/5.0 (Linux; Android 10; SM-G973F) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.77 Mobile Safari/537.36";
        let info = parse_user_agent(ua);

        assert_eq!(info.device_type, DeviceType::Mobile);
        assert_eq!(info.os, OperatingSystem::Android);
        assert_eq!(info.browser, Browser::Chrome);
        assert!(!info.is_bot);
        assert!(info.is_mobile());
    }

    #[test]
    fn test_tablet_ipad() {
        let ua = "Mozilla/5.0 (iPad; CPU OS 14_6 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/14.1.1 Mobile/15E148 Safari/604.1";
        let info = parse_user_agent(ua);

        assert_eq!(info.device_type, DeviceType::Tablet);
        assert_eq!(info.os, OperatingSystem::iOS);
        assert_eq!(info.browser, Browser::Safari);
        assert!(!info.is_bot);
        assert!(info.is_tablet());
    }

    #[test]
    fn test_iphone_safari() {
        let ua = "Mozilla/5.0 (iPhone; CPU iPhone OS 14_6 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/14.1.1 Mobile/15E148 Safari/604.1";
        let info = parse_user_agent(ua);

        assert_eq!(info.device_type, DeviceType::Mobile);
        assert_eq!(info.os, OperatingSystem::iOS);
        assert_eq!(info.browser, Browser::Safari);
        assert!(!info.is_bot);
        assert!(info.is_mobile());
    }

    #[test]
    fn test_macos_safari() {
        let ua = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/14.1.1 Safari/605.1.15";
        let info = parse_user_agent(ua);

        assert_eq!(info.device_type, DeviceType::Desktop);
        assert_eq!(info.os, OperatingSystem::MacOS);
        assert_eq!(info.browser, Browser::Safari);
        assert!(!info.is_bot);
        assert!(info.is_desktop());
    }

    #[test]
    fn test_linux_firefox() {
        let ua = "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:89.0) Gecko/20100101 Firefox/89.0";
        let info = parse_user_agent(ua);

        assert_eq!(info.device_type, DeviceType::Desktop);
        assert_eq!(info.os, OperatingSystem::Linux);
        assert_eq!(info.browser, Browser::Firefox);
        assert!(!info.is_bot);
    }

    #[test]
    fn test_edge_windows() {
        let ua = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36 Edg/91.0.864.59";
        let info = parse_user_agent(ua);

        assert_eq!(info.device_type, DeviceType::Desktop);
        assert_eq!(info.os, OperatingSystem::Windows);
        assert_eq!(info.browser, Browser::Edge);
        assert!(!info.is_bot);
    }

    #[test]
    fn test_googlebot() {
        let ua = "Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)";
        let info = parse_user_agent(ua);

        assert_eq!(info.device_type, DeviceType::Bot);
        assert!(info.is_bot);
        assert!(!info.is_mobile());
    }

    #[test]
    fn test_facebook_bot() {
        let ua = "facebookexternalhit/1.1 (+http://www.facebook.com/externalhit_uatext.php)";
        let info = parse_user_agent(ua);

        assert!(info.is_bot);
    }

    #[test]
    fn test_unknown_user_agent() {
        let ua = "SomeCustomClient/1.0";
        let info = parse_user_agent(ua);

        assert_eq!(info.device_type, DeviceType::Unknown);
        assert_eq!(info.os, OperatingSystem::Unknown);
        assert_eq!(info.browser, Browser::Unknown);
        assert!(!info.is_bot);
    }

    #[test]
    fn test_empty_user_agent() {
        let ua = "";
        let info = parse_user_agent(ua);

        assert_eq!(info.device_type, DeviceType::Unknown);
        assert_eq!(info.os, OperatingSystem::Unknown);
        assert_eq!(info.browser, Browser::Unknown);
        assert!(!info.is_bot);
    }

    #[test]
    fn test_is_bot_helper() {
        assert!(is_bot("Mozilla/5.0 (compatible; Googlebot/2.1)"));
        assert!(is_bot("Mozilla/5.0 (compatible; bingbot/2.0)"));
        assert!(is_bot("facebookexternalhit/1.1"));
        assert!(!is_bot("Mozilla/5.0 (Windows NT 10.0) Chrome/91.0"));
    }

    #[test]
    fn test_is_mobile_helper() {
        assert!(is_mobile("Mozilla/5.0 (Linux; Android 10) Chrome/91.0"));
        assert!(is_mobile("Mozilla/5.0 (iPhone) Safari/605.1.15"));
        assert!(!is_mobile("Mozilla/5.0 (iPad) Safari/605.1.15"));
        assert!(!is_mobile("Mozilla/5.0 (Windows NT 10.0) Chrome/91.0"));
    }

    #[test]
    fn test_os_version_parsing() {
        let ua = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) Chrome/91.0";
        let info = parse_user_agent(ua);

        assert_eq!(info.os, OperatingSystem::Windows);
        assert_eq!(info.os_version, Some("10".to_string()));
    }

    #[test]
    fn test_browser_version_parsing() {
        let ua = "Mozilla/5.0 (Windows NT 10.0) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36";
        let info = parse_user_agent(ua);

        assert_eq!(info.browser, Browser::Chrome);
        assert_eq!(info.browser_version, Some("91.0.4472.124".to_string()));
    }
}
