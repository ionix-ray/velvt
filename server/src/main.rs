use velvet_server::config::{CliArgs, Config};

#[tokio::main]
async fn main() {
    let default_level = if std::env::var("DEBUG").is_ok() {
        "debug"
    } else {
        "error,velvet_server=info"
    };

    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(default_level));

    if std::env::var("DEBUG").is_ok() {
        tracing_subscriber::fmt()
            .with_env_filter(env_filter)
            .init();
    } else {
        tracing_subscriber::fmt()
            .json()
            .with_env_filter(env_filter)
            .init();
    }

    let args = parse_args();
    let config = Config::default().with_cli(args);

    let app = velvet_server::app(config.clone());
    if let Err(e) = velvet_server::serve(app, &config.addr).await {
        tracing::error!(error = %e, "server failed to start");
        std::process::exit(1);
    }
}

fn parse_args() -> CliArgs {
    parse_args_from(std::env::args().skip(1))
}

fn parse_args_from(mut it: impl Iterator<Item = String>) -> CliArgs {
    let mut args = CliArgs::default();

    while let Some(arg) = it.next() {
        match arg.as_str() {
            "--addr" => args.addr = it.next(),
            "--static-root" => args.static_root = it.next(),
            _ => {}
        }
    }
    args
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(items: &[&str]) -> CliArgs {
        parse_args_from(items.iter().map(|s| s.to_string()))
    }

    #[test]
    fn parses_known_flags() {
        let parsed = args(&["--addr", "127.0.0.1:9000", "--static-root", "/srv/web"]);
        assert_eq!(parsed.addr, Some("127.0.0.1:9000".to_string()));
        assert_eq!(parsed.static_root, Some("/srv/web".to_string()));
    }

    #[test]
    fn ignores_unknown_flags_and_defaults_to_none() {
        let parsed = args(&["--bogus", "value"]);
        assert_eq!(parsed.addr, None);
        assert_eq!(parsed.static_root, None);
    }

    #[test]
    fn flag_with_missing_value_yields_none() {
        let parsed = args(&["--addr"]);
        assert_eq!(parsed.addr, None);
    }
}
