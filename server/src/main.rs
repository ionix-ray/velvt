use velvet_server::config::{CliArgs, Config};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let args = parse_args();
    let config = Config::default().with_cli(args);

    let app = velvet_server::app(config.clone());
    velvet_server::serve(app, &config.addr)
        .await
        .expect("server failed to start");
}

fn parse_args() -> CliArgs {
    let mut args = CliArgs::default();
    let mut it = std::env::args().skip(1);

    while let Some(arg) = it.next() {
        match arg.as_str() {
            "--addr" => args.addr = it.next(),
            "--static-root" => args.static_root = it.next(),
            _ => {}
        }
    }
    args
}
