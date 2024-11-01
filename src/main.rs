use std::env;

mod xinghuo;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[0];
    
    

    println!("Hello, world!");
}
