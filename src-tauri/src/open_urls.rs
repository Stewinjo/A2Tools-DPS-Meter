//! Links the meter may hand to the operating system's browser.
//!
//! https only, on a fixed list of hosts, with no userinfo and no explicit port.
//! Anything else, including `file:` and a URL built to carry a character name
//! or a fight id to some other site, is refused. The upload preview may still
//! open its own folder on this computer.

use std::path::Path;

/// Registrable hosts. A link may use the host itself or a subdomain
/// (`www.paypal.me`, `logs.a2tools.app`).
pub const BROWSER_HOSTS: &[&str] = &[
    "a2tools.app",
    "afdian.com",
    "discord.com",
    "discord.gg",
    "exitlag.com",
    "github.com",
    "ko-fi.com",
    "nowpayments.io",
    "npcap.com",
    "patreon.com",
    "paypal.com",
    "paypal.me",
];

pub fn allowed_browser_url(raw: &str) -> bool {
    let Ok(url) = url::Url::parse(raw) else {
        return false;
    };
    if url.scheme() != "https" {
        return false;
    }
    if !url.username().is_empty() || url.password().is_some() {
        return false;
    }
    if url.port().is_some() {
        return false;
    }
    let Some(host) = url.host_str() else {
        return false;
    };
    let host = host.trim_end_matches('.').to_ascii_lowercase();
    if host.parse::<std::net::IpAddr>().is_ok() {
        return false;
    }
    BROWSER_HOSTS.iter().any(|allowed| host == *allowed || host.ends_with(&format!(".{allowed}")))
}

/// The path the share preview opens: that directory itself, not a file inside
/// it and not a `file:` URL.
pub fn is_share_preview_dir(given: &str, preview: &Path) -> bool {
    if given.contains("://") || given.to_ascii_lowercase().starts_with("file:") {
        return false;
    }
    let Ok(given) = Path::new(given).canonicalize() else {
        return false;
    };
    let Ok(preview) = preview.canonicalize() else {
        return false;
    };
    given == preview
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn support_links_open_and_everything_else_does_not() {
        assert!(allowed_browser_url("https://discord.gg/Aion2Global"));
        assert!(allowed_browser_url("https://ko-fi.com/hiddencube"));
        assert!(allowed_browser_url("https://github.com/taengu/A2Tools-DPS-Meter/releases"));
        assert!(allowed_browser_url("https://www.paypal.me/taengoo"));
        assert!(allowed_browser_url("https://www.patreon.com/hiddencube"));
        assert!(allowed_browser_url("https://www.exitlag.com/refer/A2TOOLS"));
        assert!(allowed_browser_url("https://afdian.com/a/hiddencube"));
        assert!(allowed_browser_url("https://npcap.com/#download"));
        assert!(allowed_browser_url("https://A2Tools.app"));
        assert!(allowed_browser_url("https://a2tools.app/#support"));
        assert!(allowed_browser_url("https://logs.a2tools.app/abc"));
        assert!(allowed_browser_url("https://nowpayments.io/donation/thehiddencube"));

        assert!(!allowed_browser_url("http://github.com/taengu/A2Tools-DPS-Meter"));
        assert!(!allowed_browser_url("file:///etc/passwd"));
        assert!(!allowed_browser_url("javascript:alert(1)"));
        assert!(!allowed_browser_url("https://user@github.com/"));
        assert!(!allowed_browser_url("https://github.com:8443/"));
        assert!(!allowed_browser_url("https://evil.example/a.png"));
        assert!(!allowed_browser_url("https://a2tools.app.evil.com/"));
        assert!(!allowed_browser_url("https://evila2tools.app/"));
        assert!(!allowed_browser_url("https://127.0.0.1/"));
        assert!(!allowed_browser_url("/home/me/share-preview"));
    }

    #[test]
    fn only_the_share_preview_directory_itself_opens_locally() {
        let root = std::env::temp_dir().join(format!("a2t-open-urls-{}", std::process::id()));
        let preview = root.join("share-preview");
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&preview).unwrap();
        let file = preview.join("fight.a2es");
        std::fs::write(&file, b"x").unwrap();
        assert!(is_share_preview_dir(&preview.display().to_string(), &preview));
        assert!(!is_share_preview_dir(&file.display().to_string(), &preview));
        assert!(!is_share_preview_dir(&format!("file://{}", preview.display()), &preview));
        assert!(!is_share_preview_dir(&root.display().to_string(), &preview));
        let _ = std::fs::remove_dir_all(&root);
    }
}
