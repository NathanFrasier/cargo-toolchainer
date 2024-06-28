use regex::Regex;
use toml::Table;

#[derive(Clone, PartialEq, clap::ValueEnum)]
pub enum ChannelFormat {
    Stable,
    Beta,
    Nightly,
    VersionLiteral,
}

impl ChannelFormat {
    /// detects which channel a given string is using, ie nightly, stable, a specific version, and
    /// if that channel is "anchored" meaning that it will always refer to the same version
    pub fn detect_channel(channel_string: &str) -> (ChannelFormat, bool) {
        let r = Regex::new(r"(stable|nightly|beta|\d+\.\d+\.\d+)(-\d+-\d+-\d+)?").unwrap();

        let captures = r.captures(channel_string).unwrap();
        let channel_text = captures.get(1).unwrap();
        let channel_enum = match channel_text.as_str() {
            "stable" => Self::Stable,
            "beta" => Self::Beta,
            "nightly" => Self::Nightly,
            _ => Self::VersionLiteral,
        };
        let is_anchored = captures.get(2).is_some() || channel_enum == Self::VersionLiteral;
        (channel_enum, is_anchored)
    }

    pub fn get_latest(&self) -> String {
        let manifest_url = self.manifest_url();

        // get the latest rust version
        let rust_manifest = reqwest::blocking::get(manifest_url)
            .expect("failed to retrieve current rust manifest")
            .text()
            .expect("failed to decode current rust manifest")
            .parse::<Table>()
            .expect("failed to parse rust manifest toml file");

        let rustc_url = rust_manifest
            .get("artifacts")
            .and_then(|a| a.get("source-code"))
            .and_then(|c| c.get("target"))
            .and_then(|t| t.get("*"))
            .and_then(|a| a.get(0))
            .and_then(|i| i.get("url"))
            .and_then(|url| url.as_str())
            .expect("couldn't find URL to extract rust version from");

        match self {
            Self::VersionLiteral => {
                //Version Literal Implies Stable, since the other releases don't have version
                //numbers yet

                let version_regex = Regex::new(r"rustc-(.*)-src").unwrap();
                version_regex
                    .captures(rustc_url)
                    .unwrap()
                    .get(1)
                    .unwrap()
                    .as_str()
                    .to_string()
            }
            Self::Stable => {
                let date_regex = Regex::new(r"dist\/(\d+-\d+-\d+)\/rustc").unwrap();
                [
                    "stable",
                    date_regex
                        .captures(rustc_url)
                        .unwrap()
                        .get(1)
                        .unwrap()
                        .as_str(),
                ]
                .join("-")
            }
            Self::Beta => {
                let date_regex = Regex::new(r"dist\/(\d+-\d+-\d+)\/rustc").unwrap();
                [
                    "beta",
                    date_regex
                        .captures(rustc_url)
                        .unwrap()
                        .get(1)
                        .unwrap()
                        .as_str(),
                ]
                .join("-")
            }
            Self::Nightly => {
                let date_regex = Regex::new(r"dist\/(\d+-\d+-\d+)\/rustc").unwrap();
                [
                    "nightly",
                    date_regex
                        .captures(rustc_url)
                        .unwrap()
                        .get(1)
                        .unwrap()
                        .as_str(),
                ]
                .join("-")
            }
        }
    }

    fn manifest_url(&self) -> &'static str {
        match self {
            Self::Stable | Self::VersionLiteral => {
                "https://static.rust-lang.org/dist/channel-rust-stable.toml"
            }
            Self::Beta => "https://static.rust-lang.org/dist/channel-rust-beta.toml",
            Self::Nightly => "https://static.rust-lang.org/dist/channel-rust-nightly.toml",
        }
    }
}
