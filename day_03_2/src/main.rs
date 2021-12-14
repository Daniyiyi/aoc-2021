use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut common: Vec<i32> = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut input_most_common: Vec<String> = Vec::new();
    let mut most_common = "".to_owned();
    let mut least_common = "".to_owned();
    // read file line by line
    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                input_most_common.push(ip);
                for (i, c) in input_most_common.last().unwrap().chars().enumerate() {
                    if c == '1' {
                        common[i] += 1;
                    } else {
                        common[i] -= 1;
                    }
                }
            }
        }
    }
    for i in 0..12 {
        if common[i] > 0 {
            most_common.push_str("1");
            least_common.push_str("0");
        } else {
            most_common.push_str("0");
            least_common.push_str("1");
        }
    }


    let mut input_least_common = input_most_common.clone();

    // this some who find the most common value
    for i in 0..12 {
        let c: Vec<char> = most_common.chars().collect();
        for j in (0..input_most_common.len()).rev() {
            let ch: Vec<char> = input_most_common[j].chars().collect();
            if ch[i] != c[i] {
                input_most_common.remove(j);
            }
            if input_most_common.len() == 1 {
                break;
            }
        }
        if input_most_common.len() == 1 {
            break;
        }
        most_common = re_calc(&input_most_common);
    }

    // this some who find the least common value
    for i in 0..12 {
        let c: Vec<char> = most_common.chars().collect();
        for j in (0..input_least_common.len()).rev() {
            let ch: Vec<char> = input_least_common[j].chars().collect();
            if ch[i] == c[i] {
                input_least_common.remove(j);
            }
            if input_least_common.len() == 1 {
                break;
            }
        }
        if input_least_common.len() == 1 {
            break;
        }
        most_common = re_calc(&input_least_common);
    }


    println!(
        "{}",
        isize::from_str_radix(&input_least_common[0], 2).unwrap()
            * isize::from_str_radix(&input_most_common[0], 2).unwrap()
    );
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn re_calc(element: &Vec<String>) -> String {
    let mut common: Vec<i32> = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut most_common: String = "".to_owned();
    for elem in element.iter() {
        for (i, c) in elem.chars().enumerate() {
            if c == '1' {
                common[i] += 1;
            } else {
                common[i] -= 1;
            }
        }
    }

    for i in 0..12 {
        if common[i] < 0 {
            most_common.push_str("0");
        } else {
            most_common.push_str("1");
        }
    }
    most_common
}

