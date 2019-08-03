extern crate getopts;
use getopts::Options;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optopt("c", "color", "change color to string ", "COLOR");
    opts.optopt("w", "word", "highliht the word", "WORD");
    println!("{:?}", args);

    match args.len() {
        1 => println!("specify target file."),
        2 => println!("collect args."),
        _ => println!("wrong args."),
    }
}
