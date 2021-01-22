extern crate pretty_env_logger;
#[macro_use] extern crate lazy_static;

use std::{env, fs, io, collections::HashMap};
use warp::Filter;
use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    targets: HashMap<String, Target>,
}

#[derive(Deserialize)]
struct Target {
    key: String,
    dest: String,
}

impl Config {
    // Load the configuration.
    pub fn new() -> Result<Self, io::Error> {
        let config_data = fs::read("diplo.toml")?;
        let config: Config = toml::from_slice(&config_data)?;
        Ok(config)
    }
}

lazy_static! {
    static ref CONFIG: Config = Config::new().expect("could not load config");
}

#[tokio::main]
async fn main() {
    // Log at the info level by default.
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "diplo=info");
    }
    pretty_env_logger::init();

    println!("{}", CONFIG.targets["foo"].key);

    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));

    let routes = hello.with(warp::log("diplo"));
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
