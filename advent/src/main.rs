use ::advent::advent::*;

const EXPENSES_FILE_NAME: &str = "files/expenses.txt";
const PASSWORDS_FILE_NAME: &str = "files/passwords.txt";
const MOUNTAIN_FILE_NAME: &str = "files/mountain.txt";
const ERROR_MESSAGE: &str = "Error reading file";

fn main() {
    let file = advent::advent::read_file(MOUNTAIN_FILE_NAME, ERROR_MESSAGE);
    let res = mountain_sliding(&file, 3, 1)
        * mountain_sliding(&file, 1, 1)
        * mountain_sliding(&file, 5, 1)
        * mountain_sliding(&file, 7, 1)
        * mountain_sliding(&file, 1, 2);
    println!("{}", res);
}
