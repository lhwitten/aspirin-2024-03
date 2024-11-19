use colored::{Color, Colorize};
use std::io::Write;

pub trait Printable {
    fn print_thing(&self, line: &str, output: &mut dyn Write) -> std::io::Result<()>;
}

pub struct ColorPrint {
    color: Color,
}

pub struct NormalPrint;

impl ColorPrint {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}

impl Printable for ColorPrint {
    fn print_thing(&self, line: &str, output: &mut dyn Write) -> std::io::Result<()> {
        writeln!(output, "{}", line.color(self.color))
    }
}

impl Printable for NormalPrint {
    fn print_thing(&self, line: &str, output: &mut dyn Write) -> std::io::Result<()> {
        writeln!(output, "{}", line)
    }
}

/// Prints the matches using the appropriate printer based on the color option.
pub fn print_matches(matches: Vec<String>, color: Option<Color>) -> std::io::Result<()> {
    let printer: Box<dyn Printable> = if let Some(color) = color {
        Box::new(ColorPrint::new(color))
    } else {
        Box::new(NormalPrint)
    };

    let mut stdout = std::io::stdout();
    for line in matches {
        printer.print_thing(&line, &mut stdout)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use colored::Color;
    use std::io::Cursor;

    #[test]
    fn test_normal_print_single_line() {
        let printer = NormalPrint;
        let mut output = Vec::new();
        let line = "Hello, world!";

        printer
            .print_thing(line, &mut output)
            .expect("Failed to print line");

        // Convert the output to a string and verify
        let output_str = String::from_utf8(output).expect("Output is not valid UTF-8");
        assert_eq!(output_str, "Hello, world!\n");
    }

    #[test]
    fn test_color_print_single_line() {
        let printer = ColorPrint::new(Color::Red);
        let mut output = Vec::new();
        let line = "Hello, colorful world!";

        printer
            .print_thing(line, &mut output)
            .expect("Failed to print colored line");

        // Convert the output to a string and verify
        let output_str = String::from_utf8(output).expect("Output is not valid UTF-8");

        // This test checks that the output contains the color escape codes.
        // The exact escape sequence may vary depending on the terminal.
        assert!(
            output_str.contains("\u{1b}[31mHello, colorful world!\u{1b}[0m"),
            "Output did not contain expected colored text. Output was: {}",
            output_str
        );
    }

    #[test]
    fn test_normal_print_multiple_lines() {
        let printer = NormalPrint;
        let mut output = Vec::new();
        let lines = vec!["Line 1", "Line 2", "Line 3"];

        for line in lines.iter() {
            printer
                .print_thing(line, &mut output)
                .expect("Failed to print line");
        }

        let output_str = String::from_utf8(output).expect("Output is not valid UTF-8");
        assert_eq!(output_str, "Line 1\nLine 2\nLine 3\n");
    }

    #[test]
    fn test_color_print_multiple_lines() {
        let printer = ColorPrint::new(Color::Green);
        let mut output = Vec::new();
        let lines = vec!["Line A", "Line B", "Line C"];

        for line in lines.iter() {
            printer
                .print_thing(line, &mut output)
                .expect("Failed to print colored line");
        }

        let output_str = String::from_utf8(output).expect("Output is not valid UTF-8");

        // Verify that each line is colored.
        for line in &lines {
            assert!(
                output_str.contains(&format!("\u{1b}[32m{}\u{1b}[0m", line)),
                "Output did not contain expected colored line: {}",
                line
            );
        }
    }

    #[test]
    fn test_print_matches_normal() {
        let matches = vec!["Match 1".to_string(), "Match 2".to_string()];
        let mut output = Vec::new();

        {
            let mut mock_stdout = Cursor::new(&mut output);
            for line in matches.iter() {
                NormalPrint
                    .print_thing(line, &mut mock_stdout)
                    .expect("Failed to print normal line");
            }
        }

        let output_str = String::from_utf8(output).expect("Output is not valid UTF-8");
        assert_eq!(output_str, "Match 1\nMatch 2\n");
    }

    #[test]
    fn test_print_matches_color() {
        let matches = vec!["Match A".to_string(), "Match B".to_string()];
        let mut output = Vec::new();

        {
            let mut mock_stdout = Cursor::new(&mut output);
            for line in matches.iter() {
                ColorPrint::new(Color::Blue)
                    .print_thing(line, &mut mock_stdout)
                    .expect("Failed to print colored line");
            }
        }

        let output_str = String::from_utf8(output).expect("Output is not valid UTF-8");

        // Verify that each line is colored.
        for line in &matches {
            assert!(
                output_str.contains(&format!("\u{1b}[34m{}\u{1b}[0m", line)),
                "Output did not contain expected colored line: {}",
                line
            );
        }
    }

    #[test]
    fn test_print_matches_empty() {
        let matches: Vec<String> = vec![];
        let mut output = Vec::new();

        let result = {
            let mut mock_stdout = Cursor::new(&mut output);
            let res = print_matches(matches, None);
            res
        };

        assert!(
            result.is_ok(),
            "print_matches returned an error with empty input"
        );

        let output_str = String::from_utf8(output).expect("Output is not valid UTF-8");
        assert!(output_str.is_empty(), "Expected no output for empty input");
    }
}
