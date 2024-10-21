use anyhow::Result;
use clap::Parser;
use colored::Color;
use serde_json::Value;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::PathBuf;

//mod input;
mod filters;
mod printers;

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
    indent: Option<usize>,

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
    //let mut filters: Vec<Filter> = Vec::new();
    // let filters = parse_filters

    //test_valid_args(args)
    let file_path = args.json_file.as_ref().expect("JSON file path is required");

    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let json_obj: serde_json::Value = serde_json::from_reader(reader).unwrap();

    // if let Some(array) = json_obj.as_array() {

    // }

    let filters = match filters::parse_filter_sequence(&args.filter_string) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error parsing filter: {}", e);
            std::process::exit(1);
        }
    };

    // Apply the filters
    let initial_values = vec![json_obj];
    let output_values = match filters::apply_filters(initial_values, &filters) {
        Ok(values) => values,
        Err(e) => {
            eprintln!("Error applying filters: {}", e);
            std::process::exit(1);
        }
    };
    let monochrome_output = args.monochrome_output.unwrap_or(false);
    let color_output = args.color_output.unwrap_or(true);
    let indent: usize = args.indent.unwrap_or(2);
    let sort_keys = args.sort_keys.unwrap_or(false);
    let compact_output = args.compact_output.unwrap_or(false);

    //Prepare the formatter
    let formatter = printers::Formatter {
        color_output: color_output && !monochrome_output,
        indent,
        compact: compact_output,
        sort_keys,
    };

    //Print the output values
    for value in output_values {
        formatter.print_value(&value);
    }
}
