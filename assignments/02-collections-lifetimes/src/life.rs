use std::{string, thread::current};

fn split_string<'a>(string: &'a str, delimeter: &str) -> Vec<&'a str> {
    //let mut new_vec: Vec<&str> = Vec::new();

    string
        .split(delimeter)
        .filter(|string: &&str| !string.is_empty())
        .collect()
}

#[derive(PartialEq, Debug)]
struct Differences<'a, 'b> {
    only_in_first: Vec<&'a str>,
    only_in_second: Vec<&'b str>,
}

fn find_differences<'a, 'b>(first_string: &'a str, second_string: &'b str) -> Differences<'a, 'b> {
    let first_vec = split_string(first_string, " ");

    let second_vec = split_string(&second_string, " ");

    let mut first_differences: Vec<&str> = Vec::new();

    let mut second_differences: Vec<&str> = Vec::new();

    'outer: for word in &first_vec {
        for other_word in &second_vec {
            if other_word.contains(word) {
                continue 'outer;
            }
        }

        first_differences.push(word);
    }

    'outer: for word in &second_vec {
        for other_word in &first_vec {
            if other_word.contains(word) {
                continue 'outer;
            }
        }

        second_differences.push(word);
    }

    Differences {
        only_in_first: first_differences,
        only_in_second: second_differences,
    }
}

pub fn append_til_switch(mut to_append: String, copy_from: String) -> (String, i32) {
    //returns the total string being appended so far
    //as well as the number of characters copied over

    let vowels = "aeiouAEIOU";

    let mut first_char = true;
    let mut count = 0;

    for character in copy_from.chars() {
        if vowels.contains(character) && !first_char {
            break;
        } else {
            to_append.push(character);
            first_char = false;
        }
        count += 1;
    }
    (to_append, count)
}

pub fn remove_first_amt(to_cut: String, num_cut: i32) -> String {
    //to cut will always be available to cut a number

    let mut new_str: String = String::new();

    let mut string_iterator = to_cut.chars();

    let mut incr = 0;

    while let Some(character) = string_iterator.next() {
        incr += 1;

        if incr < num_cut + 1 {
            continue;
        }
        new_str.push(character);
    }

    new_str
}

pub fn merge_names(first_name: &str, second_name: &str) -> String {
    let mut string_buffer: String = String::new();
    let mut dynamic_first: String = String::from(first_name);
    let mut dynamic_second: String = String::from(second_name);

    //will be either 1 or 2
    let mut current_name = 1;
    let mut to_cut: i32 = 0;

    loop {
        println!("string buffer is : {}", string_buffer);
        println!("dynamic first is : {}", dynamic_first);
        println!("dynamic second is : {}", dynamic_second);
        match current_name {
            1 => {
                current_name = 2;

                if dynamic_first == "" && dynamic_second == "" {
                    current_name = 0;
                } else if dynamic_first == "" {
                    continue;
                }

                (string_buffer, to_cut) =
                    append_til_switch(string_buffer.clone(), dynamic_first.clone());
                dynamic_first = remove_first_amt(dynamic_first.clone(), to_cut);

                // if dynamic_first.chars().count() == 1 && to_cut == 1 {
                //     dynamic_first = String::from("");
                // }
            }
            2 => {
                current_name = 1;

                if dynamic_first == "" && dynamic_second == "" {
                    current_name = 0;
                } else if dynamic_second == "" {
                    continue;
                }

                (string_buffer, to_cut) =
                    append_til_switch(string_buffer.clone(), dynamic_second.clone());
                dynamic_second = remove_first_amt(dynamic_second.clone(), to_cut);

                // if dynamic_second.chars().count() == 1 && to_cut == 1 {
                //     dynamic_second = String::from("");
                // }
            }
            _ => {
                break;
            }
        }
    }

    string_buffer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_string() {
        // First, make sure the lifetimes were correctly marked
        let matches;
        let string_to_split = String::from("Hello, World!");

        {
            let delimeter = String::from(", ");
            matches = split_string(&string_to_split, &delimeter);
        }
        println!("Matches can be printed! See: {:?}", matches);

        // Now check the split logic
        assert_eq!(split_string(&"", &""), Vec::<&str>::new());
        assert_eq!(
            split_string(&"Hello, World!", &", "),
            vec!["Hello", "World!"]
        );
        assert_eq!(
            split_string(
                &"I this think this that this sentence this is this very this confusing this ",
                &" this "
            ),
            vec!["I", "think", "that", "sentence", "is", "very", "confusing"]
        );
        assert_eq!(
            split_string(&"apple🍎banana🍎orange", &"🍎"),
            vec!["apple", "banana", "orange"]
        );
        assert_eq!(
            split_string(
                &"Ayush;put|a,lot~of`random;delimeters|in|this,sentence",
                &";"
            ),
            vec![
                "Ayush",
                "put|a,lot~of`random",
                "delimeters|in|this,sentence"
            ]
        );
    }

    #[test]
    fn test_find_differences() {
        assert_eq!(
            find_differences(&"", &""),
            Differences {
                only_in_first: Vec::new(),
                only_in_second: Vec::new()
            }
        );
        assert_eq!(
            find_differences(&"pineapple pen", &"apple"),
            Differences {
                only_in_first: vec!["pineapple", "pen"],
                only_in_second: Vec::new()
            }
        );
        assert_eq!(
            find_differences(
                &"Sally sold seashells at the seashore",
                &"Seashells seashells at the seashore"
            ),
            Differences {
                only_in_first: vec!["Sally", "sold"],
                only_in_second: vec!["Seashells"]
            }
        );
        assert_eq!(
            find_differences(
                "How much wood could a wood chuck chuck",
                "If a wood chuck could chuck wood"
            ),
            Differences {
                only_in_first: vec!["How", "much"],
                only_in_second: vec!["If"]
            }
        );
        assert_eq!(
            find_differences(
                &"How much ground would a groundhog hog",
                &"If a groundhog could hog ground"
            ),
            Differences {
                only_in_first: vec!["How", "much", "would"],
                only_in_second: vec!["If", "could"]
            }
        );
    }

    #[test]
    fn test_merge_names() {
        assert_eq!(merge_names(&"alex", &"jake"), "aljexake");
        assert_eq!(merge_names(&"steven", &"stephen"), "ststevephenen");
        assert_eq!(merge_names(&"gym", &"rhythm"), "gymrhythm");
        assert_eq!(merge_names(&"walter", &"gibraltor"), "wgaltibreraltor");
        assert_eq!(merge_names(&"baker", &"quaker"), "bqakueraker");
        assert_eq!(merge_names(&"", &""), "");
        assert_eq!(merge_names(&"samesies", &"samesies"), "ssamamesesiieses");
        assert_eq!(merge_names(&"heather", &"meagan"), "hmeeathageran");
        assert_eq!(merge_names(&"panda", &"turtle"), "ptandurtlae");
        assert_eq!(merge_names(&"hot", &"sauce"), "hsotauce");
        assert_eq!(merge_names(&"", &"second"), "second");
        assert_eq!(merge_names(&"first", &""), "first");
    }
}
