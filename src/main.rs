extern crate getopts;
use getopts::Options;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optflag("f", "", "follow　postscript");
    opts.optopt("n", "number", "number of lines", "NUMBER");
    opts.optopt("c", "color", "change color to string", "COLOR");
    opts.optopt("w", "word", "highliht the word", "WORD");

    let arguments = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(why) => panic!("Error: {}", why),
    };
    let line = if arguments.opt_present("n") {
        let l = match arguments.opt_str("n") {
            Some(num) => num,
            None => panic!("check your number of lines."),
        };
        l.parse().unwrap()
    } else {
        1
    };
    match tail_all(&args[1], line) {
        Ok(()) => (),
        Err(err) => println!("Error: {}", err.to_string()),
    }
}

fn tail_all(file_name: &String, number: i32) -> Result<(), std::io::Error> {
    let mut file = File::open(file_name)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("{}", contents);
    Ok(())
}
