use std::error::Error;
use std::str::FromStr;
use tokio::time::{sleep, Duration as tokDur};
use std::time::{Duration};
use url::Url;
use reqwest;


pub struct Config {
    interval: u64,
    url: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, Box<dyn Error>> {
        if args.len() != 3 {
            return Err(Box::try_from("bad args number").unwrap());
        }
        let interval = u64::from_str(&args[1])?;

        if let Err(e) =  Url::parse(&args[2].clone()) {
                println!("URL parsing error");
                return Err(Box::try_from(e).unwrap());
        }

        Ok(Config { interval: interval, url: args[2].clone() })
    }
}

fn display_return(status_code: u16, url: &str) {
   let res = if status_code == 200 {"OK"} else {"ERR"};
   println!("Checking '{}'. Result: {}({})", url, res, status_code);
}

pub async fn run(config: Config) -> Result<(), reqwest::Error> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(config.interval + 2)) // 2 is hardcoded delay picked randomly
        .build()?;
    loop {
            let status_code = match client.get(&config.url).send().await {
                Ok(r) => r.status().as_u16(),
                Err(e) => {
                    if e.is_timeout(){
                        display_return(0, &config.url);
                        continue
                    } else if e.is_connect() {
                        0
                    } else {
                        return Err(e);
                    }
                }
            };
            display_return(status_code, &config.url);
            sleep(tokDur::from_secs(config.interval)).await;
    }
}
