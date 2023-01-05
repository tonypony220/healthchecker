use std::env;
use std::process;
use healthchecker::Config;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|_err| {
        process::exit(1);
    });
    if let Err(_err) = healthchecker::run(config).await {
        process::exit(1);
    }
}