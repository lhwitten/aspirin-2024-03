pub fn print_fizz_buzz(max_num: u32) {
    for i in 1..max_num {
        let mut div = 0;

        if i % 3 == 0 {
            div += 1;
            print!("Fizz");
        }
        if i % 5 == 0 {
            print!("Buzz");
            div += 1;
        }
        if div == 0 {
            print!("{}", i);
        }
        println!();
    }
}
