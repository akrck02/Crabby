mod commands;
mod io;
mod logger;

fn main() {
    let pattern = std::env::args().nth(1).expect("no pattern given");
    let args = std::env::args();

    match pattern.as_str() {
        "crab" | "cb" => {
            commands::crab::crab();
        },
        "help" | "h" => {
            help();
        },
        _ => {
            println!("[Crabby] No command matching {}", pattern);
        }
    }
}

fn help() {
    logger::title("Available commands:");
    logger::clog(" cb |  crab        -> Display ASCII crabby");
    println!("");
}