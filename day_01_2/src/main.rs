use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec::Vec;

fn main() {
    let mut _last = 0;
    let mut count = 0;
    let mut vec = Vec::new();

    // read file line by line
    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                // convert string to int and pushes it to vector
                vec.push(ip.parse::<i32>().unwrap());
            }
        }
    }

    for i in 0..(vec.len() - 2) {
        let cur = vec[i + 0] + vec[i + 1] + vec[i + 2];
        if _last == 0 {
            _last = cur;
        } else if _last < cur {
            count += 1;
        }
        _last = cur;
    }
    println!("{}", count);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
