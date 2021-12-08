use std::fmt::Display;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::PathBuf;
use std::str::FromStr;

use log::info;
use structopt;
use structopt::StructOpt;
use thiserror::Error;

#[derive(Error, Debug)]
struct ParsePartError {
    err: String,
}

impl Display for ParsePartError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.err)
    }
}

fn parse_part(src: &str) -> Result<u8, ParsePartError> {
    let res = match u8::from_str(src) {
        Ok(n) => n,
        Err(e) => {
            return Err(ParsePartError {
                err: format!("Not a usize {}: {}", src, e),
            })
        }
    };
    if 1 <= res && res <= 2 {
        Ok(res)
    } else {
        Err(ParsePartError {
            err: format!("Unsupported part {}", res),
        })
    }
}

#[derive(Debug, StructOpt)]
pub struct Opt {
    #[structopt(short, long, parse(try_from_str=parse_part))]
    pub part: u8,
    #[structopt(parse(from_os_str))]
    pub infile: PathBuf,
}

pub fn read_input_lines_usize(filepath: &PathBuf) -> Vec<usize> {
    BufReader::new(File::open(filepath).unwrap())
        .lines()
        .map(|i| usize::from_str(&i.unwrap()).unwrap())
        .collect()
}
pub fn read_input_lines_string(filepath: &PathBuf) -> Vec<String> {
    BufReader::new(File::open(filepath).unwrap())
        .lines()
        .collect::<Result<Vec<String>, _>>()
        .unwrap()
}

pub fn run<T: Display>(p1: fn(&PathBuf) -> T, p2: fn(&PathBuf) -> T) {
    let opt = init();
    if opt.part == 1 {
        let result_part1 = p1(&opt.infile);
        println!("Part 1: {}", result_part1);
    } else if opt.part == 2 {
        let result_part2 = p2(&opt.infile);
        println!("Part 2: {}", result_part2);
    }
}

pub fn init() -> Opt {
    env_logger::init();
    Opt::from_args()
}
