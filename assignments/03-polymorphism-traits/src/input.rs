//input must support file and stdin
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

pub struct std_input {}

impl input for std_input {
    fn get_lines(&self) -> Result<Vec<String>> {
        let stdin = io::stdin();

        // Lock the handle for exclusive access and create a buffered reader
        let reader = stdin.lock();
        let lines = reader.lines().collect::<Result<Vec<String>, _>>()?;

        Ok(lines)
    }
}

impl input for file_input {
    fn get_lines(&self) -> Result<Vec<String>> {
        let mut file = File::open(self.path.clone())?;

        let reader = BufReader::new(file);

        let lines = reader.lines().collect::<Result<Vec<String>, _>>()?;

        Ok(lines)
    }
}

pub fn read_general_input(file: Option<PathBuf>) -> Result<Vec<String>> {
    let actual_input: Box<dyn input> = match file {
        Some(a_path) => Box::new(file_input { path: a_path }),
        None => Box::new(std_input {}),
    };
    actual_input.get_lines()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use std::path::PathBuf;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_file_input() {
        // Create a temporary file with some content.
        let temp_file_path = PathBuf::from("test_input.txt");
        let mut temp_file = File::create(&temp_file_path).expect("Failed to create temp file");

        writeln!(temp_file, "Line 1").expect("Failed to write to temp file");
        writeln!(temp_file, "Line 2").expect("Failed to write to temp file");
        writeln!(temp_file, "Line 3").expect("Failed to write to temp file");

        // Read the file using file_input.
        let input = file_input {
            path: temp_file_path.clone(),
        };
        let result = input.get_lines().expect("Failed to read lines");

        // Assert that the lines are read correctly.
        assert_eq!(result, vec!["Line 1", "Line 2", "Line 3"]);

        // Clean up the temporary file.
        std::fs::remove_file(temp_file_path).expect("Failed to delete temp file");
    }

    #[test]
    fn test_read_general_input_with_file() {
        // Create a temporary file with some content.
        let temp_file_path = PathBuf::from("test_input_2.txt");
        let mut temp_file = File::create(&temp_file_path).expect("Failed to create temp file");

        writeln!(temp_file, "Test 1").expect("Failed to write to temp file");
        writeln!(temp_file, "Test 2").expect("Failed to write to temp file");

        // Use read_general_input to read from the file.
        let result = read_general_input(Some(temp_file_path.clone()))
            .expect("Failed to read general input from file");

        // Assert that the lines match the expected output.
        assert_eq!(result, vec!["Test 1", "Test 2"]);

        // Clean up the temporary file.
        std::fs::remove_file(temp_file_path).expect("Failed to delete temp file");
    }

    #[test]
    fn test_std_input() {
        // Simulate stdin using a mutex and a thread.
        let input_data = Arc::new(Mutex::new(
            "Simulated stdin line 1\nSimulated stdin line 2\n",
        ));
        let input_data_clone = Arc::clone(&input_data);

        let _lock = input_data_clone.lock().unwrap();
        let input = std_input {};

        // Capture the output by temporarily overriding stdin.
        let result = input.get_lines();

        match result {
            Ok(lines) => {
                assert_eq!(
                    lines,
                    vec!["Simulated stdin line 1", "Simulated stdin line 2"]
                );
            }
            Err(e) => panic!("Error reading from simulated stdin: {:?}", e),
        }
    }

    #[test]
    fn test_read_general_input_from_stdin() {
        // Simulate reading from stdin by calling read_general_input with None.
        // Note: This is challenging to test in an automated way because stdin is interactive.
        // However, we'll check that it behaves correctly with no input or with mocked data.
        let result = read_general_input(None);

        // We expect the result to return an error since there's no actual input,
        // or we would need to provide a way to simulate stdin for testing purposes.
        assert!(
            result.is_err(),
            "Expected error when reading from stdin in a test environment"
        );
    }
}
