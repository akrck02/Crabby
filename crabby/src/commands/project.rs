use crate::commands::crab;
use crate::common::constants;
use crate::logger;
use std::fs;

pub fn new(mut args: std::env::Args) {
    logger::title("---------------  Crabby GTDF Assistant v1.0  ---------------");
    crab::crab();
    logger::title("Generating project...");

    let name: String = args.nth(2).expect("No name given");
    let mut base_dir = constants::BASE_DIR.to_owned();
    base_dir.push_str(&name);
    base_dir.push_str("/");

    for route in constants::ROUTES {
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
}   
