//! Skill and buff icons from NCSoft's game-data CDN.
//!
//! This is the private build's only download. The address has to be one file
//! under a fixed prefix, the response is not followed if it redirects, and the
//! bytes are kept only when they are an image.

use std::io::Read;
use std::time::Duration;

/// One megabyte. A skill icon is a few kilobytes; anything larger is refused.
pub const MAX_ICON_BYTES: u64 = 1024 * 1024;

const PREFIX: &str = "https://assets.playnccdn.com/static-aion2-gamedata/resources/";

/// `https://assets.playnccdn.com/static-aion2-gamedata/resources/<file>` with a
/// single image file name. No query, no other host, no `..`.
pub fn icon_url_allowed(url: &str) -> bool {
    let Some(name) = url.strip_prefix(PREFIX) else {
        return false;
    };
    if name.is_empty() || name.len() > 180 || name.contains(['/', '\\', '?', '#', '%']) || name.contains("..") {
        return false;
    }
    if !name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-' || c == '.') {
        return false;
    }
    let lower = name.to_ascii_lowercase();
    [".png", ".jpg", ".jpeg", ".gif", ".webp"].iter().any(|ext| lower.ends_with(ext))
}

/// The cache file name: the URL's last segment, which `icon_url_allowed` already
/// checked is a single plain file name.
pub fn cache_key(url: &str) -> Option<&str> {
    if !icon_url_allowed(url) {
        return None;
    }
    url.rsplit('/').next()
}

/// PNG, JPEG, GIF, or WebP, and not larger than [`MAX_ICON_BYTES`].
pub fn image_bytes_ok(bytes: &[u8]) -> bool {
    if bytes.is_empty() || bytes.len() as u64 > MAX_ICON_BYTES {
        return false;
    }
    let png = bytes.starts_with(&[0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A]);
    let jpeg = bytes.starts_with(&[0xFF, 0xD8, 0xFF]);
    let gif = bytes.starts_with(b"GIF87a") || bytes.starts_with(b"GIF89a");
    let webp = bytes.len() >= 12 && bytes.starts_with(b"RIFF") && &bytes[8..12] == b"WEBP";
    png || jpeg || gif || webp
}

/// Standard base64, as `share::base64` writes it and as the webview's `btoa` does.
pub fn decode_base64(input: &str) -> Option<Vec<u8>> {
    fn val(c: u8) -> Option<u8> {
        match c {
            b'A'..=b'Z' => Some(c - b'A'),
            b'a'..=b'z' => Some(c - b'a' + 26),
            b'0'..=b'9' => Some(c - b'0' + 52),
            b'+' => Some(62),
            b'/' => Some(63),
            _ => None,
        }
    }
    let bytes = input.as_bytes();
    if bytes.is_empty() {
        return Some(Vec::new());
    }
    if bytes.len() % 4 != 0 {
        return None;
    }
    let mut out = Vec::with_capacity(bytes.len() / 4 * 3);
    for chunk in bytes.chunks_exact(4) {
        let (a, b, c, d) = (chunk[0], chunk[1], chunk[2], chunk[3]);
        if a == b'=' || b == b'=' || (c == b'=' && d != b'=') {
            return None;
        }
        let av = val(a)?;
        let bv = val(b)?;
        let cv = if c == b'=' { 0 } else { val(c)? };
        let dv = if d == b'=' { 0 } else { val(d)? };
        let n = (u32::from(av) << 18) | (u32::from(bv) << 12) | (u32::from(cv) << 6) | u32::from(dv);
        out.push((n >> 16) as u8);
        if c != b'=' {
            out.push((n >> 8) as u8);
        }
        if d != b'=' {
            out.push(n as u8);
        }
    }
    Some(out)
}

/// A 2xx body that is an image. A redirect status is not.
pub fn icon_response_ok(status: u16, body: &[u8]) -> bool {
    (200..300).contains(&status) && image_bytes_ok(body)
}

pub(crate) fn icon_agent() -> ureq::Agent {
    // redirects(0): a 302 is an error, not a request to some other host.
    ureq::AgentBuilder::new()
        .redirects(0)
        .timeout(Duration::from_secs(15))
        .build()
}

/// GET one allowed icon URL. Does not follow redirects.
pub fn download_icon(url: &str) -> Result<Vec<u8>, String> {
    if !icon_url_allowed(url) {
        return Err("icon url not allowed".into());
    }
    let response = icon_agent().get(url).call().map_err(|e| e.to_string())?;
    let status = response.status();
    let mut body = Vec::new();
    response
        .into_reader()
        .take(MAX_ICON_BYTES + 1)
        .read_to_end(&mut body)
        .map_err(|e| e.to_string())?;
    if !icon_response_ok(status, &body) {
        return Err("not an image".into());
    }
    Ok(body)
}

#[cfg(test)]
mod tests {
    use super::*;

    const ICON: &str = "https://assets.playnccdn.com/static-aion2-gamedata/resources/ICON_CH_SKILL_Passive_007.png";

    #[test]
    fn only_one_image_file_under_the_game_data_prefix_is_allowed() {
        assert!(icon_url_allowed(ICON));
        assert_eq!(cache_key(ICON), Some("ICON_CH_SKILL_Passive_007.png"));
        assert!(icon_url_allowed(
            "https://assets.playnccdn.com/static-aion2-gamedata/resources/Icon_Item_Usable_Godstone_WP_r_00a.png"
        ));
        assert!(!icon_url_allowed("http://assets.playnccdn.com/static-aion2-gamedata/resources/a.png"));
        assert!(!icon_url_allowed("https://evil.example/static-aion2-gamedata/resources/a.png"));
        assert!(!icon_url_allowed("https://assets.playnccdn.com/other/a.png"));
        assert!(!icon_url_allowed(&format!("{ICON}?x=1")));
        assert!(!icon_url_allowed(
            "https://assets.playnccdn.com/static-aion2-gamedata/resources/../a.png"
        ));
        assert!(!icon_url_allowed(
            "https://assets.playnccdn.com/static-aion2-gamedata/resources/a.png/b.png"
        ));
        assert!(!icon_url_allowed(
            "https://assets.playnccdn.com/static-aion2-gamedata/resources/a.exe"
        ));
        assert!(cache_key("https://evil.example/a.png").is_none());
    }

    #[test]
    fn only_image_bytes_under_the_cap_are_kept() {
        assert!(image_bytes_ok(&[0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A, 0]));
        assert!(image_bytes_ok(&[0xFF, 0xD8, 0xFF, 0xD9]));
        assert!(image_bytes_ok(b"GIF89a"));
        assert!(image_bytes_ok(b"RIFF\0\0\0\0WEBP"));
        assert!(!image_bytes_ok(b"MZ\x90\0"));
        assert!(!image_bytes_ok(b"<!DOCTYPE html>"));
        assert!(!image_bytes_ok(b""));
        let mut big = vec![0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A];
        big.resize(MAX_ICON_BYTES as usize + 1, 0);
        assert!(!image_bytes_ok(&big));
        assert!(!icon_response_ok(302, &[0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A]));
        assert!(!icon_response_ok(200, b"MZ"));
    }

    #[test]
    fn base64_round_trips_an_icon() {
        let raw = [0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A];
        let encoded = crate::share::base64(&raw);
        assert_eq!(decode_base64(&encoded).as_deref(), Some(raw.as_slice()));
        assert!(decode_base64("@@@@").is_none());
        assert!(decode_base64("Zg=").is_none());
    }

    /// The same agent `download_icon` uses. A 302 must come back as an error,
    /// not as a second request to the Location.
    #[test]
    fn the_icon_agent_does_not_follow_a_redirect() {
        use std::io::{Read, Write};
        use std::net::TcpListener;
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        let server = std::thread::spawn(move || {
            let (mut sock, _) = listener.accept().unwrap();
            let mut buf = [0u8; 2048];
            let _ = sock.read(&mut buf);
            let _ = sock.write_all(
                b"HTTP/1.1 302 Found\r\nLocation: http://127.0.0.1/nope.png\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
            );
        });
        let err = icon_agent().get(&format!("http://127.0.0.1:{port}/icon.png")).call();
        match err {
            Err(ureq::Error::Status(code, _)) => assert_eq!(code, 302),
            other => panic!("expected the redirect status, got {other:?}"),
        }
        server.join().unwrap();
    }
}
