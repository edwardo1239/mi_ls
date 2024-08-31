use std::env;
use std::process;
use mi_ls::{run, Config};


fn main(){
    // let args: Vec<String> = env::args().collect();

    let config = Config::build(env::args()).unwrap_or_else(|err | {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
    
}
