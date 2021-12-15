use crate::commands::crab;
use crate::logger;
use std::fs;

pub fn new(mut args: std::env::Args) {
    logger::title("---------------  Crabby GTDF Assistant v1.0  ---------------");
    crab::crab();
    logger::title("Generating project...");

    let name: String = args.nth(2).expect("No name given");
    let mut base_dir = "./".to_string();
    base_dir.push_str(&name);
    base_dir.push_str("/");

    let routes = [
        "",
        "api/",
        "app/",
        "db/",
        "docs/",
        "meta/",
        "resources/",
        "resources/audios",
        "resources/fonts",
        "resources/images",
        "resources/json",
        "resources/videos",
        "src/",
        "src/components/",
        "src/config/",
        "src/core/",
        "src/lib/",
        "src/services/",
        "src/views/",
        "style/",
        "temp/",
        "test/",
    ];

    for route in routes {
        let mut base_ref = base_dir.to_owned();
        base_ref.push_str(&route);

        if fs::create_dir_all(&base_ref).is_ok() {
            let msg = "[Created] directory: ".to_string() + &base_ref;
            logger::log(&msg);
        } else {
            let msg = "[Failed] Create directory: ".to_string() + &base_ref;
            logger::log(&msg);
        }
    }

    let api_dir = String::new();
    api_dir.to_string().push_str("../out/api");

    fs::create_dir_all(api_dir).expect("Failed to create api directory");
}
