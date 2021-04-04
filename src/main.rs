use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In filen{}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    println!("The files contains:\n{}", contents);
}
