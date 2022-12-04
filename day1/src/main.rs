// https://adventofcode.com/2022/day/1

use std::cmp::max;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./calories") {
        // Consumes the iterator, returns an (Optional) String
        let mut current_calories: f32 = 0.0;
        let mut max_calories: f32 = 0.0;
        for line in lines {
            if let Ok(inp) = line {
                println!("{}", inp);
                if inp.len() == 0 {
                    if max_calories < current_calories {
                        max_calories = current_calories;
                    }
                    current_calories = 0.0;
                    continue;
                }
                let calorie: f32 = inp.parse().unwrap();
                current_calories += calorie;
            }
        }
        println!("{}", max_calories)
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
