extern crate getopts;
extern crate memmap;
use getopts::Options;
use memmap::MmapOptions;
use std::fs::File;
use std::{env, thread, time};

struct ReadBufResult {
    buf: Vec<u8>,
    length: usize,
}

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
    } else {
        tail_follow(&args[1], line);
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

fn tail_follow(file_name: &String, line: i32) {
    let file = match File::open(file_name) {
        Ok(result) => result,
        Err(e) => panic!("error: {}", e),
    };
    let mmap = unsafe {
        match MmapOptions::new().map(&file) {
            Ok(result) => result,
            Err(e) => panic!("error: {}", e),
        }
    };
    let mut char_length = mmap.len() as usize;
    let start: usize = get_start_pos(&mmap, char_length, line);
    let buf = mmap[start..char_length].to_vec();
    print_buf(buf);
    loop {
        match read_the_rest(file_name, char_length) {
            Some(result) => {
                print_buf(result.buf);
                char_length = result.length;
            }
            None => {
                thread::sleep(time::Duration::from_secs(1));
            }
        }
    }
}

fn read_the_rest(file_name: &String, start_pos: usize) -> Option<ReadBufResult> {
    let file = match File::open(file_name) {
        Ok(result) => result,
        Err(e) => panic!("error: {}", e),
    };
    let mmap = unsafe {
        match MmapOptions::new().map(&file) {
            Ok(result) => result,
            Err(e) => panic!("error: {}", e),
        }
    };
    let length = mmap.len() as usize;
    if length <= start_pos {
        return None;
    }
    Some(ReadBufResult {
        buf: mmap[start_pos..length].to_vec(),
        length,
    })
}
