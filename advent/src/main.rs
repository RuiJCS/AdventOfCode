use ::advent::advent::*;
const EXPENSES_FILE_NAME: &str = "files/expenses.txt";
const PASSWORDS_FILE_NAME: &str = "files/passwords.txt";
const MOUNTAIN_FILE_NAME: &str = "files/mountain.txt";
const PASSPORTS_FILE_NAME: &str = "files/passports.txt";
const TICKETS_FILE_NAME: &str = "files/tickets.txt";
const ANSWERS_FILE_NAME: &str = "files/answers.txt";
const RULES_FILE_NAME: &str = "files/rules.txt";
const ERROR_MESSAGE: &str = "Error reading input";

fn main() {
    println!("Day 1");
    let input = advent::advent::read_file(EXPENSES_FILE_NAME, ERROR_MESSAGE);
    println!("Two entries expenses {}", twenty_twenty_two_entries(input.clone()));
    //needs performance fix
    //println!("Three entries expenses {}", twenty_twenty_three_entries(input.clone()));

    println!("Day 2");
    let input = advent::advent::read_file(PASSWORDS_FILE_NAME, ERROR_MESSAGE);
    println!("Valid (interval) password entries {}", password_parsing_min_max(input.clone()));
    println!("Valid (position) password entries {}", password_parsing_pos(input.clone()));

    println!("Day 3");
    let input = read_file(MOUNTAIN_FILE_NAME, ERROR_MESSAGE);
    println!("Number of trees with slope (3,1) {}", mountain_sliding(&input, 3, 1));
    let res = mountain_sliding(&input, 3, 1)
        * mountain_sliding(&input, 1, 1)
        * mountain_sliding(&input, 5, 1)
        * mountain_sliding(&input, 7, 1)
        * mountain_sliding(&input, 1, 2);
    println!("Multiplication of trees found in different slopes {}", res);

    println!("Day 4");    
    let input = read_file(PASSPORTS_FILE_NAME, ERROR_MESSAGE);
    println!("Number of valid (fields) passports {}", valid_passports_missing_fields(&input));
    // needs performance fix
    // println!("Number of valid (rules) passports {}", valid_passports_rules(&input));

    println!("Day 5");
    let input = read_file(TICKETS_FILE_NAME, ERROR_MESSAGE);
    println!("Highest seat on a boarding pass is {}", highest_ticket(&input));
    println!("My seat is {}", my_seat(&input));

    println!("Day 6");
    let input = read_file(ANSWERS_FILE_NAME, ERROR_MESSAGE);
    println!("Number of times anyone answered yes {}", anyone_answers_yes(&input));
    println!("Number of times everyone answered yes {}", everyone_answers_yes(&input));

}
