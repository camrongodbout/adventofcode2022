// https://adventofcode.com/2022/day/1

use std::cmp::max;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./calories") {
        // Consumes the iterator, returns an (Optional) String
        let mut current_calories: i64 = 0;
        let mut calorie_counts = Vec::new();
        for line in lines {
            if let Ok(inp) = line {
                println!("{}", inp);
                if inp.len() == 0 {
                    calorie_counts.push(current_calories);
                    current_calories = 0;
                    continue;
                }
                let calorie: i64 = inp.parse().unwrap();
                current_calories += calorie;
            }
        }
        calorie_counts.sort();
        println!("Top 3 max values: {:?}", &calorie_counts[calorie_counts.len()-3 ..]);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
