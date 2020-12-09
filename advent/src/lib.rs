pub mod advent {

    use itertools::Itertools;
    use regex::Regex;
    use std::fs::File;
    use std::io::prelude::*;
    use std::{collections::HashMap, vec::Vec};

    pub fn read_file(file_name: &str, error_message: &str) -> String {
        let mut file = File::open(file_name).expect(error_message);
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        contents
    }

    fn read_string_to_vec_numbers(input: String) -> Vec<i32> {
        let mut vals: Vec<i32> = Vec::new();
        input
            .lines()
            .filter(|val| String::from(*val).parse::<i32>().is_ok())
            .for_each(|val| vals.push(i32::from_str_radix(val, 10).unwrap()));
        vals
    }

    fn read_string_to_vec(input: &String) -> Vec<String> {
        input.lines().map(|s| s.to_string()).collect()
    }

    pub fn twenty_twenty_two_entries(input: String) {
        let vals = read_string_to_vec_numbers(input);
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
        let vals = read_string_to_vec_numbers(input);
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

    pub fn password_parsing_min_max(input: String) -> u32 {
        let passwords: Vec<&str> = input.split('\n').collect();
        let (mut keys, mut passes): (Vec<&str>, Vec<&str>) = (Vec::new(), Vec::new());
        for password in passwords {
            if !password.is_empty() {
                let res: (&str, &str) = password.split(':').collect_tuple().unwrap();
                keys.push(res.0);
                passes.push(res.1);
            }
        }

        let correct_passwords = keys
            .into_iter()
            .zip(passes)
            .filter(|(key, pass)| {
                let key_string = String::from(*key);
                let (x, y, z): (&str, &str, &str) = key_string
                    .split(|c| c == '-' || c == ' ')
                    .collect_tuple()
                    .unwrap();
                let (min, max, letter) = (
                    u32::from_str_radix(x.to_string().as_str(), 10).unwrap(),
                    u32::from_str_radix(y.to_string().as_str(), 10).unwrap(),
                    z.chars().nth(0).unwrap(),
                );
                let count = String::from(*pass)
                    .chars()
                    .into_iter()
                    .filter(|c| c.eq(&letter))
                    .count() as u32;
                count >= min && count <= max
            })
            .count();
        correct_passwords as u32
    }

    pub fn password_parsing_pos(input: String) -> u32 {
        let passwords: Vec<&str> = input.split('\n').collect();
        let (mut keys, mut passes): (Vec<&str>, Vec<&str>) = (Vec::new(), Vec::new());
        for password in passwords {
            if !password.is_empty() {
                let res: (&str, &str) = password.split(':').collect_tuple().unwrap();
                keys.push(res.0);
                passes.push(res.1);
            }
        }

        let correct_passwords = keys
            .into_iter()
            .zip(passes)
            .filter(|(key, pass)| {
                let key_string = String::from(*key);
                let (x, y, z): (&str, &str, &str) = key_string
                    .split(|c| c == '-' || c == ' ')
                    .collect_tuple()
                    .unwrap();
                let (first, second, letter) = (
                    u32::from_str_radix(x.to_string().as_str(), 10).unwrap(),
                    u32::from_str_radix(y.to_string().as_str(), 10).unwrap(),
                    z.chars().nth(0).unwrap(),
                );
                println!("YOLO{} {} {} {}", pass, first, second, pass.len());
                let x = pass.clone().chars().nth(first as usize).unwrap();
                let y = pass.clone().chars().nth(second as usize).unwrap();
                // let (x, y) = (
                //     pass.chars().nth(first as usize).unwrap(),
                //     pass.chars().nth(second as usize).unwrap(),
                // );
                x != y && (x == letter || y == letter)
            })
            .count();
        correct_passwords as u32
    }

    pub fn mountain_sliding(input: &String, velocity_x: i32, velocity_y: u32) -> u32 {
        let mountain: Vec<String> = read_string_to_vec(input)
            .into_iter()
            .filter(|s| !s.is_empty())
            .collect();
        let ys: Vec<usize> = (0..mountain.len()).step_by(velocity_y as usize).collect();
        // mountain.iter().for_each(|s| println!("{}", *s));
        let (acc, _) = mountain
            .into_iter()
            .enumerate()
            .filter(|e| ys.contains(&e.0))
            .fold((0u32, 0), |(acc, x), (_, s)| {
                let shift_x = velocity_x * (x + velocity_x < s.len() as i32) as i32
                    + (velocity_x - s.len() as i32) * (x + velocity_x >= s.len() as i32) as i32;
                let tree = 1 * s.chars().nth((x) as usize).unwrap().eq(&'#') as u32;
                (acc + tree, x + shift_x)
            });
        acc
    }

    fn read_passports(input: &String) -> Vec<String> {
        input.split("\n\n").map(|s| s.to_string()).collect()
    }

    pub fn valid_passports_missing_fields(input: &String) -> u32 {
        let keywords: Vec<&str> = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
        let passports = read_passports(input);
        let count = passports
            .iter()
            .filter(|s| keywords.iter().fold(true, |b, ss| b && (*s).contains(*ss)))
            .count();
        count as u32
    }

    pub fn parse_passport(passport: &HashMap<String, String>) -> bool {
        let keywords: Vec<&str> = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
        passport
            .iter()
            .filter(|(key, val)| match (*key).as_str() {
                "byr" => {
                    let regex = Regex::new("^((19[2-9][0-9])|(200[0-2]))$").unwrap();
                    regex.is_match(val)
                }
                "iyr" => {
                    let regex = Regex::new("^20(1\\d|20)$").unwrap();
                    regex.is_match(val)
                }
                "eyr" => {
                    let regex = Regex::new("^20(2[0-9]|30)$").unwrap();
                    regex.is_match(val)
                }
                "hgt" => {
                    if val.contains("cm") {
                        let regex = Regex::new("^1([5-8][0-9]|9[0-3])cm$").unwrap();
                        regex.is_match(val)
                    } else if val.contains("in") {
                        let regex = Regex::new("^(59|6[0-9]|7[0-6])in$").unwrap();
                        regex.is_match(val)
                    } else {
                        false
                    }
                }
                "hcl" => {
                    let regex = Regex::new("^#[A-Fa-f0-9]{6}$").unwrap();
                    regex.is_match(val)
                }
                "ecl" => {
                    let regex = Regex::new("^[a-zA-Z]{3}$").unwrap();
                    let eye = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
                    eye.contains(&val.as_str()) && regex.is_match(val)
                }
                "pid" => {
                    let regex = Regex::new("^[0-9]{9}$").unwrap();
                    regex.is_match(val)
                }
                _ => false,
            })
            .count()
            >= 7
            && keywords.iter().fold(true, |res, key| {
                passport.contains_key(&(*key).to_string()) && res
            })
    }

    pub fn valid_passports_rules(input: &String) -> u32 {
        let passports = read_passports(input);
        let passports =
            passports
                .iter()
                .fold(Vec::<HashMap<String, String>>::new(), |mut entry, s| {
                    entry.push((*s).replace(" ", "\n").lines().collect_vec().iter().fold(
                        HashMap::<String, String>::new(),
                        |mut field, s| {
                            let tuple: (&str, &str) = (*s).split(":").collect_tuple().unwrap();
                            field.insert(tuple.0.to_string(), tuple.1.to_string());
                            field
                        },
                    ));
                    entry
                });
        passports
            .iter()
            .fold(0, |res, m| parse_passport(m) as u32 + res)
    }
}
