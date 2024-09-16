#[derive(PartialEq, Debug)]
enum PrefixNum {
    Decimal,
    Hex,
    Binary,
}
#[derive(PartialEq, Debug)]
enum MathOperations {
    OpAND,
    OpOR,
    OpXOR,
}

// Prefix Checking
fn amit_this_is_ridiculous(number_str: &str) -> PrefixNum {
    let char_count = number_str.chars().count();

    // length check
    if char_count == 0 {
        panic!("ERROR OUT 0 CHARACTER NUMBER");
    } else if char_count <= 2 {
        return PrefixNum::Decimal;
    }

    // check prefix
    let first_2_chars: String = number_str.chars().take(2).collect();
    if first_2_chars == "0x" {
        PrefixNum::Hex
    } else if first_2_chars == "0b" {
        PrefixNum::Binary
    } else {
        PrefixNum::Decimal
    }
}

fn ayush_this_is_ridiculous(oper_str: &str) -> MathOperations {
    //parse operation argument
    if oper_str == "&" || oper_str.eq_ignore_ascii_case("AND") {
        MathOperations::OpAND
    } else if oper_str == "|" || oper_str.eq_ignore_ascii_case("OR") {
        MathOperations::OpOR
    } else if oper_str == "^" || oper_str.eq_ignore_ascii_case("XOR") {
        MathOperations::OpXOR
    } else {
        panic!("No valid operation");
    }
    //Note - SCALABLE CODE - SCALE HERE
}

fn lily_give_me_100(num_repr: &str, prefix_type: &PrefixNum) -> String {
    //convert all representations to binary
    let without_first_two: String = num_repr.chars().skip(2).collect();

    let integer_repr: u32 = match prefix_type {
        PrefixNum::Hex => u32::from_str_radix(&without_first_two, 16).unwrap(),
        PrefixNum::Binary => u32::from_str_radix(&without_first_two, 2).unwrap(),
        PrefixNum::Decimal => num_repr.parse().unwrap(),
    };

    format!("{:b}", integer_repr)
}

fn clean_binaries(num1: &str, num2: &str) -> (String, String) {
    //make binary strings the same . Pads with 0s to most significant places
    let length_1 = num1.len();
    let length_2 = num2.len();

    let bigger_length = std::cmp::max(length_1, length_2);

    let mut op_string_1 = String::new();
    let mut op_string_2 = String::new();

    op_string_1.push_str(&"0".repeat(bigger_length - length_1));
    op_string_1.push_str(num1);

    op_string_2.push_str(&"0".repeat(bigger_length - length_2));
    op_string_2.push_str(num2);

    (op_string_1, op_string_2)
}

fn do_xor(num1: &str, num2: &str) -> String {
    //performs xor operation. Allows unclean binary representations as start

    let (clean_str_1, clean_str_2) = clean_binaries(num1, num2);
    let mut output_string = String::new();

    for (char1, char2) in clean_str_1.chars().zip(clean_str_2.chars()) {
        if char1 != char2 {
            output_string.push('1');
        } else {
            output_string.push('0');
        }
    }

    output_string
}

fn do_or(num1: &str, num2: &str) -> String {
    //performs or operation. Allows unclean binary representations as start
    let (clean_str_1, clean_str_2) = clean_binaries(num1, num2);
    let mut output_string = String::new();

    for (char1, char2) in clean_str_1.chars().zip(clean_str_2.chars()) {
        if char1 == '1' || char2 == '1' {
            output_string.push('1');
        } else {
            output_string.push('0');
        }
    }

    output_string
}

fn do_and(num1: &str, num2: &str) -> String {
    let (clean_str_1, clean_str_2) = clean_binaries(num1, num2);
    let mut output_string = String::new();

    for (char1, char2) in clean_str_1.chars().zip(clean_str_2.chars()) {
        if char1 == char2 && char1 == '1' {
            output_string.push('1');
        } else {
            output_string.push('0');
        }
    }

    output_string
}

fn aditi_please_grade_nicely(binary_1: &str, binary_2: &str, operation_todo: MathOperations) {
    //perform operations and print
    let output_string = match operation_todo {
        MathOperations::OpAND => do_and(binary_1, binary_2),
        MathOperations::OpXOR => do_xor(binary_1, binary_2),
        MathOperations::OpOR => do_or(binary_1, binary_2),
        //Note - SCALABLE CODE - SCALE HERE
    };
    let final_out = u32::from_str_radix(&output_string, 2).unwrap();
    println!("Result of Operation is {}", final_out);
    println!("Result of Operation is {} in binary", output_string);
}

pub fn formal_complaint() {
    //the function that prompts the calculator and runs

    //grab input 1
    let input = String::new();
    let input = input.trim();

    println!("First input is {}", input);

    //grab prefixes
    let prefix_1 = amit_this_is_ridiculous(input);

    //grab input 2
    let input_2 = String::new();
    let input_2 = input_2.trim();

    println!("Second input is {}", input_2);

    let prefix_2 = amit_this_is_ridiculous(input_2);

    //check prefixes against each other
    if prefix_2 != prefix_1 {
        panic!("Prefixes don't match. Try again");
    }
    //grab operation
    let input_3 = String::new();
    let input_3 = input_3.trim();

    //parse operation message
    let operation_todo = ayush_this_is_ridiculous(input_3);

    //parse inputs into binary
    let binary_1 = lily_give_me_100(input, &prefix_1);
    let binary_2 = lily_give_me_100(input_2, &prefix_2);

    //perform operations
    aditi_please_grade_nicely(&binary_1, &binary_2, operation_todo);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_amit_this_is_ridiculous() {
        assert_eq!(amit_this_is_ridiculous("42"), PrefixNum::Decimal);
        assert_eq!(amit_this_is_ridiculous("0x2A"), PrefixNum::Hex);
        assert_eq!(amit_this_is_ridiculous("0b1010"), PrefixNum::Binary);
    }

    #[test]
    fn test_ayush_this_is_ridiculous() {
        assert_eq!(ayush_this_is_ridiculous("&"), MathOperations::OpAND);
        assert_eq!(ayush_this_is_ridiculous("AND"), MathOperations::OpAND);
        assert_eq!(ayush_this_is_ridiculous("and"), MathOperations::OpAND);
        assert_eq!(ayush_this_is_ridiculous("|"), MathOperations::OpOR);
        assert_eq!(ayush_this_is_ridiculous("or"), MathOperations::OpOR);
        assert_eq!(ayush_this_is_ridiculous("OR"), MathOperations::OpOR);
        assert_eq!(ayush_this_is_ridiculous("XOR"), MathOperations::OpXOR);
        assert_eq!(ayush_this_is_ridiculous("xor"), MathOperations::OpXOR);
        assert_eq!(ayush_this_is_ridiculous("^"), MathOperations::OpXOR);
    }

    #[test]
    fn test_lily_give_me_100() {
        assert_eq!(lily_give_me_100("0x2A", &PrefixNum::Hex), "101010");
        assert_eq!(lily_give_me_100("42", &PrefixNum::Decimal), "101010");
        assert_eq!(lily_give_me_100("0b101010", &PrefixNum::Binary), "101010");
        assert_eq!(lily_give_me_100("0b0", &PrefixNum::Binary), "0");
        assert_eq!(lily_give_me_100("0b1110111", &PrefixNum::Binary), "1110111");
        assert_eq!(
            lily_give_me_100("10000", &PrefixNum::Decimal),
            "10011100010000"
        );
        assert_eq!(
            lily_give_me_100("0xaBCDEf", &PrefixNum::Hex),
            "101010111100110111101111"
        );
    }

    #[test]
    fn test_clean_binaries() {
        let (clean1, clean2) = clean_binaries("1010", "11");
        assert_eq!(clean1, "1010");
        assert_eq!(clean2, "0011");
    }

    #[test]
    fn test_do_xor() {
        assert_eq!(do_xor("1010", "0011"), "1001");
        assert_eq!(do_xor("1111111", "0011"), "1111100");
    }

    #[test]
    fn test_do_or() {
        assert_eq!(do_or("1010", "0011"), "1011");
        assert_eq!(do_or("111111", "0011"), "111111");
    }

    #[test]
    fn test_do_and() {
        assert_eq!(do_and("1010", "0011"), "0010");
        assert_eq!(do_and("111111", "0011"), "000011");
    }

    #[test]
    fn test_aditi_please_grade_nicely() {
        let binary_1 = "1010";
        let binary_2 = "0011";
        let mut result;

        result = do_and(binary_1, binary_2);
        assert_eq!(result, "0010");

        result = do_xor(binary_1, binary_2);
        assert_eq!(result, "1001");

        result = do_or(binary_1, binary_2);
        assert_eq!(result, "1011");
    }
}
