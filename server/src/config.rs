use serde::Deserialize;
use std::path::PathBuf;

/// Effective server configuration.
///
/// Built by layering, lowest priority first:
/// built-in defaults → TOML file → environment → CLI flags.
/// See [`Config::load`].
#[derive(Debug, Clone, PartialEq)]
pub struct Config {
    pub addr: String,
    pub static_root: String,
    /// SPA shell served for unmatched routes. When never explicitly set it
    /// stays derived as `<static_root>/index.html` and re-derives whenever
    /// `static_root` is overridden at any layer.
    pub index_path: String,
    /// Path of the TOML file the config was loaded from, if any.
    pub config_file: Option<PathBuf>,
    /// Per-request timeout in seconds (tower `TimeoutLayer` → HTTP 408).
    pub request_timeout_secs: u64,
    /// Maximum number of concurrently processed requests (tower
    /// `ConcurrencyLimitLayer`; excess requests queue).
    pub max_concurrent_requests: usize,
    /// Maximum request body size in bytes (axum `DefaultBodyLimit` → HTTP 413).
    pub max_body_bytes: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            addr: default_addr(),
            static_root: default_static_root(),
            index_path: "".to_string(),
            config_file: None,
            request_timeout_secs: default_request_timeout_secs(),
            max_concurrent_requests: default_max_concurrent_requests(),
            max_body_bytes: default_max_body_bytes(),
        }
    }
}

/// On-disk TOML schema. Every field is optional so the loader can tell
/// "absent from file" (`None`) from an explicit value — required to keep
/// `index_path` derivation (`<static_root>/index.html`) correct across
/// override layers.
#[derive(Debug, Default, Deserialize, PartialEq)]
struct FileConfig {
    #[serde(default)]
    addr: Option<String>,
    #[serde(default)]
    static_root: Option<String>,
    #[serde(default)]
    index_path: Option<String>,
    #[serde(default)]
    request_timeout_secs: Option<u64>,
    #[serde(default)]
    max_concurrent_requests: Option<usize>,
    #[serde(default)]
    max_body_bytes: Option<usize>,
}

/// File picked up automatically from the working directory, if present.
const DEFAULT_CONFIG_FILE: &str = "velvet-server.toml";

const ENV_CONFIG: &str = "VELVET_SERVER_CONFIG";
const ENV_ADDR: &str = "VELVET_SERVER_ADDR";
const ENV_STATIC_ROOT: &str = "VELVET_SERVER_STATIC_ROOT";
const ENV_INDEX_PATH: &str = "VELVET_SERVER_INDEX_PATH";
const ENV_REQUEST_TIMEOUT_SECS: &str = "VELVET_SERVER_REQUEST_TIMEOUT_SECS";
const ENV_MAX_CONCURRENT_REQUESTS: &str = "VELVET_SERVER_MAX_CONCURRENT_REQUESTS";
const ENV_MAX_BODY_BYTES: &str = "VELVET_SERVER_MAX_BODY_BYTES";

fn default_addr() -> String {
    "0.0.0.0:8080".to_string()
}

fn default_static_root() -> String {
    "/srv".to_string()
}

fn default_request_timeout_secs() -> u64 {
    10
}

fn default_max_concurrent_requests() -> usize {
    256
}

fn default_max_body_bytes() -> usize {
    2 * 1024 * 1024
}

#[derive(Clone, Default, Debug)]
pub struct CliArgs {
    pub addr: Option<String>,
    pub static_root: Option<String>,
    pub config_file: Option<PathBuf>,
    pub index_path: Option<String>,
    pub request_timeout_secs: Option<u64>,
    pub max_concurrent_requests: Option<usize>,
    pub max_body_bytes: Option<usize>,
}

impl Config {
    /// Derive a `PathBuf` from `static_root` + "index.html".
    pub fn index_html(&self) -> std::path::PathBuf {
        if self.index_path.is_empty() {
            std::path::PathBuf::from(&self.static_root).join("index.html")
        } else {
            std::path::PathBuf::from(&self.index_path)
        }
    }
    /// Load configuration with precedence CLI flag > environment variable >
    /// TOML file > built-in default.
    ///
    /// The config file is resolved from (a) the `--config` CLI flag, (b) the
    /// `VELVET_SERVER_CONFIG` env var, or (c) `velvet-server.toml` in the
    /// current directory — (c) only when the file exists. A missing or
    /// malformed explicit file logs an error and degrades to defaults rather
    /// than aborting startup. One-shot startup work: read synchronously
    /// before the server accepts connections.
    pub fn load(cli: CliArgs) -> Config {
        Self::load_with(
            cli,
            |key| std::env::var(key).ok(),
            || std::env::current_dir().ok(),
        )
    }

    /// [`Config::load`] with injectable env/cwd lookups for deterministic tests.
    pub(crate) fn load_with(
        cli: CliArgs,
        env: impl Fn(&str) -> Option<String>,
        cwd: impl Fn() -> Option<PathBuf>,
    ) -> Config {
        let mut cfg = Config::default();
        let mut index_explicit = false;

        // Layer 1: TOML file — `--config` flag > VELVET_SERVER_CONFIG env >
        // ./velvet-server.toml (default candidate only considered when present).
        let explicit = cli
            .config_file
            .clone()
            .or_else(|| env(ENV_CONFIG).map(PathBuf::from));
        let resolved = match explicit {
            Some(path) => Some((path, true)),
            None => cwd()
                .map(|dir| dir.join(DEFAULT_CONFIG_FILE))
                .filter(|path| path.is_file())
                .map(|path| (path, false)),
        };
        if let Some((path, explicitly_requested)) = resolved {
            match std::fs::read_to_string(&path) {
                Ok(text) => match toml::from_str::<FileConfig>(&text) {
                    Ok(file) => {
                        apply_file(&mut cfg, file, &mut index_explicit);
                        cfg.config_file = Some(path);
                    }
                    Err(e) => tracing::error!(
                        path = %path.display(),
                        error = %e,
                        "config file is malformed — using built-in defaults"
                    ),
                },
                Err(e) if explicitly_requested => tracing::error!(
                    path = %path.display(),
                    error = %e,
                    "config file unreadable — using built-in defaults"
                ),
                Err(e) => tracing::warn!(
                    path = %path.display(),
                    error = %e,
                    "default config file unreadable — using built-in defaults"
                ),
            }
        }

        // Layer 2: environment variables.
        apply_env(&mut cfg, &mut index_explicit, &env);

        // Layer 3: CLI flags (highest priority).
        cfg.apply_cli(&cli, index_explicit);
        cfg
    }

    pub fn with_cli(mut self, args: CliArgs) -> Self {
        self.apply_cli(&args, false);
        self
    }

    fn apply_cli(&mut self, args: &CliArgs, index_explicit: bool) {
        if let Some(addr) = &args.addr {
            self.addr = addr.clone();
        }
        if let Some(root) = &args.static_root {
            self.static_root = root.clone();
            if !index_explicit {
                self.index_path = "".to_string();
            }
        }
        if let Some(index) = &args.index_path {
            self.index_path = index.clone();
        }
        if let Some(secs) = args.request_timeout_secs {
            self.request_timeout_secs = secs;
        }
        if let Some(max) = args.max_concurrent_requests {
            self.max_concurrent_requests = max;
        }
        if let Some(max) = args.max_body_bytes {
            self.max_body_bytes = max;
        }
    }
}

/// Apply the file layer. `index_path` from the file wins over the
/// `<static_root>/index.html` derivation; the derivation only kicks in when
/// the file overrides `static_root` without specifying `index_path`.
fn apply_file(cfg: &mut Config, file: FileConfig, index_explicit: &mut bool) {
    if let Some(addr) = file.addr {
        cfg.addr = addr;
    }
    if let Some(root) = file.static_root {
        cfg.static_root = root;
        if !*index_explicit {
            cfg.index_path = "".to_string();
        }
    }
    if let Some(index) = file.index_path {
        cfg.index_path = index;
        *index_explicit = true;
    }
    if let Some(secs) = file.request_timeout_secs {
        cfg.request_timeout_secs = secs;
    }
    if let Some(max) = file.max_concurrent_requests {
        cfg.max_concurrent_requests = max;
    }
    if let Some(max) = file.max_body_bytes {
        cfg.max_body_bytes = max;
    }
}

fn apply_env(cfg: &mut Config, index_explicit: &mut bool, env: &impl Fn(&str) -> Option<String>) {
    if let Some(addr) = env(ENV_ADDR) {
        cfg.addr = addr;
    }
    if let Some(root) = env(ENV_STATIC_ROOT) {
        cfg.static_root = root;
        if !*index_explicit {
            cfg.index_path = "".to_string();
        }
    }
    if let Some(index) = env(ENV_INDEX_PATH) {
        cfg.index_path = index;
        *index_explicit = true;
    }
    if let Some(secs) =
        parse_or_warn::<u64>(ENV_REQUEST_TIMEOUT_SECS, env(ENV_REQUEST_TIMEOUT_SECS))
    {
        cfg.request_timeout_secs = secs;
    }
    if let Some(max) = parse_or_warn::<usize>(
        ENV_MAX_CONCURRENT_REQUESTS,
        env(ENV_MAX_CONCURRENT_REQUESTS),
    ) {
        cfg.max_concurrent_requests = max;
    }
    if let Some(max) = parse_or_warn::<usize>(ENV_MAX_BODY_BYTES, env(ENV_MAX_BODY_BYTES)) {
        cfg.max_body_bytes = max;
    }
}

/// Parse an env override, logging and dropping invalid values instead of
/// failing startup.
fn parse_or_warn<T>(key: &'static str, raw: Option<String>) -> Option<T>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
{
    let value = raw?;
    match value.parse::<T>() {
        Ok(parsed) => Some(parsed),
        Err(e) => {
            tracing::warn!(key, value = %value, error = %e, "invalid env override — using default");
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::path::Path;

    type TestResult = Result<(), Box<dyn std::error::Error>>;

    fn temp_dir(name: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
        let dir =
            std::env::temp_dir().join(format!("velvet-config-test-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir)?;
        Ok(dir)
    }

    fn write_config(dir: &Path, contents: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
        let path = dir.join(DEFAULT_CONFIG_FILE);
        std::fs::write(&path, contents)?;
        Ok(path)
    }

    /// Deterministic env lookup backed by a map.
    fn fake_env(pairs: Vec<(&'static str, String)>) -> impl Fn(&str) -> Option<String> {
        let map: HashMap<String, String> =
            pairs.into_iter().map(|(k, v)| (k.to_string(), v)).collect();
        move |key| map.get(key).cloned()
    }

    fn no_env() -> impl Fn(&str) -> Option<String> {
        |_| None
    }

    fn no_cwd() -> Option<PathBuf> {
        None
    }

    #[test]
    fn default_config_is_secure() {
        let cfg = Config::default();
        assert_eq!(cfg.addr, "0.0.0.0:8080");
        assert_eq!(cfg.static_root, "/srv");
        // Empty ⇒ derived from static_root at router build (see `index_html`).
        assert_eq!(cfg.index_path, "");
        assert_eq!(cfg.index_html(), PathBuf::from("/srv/index.html"));
        assert_eq!(cfg.request_timeout_secs, 10);
        assert_eq!(cfg.max_concurrent_requests, 256);
        assert_eq!(cfg.max_body_bytes, 2 * 1024 * 1024);
        assert_eq!(cfg.config_file, None);
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
    fn cli_override_static_root_re_derives_index_path() {
        let cfg = Config::default().with_cli(CliArgs {
            addr: None,
            static_root: Some("/custom".to_string()),
            ..Default::default()
        });
        assert_eq!(cfg.static_root, "/custom");
        assert_eq!(cfg.index_path, "");
        assert_eq!(cfg.index_html(), PathBuf::from("/custom/index.html"));
    }

    #[test]
    fn cli_explicit_index_path_beats_static_root_derivation() {
        let cfg = Config::default().with_cli(CliArgs {
            static_root: Some("/custom".to_string()),
            index_path: Some("/custom/shell.html".to_string()),
            ..Default::default()
        });
        assert_eq!(cfg.index_html(), PathBuf::from("/custom/shell.html"));
    }

    #[test]
    fn load_parses_valid_toml_file() -> TestResult {
        let dir = temp_dir("valid")?;
        let path = write_config(
            &dir,
            r#"
                addr = "127.0.0.1:9001"
                static_root = "/srv/web"
                index_path = "/srv/web/shell.html"
                request_timeout_secs = 30
                max_concurrent_requests = 64
                max_body_bytes = 4096
            "#,
        )?;
        let path_str = path.to_string_lossy().into_owned();

        let cfg = Config::load_with(
            CliArgs::default(),
            fake_env(vec![(ENV_CONFIG, path_str)]),
            no_cwd,
        );

        assert_eq!(cfg.addr, "127.0.0.1:9001");
        assert_eq!(cfg.static_root, "/srv/web");
        assert_eq!(cfg.index_path, "/srv/web/shell.html");
        assert_eq!(cfg.index_html(), PathBuf::from("/srv/web/shell.html"));
        assert_eq!(cfg.request_timeout_secs, 30);
        assert_eq!(cfg.max_concurrent_requests, 64);
        assert_eq!(cfg.max_body_bytes, 4096);
        assert_eq!(cfg.config_file, Some(path));

        let _ = std::fs::remove_dir_all(&dir);
        Ok(())
    }

    #[test]
    fn missing_config_file_falls_back_to_defaults() -> TestResult {
        let dir = temp_dir("missing")?;
        let missing = dir.join("does-not-exist.toml");
        let path_str = missing.to_string_lossy().into_owned();

        let cfg = Config::load_with(
            CliArgs::default(),
            fake_env(vec![(ENV_CONFIG, path_str)]),
            no_cwd,
        );

        assert_eq!(cfg, Config::default());

        let _ = std::fs::remove_dir_all(&dir);
        Ok(())
    }

    #[test]
    fn malformed_toml_logs_error_and_uses_defaults() -> TestResult {
        crate::test_support::install_capture_once();
        crate::test_support::clear_log();

        let dir = temp_dir("malformed")?;
        let path = write_config(&dir, "addr = [this is not toml")?;
        let path_str = path.to_string_lossy().into_owned();

        let cfg = Config::load_with(
            CliArgs::default(),
            fake_env(vec![(ENV_CONFIG, path_str)]),
            no_cwd,
        );

        assert_eq!(cfg, Config::default());
        let log = crate::test_support::take_log();
        assert!(log.contains("ERROR"), "expected an error log, got: {log}");
        assert!(log.contains("malformed"), "expected cause, got: {log}");

        let _ = std::fs::remove_dir_all(&dir);
        Ok(())
    }

    #[test]
    fn cli_flag_beats_env_and_file() -> TestResult {
        let dir = temp_dir("precedence")?;
        let path = write_config(&dir, r#"addr = "1.1.1.1:1""#)?;
        let path_str = path.to_string_lossy().into_owned();

        // File only → file value.
        let from_file = Config::load_with(
            CliArgs::default(),
            fake_env(vec![(ENV_CONFIG, path_str.clone())]),
            no_cwd,
        );
        assert_eq!(from_file.addr, "1.1.1.1:1");

        // Env overrides file.
        let from_env = Config::load_with(
            CliArgs::default(),
            fake_env(vec![
                (ENV_CONFIG, path_str),
                (ENV_ADDR, "2.2.2.2:2".to_string()),
            ]),
            no_cwd,
        );
        assert_eq!(from_env.addr, "2.2.2.2:2");

        // CLI overrides env (and file).
        let from_cli = Config::load_with(
            CliArgs {
                addr: Some("3.3.3.3:3".to_string()),
                ..Default::default()
            },
            fake_env(vec![
                (ENV_CONFIG, path.to_string_lossy().into_owned()),
                (ENV_ADDR, "2.2.2.2:2".to_string()),
            ]),
            no_cwd,
        );
        assert_eq!(from_cli.addr, "3.3.3.3:3");

        let _ = std::fs::remove_dir_all(&dir);
        Ok(())
    }

    #[test]
    fn default_file_in_cwd_used_when_present() -> TestResult {
        let dir = temp_dir("cwd-default")?;
        write_config(&dir, r#"static_root = "/srv/from-cwd""#)?;

        let cwd = dir.clone();
        let cfg = Config::load_with(CliArgs::default(), no_env(), move || Some(cwd.clone()));

        assert_eq!(cfg.static_root, "/srv/from-cwd");
        // No explicit source: config_file records the resolved default path.
        assert_eq!(cfg.config_file, Some(dir.join(DEFAULT_CONFIG_FILE)));

        let _ = std::fs::remove_dir_all(&dir);
        Ok(())
    }

    #[test]
    fn no_file_anywhere_yields_plain_defaults() -> TestResult {
        let dir = temp_dir("no-file")?;
        let cwd = dir.clone();
        let cfg = Config::load_with(CliArgs::default(), no_env(), move || Some(cwd.clone()));
        assert_eq!(cfg, Config::default());
        let _ = std::fs::remove_dir_all(&dir);
        Ok(())
    }

    #[test]
    fn file_static_root_leaves_index_path_unsettled_for_derivation() -> TestResult {
        let dir = temp_dir("derive-index")?;
        let path = write_config(&dir, r#"static_root = "/srv/derived""#)?;
        let path_str = path.to_string_lossy().into_owned();

        let cfg = Config::load_with(
            CliArgs::default(),
            fake_env(vec![(ENV_CONFIG, path_str)]),
            no_cwd,
        );
        assert_eq!(cfg.index_path, "");
        assert_eq!(cfg.index_html(), PathBuf::from("/srv/derived/index.html"));

        let _ = std::fs::remove_dir_all(&dir);
        Ok(())
    }

    #[test]
    fn env_index_path_override_wins_over_static_root_derivation() {
        // Explicit env index path survives an env static_root override…
        let explicit = Config::load_with(
            CliArgs::default(),
            fake_env(vec![
                (ENV_STATIC_ROOT, "/env/root".to_string()),
                (ENV_INDEX_PATH, "/env/shell.html".to_string()),
            ]),
            no_cwd,
        );
        assert_eq!(explicit.static_root, "/env/root");
        assert_eq!(explicit.index_html(), PathBuf::from("/env/shell.html"));

        // …while an env root without an index override re-derives at use.
        let derived = Config::load_with(
            CliArgs::default(),
            fake_env(vec![(ENV_STATIC_ROOT, "/env/root".to_string())]),
            no_cwd,
        );
        assert_eq!(derived.index_path, "");
        assert_eq!(derived.index_html(), PathBuf::from("/env/root/index.html"));
    }

    #[test]
    fn env_overrides_every_new_tunable() {
        let cfg = Config::load_with(
            CliArgs::default(),
            fake_env(vec![
                (ENV_REQUEST_TIMEOUT_SECS, "5".to_string()),
                (ENV_MAX_CONCURRENT_REQUESTS, "8".to_string()),
                (ENV_MAX_BODY_BYTES, "1234".to_string()),
            ]),
            no_cwd,
        );
        assert_eq!(cfg.request_timeout_secs, 5);
        assert_eq!(cfg.max_concurrent_requests, 8);
        assert_eq!(cfg.max_body_bytes, 1234);
    }

    #[test]
    fn invalid_numeric_env_falls_back_to_default() {
        let cfg = Config::load_with(
            CliArgs::default(),
            fake_env(vec![(ENV_REQUEST_TIMEOUT_SECS, "not-a-number".to_string())]),
            no_cwd,
        );
        assert_eq!(cfg.request_timeout_secs, 10);
    }

    #[test]
    fn cli_overrides_new_tunables() {
        let cfg = Config::load_with(
            CliArgs {
                request_timeout_secs: Some(7),
                max_concurrent_requests: Some(9),
                max_body_bytes: Some(100),
                ..Default::default()
            },
            no_env(),
            no_cwd,
        );
        assert_eq!(cfg.request_timeout_secs, 7);
        assert_eq!(cfg.max_concurrent_requests, 9);
        assert_eq!(cfg.max_body_bytes, 100);
    }

    #[test]
    fn production_load_returns_defaults_when_no_source_present() {
        // Guard against ambient state that would make this nondeterministic.
        let default_file = std::env::current_dir()
            .map(|dir| dir.join(DEFAULT_CONFIG_FILE))
            .unwrap_or_default();
        if std::env::var(ENV_CONFIG).is_ok() || default_file.is_file() {
            return;
        }
        let cfg = Config::load(CliArgs::default());
        assert_eq!(cfg, Config::default());
    }
}
