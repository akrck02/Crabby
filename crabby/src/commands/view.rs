use crate::logger;
use crate::io::class::instance;

use std::fs::File;
use std::fs;
use std::io::Write as _;




pub fn view(mut args : std::env::Args) {

    let name = args.nth(2).expect("No name given for view.");
    
    if write_file(name.to_string()).is_ok() {
        logger::log("View generated.");
    } else {
        logger::log("Cannot generate view");
    } 
   
}

fn write_file(name : String) -> std::io::Result<()>  {


    /*
        Create view directory
    */
    let mut base_url : String = String::new(); 
    base_url.push_str("./views/");
    base_url.push_str(name.as_str());
    base_url.push_str("/");
  
    fs::create_dir_all(base_url.to_owned()).expect("Cannot create directory");

    /*
        Create view typescript file
    */
    let mut url : String = String::new(); 
    url.push_str(base_url.to_owned().as_str());
    url.push_str(name.as_str());
    url.push_str(".ts");

    let mut file = File::create(url)?;


    /* 
        Start writing typescript file
    */
    let mut class = instance();
    let mut line = String::new();
    line.push_str("export default class ");
    line.push_str(name.to_owned().as_str());
    line.push_str(" {");

    class.writeln(line.as_str())
    .indent()
    .writeln("public show() : void {")
    .writeln("")
    .writeln("}")
    .indent_back()
    .writeln("}");

    return file.write_all(class.to_string().as_bytes());
}