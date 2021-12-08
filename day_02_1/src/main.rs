use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut _test: Vec<String>;
    let mut horizontal = 0;
    let mut death = 0;
    // let mut vec = Vec::new();

    // read file line by line
    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                // split the string and saves it in a vector
                _test = ip.split_whitespace().map(|s| s.to_string()).collect();
                let tmp = _test[1].parse::<i32>().unwrap();
                if _test[0] == "forward" {
                    horizontal += tmp;
                } else if _test[0] == "up" {
                    death -= tmp;
                } else {
                    death += tmp;
                }
            }
        }
    }
    println!("{}", horizontal * death);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
