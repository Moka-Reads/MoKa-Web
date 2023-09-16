use mokareads_core::Result;
use reqwest::header;
use serde::Deserialize;
use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Platforms {
    Linux,
    Debian,
    WindowsExe,
    WindowsMSI,
    Darwin,
    Source,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version(u8, u8, u8);

impl Version {
    pub fn parse(s: &str) -> Option<Self> {
        let mut split = s.split('.');
        let major = split.next()?.parse().ok()?;
        let minor = split.next()?.parse().ok()?;
        let patch = split.next()?.parse().ok()?;
        Some(Self(major, minor, patch))
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.0, self.1, self.2)
    }
}

impl Platforms {
    pub fn file_name(&self, version: Version) -> String {
        match self {
            Platforms::Linux => format!("moka-desktop_{}_amd64.AppImage", version),
            Platforms::Debian => format!("moka-desktop_{}_amd64.deb", version),
            Platforms::WindowsExe => format!("moka-desktop_{}_x64-setup.exe", version),
            Platforms::WindowsMSI => format!("moka-desktop_{}_x64_en-US.msi", version),
            Platforms::Darwin => format!("moka-desktop_{}_x64.dmg", version),
            Platforms::Source => format!("moka-desktop_x64.app.tar.gz"),
        }
    }
    pub fn download_link(&self, version: Version) -> String {
        format!(
            "https://github.com/Moka-Reads/MoKa-Desktop/releases/download/v{version}/{}",
            self.file_name(version)
        )
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "linux" => Some(Self::Linux),
            "debian" => Some(Self::Debian),
            "windows-exe" => Some(Self::WindowsExe),
            "windows-msi" => Some(Self::WindowsMSI),
            "darwin" => Some(Self::Darwin),
            "source" => Some(Self::Source),
            _ => None,
        }
    }
}

/// `/download/<platform>/<version>`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Downloader {
    platform: Platforms,
    version: Version,
}

impl Downloader {
    pub fn new(platform: Platforms, version: Version) -> Self {
        Self { platform, version }
    }
    pub fn download_link(&self) -> String {
        self.platform.download_link(self.version)
    }
}

#[derive(Debug, Deserialize)]
pub struct GitHubTag {
    name: String,
}

impl GitHubTag {
    pub async fn fetch_tags() -> Result<Vec<String>> {
        let user_agent = header::HeaderValue::from_static("MoKa-Tags");
        let client = reqwest::Client::builder()
            .default_headers({
                let mut headers = header::HeaderMap::new();
                headers.insert(header::USER_AGENT, user_agent);
                headers
            })
            .build()?;
        let resp = client
            .get("https://api.github.com/repos/Moka-Reads/MoKa-Desktop/tags")
            .send()
            .await
            .map_err(|_| "Failed to send request")?;

        if !resp.status().is_success() {
            return Err(format!("Received a {} from server", resp.status()).into());
        }

        let tags: Vec<GitHubTag> = resp
            .json()
            .await
            .map_err(|_| "Failed to deserialize response")?;

        let tag_names: Vec<String> = tags
            .into_iter()
            .map(|tag| tag.name.replace("v", ""))
            .collect();
        Ok(tag_names)
    }
}

#[test]
fn test_sort() {
    let mut versions = vec![Version(0, 0, 1), Version(1, 1, 0), Version(0, 1, 0)];
    let sorted = vec![Version(0, 0, 1), Version(0, 1, 0), Version(1, 1, 0)];

    versions.sort();
    assert_eq!(versions, sorted);
}
