mod random_writer;
mod drawable;

extern crate getopts;
use getopts::Options;
use getopts::Matches;
use std::fs::File;
use std::io::prelude::*;
use std::env;
use random_writer::random_writer::RandomWriter;

fn read_file(infile: &str) -> String {
    let mut inhandle = match File::open(&infile) {
        Ok(handle) => handle,
        Err(e) => panic!("{} not found", infile)
    };
    let mut contents = String::new();
    inhandle.read_to_string(&mut contents);
    contents
}

fn match_arg(arg_short: &str, matches: &Matches) -> String {
    match matches.opt_str(arg_short) {
        Some(val) => val,
        None => panic!("Argument not supplied: {}", arg_short)
    }
}

fn construct_opts(args: Vec<String>) -> Matches {
    let mut opts = Options::new();
    opts.optopt("f", "file", "file containing input text", "INPUT");
    opts.optopt("k", "", "seed length", "K");
    opts.optopt("l", "len", "output length", "LENGTH");
    match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(e) => { panic!(e.to_string()) }        
    }    
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let matches = construct_opts(args);
    let infile = match_arg("f", &matches);
    let k: u32 = match_arg("k", &matches).parse().unwrap();
    let len: u32 = match_arg("l", &matches).parse().unwrap();
    let contents = read_file(&infile);
    let contents_array = &*contents.chars().collect::<Vec<char>>();
    let writer = RandomWriter::new(&contents_array, k, len);
    let prefix_counts = writer.get_prefix_counts();
    let distribution = writer.get_prefix_distribution();    
    print!("{}", contents);
    writer.display_nested_hash_map(distribution);
    
    // print!("{}", prefix_counts.to_string());
}

