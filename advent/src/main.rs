use ::advent::advent::*;

const EXPENSES_FILE_NAME: &str = "expenses.txt";
const PASSWORDS_FILE_NAME: &str = "passwords.txt";
const ERROR_MESSAGE: &str = "Error reading file";

fn main() {
    let file = advent::advent::read_file(PASSWORDS_FILE_NAME, ERROR_MESSAGE);
    password_parsing_pos(file);
}
