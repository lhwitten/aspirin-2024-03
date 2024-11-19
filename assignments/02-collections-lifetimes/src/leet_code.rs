use std::collections::HashMap;
use std::fmt::Display;

fn longest_equal_sequence_prescriptive<T>(sequence: &[T]) -> i32
where
    T: Clone,
    T: PartialEq,
{
    //shouldn't need lifetime

    let mut longest_seq: i32 = 0;
    let mut current_seq: i32 = 0;
    let mut last_thing: Option<T> = None;
    for thing in sequence {
        match &last_thing {
            Some(last_thing_value) => {
                if thing == last_thing_value {
                    current_seq += 1;
                } else {
                    current_seq = 1;
                }
                last_thing = Some(thing.clone());
            }
            //first thing in sequence
            None => {
                last_thing = Some(thing.clone());
                current_seq = 1;
            }
        }

        if current_seq > longest_seq {
            longest_seq = current_seq;
        }
    }
    longest_seq
}

fn longest_equal_sequence_functional<T>(sequence: &[T]) -> i32
where
    T: PartialEq,
{
    // sequence.

    if sequence.is_empty() {
        return 0;
    }

    let (max, last_length, last_value) = sequence.iter().skip(1).fold(
        (1, 1, &sequence[0]),
        |(longest_seq, current_sequence, last_num), value| {
            //if last_num is None
            if last_num == value {
                let current_length = current_sequence + 1;

                if current_length > longest_seq {
                    // println!(
                    //     "maximum: {:?}, current: {:?}, value: {}",
                    //     current_length, current_length, &value
                    // );
                    (current_length, current_length, &value)
                } else {
                    // println!(
                    //     "maximum: {:?}, current: {:?}, value: {}",
                    //     longest_seq, current_length, &value
                    // );
                    (longest_seq, current_length, &value)
                }
            } else {
                // println!(
                //     "maximum: {:?}, current: {:?}, value: {}",
                //     longest_seq, 1, &value
                // );
                (longest_seq, 1, &value)

                //do something
            } //need to iterate through a list
        },
    );
    max
}

pub fn is_valid_paranthesis(parenthesis: &str) -> bool {
    //construct a list of open parenthesis, popping when relevant

    let mut paren_stack: Vec<char> = Vec::new();
    let mut return_bool: bool = true;

    let mut expected_char: char;
    let mut last_pop: char = 'p';

    for character in parenthesis.chars() {
        // println!("paren stack:{:?}", paren_stack);
        if !return_bool {
            break;
        }
        if "({[".contains(character) {
            paren_stack.push(character);
        } else {
            match character {
                ')' => expected_char = '(',
                ']' => expected_char = '[',
                '}' => expected_char = '{',
                _ => expected_char = 'p',
            }
            let last_pop: Option<char> = paren_stack.pop();
            if last_pop == None {
                return_bool = false;
                continue;
            }
            if Some(expected_char) != last_pop {
                // println!(
                //     "last pop is {:?} and expected char is {}",
                //     last_pop, expected_char
                // );
                return_bool = false;
            }
        }
    }
    if !paren_stack.is_empty() {
        return_bool = false;
    }

    return_bool
}

fn longest_common_substring<'a>(first_str: &'a str, second_str: &'a str) -> &'a str {
    if first_str.is_empty() || second_str.is_empty() {
        return "";
    }

    let (smaller_str, larger_str) = if first_str.len() < second_str.len() {
        (first_str, second_str)
    } else {
        (second_str, first_str)
    };

    let mut longest = "";

    for i in 0..smaller_str.len() {
        for j in i + 1..=smaller_str.len() {
            let substring = &smaller_str[i..j];
            if larger_str.contains(substring) && substring.len() > longest.len() {
                longest = substring;
            }
        }
    }

    longest
}

fn longest_common_substring_multiple<'a>(strings: &'a [&'a str]) -> &'a str {
    if strings.is_empty() {
        return "";
    }

    let mut common_substring = strings[0];

    for s in strings.iter().skip(1) {
        common_substring = longest_common_substring(common_substring, s);
        if common_substring.is_empty() {
            break;
        }
    }

    common_substring
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_equal_sequence_prescriptive() {
        assert_eq!(longest_equal_sequence_prescriptive(&vec![1, 1, 1, 1, 1]), 5);
        assert_eq!(
            longest_equal_sequence_prescriptive(&vec![1.0, 2.0, 2.0, 2.0, 3.0, 4.0, 4.0]),
            3
        );
        assert_eq!(longest_equal_sequence_prescriptive(&vec![-100]), 1);
        let empty_vec: Vec<char> = Vec::new();
        assert_eq!(longest_equal_sequence_prescriptive(&empty_vec), 0);
        assert_eq!(
            longest_equal_sequence_prescriptive(&vec![
                1000, 1000, 2000, 2000, 2000, 3000, 3000, 3000, 3000
            ]),
            4
        );
        assert_eq!(
            longest_equal_sequence_prescriptive(&vec!['a', 'b', 'a', 'b', 'a', 'b']),
            1
        );
        let vec: Vec<u8> = vec![5, 5, 5, 1, 2, 3];
        assert_eq!(longest_equal_sequence_prescriptive(&vec), 3);
        assert_eq!(
            longest_equal_sequence_prescriptive(&vec![1, 2, 3, 4, 4, 4]),
            3
        );
        assert_eq!(longest_equal_sequence_prescriptive(&vec![1, 2, 3, 4, 5]), 1);
        assert_eq!(
            longest_equal_sequence_prescriptive(&vec![1, 1, 2, 2, 2, 3, 1, 1, 1, 1, 1]),
            5
        );
    }
    #[test]
    fn test_longest_equal_sequence_functional() {
        assert_eq!(longest_equal_sequence_functional(&vec![1, 1, 1, 1, 1]), 5);
        assert_eq!(
            longest_equal_sequence_functional(&vec![1.0, 2.0, 2.0, 2.0, 3.0, 4.0, 4.0]),
            3
        );
        assert_eq!(longest_equal_sequence_functional(&vec![-100]), 1);
        let empty_vec: Vec<char> = Vec::new();
        assert_eq!(longest_equal_sequence_functional(&empty_vec), 0);
        assert_eq!(
            longest_equal_sequence_functional(&vec![
                1000, 1000, 2000, 2000, 2000, 3000, 3000, 3000, 3000
            ]),
            4
        );
        assert_eq!(
            longest_equal_sequence_functional(&vec!['a', 'b', 'a', 'b', 'a', 'b']),
            1
        );
        let vec: Vec<u8> = vec![5, 5, 5, 1, 2, 3];
        assert_eq!(longest_equal_sequence_functional(&vec), 3);
        assert_eq!(
            longest_equal_sequence_functional(&vec![1, 2, 3, 4, 4, 4]),
            3
        );
        assert_eq!(longest_equal_sequence_functional(&vec![1, 2, 3, 4, 5]), 1);
        assert_eq!(
            longest_equal_sequence_functional(&vec![1, 1, 2, 2, 2, 3, 1, 1, 1, 1, 1]),
            5
        );
    }

    #[test]
    fn test_is_valid_paranthesis() {
        assert_eq!(is_valid_paranthesis(&String::from("{}")), true);
        assert_eq!(is_valid_paranthesis(&String::from("()")), true);
        assert_eq!(is_valid_paranthesis(&String::from("()[]{}")), true);
        assert_eq!(is_valid_paranthesis(&String::from("({[]})")), true);
        assert_eq!(is_valid_paranthesis(&String::from("([]){}{}([]){}")), true);
        assert_eq!(is_valid_paranthesis(&String::from("()(")), false);
        assert_eq!(is_valid_paranthesis(&String::from("(()")), false);
        assert_eq!(is_valid_paranthesis(&String::from("([)]{[})")), false);
        assert_eq!(
            is_valid_paranthesis(&String::from("({[()]}){[([)]}")),
            false
        );
        assert_eq!(
            is_valid_paranthesis(&String::from("()[]{}(([])){[()]}(")),
            false
        );
    }

    #[test]
    fn test_common_substring() {
        assert_eq!(longest_common_substring(&"abcdefg", &"bcdef"), "bcdef");
        assert_eq!(longest_common_substring(&"apple", &"pineapple"), "apple");
        assert_eq!(longest_common_substring(&"dog", &"cat"), "");
        assert_eq!(longest_common_substring(&"racecar", &"racecar"), "racecar");
        assert_eq!(longest_common_substring(&"ababc", &"babca"), "babc");
        assert_eq!(longest_common_substring(&"xyzabcxyz", &"abc"), "abc");
        assert_eq!(longest_common_substring(&"", &"abc"), "");
        assert_eq!(longest_common_substring(&"abcdefgh", &"defghijk"), "defgh");
        assert_eq!(longest_common_substring(&"xyabcz", &"abcxy"), "abc");
        assert_eq!(longest_common_substring(&"ABCDEFG", &"abcdefg"), "");
        assert_eq!(
            longest_common_substring(
                &"thisisaverylongstringwithacommonsubstring",
                &"anotherlongstringwithacommonsubstring"
            ),
            "longstringwithacommonsubstring"
        );
        assert_eq!(longest_common_substring("a", "a"), "a");
    }

    #[test]
    fn test_common_substring_multiple() {
        assert_eq!(
            longest_common_substring_multiple(&vec!["abcdefg", "cdef"]),
            "cdef"
        );
        assert_eq!(
            longest_common_substring_multiple(&vec!["apple", "pineapple", "maple", "snapple"]),
            "ple"
        );
        assert_eq!(
            longest_common_substring_multiple(&vec!["dog", "cat", "fish"]),
            ""
        );
        assert_eq!(
            longest_common_substring_multiple(&vec!["racecar", "car", "scar"]),
            "car"
        );
        assert_eq!(
            longest_common_substring_multiple(&vec!["ababc", "babca", "abcab"]),
            "abc"
        );
        assert_eq!(
            longest_common_substring_multiple(&vec!["xyzabcxyz", "abc", "zabcy", "abc"]),
            "abc"
        );
        assert_eq!(
            longest_common_substring_multiple(&vec!["", "abc", "def"]),
            ""
        );
        assert_eq!(
            longest_common_substring_multiple(&vec![
                "abcdefgh",
                "bcd",
                "bcdtravels",
                "abcs",
                "webcam"
            ]),
            "bc"
        );
        assert_eq!(
            longest_common_substring_multiple(&vec!["identical", "identical", "identical"]),
            "identical"
        );
        assert_eq!(
            longest_common_substring_multiple(&vec!["xyabcz", "abcxy", "zabc"]),
            "abc"
        );
        assert_eq!(longest_common_substring_multiple(&vec!["a", "a", "a"]), "a");
        assert_eq!(
            longest_common_substring_multiple(&vec![
                "thisisaverylongstringwiththecommonsubstring",
                "anotherlongstringwithacommonsubstring",
                "yetanotherstringthatcontainsacommonsubstring",
            ]),
            "commonsubstring",
        );
    }
}
