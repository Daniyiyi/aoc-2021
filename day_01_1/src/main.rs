use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut _last = 0;
    let mut count = 0;
    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let cur: i32 = ip.parse().unwrap();
                if _last == 0 {
                    _last = cur;
                } else if _last < cur {
                    count += 1;
                }
                _last = cur;
            }
        }
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
