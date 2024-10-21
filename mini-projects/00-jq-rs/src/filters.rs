use core::panic;
use serde_json::Value;

use clap::{Arg, Command};
use std::env;
use std::fs::File;
use std::io::BufReader;

#[derive(Debug, PartialEq)]
pub enum Filter {
    Identity,
    ObjectIdentifier(String),
    ArrayIndex(isize),
    ArraySlice(Option<isize>, Option<isize>),
    ArrayIterator,
    Add,
    Length,
    Del(Box<Filter>),
    // Other filters as needed
}

//name:
//obj_identify -- .<KEY_NAME>
//array_index --  .[i32]
//array_slice -- .[<start_number>:<end_number>]
//pipe -- pass output of one filter to another
// -- handle in main loop?
//array_iterator -- turn an array into an iterator
// -- effectively expands an array into multiple objects

//add -- adds all values in an array
//length -- returns the length of the input
//del -- del(.[<indexes>]) -- return everything but deleted

pub fn apply_filters(values: Vec<Value>, filters: &[Filter]) -> Result<Vec<Value>, String> {
    let mut current_values = values;

    for filter in filters {
        let mut next_values = Vec::new();
        for value in current_values {
            let result = apply_single_filter(value, filter)?;
            next_values.extend(result);
        }
        current_values = next_values;
    }

    Ok(current_values)
}

pub fn apply_single_filter(value: Value, filter: &Filter) -> Result<Vec<Value>, String> {
    match filter {
        Filter::Identity => Ok(vec![value]),
        Filter::ObjectIdentifier(key) => {
            if let Value::Object(map) = value {
                match map.get(key) {
                    Some(v) => Ok(vec![v.clone()]),
                    None => Err(format!("Key '{}' not found in object.", key)),
                }
            } else {
                Err("Value is not an object.".to_string())
            }
        }
        Filter::ArrayIndex(index) => {
            if let Value::Array(array) = value {
                let idx = if *index < 0 {
                    array.len() as isize + index
                } else {
                    *index
                };
                if idx >= 0 && (idx as usize) < array.len() {
                    Ok(vec![array[idx as usize].clone()])
                } else {
                    Err("Array index out of bounds.".to_string())
                }
            } else {
                Err("Value is not an array.".to_string())
            }
        }
        Filter::ArraySlice(start_opt, end_opt) => {
            if let Value::Array(array) = value {
                let len = array.len() as isize;
                let start = start_opt.unwrap_or(0);
                let end = end_opt.unwrap_or(len);
                let start = if start < 0 { len + start } else { start };
                let end = if end < 0 { len + end } else { end };
                let start = start.max(0) as usize;
                let end = end.min(len) as usize;
                if start > end || start >= array.len() {
                    Ok(vec![Value::Array(vec![])])
                } else {
                    let slice = array[start..end].to_vec();
                    Ok(vec![Value::Array(slice)])
                }
            } else {
                Err("Value is not an array.".to_string())
            }
        }
        Filter::ArrayIterator => {
            if let Value::Array(array) = value {
                Ok(array.clone())
            } else {
                Err("Value is not an array.".to_string())
            }
        }
        Filter::Add => {
            if let Value::Array(array) = value {
                let mut sum = Value::Null;
                for item in array {
                    sum = add_values(&sum, &item)?;
                }
                Ok(vec![sum])
            } else {
                Err("Value is not an array.".to_string())
            }
        }
        Filter::Length => {
            let len = match &value {
                Value::Null => 0,
                Value::Bool(_) => 1,
                Value::Number(_) => 1,
                Value::String(s) => s.chars().count(),
                Value::Array(a) => a.len(),
                Value::Object(o) => o.len(),
            };
            Ok(vec![Value::Number(serde_json::Number::from(len))])
        }
        Filter::Del(inner_filter) => {
            let result = del_value(&value, inner_filter)?;
            Ok(vec![result])
        } // Handle other filters as needed
    }
}

pub fn apply_identify(object: &str, to_apply: Filter) {}

pub fn parse_filter_sequence(filter_str: &str) -> Result<Vec<Filter>, String> {
    let filter_parts: Vec<&str> = filter_str.split('|').map(|s| s.trim()).collect();
    let mut filters = Vec::new();

    for part in filter_parts {
        let filter = parse_single_filter(part)?;
        filters.push(filter);
    }

    Ok(filters)
}

pub fn parse_single_filter(filter_str: &str) -> Result<Filter, String> {
    let filter_str = filter_str.trim();
    if filter_str == "." {
        Ok(Filter::Identity)
    } else if filter_str.starts_with('.') && filter_str.chars().nth(1) != Some('[') {
        // Object Identifier
        let key = &filter_str[1..];
        Ok(Filter::ObjectIdentifier(key.to_string()))
    } else if filter_str.starts_with(".[") && filter_str.ends_with(']') {
        // Array Index or Slice
        let inside_brackets = &filter_str[2..filter_str.len() - 1];
        if inside_brackets.is_empty() {
            // Array Iterator
            Ok(Filter::ArrayIterator)
        } else if inside_brackets.contains(':') {
            // Array Slice
            let parts: Vec<&str> = inside_brackets.split(':').collect();
            if parts.len() != 2 {
                return Err("Invalid array slice syntax.".to_string());
            }
            let start = if parts[0].is_empty() {
                None
            } else {
                Some(
                    parts[0]
                        .parse::<isize>()
                        .map_err(|_| "Invalid start index.")?,
                )
            };
            let end = if parts[1].is_empty() {
                None
            } else {
                Some(
                    parts[1]
                        .parse::<isize>()
                        .map_err(|_| "Invalid end index.")?,
                )
            };
            Ok(Filter::ArraySlice(start, end))
        } else {
            // Array Index
            let index = inside_brackets
                .parse::<isize>()
                .map_err(|_| "Invalid array index.")?;
            Ok(Filter::ArrayIndex(index))
        }
    } else if filter_str == "add" {
        Ok(Filter::Add)
    } else if filter_str == "length" {
        Ok(Filter::Length)
    } else if filter_str.starts_with("del(") && filter_str.ends_with(')') {
        let inner_filter_str = &filter_str[4..filter_str.len() - 1];
        let inner_filter = parse_single_filter(inner_filter_str)?;
        Ok(Filter::Del(Box::new(inner_filter)))
    } else {
        Err(format!("Unsupported or invalid filter: '{}'.", filter_str))
    }
}

pub fn del_value(value: &Value, filter: &Filter) -> Result<Value, String> {
    match (value, filter) {
        (Value::Object(map), Filter::ObjectIdentifier(key)) => {
            let mut new_map = map.clone();
            new_map.remove(key);
            Ok(Value::Object(new_map))
        }
        (Value::Array(array), Filter::ArrayIndex(index)) => {
            let idx = if *index < 0 {
                array.len() as isize + index
            } else {
                *index
            };
            if idx >= 0 && (idx as usize) < array.len() {
                let mut new_array = array.clone();
                new_array.remove(idx as usize);
                Ok(Value::Array(new_array))
            } else {
                Err("Array index out of bounds.".to_string())
            }
        }
        (Value::Array(array), Filter::ArraySlice(start_opt, end_opt)) => {
            let len = array.len() as isize;
            let start = start_opt.unwrap_or(0);
            let end = end_opt.unwrap_or(len);
            let start = if start < 0 { len + start } else { start };
            let end = if end < 0 { len + end } else { end };
            let start = start.max(0) as usize;
            let end = end.min(len) as usize;
            if start >= end || start >= array.len() {
                Ok(Value::Array(array.clone()))
            } else {
                let mut new_array = array.clone();
                new_array.drain(start..end);
                Ok(Value::Array(new_array))
            }
        }
        _ => Err(
            "del function can only be applied to objects or arrays with appropriate filters."
                .to_string(),
        ),
    }
}

fn add_values(a: &Value, b: &Value) -> Result<Value, String> {
    match (a, b) {
        // If either value is Null, return the other
        (Value::Null, _) => Ok(b.clone()),
        (_, Value::Null) => Ok(a.clone()),

        // If both are numbers, sum them
        (Value::Number(n1), Value::Number(n2)) => {
            let sum = n1.as_f64().unwrap() + n2.as_f64().unwrap();

            // Safely create a Number from the sum
            serde_json::Number::from_f64(sum)
                .map(Value::Number)
                .ok_or_else(|| "Error creating number from sum.".to_string())
        }

        // If both are strings, concatenate them
        (Value::String(s1), Value::String(s2)) => Ok(Value::String(format!("{}{}", s1, s2))),

        // If one is a string and one is a number, concatenate them as strings
        (Value::String(s), Value::Number(n)) | (Value::Number(n), Value::String(s)) => {
            Ok(Value::String(format!("{}{}", s, n)))
        }

        // For unsupported types, return an error
        _ => Err("Unsupported types for addition.".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_parse_single_filter_identity() {
        assert_eq!(parse_single_filter(".").unwrap(), Filter::Identity);
    }

    #[test]
    fn test_parse_single_filter_object_identifier() {
        assert_eq!(
            parse_single_filter(".key").unwrap(),
            Filter::ObjectIdentifier("key".to_string())
        );
    }

    #[test]
    fn test_parse_single_filter_array_index() {
        assert_eq!(parse_single_filter(".[0]").unwrap(), Filter::ArrayIndex(0));
        assert_eq!(
            parse_single_filter(".[-1]").unwrap(),
            Filter::ArrayIndex(-1)
        );
    }

    #[test]
    fn test_parse_single_filter_array_slice() {
        assert_eq!(
            parse_single_filter(".[1:3]").unwrap(),
            Filter::ArraySlice(Some(1), Some(3))
        );
        assert_eq!(
            parse_single_filter(".[-3:-1]").unwrap(),
            Filter::ArraySlice(Some(-3), Some(-1))
        );
    }

    #[test]
    fn test_parse_single_filter_array_iterator() {
        assert_eq!(parse_single_filter(".[]").unwrap(), Filter::ArrayIterator);
    }

    #[test]
    fn test_parse_single_filter_add() {
        assert_eq!(parse_single_filter("add").unwrap(), Filter::Add);
    }

    #[test]
    fn test_parse_single_filter_length() {
        assert_eq!(parse_single_filter("length").unwrap(), Filter::Length);
    }

    #[test]
    fn test_parse_single_filter_del() {
        assert_eq!(
            parse_single_filter("del(.key)").unwrap(),
            Filter::Del(Box::new(Filter::ObjectIdentifier("key".to_string())))
        );
    }

    #[test]
    fn test_parse_filter_sequence() {
        let filters = parse_filter_sequence(".|length").unwrap();
        assert_eq!(filters.len(), 2);
        assert_eq!(filters[0], Filter::Identity);
        assert_eq!(filters[1], Filter::Length);
    }

    #[test]
    fn test_apply_single_filter_identity() {
        let value = json!({"key": "value"});
        let result = apply_single_filter(value.clone(), &Filter::Identity).unwrap();
        assert_eq!(result, vec![value]);
    }

    #[test]
    fn test_apply_single_filter_object_identifier() {
        let value = json!({"key": "value", "other": 123});
        let result =
            apply_single_filter(value, &Filter::ObjectIdentifier("key".to_string())).unwrap();
        assert_eq!(result, vec![json!("value")]);
    }

    #[test]
    fn test_apply_single_filter_array_index() {
        let value = json!(["a", "b", "c"]);
        let result = apply_single_filter(value, &Filter::ArrayIndex(1)).unwrap();
        assert_eq!(result, vec![json!("b")]);
    }

    #[test]
    fn test_apply_single_filter_array_slice() {
        let value = json!(["a", "b", "c", "d"]);
        let result = apply_single_filter(value, &Filter::ArraySlice(Some(1), Some(3))).unwrap();
        assert_eq!(result, vec![json!(["b", "c"])]);
    }

    #[test]
    fn test_apply_single_filter_array_iterator() {
        let value = json!(["a", "b", "c"]);
        let result = apply_single_filter(value, &Filter::ArrayIterator).unwrap();
        assert_eq!(result, vec![json!("a"), json!("b"), json!("c")]);
    }

    #[test]
    fn test_apply_single_filter_add_numbers() {
        let value = json!([1, 2, 3]);
        let result = apply_single_filter(value, &Filter::Add).unwrap();
        assert_eq!(result, vec![json!(6.0)]);
    }

    #[test]
    fn test_apply_single_filter_add_strings() {
        let value = json!(["foo", "bar"]);
        let result = apply_single_filter(value, &Filter::Add).unwrap();
        assert_eq!(result, vec![json!("foobar")]);
    }

    #[test]
    fn test_apply_single_filter_length() {
        let value = json!({"a": 1, "b": 2});
        let result = apply_single_filter(value, &Filter::Length).unwrap();
        assert_eq!(result, vec![json!(2)]);
    }

    #[test]
    fn test_apply_single_filter_del_object_key() {
        let value = json!({"a": 1, "b": 2, "c": 3});
        let result = apply_single_filter(
            value,
            &Filter::Del(Box::new(Filter::ObjectIdentifier("b".to_string()))),
        )
        .unwrap();
        assert_eq!(result, vec![json!({"a": 1, "c": 3})]);
    }

    #[test]
    fn test_apply_single_filter_del_array_index() {
        let value = json!(["a", "b", "c"]);
        let result =
            apply_single_filter(value, &Filter::Del(Box::new(Filter::ArrayIndex(1)))).unwrap();
        assert_eq!(result, vec![json!(["a", "c"])]);
    }

    #[test]
    fn test_apply_single_filter_del_array_slice() {
        let value = json!(["a", "b", "c", "d"]);
        let result = apply_single_filter(
            value,
            &Filter::Del(Box::new(Filter::ArraySlice(Some(1), Some(3)))),
        )
        .unwrap();
        assert_eq!(result, vec![json!(["a", "d"])]);
    }

    #[test]
    fn test_apply_filters_sequence() {
        let value = json!({"array": [1, 2, 3, 4, 5]});
        let filters = vec![
            Filter::ObjectIdentifier("array".to_string()),
            Filter::ArraySlice(Some(1), Some(4)),
            Filter::Add,
        ];
        let result = apply_filters(vec![value], &filters).unwrap();
        assert_eq!(result, vec![json!(9.0)]);
    }

    #[test]
    fn test_del_value_object() {
        let value = json!({"a": 1, "b": 2, "c": 3});
        let result = del_value(&value, &Filter::ObjectIdentifier("b".to_string())).unwrap();
        assert_eq!(result, json!({"a": 1, "c": 3}));
    }

    #[test]
    fn test_del_value_array_index() {
        let value = json!(["a", "b", "c", "d"]);
        let result = del_value(&value, &Filter::ArrayIndex(2)).unwrap();
        assert_eq!(result, json!(["a", "b", "d"]));
    }

    #[test]
    fn test_del_value_array_slice() {
        let value = json!(["a", "b", "c", "d"]);
        let result = del_value(&value, &Filter::ArraySlice(Some(1), Some(3))).unwrap();
        assert_eq!(result, json!(["a", "d"]));
    }

    #[test]
    fn test_add_values_numbers() {
        let sum = add_values(&json!(1), &json!(2)).unwrap();
        assert_eq!(sum, json!(3.0));
    }

    #[test]
    fn test_add_values_strings() {
        let sum = add_values(&json!("foo"), &json!("bar")).unwrap();
        assert_eq!(sum, json!("foobar"));
    }

    #[test]
    fn test_add_values_mixed() {
        let sum = add_values(&json!("foo"), &json!(123)).unwrap();
        assert_eq!(sum, json!("foo123"));
    }

    #[test]
    fn test_add_values_null() {
        let sum = add_values(&json!(null), &json!(5)).unwrap();
        assert_eq!(sum, json!(5));
    }

    #[test]
    fn test_add_values_invalid() {
        let result = add_values(&json!({"a":1}), &json!(2));
        assert!(result.is_err());
    }
}
