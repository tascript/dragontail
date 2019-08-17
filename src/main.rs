extern crate getopts;
use getopts::Options;
use memmap::MmapOptions;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optflag("f", "", "follow　postscript");
    opts.optopt("n", "number", "number of lines", "NUMBER");
    opts.optopt("c", "color", "change color to string", "COLOR");
    opts.optopt("w", "word", "highlight the word", "WORD");

    let arguments = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(why) => panic!("Error: {}", why),
    };
    let line: i32 = if arguments.opt_present("n") {
        let l = match arguments.opt_str("n") {
            Some(num) => num,
            None => panic!("check your number of lines."),
        };
        l.parse().unwrap()
    } else {
        10
    };
    match tail_with_line(&args[1], line) {
        Ok(()) => (),
        Err(err) => println!("Error: {}", err.to_string()),
    }
}

fn tail_all(file_name: &String) -> Result<(), std::io::Error> {
    let mut file = File::open(file_name)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("{}", contents);
    Ok(())
}

fn tail_with_line(file_name: &String, line: i32) -> Result<(), std::io::Error> {
    let file = File::open(file_name)?;
    let mmap = unsafe { MmapOptions::new().map(&file)? };
    let f = BufReader::new(file);
    for line in f.lines() {
        println!("{}", line.unwrap());
    }
    Ok(())
}
