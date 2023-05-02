use std::env;
use std::process;
use mini_grep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let is_case_sensitive = env::var("CASE_SENSITIVE").is_err();

    println!("{}", is_case_sensitive);

    let config = Config::new(&args, is_case_sensitive).unwrap_or_else(|err| {
        eprintln!("Error parsing arg: {}", err);
        process::exit(1);
    });

    println!("Searching for: {:?}", config.query);
    println!("In file: {:?}", config.filename);

    if let Err(_) = mini_grep::run(config) {
        eprintln!("File doesn't exists");
        process::exit(1);
    } 
}
