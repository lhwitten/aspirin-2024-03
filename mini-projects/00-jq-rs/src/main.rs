use anyhow::Result;
use clap::Parser;
use colored::Color;
use serde_json::Value;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::PathBuf;

//mod input;
mod filters;

// - `color-output` (True)
// - `monochrome-output` (False)
// - `sort-keys` (False)
// - `indent` (2) - this value must be in the range of 0-7 inclusive
// - `compact-output` (False)

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long)]
    color_output: Option<bool>,

    #[clap(short = 'v', long)]
    compact_output: Option<bool>,

    #[clap(short, long)]
    monochrome_output: Option<bool>,

    #[clap(short, long)]
    sort_keys: Option<bool>,

    #[clap(short, long)]
    indent: Option<i32>,

    filter_string: String,

    json_file: Option<PathBuf>,
}

fn main() {
    println!("Hello World!");
    let args = Args::parse();
    //println!("{:?}", args);

    // println!("{:?}", args);
    //let input_vec = input::read_general_input(args.file)?;

    //parse filter string
    let mut filters: Vec<Filter> = Vec::new();
    // let filters = parse_filters

    //test_valid_args(args)

    let file = File::open(args.json_file).unwrap();
    let reader = BufReader::new(file);
    let json_obj: serde_json::Value = serde_json::from_reader(reader).unwrap();

    // if let Some(array) = json_obj.as_array() {

    // }

    let operations = parse_operations(args.filter_string);

    let mut filter_chain: Result<Vec<serde_json::Value>> = Result::new();

    for operation in operations {
        filter_chain = apply_operation(filter_chain, operation);
    }
    //takes in Result<Vec<serde_json::Value>>
    output_all(filter_chain)

    //println!("{}", json_obj);
}
