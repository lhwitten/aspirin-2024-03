use ansi_term::Colour;
use serde::Serialize;
use serde_json::Value;

pub struct Formatter {
    pub color_output: bool,
    pub indent: usize,
    pub compact: bool,
    pub sort_keys: bool,
}

impl Formatter {
    pub fn print_value(&self, value: &Value) {
        if self.color_output {
            // Use colored output
            let colored_str = self.serialize_with_colors(value, 0);
            println!("{}", colored_str);
        } else {
            // Monochrome output
            if self.compact {
                println!("{}", value);
            } else {
                // Store the indentation bytes in a variable to extend its lifetime
                let indent_bytes = vec![b' '; self.indent];
                let formatter = serde_json::ser::PrettyFormatter::with_indent(&indent_bytes);
                let mut ser = serde_json::Serializer::with_formatter(Vec::new(), formatter);
                if self.sort_keys {
                    let sorted_value = sort_json_keys(value);
                    sorted_value.serialize(&mut ser).unwrap();
                } else {
                    value.serialize(&mut ser).unwrap();
                }
                let output = String::from_utf8(ser.into_inner()).unwrap();
                println!("{}", output);
            }
        }
    }

    fn serialize_with_colors(&self, value: &Value, indent_level: usize) -> String {
        // Define colors for different JSON types
        let null_color = Colour::Fixed(90); // Gray
        let bool_color = Colour::Fixed(33); // Yellow
        let number_color = Colour::Fixed(32); // Green
        let string_color = Colour::Fixed(31); // Red
        let key_color = Colour::Fixed(36); // Cyan
        let punctuation_color = Colour::Fixed(37); // White

        let indent = if self.compact {
            String::new()
        } else {
            " ".repeat(self.indent * indent_level)
        };

        match value {
            Value::Null => null_color.paint("null").to_string(),
            Value::Bool(b) => bool_color.paint(b.to_string()).to_string(),
            Value::Number(n) => number_color.paint(n.to_string()).to_string(),
            Value::String(s) => {
                let quoted = format!("\"{}\"", s);
                string_color.paint(quoted).to_string()
            }
            Value::Array(arr) => {
                let mut result = String::new();
                let newline = if self.compact { "" } else { "\n" };
                let separator = if self.compact { "" } else { " " };

                result.push_str(&punctuation_color.paint("[").to_string());
                if !arr.is_empty() {
                    result.push_str(newline);
                    let mut first = true;
                    for item in arr {
                        if first {
                            first = false;
                        } else {
                            result.push_str(&punctuation_color.paint(",").to_string());
                            result.push_str(newline);
                        }
                        result.push_str(&indent);
                        if !self.compact {
                            result.push_str(&" ".repeat(self.indent));
                        }
                        let item_str = self.serialize_with_colors(item, indent_level + 1);
                        result.push_str(&item_str);
                    }
                    result.push_str(newline);
                    result.push_str(&indent);
                }
                result.push_str(&punctuation_color.paint("]").to_string());
                result
            }
            Value::Object(map) => {
                let mut result = String::new();
                let newline = if self.compact { "" } else { "\n" };
                let separator = if self.compact { "" } else { " " };

                result.push_str(&punctuation_color.paint("{").to_string());
                if !map.is_empty() {
                    result.push_str(newline);
                    let mut entries: Vec<_> = map.iter().collect();
                    if self.sort_keys {
                        entries.sort_by_key(|(k, _)| k.clone());
                    }
                    let mut first = true;
                    for (key, val) in entries {
                        if first {
                            first = false;
                        } else {
                            result.push_str(&punctuation_color.paint(",").to_string());
                            result.push_str(newline);
                        }
                        result.push_str(&indent);
                        if !self.compact {
                            result.push_str(&" ".repeat(self.indent));
                        }
                        let key_str = format!("\"{}\"", key);
                        let colored_key = key_color.paint(key_str);
                        result.push_str(&colored_key.to_string());
                        result.push_str(&punctuation_color.paint(":").to_string());
                        result.push_str(separator);
                        let val_str = self.serialize_with_colors(val, indent_level + 1);
                        result.push_str(&val_str);
                    }
                    result.push_str(newline);
                    result.push_str(&indent);
                }
                result.push_str(&punctuation_color.paint("}").to_string());
                result
            }
        }
    }
}

fn sort_json_keys(value: &Value) -> Value {
    match value {
        Value::Object(map) => {
            let mut entries: Vec<_> = map.iter().collect();
            entries.sort_by_key(|(k, _)| k.clone());
            let sorted_map = entries
                .into_iter()
                .map(|(k, v)| (k.clone(), sort_json_keys(v)))
                .collect();
            Value::Object(sorted_map)
        }
        Value::Array(arr) => {
            let sorted_array = arr.iter().map(sort_json_keys).collect();
            Value::Array(sorted_array)
        }
        _ => value.clone(),
    }
}

// printers.rs

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_sort_json_keys_simple() {
        let value = json!({"b": 1, "a": 2, "c": 3});
        let sorted = sort_json_keys(&value);
        assert_eq!(sorted, json!({"a": 2, "b": 1, "c": 3}));
    }

    #[test]
    fn test_sort_json_keys_nested() {
        let value = json!({
            "z": {"d": 4, "c": 3},
            "a": {"b": 2, "a": 1}
        });
        let sorted = sort_json_keys(&value);
        assert_eq!(
            sorted,
            json!({
                "a": {"a": 1, "b": 2},
                "z": {"c": 3, "d": 4}
            })
        );
    }

    #[test]
    fn test_formatter_print_value_no_color() {
        let formatter = Formatter {
            color_output: false,
            indent: 2,
            compact: false,
            sort_keys: false,
        };
        let value = json!({"b": 1, "a": 2});
        formatter.print_value(&value);
    }

    #[test]
    fn test_formatter_print_value_with_color() {
        let formatter = Formatter {
            color_output: true,
            indent: 2,
            compact: false,
            sort_keys: false,
        };
        let value = json!({"b": 1, "a": 2});
        formatter.print_value(&value);
    }

    #[test]
    fn test_formatter_sort_keys() {
        let formatter = Formatter {
            color_output: false,
            indent: 2,
            compact: false,
            sort_keys: true,
        };
        let value = json!({"b": 1, "a": 2});
        formatter.print_value(&value);
    }

    #[test]
    fn test_formatter_compact_output() {
        let formatter = Formatter {
            color_output: false,
            indent: 0,
            compact: true,
            sort_keys: false,
        };
        let value = json!({"b": 1, "a": 2});
        formatter.print_value(&value);
    }

    #[test]
    fn test_formatter_indent_levels() {
        for indent in 0..=7 {
            let formatter = Formatter {
                color_output: false,
                indent,
                compact: false,
                sort_keys: false,
            };
            let value = json!({"key": {"nested_key": "value"}});
            formatter.print_value(&value);
        }
    }
}
