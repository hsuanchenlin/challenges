use atty::Stream;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Stdin};
use std::path::Path;

enum MyOption {
    Bytes,
    Lines,
    Words,
    Characters,
}
impl MyOption {
    fn from_str(s: &str) -> MyOption {
        match s {
            "-c" => MyOption::Bytes,
            "-l" => MyOption::Lines,
            "-w" => MyOption::Words,
            "-m" => MyOption::Characters,
            _ => panic!("Invalid option"),
        }
    }
}
struct WcResult {
    bytes: u64,
    lines: usize,
    words: usize,
}
impl WcResult {
    fn new() -> WcResult {
        WcResult {
            bytes: 0,
            lines: 0,
            words: 0,
        }
    }
    fn set_bytes(&mut self, byte: u64) {
        self.bytes = byte;
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 3 {
        let option: MyOption = MyOption::from_str(&args[1]);
        let file_path = &args[2];
        let file = File::open(&file_path).expect("Something went wrong reading the file");
        let file_name = Path::new(file_path)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("No valid file name");
        match option {
            MyOption::Bytes => {
                println!("{} {}", file.metadata().unwrap().len(), file_name);
            }
            _ => {
                let reader = BufReader::new(file);
                println!("{} {}", count(reader, option), file_name);
            }
        }
        return;
    }
    if atty::is(Stream::Stdin) && args.len() == 2 {
        let file_path = &args[1];
        let file = File::open(&file_path).expect("Something went wrong reading the file");
        let file_name = Path::new(file_path)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("No valid file name");
        let file_size = file.metadata().unwrap().len();
        let mut r = count_all(BufReader::new(file));
        r.set_bytes(file_size);
        println!("{} {} {} {}", r.bytes, r.lines, r.words, file_name);
        return;
    }
    if atty::isnt(Stream::Stdin) {
        println!("N Stdin");
        println!("{:?}", args);
        let stdin = io::stdin();
        if args.len() == 1 {
            let mut r = count_all_pipe(stdin);
            println!("{} {} {}", r.bytes, r.lines, r.words);
        } else if args.len() == 2 {
            let option: MyOption = MyOption::from_str(&args[1]);
            println!("{:?}", count_pip(stdin, option));
        }
        return;
    }
}

fn count(buf_read: BufReader<File>, option: MyOption) -> usize {
    let r = match option {
        MyOption::Lines => buf_read.lines().count(),
        MyOption::Words => buf_read
            .lines()
            .map(|l| l.unwrap().split(" ").count())
            .sum(),
        MyOption::Characters => buf_read.lines().map(|l| l.unwrap().chars().count()).sum(),
        _ => panic!("Invalid option"),
    };
    r
}

fn count_pip(buf_read: Stdin, option: MyOption) -> usize {
    let r = match option {
        MyOption::Bytes => buf_read.lines().map(|l| l.unwrap().len()).sum(),
        MyOption::Lines => buf_read.lines().count(),
        MyOption::Words => buf_read
            .lines()
            .map(|l| l.unwrap().split(" ").count())
            .sum(),
        MyOption::Characters => buf_read.lines().map(|l| l.unwrap().chars().count()).sum(),
    };
    r
}
fn count_all(buf_read: BufReader<File>) -> WcResult {
    let mut line_cnt = 0;
    let mut word_cnt = 0;
    for l in buf_read.lines() {
        line_cnt += 1;
        let line = l.unwrap();
        let v: Vec<&str> = line.split(" ").collect();
        word_cnt += v.len();
        line.chars().count();
    }
    WcResult {
        bytes: 0,
        lines: line_cnt,
        words: word_cnt,
    }
}
fn count_all_pipe(buf_read: Stdin) -> WcResult {
    let mut line_cnt = 0;
    let mut word_cnt = 0;
    let mut byte_cnt = 0;
    for l in buf_read.lock().lines() {
        line_cnt += 1;
        let line = l.unwrap();
        let v: Vec<&str> = line.split(" ").collect();
        word_cnt += v.len();
        byte_cnt += line.chars().count();
    }
    WcResult {
        bytes: byte_cnt as u64,
        lines: line_cnt,
        words: word_cnt,
    }
}
