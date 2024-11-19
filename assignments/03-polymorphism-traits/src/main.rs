use anyhow::Result;
use clap::Parser;
use colored::Color;
use std::path::PathBuf;

mod input;
mod printer;
mod searcher;

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long)]
    ignore_case: bool,

    #[clap(short = 'v', long)]
    invert_match: bool,

    #[clap(short, long)]
    regex: bool,

    #[clap(short, long)]
    color: Option<Color>,

    needle: String,

    file: Option<PathBuf>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    //println!("{:?}", args);

    // println!("{:?}", args);
    let input_vec = input::read_general_input(args.file)?;
    //println!("{:?}", input_vec);

    let actual_matches = searcher::get_matches(
        input_vec,
        args.needle,
        args.ignore_case,
        args.regex,
        args.invert_match,
    );

    printer::print_matches(actual_matches, args.color)?;

    Ok(())
}
