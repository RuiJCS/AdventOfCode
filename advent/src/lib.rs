pub mod advent {

    use itertools::Itertools;
    use regex::Regex;
    use std::{collections::BTreeMap, fs::File};
    use std::io::prelude::*;
    use std::{collections::HashMap, vec::Vec};

    pub fn read_file(file_name: &str, error_message: &str) -> String {
        let mut file = File::open(file_name).expect(error_message);
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        contents
    }

    fn read_string_to_vec_numbers(input: String) -> Vec<u32> {
        let mut vals: Vec<u32> = Vec::new();
        input
            .lines()
            .filter(|val| String::from(*val).parse::<i32>().is_ok())
            .for_each(|val| vals.push(u32::from_str_radix(val, 10).unwrap()));
        vals
    }

    fn read_string_to_vec(input: &String) -> Vec<String> {
        input.lines().map(|s| s.to_string()).collect()
    }


    /// Resolution for challenge 1 in Advent of Code 
    ///
    /// https://adventofcode.com/2020/day/1
    pub fn twenty_twenty_two_entries(input: String) -> u32 {
        let vals = read_string_to_vec_numbers(input);
        let res: Vec<u32> = vals
            .clone()
            .into_iter()
            .map(|x| 2020 - x)
            .filter(|x| vals.contains(x))
            .collect();
        res.into_iter().fold(1, |mul, x| mul * x )
    }

    /// This makes all the possible permutations, must find a smarter and more efficient way of doing this
    pub fn twenty_twenty_three_entries(input: String) -> u32 {
        let vals = read_string_to_vec_numbers(input);
        let res = &vals
            .clone()
            .into_iter()
            .permutations(3)
            .unique()
            .filter(|val| val[0] + val[1] + val[2] == 2020)
            .collect::<Vec<Vec<u32>>>()[0];
        res.iter().fold(1, |mul, x| mul * x)
    }

    /// Resolution for challenge 2 in Advent of Code
    /// 
    /// https://adventofcode.com/2020/day/2
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
                let x = pass.clone().chars().nth(first as usize).unwrap();
                let y = pass.clone().chars().nth(second as usize).unwrap();
                x != y && (x == letter || y == letter)
            })
            .count();
        correct_passwords as u32
    }

    /// Resolution for challenge 3 in Advent of Code
    /// 
    /// https://adventofcode.com/2020/day/3
    pub fn mountain_sliding(input: &String, velocity_x: i32, velocity_y: u32) -> u32 {
        let mountain: Vec<String> = read_string_to_vec(input)
            .into_iter()
            .filter(|s| !s.is_empty())
            .collect();
        let ys: Vec<usize> = (0..mountain.len()).step_by(velocity_y as usize).collect();
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

    /// Resolution for challenge 4 in Advent of Code
    /// 
    /// https://adventofcode.com/2020/day/4
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


    /// Resolution for challenge 5 in Advent of Code
    /// 
    /// https://adventofcode.com/2020/day/5
    fn parse_tickets(input: &String) -> Vec<(Vec<u32>, Vec<u32>)> {
        input
            .lines()
            .map(|s| {
                let parsed_string = s
                    .to_string()
                    .replace("F", "0")
                    .replace("B", "1")
                    .replace("L", "0")
                    .replace("R", "1");
                let (row, seat) = parsed_string.split_at(7);
                (
                    row.chars()
                        .map(|c| c.to_digit(10).unwrap())
                        .collect::<Vec<u32>>(),
                    seat.chars()
                        .map(|c| c.to_digit(10).unwrap())
                        .collect::<Vec<u32>>(),
                )
            })
            .collect::<Vec<(Vec<u32>, Vec<u32>)>>()
    }

    fn binary_to_decimal(base: &u32, exponent: &u32) -> u32 {
        base * 2u32.pow(exponent - 1)
    }

    fn parse_tickets_to_map(input: &String) -> HashMap<Vec<u32>, Vec<Vec<u32>>> {
        let tickets = input
            .lines()
            .map(|s| {
                let parsed_string = s
                    .to_string()
                    .replace("F", "0")
                    .replace("B", "1")
                    .replace("L", "0")
                    .replace("R", "1");
                let (row, seat) = parsed_string.split_at(7);
                (
                    row.chars()
                        .map(|c| c.to_digit(10).unwrap())
                        .collect::<Vec<u32>>(),
                    seat.chars()
                        .map(|c| c.to_digit(10).unwrap())
                        .collect::<Vec<u32>>(),
                )
            })
            .collect::<Vec<(Vec<u32>, Vec<u32>)>>();
        tickets.iter().fold(
            HashMap::<Vec<u32>, Vec<Vec<u32>>>::new(),
            |mut map, (row, seat)| {
                let entry = map.entry(row.clone()).or_insert(vec![seat.clone()]);
                if !entry.contains(&seat) {
                    entry.push(seat.clone());
                }
                map
            },
        )
    }

    pub fn highest_ticket(input: &String) -> u32 {
        let tickets = parse_tickets(input);
        tickets.iter().fold(0u32, |max, (row, seat)| {
            let row_number = row
                .iter()
                .fold((0u32, 7u32), |(a, e), r| {
                    (binary_to_decimal(r, &e) + a, e - 1)
                })
                .0
                * 8u32;
            let row_number = row_number
                + seat
                    .iter()
                    .fold((0u32, 3u32), |(a, e), s| {
                        (binary_to_decimal(s, &e) + a, e - 1)
                    })
                    .0;
            max * (max >= row_number) as u32 + row_number * (max < row_number) as u32
        })
    }

    pub fn my_seat(input: &String) -> u32 {
        let tickets = parse_tickets_to_map(input);
        let my_row = tickets
            .into_iter()
            .filter(|(k, v)| {
                (*k.as_slice() != [0; 7] && *k.as_slice() != [1; 7]) && (*v).len() != 8
            })
            .collect::<HashMap<Vec<u32>, Vec<Vec<u32>>>>();
        let seats: Vec<Vec<u32>> = vec![
            vec![0, 1, 0],
            vec![1, 0, 1],
            vec![0, 1, 1],
            vec![1, 1, 0],
            vec![0, 0, 1],
            vec![1, 0, 0],
            vec![1, 1, 1],
            vec![0, 0, 0],
        ];
        my_row
            .iter()
            .fold((0, seats), |(acc, seats), (k, v)| {
                let seat_number = k
                    .iter()
                    .fold((0, 7), |(sum, exponent), n| {
                        (sum + binary_to_decimal(n, &exponent), exponent - 1)
                    })
                    .0;
                let seat: Vec<u32> = (seats
                    .clone()
                    .into_iter()
                    .filter(|l| !v.contains(l))
                    .collect::<Vec<Vec<u32>>>())[0]
                    .clone();
                let seat_number = seat_number * 8
                    + seat
                        .iter()
                        .fold((0, 3), |(acc, exponent), i| {
                            (acc + binary_to_decimal(i, &exponent), exponent - 1)
                        })
                        .0;
                (acc + seat_number, seats)
            })
            .0
    }

    /// Resolution for challenge 6 in Advent of Code
    /// 
    /// https://adventofcode.com/2020/day/6
    pub fn anyone_answers_yes(input: &String) -> u32 {
        input
            .split("\n\n")
            .collect::<Vec<&str>>()
            .iter()
            .fold(0, |acc, s| {
                acc + s
                    .to_string()
                    .replace("\n", "")
                    .chars()
                    .sorted()
                    .dedup()
                    .collect_vec()
                    .len() as u32
            })
    }

    pub fn everyone_answers_yes(input: &String) -> u32 {
        // Break up by group of answers
        input.split("\n\n").collect_vec().iter().fold(0, |acc, s| {
            //Separo por linha
            let vec_string = s.split("\n").collect_vec();
            let res = vec_string
                .iter()
                .fold(
                    HashMap::<char, u32>::new(),
                    |mut map: HashMap<char, u32>, s| {
                        //count the charts ocurrence
                        s.chars().for_each(|c| {
                            let entry = map.entry(c).or_insert(0);
                            *entry += 1;
                        });
                        map
                    },
                )
                .into_iter()
                .filter(|(key, value)| {
                    // see if the occurrence of each character is the same as the number of lines
                    *value >= vec_string.len() as u32
                })
                .count() as u32;
            res + acc
        })
    }

    // pub fn count_shinny(input: &String) -> u32 {
    //     let rules = input.lines().collect_vec();
    //     let regex = Regex::new(".*\\d\\s.*").unwrap();
    //     let gold_bags = input.lines().filter(|s| regex.is_match(s));
    //     let gold_allowed = rules.iter().fold(HashMap::<BTreeMap<(&str,u32)>::new(), |mut map|);

    //     0
    // }
}
