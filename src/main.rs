extern crate pretty_env_logger;

use std::{env, fs, collections::HashMap};
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

#[tokio::main]
async fn main() {
    // Log at the info level by default.
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "diplo=info");
    }
    pretty_env_logger::init();

    // Load config.
    let config_data = fs::read("diplo.toml").unwrap();
    let config: Config = toml::from_slice(&config_data).unwrap();
    println!("{}", config.targets["foo"].key);

    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));

    let routes = hello.with(warp::log("diplo"));
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
