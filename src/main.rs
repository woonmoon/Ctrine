use std::env;
use std::fs;
 
fn main() {
    let args: Vec<String> = env::args().collect();
    // &args[0] is "target/debug/chocolat"
    let filename = &args[1];
    let program = fs::read_to_string(filename).expect("Something went wrong reading the file");
    println!("Got program \n {}", program);
}
