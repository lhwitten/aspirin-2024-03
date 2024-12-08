use anyhow::Result;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::{Path, PathBuf};

pub trait input {
    fn get_lines(&self) -> Result<Vec<String>>;
}

pub struct file_input {
    path: PathBuf,
}

impl input for file_input {
    fn get_lines(&self) -> Result<Vec<String>> {
        let mut file = File::open(self.path.clone())?;

        let reader = BufReader::new(file);

        let lines = reader.lines().collect::<Result<Vec<String>, _>>()?;

        Ok(lines)
    }
}

pub fn read_general_input(file: PathBuf) -> Result<Vec<String>> {
    let actual_input: Box<dyn input> = Box::new(file_input { path: file });

    actual_input.get_lines()
}
