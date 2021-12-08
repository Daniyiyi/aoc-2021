use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut common: Vec<i32> = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut most_common = "".to_owned();
    let mut least_common = "".to_owned();
    // read file line by line
    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                for (i, c) in ip.chars().enumerate() {
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
    println!(
        "{}",
        isize::from_str_radix(&most_common, 2).unwrap()
            * isize::from_str_radix(&least_common, 2).unwrap()
    );
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
