extern crate getopts;
use getopts::Options;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optflag("n", "", "number of lines");
    opts.optflag("f", "", "follow　postscript");
    opts.optopt("c", "color", "change color to string", "COLOR");
    opts.optopt("w", "word", "highliht the word", "WORD");
    let cmd_options = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };
    println!("{:?}", cmd_options);

    match args.len() {
        2 => match tail_all(&args[1]) {
            Ok(()) => (),
            Err(err) => println!("Error: {}", err.to_string()),
        },
        _ => {
            println!("Error: set file as args. (ex: dtail <FILE_NAME>)");
            process::exit(1);
        }
    }
}

fn tail_all(file_name: &String) -> Result<(), std::io::Error> {
    let mut file = File::open(file_name)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("{}", contents);
    Ok(())
}
