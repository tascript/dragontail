extern crate getopts;
extern crate memmap;
use getopts::Options;
use memmap::MmapOptions;
use std::env;
use std::fs::File;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optflag("f", "", "follow　change of file");
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
    if !arguments.opt_present("f") {
        match tail(&args[1], line) {
            Ok(()) => (),
            Err(err) => println!("Error: {}", err.to_string()),
        }
    }
}

fn tail(file_name: &String, line: i32) -> Result<(), std::io::Error> {
    let file = File::open(file_name)?;
    let mmap = unsafe { MmapOptions::new().map(&file)? };
    let char_length = mmap.len() as usize;
    let start: usize = get_start_pos(&mmap, char_length, line);
    let buf = mmap[start..char_length].to_vec();
    print_buf(buf);
    Ok(())
}

fn get_start_pos(mmap: &memmap::Mmap, character_num: usize, line: i32) -> usize {
    let mut i = character_num - 1;
    let mut newline_num = line;
    loop {
        if i <= 0 {
            break;
        }
        if &mmap[i..(i + 1)] == b"\n" {
            newline_num -= 1;
            if newline_num <= 0 {
                break;
            }
        }
        i -= 1;
    }
    i
}

fn print_buf(buf: Vec<u8>) {
    for line in buf.split(|x| *x == b'\n') {
        match encode(line) {
            Some(encoded) => println!("{}", encoded),
            None => panic!("encode error."),
        }
    }
}

fn encode(buf: &[u8]) -> Option<String> {
    match String::from_utf8(buf.to_vec()) {
        Ok(result) => Some(result),
        FromUtf8Error => None,
    }
}

fn tail_follow() {}
