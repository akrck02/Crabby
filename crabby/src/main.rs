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
        "new" | "n" => {
            commands::project::new(args);
        },
        "view" | "v" => {
            commands::view::view(args);
        },
        "component" | "c" => {
            commands::component::component(args);
        },
        "service" | "s" => {
            commands::service::service(args);
        },
        "test"| "t" => {
            commands::test::test(args);
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
    logger::clog(" n  |  new         -> Start a GTDF project");
    logger::clog(" v  |  view        -> Generate new view");
    logger::clog(" c  |  component   -> Generate new component");
    logger::clog(" s  |  service     -> Generate new service");
    logger::clog(" t  |  test        -> Generate new test");

    println!("");
}