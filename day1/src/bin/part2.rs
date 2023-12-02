use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

fn main() {
    let ans = solve_problem("input2.txt").unwrap();
    let mut sum: i32 = 0;
    for i in ans {
        println!("{}", i);
        sum += i;
    }
    println!("Sum: {}", sum);
}

fn solve_problem<P>(filename: P)-> io::Result<Vec<i32>>
where P: AsRef<Path>, {
    let lines: Vec<String> = read_input(filename)?;
    let mut numbers: Vec<i32> = Vec::new();
    for line in lines {
        let result = process_line(&line);
        numbers.push(result);
    }
    Ok(numbers)
}

fn reverse_string(s: &str) -> String {
    s.chars().rev().collect::<String>()
}

fn process_line(line: &str) -> i32 {
    let forward_pattern = Regex::new(r"one|two|three|four|five|six|seven|eight|nine|[0-9]").unwrap();
    let first_match = forward_pattern.find(line).map_or("", |m| m.as_str());
    let reverse_pattern = Regex::new(r"eno|owt|eerht|ruof|evif|xis|neves|thgie|enin|[0-9]").unwrap();
    let reversed_line = reverse_string(line);
    let last_match = reverse_pattern.find(&reversed_line).map_or("", |m| m.as_str());
    return string_to_number(first_match) * 10 + string_to_number(last_match);
}

fn string_to_number(s: &str) -> i32 {
    match s {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        "eno" => 1,
        "owt" => 2,
        "eerht" => 3,
        "ruof" => 4,
        "evif" => 5,
        "xis" => 6,
        "neves" => 7,
        "thgie" => 8,
        "enin" => 9,
        "1" => 1,
        "2" => 2,
        "3" => 3,
        "4" => 4,
        "5" => 5,
        "6" => 6,
        "7" => 7,
        "8" => 8,
        "9" => 9,
        _ => 0,
    }
}

fn read_input<P>(filename: P) -> io::Result<Vec<String>>
where P: AsRef<Path>, {
    let file = File::open(&filename)?;
    let reader = io::BufReader::new(file);
    let lines = reader.lines().collect::<Result<_, _>>()?;
    Ok(lines)
}

/////////////////////////////   TESTS   /////////////////////////////

#[cfg(test)]
mod tests {
    use crate::read_input;
    use crate::solve_problem;
    use crate::string_to_number;
    use crate::process_line;

    #[test]
    fn test_process_line(){
        assert_eq!(process_line("two1nine"), 29);
        assert_eq!(process_line("eightwothree"), 83);
        assert_eq!(process_line("abcone2threexyz"), 13);
        assert_eq!(process_line("xtwone3four"), 24);
        assert_eq!(process_line("tfphzkcxh3twofour9oneightg"), 38);
        assert_eq!(process_line("85ntwonexlm"), 81);
    }

    #[test]
    fn validate_input_read() {
        let lines: Vec<String> = read_input("testinput2.txt").unwrap();
        assert_eq!(lines.len(), 7);
        assert_eq!(lines[0], "two1nine");
        assert_eq!(lines[1], "eightwothree");
        assert_eq!(lines[2], "abcone2threexyz");
        assert_eq!(lines[3], "xtwone3four");
        assert_eq!(lines[4], "4nineeightseven2");
        assert_eq!(lines[5], "zoneight234");
        assert_eq!(lines[6], "7pqrstsixteen");
    }

    #[test]
    fn validate_test_case(){
        let ans = solve_problem("testinput2.txt").unwrap();
        assert_eq!(ans.len(), 7);
        assert_eq!(ans[0], 29);
        assert_eq!(ans[1], 83);
        assert_eq!(ans[2], 13);
        assert_eq!(ans[3], 24);
        assert_eq!(ans[4], 42);
        assert_eq!(ans[5], 14);
        assert_eq!(ans[6], 76);
    }

    #[test]
    fn test_string_to_number(){
        assert_eq!(string_to_number("one"), 1);
        assert_eq!(string_to_number("two"), 2);
        assert_eq!(string_to_number("three"), 3);
        assert_eq!(string_to_number("four"), 4);
        assert_eq!(string_to_number("five"), 5);
        assert_eq!(string_to_number("six"), 6);
        assert_eq!(string_to_number("seven"), 7);
        assert_eq!(string_to_number("eight"), 8);
        assert_eq!(string_to_number("nine"), 9);
        assert_eq!(string_to_number("1"), 1);
        assert_eq!(string_to_number("2"), 2);
        assert_eq!(string_to_number("3"), 3);
        assert_eq!(string_to_number("4"), 4);
        assert_eq!(string_to_number("5"), 5);
        assert_eq!(string_to_number("6"), 6);
        assert_eq!(string_to_number("7"), 7);
        assert_eq!(string_to_number("8"), 8);
        assert_eq!(string_to_number("9"), 9);
        assert_eq!(string_to_number("0"), 0);
        assert_eq!(string_to_number("ten"), 0);
    }
}

