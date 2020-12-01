pub mod advent {

    use std::fs::File;
    use std::io::prelude::*;
    use std::vec::Vec;

    pub fn readFile(file_name: &str, error_message: &str) -> String {
        let mut file = File::open(file_name).expect(error_message);
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        contents
    }

    pub fn twenty_twenty_two_entries(input: String) {
        let mut vals: Vec<i32> = Vec::new();
        input
            .split("\n")
            .filter(|val| String::from(*val).parse::<i32>().is_ok())
            .for_each(|val| vals.push(i32::from_str_radix(val, 10).unwrap()));
        let res: Vec<i32> = vals
            .clone()
            .into_iter()
            .map(|x| 2020 - x)
            .filter(|x| vals.contains(x))
            .collect();
        println!("{:?}", res);
        println!("{:?}", res.into_iter().fold(1, |mul, x| mul * x));
    }

    pub fn twenty_twenty_three_entries(input: String) {
        let mut vals: Vec<i32> = Vec::new();
        input
            .split("\n")
            .filter(|val| String::from(*val).parse::<i32>().is_ok())
            .for_each(|val| vals.push(i32::from_str_radix(val, 10).unwrap()));
        let res: Vec<i32> = vals
            .clone()
            .into_iter()
            .map(|x| 2020 - x)
            .map(|x| 2020 - x)
            // .filter(|x| *x > 0)
            // .map(|x| 2020 - x)
            // .filter(|x| vals.contains(x))
            .collect();
        println!("{:?}", res);
        // println!("{:?}", res.into_iter().fold(1, |mul, x| mul * x));
    }
}
