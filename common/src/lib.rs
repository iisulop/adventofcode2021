use std::fmt::Display;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::PathBuf;
use std::str::FromStr;

use log::info;
use structopt::StructOpt;
use structopt;
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

pub fn init() -> Opt {
    env_logger::init();
    let opt = Opt::from_args();
    opt
}
