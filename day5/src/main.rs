use std::cmp::max;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct ParsedLine {
    count: i32,
    from: i32,
    destination: i32
}

fn main() {
    /*
    1. Construct Cargo
    2. Read in step
    3. Perform step
    4. Do until all steps then print out top of each stack

                [J] [Z] [G]
            [Z] [T] [S] [P] [R]
[R]         [Q] [V] [B] [G] [J]
[W] [W]     [N] [L] [V] [W] [C]
[F] [Q]     [T] [G] [C] [T] [T] [W]
[H] [D] [W] [W] [H] [T] [R] [M] [B]
[T] [G] [T] [R] [B] [P] [B] [G] [G]
[S] [S] [B] [D] [F] [L] [Z] [N] [L]
 1   2   3   4   5   6   7   8   9

     */
    let mut cargo_1 = vec!['S', 'T', 'H', 'F', 'W', 'R'];
    let mut cargo_2 = Vec::from(['S', 'G', 'D', 'Q', 'W']);
    let mut cargo_3 = Vec::from(['B', 'T', 'W']);
    let mut cargo_4 = Vec::from(['D', 'R', 'W', 'T', 'N', 'Q', 'Z']);
    let mut cargo_5 = Vec::from(['F', 'B', 'H', 'G', 'L', 'V', 'T', 'J']);
    let mut cargo_6 = Vec::from(['L', 'P', 'T', 'C', 'V', 'B', 'S', 'Z']);
    let mut cargo_7 = Vec::from(['Z', 'B', 'R', 'T', 'W', 'G', 'P', 'G']);
    let mut cargo_8 = Vec::from(['N', 'G', 'M', 'T', 'C', 'J', 'R']);
    let mut cargo_9 = Vec::from(['L', 'G', 'B', 'W']);

    let mut platform = vec![cargo_1, cargo_2, cargo_3, cargo_4, cargo_5, cargo_6, cargo_7, cargo_8, cargo_9];

    if let Ok(lines) = read_lines("./instructions") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(inp) = line {
                println!("{}", inp);
                let parsed_line: ParsedLine = parse_line(inp.parse().unwrap());
                for i in 1..=parsed_line.count {
                    let item = platform[parsed_line.from as usize].pop();
                    platform[parsed_line.destination as usize].push(item.unwrap());
                }
            }
        }
    }
    println!("{:?}", platform);
}

fn parse_int(input: &str) -> Option<i32> {
    input.parse().ok()
}

fn parse_line(line: String) -> ParsedLine {
    /// We want to parse out the number of crates to move and where to move them, this should return
    /// a tuple of (number of crates, from, to)
    let mut numbers = Vec::new();
    for substring in line.split_whitespace() {
        if let Some(num) = parse_int(substring) {
            numbers.push(num)
        }
    }
    let parsed_line = ParsedLine{
        count: numbers[0],
        from: numbers[1],
        destination: numbers[2]
    };
    return parsed_line;
}


// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}