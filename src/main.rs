extern crate getopts;
use getopts::Options;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optopt("-n", "", "specify line of file", "LINE");
    opts.optopt("-f", "", "follow　postscript", "FOLLOW");
    opts.optopt("c", "color", "change color to string", "COLOR");
    opts.optopt("w", "word", "highliht the word", "WORD");

    match args.len() {
        2 => println!("collect args."),
        _ => {
            println!("Error: set file as args. (ex: dtail <FILE_NAME>)");
            process::exit(1);
        }
    }
}
