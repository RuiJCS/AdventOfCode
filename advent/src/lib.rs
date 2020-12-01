pub mod advent {

    use itertools::Itertools;
    use std::fs::File;
    use std::io::prelude::*;
    use std::vec::Vec; // 0.8.2

    pub fn readFile(file_name: &str, error_message: &str) -> String {
        let mut file = File::open(file_name).expect(error_message);
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        contents
    }

    fn read_string_to_vec(input: String) -> Vec<i32> {
        let mut vals: Vec<i32> = Vec::new();
        input
            .split("\n")
            .filter(|val| String::from(*val).parse::<i32>().is_ok())
            .for_each(|val| vals.push(i32::from_str_radix(val, 10).unwrap()));
        vals
    }

    pub fn twenty_twenty_two_entries(input: String) {
        let mut vals = read_string_to_vec(input);
        let res: Vec<i32> = vals
            .clone()
            .into_iter()
            .map(|x| 2020 - x)
            .filter(|x| vals.contains(x))
            .collect();
        println!("{:?}", res);
        println!("{:?}", res.into_iter().fold(1, |mul, x| mul * x));
    }

    /// This makes all the possible permutations, must find a smarter and more efficient way of doing this
    pub fn twenty_twenty_three_entries(input: String) {
        let mut vals = read_string_to_vec(input);
        let res = &vals
            .clone()
            .into_iter()
            .permutations(3)
            .unique()
            .filter(|val| val[0] + val[1] + val[2] == 2020)
            .collect::<Vec<Vec<i32>>>()[0];
        println!(
            "{:?} {} {}",
            res,
            res.iter().fold(0, |acc, x| acc + x),
            res.iter().fold(1, |mul, x| mul * x)
        );
    }
}
