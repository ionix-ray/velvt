use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Config {
    #[serde(default = "default_addr")]
    pub addr: String,
    #[serde(default = "default_static_root")]
    pub static_root: String,
    #[serde(default = "default_index_path")]
    pub index_path: String,
    #[serde(default)]
    pub config_file: Option<PathBuf>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            addr: default_addr(),
            static_root: default_static_root(),
            index_path: default_index_path(),
            config_file: None,
        }
    }
}

fn default_addr() -> String {
    "0.0.0.0:8080".to_string()
}

fn default_static_root() -> String {
    "/srv".to_string()
}

fn default_index_path() -> String {
    "/srv/index.html".to_string()
}

#[derive(Clone, Default, Debug)]
pub struct CliArgs {
    pub addr: Option<String>,
    pub static_root: Option<String>,
}

impl Config {
    pub fn with_cli(mut self, args: CliArgs) -> Self {
        if let Some(addr) = args.addr {
            self.addr = addr;
        }
        if let Some(root) = args.static_root {
            self.static_root = root;
            self.index_path = format!("{}/index.html", self.static_root);
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_is_secure() {
        let cfg = Config::default();
        assert_eq!(cfg.addr, "0.0.0.0:8080");
        assert_eq!(cfg.static_root, "/srv");
        assert_eq!(cfg.index_path, "/srv/index.html");
    }

    #[test]
    fn cli_override_addr() {
        let cfg = Config::default().with_cli(CliArgs {
            addr: Some("127.0.0.1:3000".to_string()),
            ..Default::default()
        });
        assert_eq!(cfg.addr, "127.0.0.1:3000");
        assert_eq!(cfg.static_root, "/srv");
    }

    #[test]
    fn cli_override_static_root_updates_index_path() {
        let cfg = Config::default().with_cli(CliArgs {
            addr: None,
            static_root: Some("/custom".to_string()),
        });
        assert_eq!(cfg.static_root, "/custom");
        assert_eq!(cfg.index_path, "/custom/index.html");
    }
}
