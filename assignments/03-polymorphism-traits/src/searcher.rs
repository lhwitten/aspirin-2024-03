use std::env::Args;

//A file where searching will occur
use anyhow::Result;
use regex::Regex;

pub trait searchable {
    fn parse_matches(&self, needle: String, case_arg: bool, invert_arg: bool) -> Vec<String>;
}

pub struct regex_vec {
    literals: Vec<String>,
}

pub struct literal_vec {
    literals: Vec<String>,
}

impl searchable for regex_vec {
    fn parse_matches(&self, needle: String, case_arg: bool, invert_arg: bool) -> Vec<String> {
        let mut re = Regex::new(&needle).expect("You have provided an invalid RegeX");

        if case_arg {
            let mut nocase_append = String::from(r"(?i)");
            nocase_append.push_str(&needle);
            re = Regex::new(&nocase_append).expect("You have provided an invalid RegeX");
        }

        //if re.is_match(self.literals) {}

        let mut matches: Vec<String> = Vec::new();
        let mut inverse_matches: Vec<String> = Vec::new();

        for line in self.literals.clone() {
            if re.is_match(&line) {
                if !invert_arg {
                    matches.push(line);
                } else {
                    inverse_matches.push(line);
                }
            } else {
                if !invert_arg {
                    inverse_matches.push(line);
                } else {
                    matches.push(line);
                }
            }
        }

        matches
    }
}

impl searchable for literal_vec {
    fn parse_matches(&self, needle: String, case_arg: bool, invert_arg: bool) -> Vec<String> {
        let mut matches: Vec<String> = Vec::new();
        let mut inverse_matches: Vec<String> = Vec::new();

        let needle = if case_arg {
            needle.to_lowercase()
        } else {
            needle
        };

        for line in &self.literals {
            let compare_1 = if case_arg {
                line.to_lowercase()
            } else {
                line.clone()
            };

            if compare_1.contains(&needle) {
                if !invert_arg {
                    matches.push(line.clone());
                } else {
                    inverse_matches.push(line.clone());
                }
            } else {
                if !invert_arg {
                    inverse_matches.push(line.clone());
                } else {
                    matches.push(line.clone());
                }
            }
        }

        matches
    }
}
// #[clap(short, long)]
// ignore_case: bool,

// #[clap(short = 'v', long)]
// invert_match: bool,

// #[clap(short, long)]
// regex: bool,

pub fn get_matches(
    raw_input: Vec<String>,
    needle: String,
    case_arg: bool,
    regex_arg: bool,
    invert_arg: bool,
) -> Vec<String> {
    let searchable: Box<dyn searchable> = match regex_arg {
        true => Box::new(regex_vec {
            literals: raw_input,
        }),
        false => Box::new(literal_vec {
            literals: raw_input,
        }),
    };

    searchable.parse_matches(needle, case_arg, invert_arg)
}

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;

    #[test]
    fn test_literal_vec_case_sensitive() {
        let input = literal_vec {
            literals: vec![
                "Hello World".to_string(),
                "hello world".to_string(),
                "Goodbye World".to_string(),
            ],
        };
        let matches = input.parse_matches("Hello".to_string(), false, false);
        assert_eq!(matches, vec!["Hello World".to_string()]);
    }

    #[test]
    fn test_literal_vec_case_insensitive() {
        let input = literal_vec {
            literals: vec![
                "Hello World".to_string(),
                "hello world".to_string(),
                "Goodbye World".to_string(),
            ],
        };
        let matches = input.parse_matches("hello".to_string(), true, false);
        assert_eq!(
            matches,
            vec!["Hello World".to_string(), "hello world".to_string()]
        );
    }

    #[test]
    fn test_literal_vec_invert_match() {
        let input = literal_vec {
            literals: vec![
                "Hello World".to_string(),
                "hello world".to_string(),
                "Goodbye World".to_string(),
            ],
        };
        let matches = input.parse_matches("Hello".to_string(), false, true);
        assert_eq!(
            matches,
            vec!["hello world".to_string(), "Goodbye World".to_string()]
        );
    }

    #[test]
    fn test_regex_vec_simple_match() {
        let input = regex_vec {
            literals: vec![
                "foo bar".to_string(),
                "bar baz".to_string(),
                "foo baz".to_string(),
            ],
        };
        let matches = input.parse_matches("foo".to_string(), false, false);
        assert_eq!(matches, vec!["foo bar".to_string(), "foo baz".to_string()]);
    }

    #[test]
    fn test_regex_vec_case_insensitive() {
        let input = regex_vec {
            literals: vec![
                "Foo bar".to_string(),
                "foo Bar".to_string(),
                "FOO baz".to_string(),
                "baz foo".to_string(),
            ],
        };
        let matches = input.parse_matches("foo".to_string(), true, false);
        assert_eq!(
            matches,
            vec![
                "Foo bar".to_string(),
                "foo Bar".to_string(),
                "FOO baz".to_string(),
                "baz foo".to_string()
            ]
        );
    }

    #[test]
    fn test_regex_vec_invert_match() {
        let input = regex_vec {
            literals: vec![
                "foo bar".to_string(),
                "bar baz".to_string(),
                "foo baz".to_string(),
            ],
        };
        let matches = input.parse_matches("foo".to_string(), false, true);
        assert_eq!(matches, vec!["bar baz".to_string()]);
    }

    #[test]
    fn test_regex_vec_invalid_regex() {
        let input = regex_vec {
            literals: vec![
                "foo bar".to_string(),
                "bar baz".to_string(),
                "foo baz".to_string(),
            ],
        };
        // Should panic due to invalid regex syntax.
        let result = std::panic::catch_unwind(|| {
            input.parse_matches("foo(".to_string(), false, false);
        });
        assert!(result.is_err());
    }

    #[test]
    fn test_get_matches_literal() {
        let input = vec![
            "Hello World".to_string(),
            "hello world".to_string(),
            "Goodbye World".to_string(),
        ];
        let matches = get_matches(input, "Hello".to_string(), false, false, false);
        assert_eq!(matches, vec!["Hello World".to_string()]);
    }

    #[test]
    fn test_get_matches_literal_case_insensitive() {
        let input = vec![
            "Hello World".to_string(),
            "hello world".to_string(),
            "Goodbye World".to_string(),
        ];
        let matches = get_matches(input, "hello".to_string(), true, false, false);
        assert_eq!(
            matches,
            vec!["Hello World".to_string(), "hello world".to_string()]
        );
    }

    #[test]
    fn test_get_matches_regex() {
        let input = vec![
            "foo bar".to_string(),
            "bar baz".to_string(),
            "foo baz".to_string(),
        ];
        let matches = get_matches(input, "foo".to_string(), false, true, false);
        assert_eq!(matches, vec!["foo bar".to_string(), "foo baz".to_string()]);
    }

    #[test]
    fn test_get_matches_regex_invert() {
        let input = vec![
            "foo bar".to_string(),
            "bar baz".to_string(),
            "foo baz".to_string(),
        ];
        let matches = get_matches(input, "foo".to_string(), false, true, true);
        assert_eq!(matches, vec!["bar baz".to_string()]);
    }
}
