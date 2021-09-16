pub fn title(title: &str) {
    println!("\n{}", "-".repeat(title.len() + 4));
    println!("  {}", title);
    println!("{}", "-".repeat(title.len() + 4));
}

pub fn log(msg: &str) {
    println!("[Crabby] {}", msg);
}

