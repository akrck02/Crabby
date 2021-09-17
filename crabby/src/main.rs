mod logger;
mod project;
mod view;
mod component;
mod service;
mod test;
mod crab;

fn main() {
    let pattern = std::env::args().nth(1).expect("no pattern given");
    let args = std::env::args();

    match pattern.as_str() {
        "crab" | "cb" => {
            crab::crab();
        },
        "new" | "n" => {
            project::new(args);
        },
        "view" | "v" => {
            view::view(args);
        },
        "component" | "c" => {
            component::component(args);
        },
        "service" | "s" => {
            service::service(args);
        },
        "test"| "t" => {
            test::test(args);
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
    logger::log("crab - display a crab");
    logger::log("start - start a GTD project");
    logger::log("exit - exit the program");

    println!("");
}