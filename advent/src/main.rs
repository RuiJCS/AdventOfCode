use ::advent::advent::*;
const EXPENSES_FILE_NAME: &str = "files/expenses.txt";
const PASSWORDS_FILE_NAME: &str = "files/passwords.txt";
const MOUNTAIN_FILE_NAME: &str = "files/mountain.txt";
const PASSPORTS_FILE_NAME: &str = "files/passports.txt";
const TICKETS_FILE_NAME: &str = "files/tickets.txt";
const ANSWERS_FILE_NAME: &str = "files/answers.txt";
const RULES_FILE_NAME: &str = "files/rules.txt";
const ERROR_MESSAGE: &str = "Error reading file";

fn main() {
    let file = advent::advent::read_file(ANSWERS_FILE_NAME, ERROR_MESSAGE);
    // let res = mountain_sliding(&file, 3, 1)
    //     * mountain_sliding(&file, 1, 1)
    //     * mountain_sliding(&file, 5, 1)
    //     * mountain_sliding(&file, 7, 1)
    //     * mountain_sliding(&file, 1, 2);
    let res = count_similar_answers(&file);
    println!("{}", res);
}
