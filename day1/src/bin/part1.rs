use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let ans = solve_problem("input1.txt").unwrap();
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
        let mut first_digit = 0;
        let mut last_digit = 0;

        for c in line.chars(){
            if c.is_digit(10) {
                if first_digit == 0 {
                    first_digit = c.to_digit(10).unwrap() as i32;
                }
                last_digit = c.to_digit(10).unwrap() as i32;
            }
        
        }
        numbers.push(first_digit*10 + last_digit);
    }
    Ok(numbers)
}

fn read_input<P>(filename: P) -> io::Result<Vec<String>>
where P: AsRef<Path>, {
    let file = File::open(&filename)?;
    let reader = io::BufReader::new(file);
    let lines = reader.lines().collect::<Result<_, _>>()?;
    Ok(lines)
}

#[cfg(test)]
mod tests {
    use crate::read_input;
    use crate::solve_problem;

    #[test]
    fn validate_input_read() {
        let lines: Vec<String> = read_input("testinput1.txt").unwrap();
        assert_eq!(lines.len(), 4);
        assert_eq!(lines[0], "1abc2");
        assert_eq!(lines[1], "pqr3stu8vwx");
        assert_eq!(lines[2], "a1b2c3d4e5f");
        assert_eq!(lines[3], "treb7uchet");
    }

    #[test]
    fn validate_test_case(){
        let ans = solve_problem("testinput1.txt").unwrap();
        assert_eq!(ans.len(), 4);
        assert_eq!(ans[0], 12);
        assert_eq!(ans[1], 38);
        assert_eq!(ans[2], 15);
        assert_eq!(ans[3], 77);
    }
}