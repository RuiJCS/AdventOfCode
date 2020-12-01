use ::advent::advent::*;

const FILE_NAME: &str = "expenses.txt";
const ERROR_MESSAGE: &str = "Error reading file";

fn main() {
    let file = advent::advent::readFile(FILE_NAME, ERROR_MESSAGE);
    twenty_twenty_three_entries(file);
}
