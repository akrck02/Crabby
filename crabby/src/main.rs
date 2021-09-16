mod logger;

fn main() {
    let pattern = std::env::args().nth(1).expect("no pattern given");
    let args = std::env::args();

    match pattern.as_str() {
        "crab" | "cb" => {
            crab();
        },
        "new" | "n" => {
            new(args);
        },
        "view" | "v" => {
            view(args);
        },
        "component" | "c" => {
            component(args);
        },
        "service" | "s" => {
            service(args);
        },
        "test"| "t" => {
            test(args);
        },
        "help" | "h" => {
            help();
        },
        _ => {
            println!("[Crabby] No command matching {}", pattern);
        }
    }

    //let path = std::env::args().nth(2).expect("no path given");
}

fn help() {
    logger::title("Available commands:");
    logger::log("crab - display a crab");
    logger::log("start - start a GTD project");
    logger::log("exit - exit the program");

    println!("");
}


fn new(args : std::env::Args) {

    logger::title("---------------  Crabby GTDF Assistant v1.0  ---------------");
    crab();
    logger::log("Generating project...");
    




    logger::log("DONE.");

}

fn view(args : std::env::Args) {


    logger::log("View generated.");
}

fn component(args : std::env::Args) {

    logger::log("Component generated.");
}

fn service(args : std::env::Args) {

    logger::log("Service generated.");
}

fn test(args : std::env::Args) {

    logger::log("Test generated.");
}

fn crab() {
    logger::log("                                             ");
    logger::log("        #                   #######          ");
    logger::log("    #  ##                       ####         ");
    logger::log("   ##   ##                 ###     ###       ");
    logger::log("  ###  ###                   ###########     ");
    logger::log("  ###   ##                          ####     ");
    logger::log("    #####   ######################  ####     ");
    logger::log("     ###################################     ");
    logger::log("       #######  C  ########  C  #######      ");
    logger::log("         #####     ########     #######      ");
    logger::log("         #############################       ");
    logger::log("          ###########################        ");
    logger::log("         #####                   #####       ");
    logger::log("        #####                    ####        ");
    logger::log("        ####                       ###       ");
    logger::log("         ###                      ###        ");
    logger::log("          ##                     ##          ");
    logger::log("                                             ");
}