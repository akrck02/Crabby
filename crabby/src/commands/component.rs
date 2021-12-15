use crate::logger;
use crate::io::class::instance;
use crate::common::constants;

use std::fs::File;
use std::fs;
use std::io::Write as _;

pub fn component(mut args : std::env::Args) {

    let name = args.nth(2).expect("No name given for component.");
    let view = args.nth(3).get_or_insert("".to_owned()).to_owned();

    if view.as_str().len() == 0 {
        if write_view_component(name, view).is_ok()  {
            logger::log("View component created.");
        } else {
            logger::log("View component creation failed.");
        }
        return;
    }

    if write_component(name.to_string()).is_ok() {
        logger::log("Component created.");
    } else {
        logger::log("Component creation failed.");
    }
   
}

fn write_component(name : String) -> std::io::Result<()>  {

    /*
        Create directory
    */
    let mut base_url : String = String::new(); 
    base_url.push_str(constants::BASE_DIR);
    base_url.push_str(constants::COMPONENTS);
  
    fs::create_dir_all(base_url.to_owned()).expect("Cannot create directory");

    /*    
        Create typescript file
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
    .writeln("public show() : Promise<any> {")
    .indent()
    .writeln("return new Promise((resolve, reject) => resolve('component is working!'));")
    .indent_back()
    .writeln("}")
    .indent_back()
    .writeln("}");

    return file.write_all(class.to_string().as_bytes());
}

/**
 * Write view component
 */
fn write_view_component(name : String, view : String) -> std::io::Result<()>  {
    /*
        Create directory
    */
    let mut base_url : String = String::new(); 
    base_url.push_str(constants::BASE_DIR);
    base_url.push_str(constants::VIEWS);
    base_url.push_str(view.as_str());
    base_url.push_str("/components/");
  
    fs::create_dir_all(base_url.to_owned()).expect("Cannot create directory");

    /*    
        Create typescript file
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
    .writeln("public show() : Promise<any> {")
    .indent()
    .writeln("return new Promise((resolve, reject) => resolve('component is working!'));")
    .indent_back()
    .writeln("}")
    .indent_back()
    .writeln("}");

    return file.write_all(class.to_string().as_bytes());
}