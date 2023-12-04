use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

pub struct Number {
    value: u32,
    row_num: i32,
    col_start: i32,
    col_end: i32,
}

pub struct Symbol {
    row_num: i32,
    col_num: i32,
}
pub trait Touches {
    fn touches(&self, sym: &Symbol) -> bool;
}

impl Touches for Number {
    fn touches(&self, sym: &Symbol) -> bool {
        if self.row_num == sym.row_num
            || self.row_num + 1 == sym.row_num
            || sym.row_num + 1 == self.row_num
        {
            if self.col_start - 1 <= sym.col_num  && self.col_end + 1 >= sym.col_num {
                return true;
            }
            return false;
        }
        return false;
    }
}

fn main() {
    let lines = read_input("input1.txt").unwrap();
    let mut numbers: Vec<Number> = Vec::new();
    let mut symbols: Vec<Symbol> = Vec::new();
    let mut row = 0;

    for line in lines {
        process_line(line.as_str(), row, &mut numbers, &mut symbols);
        row += 1;
    }

    let mut sum = 0;

    for num in numbers {
        for sym in &symbols {
            if num.touches(sym) {
                println!("Number: {} {} {} {}", num.value, num.row_num, num.col_start, num.col_end);
                println!("Symbol: {} {}", sym.row_num, sym.col_num);
                sum += num.value;
            }
        }
    }

    println!("Sum: {}", sum);
}

fn process_line(line: &str, row: i32, numbers: &mut Vec<Number>, symbols: &mut Vec<Symbol>){
    let number_regex = Regex::new(r"\d+").unwrap();
    let symbol_regex = Regex::new(r"[^\w.]").unwrap();

    for number_match in number_regex.find_iter(line) {
        let number: u32 = number_match.as_str().parse::<u32>().unwrap();
        numbers.push(Number {value: number , row_num: row, col_start: number_match.start() as i32, col_end: number_match.end() as i32 - 1});
    }

    for symbol_match in symbol_regex.find_iter(line) {
        symbols.push(Symbol { row_num: row, col_num: symbol_match.start() as i32});
    }
} 

fn read_input<P>(filename: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = File::open(&filename)?;
    let reader = io::BufReader::new(file);
    let lines = reader.lines().collect::<Result<_, _>>()?;
    Ok(lines)
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn validate_input_read() {
        let lines: Vec<String> = read_input("testinput1.txt").unwrap();
        assert_eq!(lines.len(), 10);
        assert_eq!(lines[0], "467..114..");
        assert_eq!(lines[1], "...*......");
        assert_eq!(lines[2], "..35..633.");
        assert_eq!(lines[3], "......#...");
        assert_eq!(lines[4], "617*......");
        assert_eq!(lines[5], ".....+.58.");
        assert_eq!(lines[6], "..592.....");
        assert_eq!(lines[7], "......755.");
        assert_eq!(lines[8], "...$.*....");
        assert_eq!(lines[9], ".664.598..");
    }

    #[test]
    fn test_touches_true_same_row() {
        let number1 = Number {
            value: 123,
            row_num: 1,
            col_start: 1,
            col_end: 3,
        };
        let symbol1 = Symbol {
            row_num: 1,
            col_num: 2,
        };
        assert!(number1.touches(&symbol1));

        let symbol2 = Symbol {
            row_num: 1,
            col_num: 0,
        };
        assert!(number1.touches(&symbol2));

        let symbol3 = Symbol {
            row_num: 1,
            col_num: 4,
        };
        assert!(number1.touches(&symbol3));

        let number2 = Number {
            value: 123,
            row_num: 0,
            col_start: 0,
            col_end: 3,
        };

        let symbol4 = Symbol {
            row_num: 0,
            col_num: 4,
        };

        assert!(number2.touches(&symbol4));
    }

    #[test]
    fn test_touches_true_above() {
        let number1 = Number {
            value: 123,
            row_num: 1,
            col_start: 1,
            col_end: 3,
        };
        let symbol1 = Symbol {
            row_num: 0,
            col_num: 2,
        };
        assert!(number1.touches(&symbol1));

        let symbol2 = Symbol {
            row_num: 0,
            col_num: 0,
        };
        assert!(number1.touches(&symbol2));

        let symbol3 = Symbol {
            row_num: 0,
            col_num: 4,
        };
        assert!(number1.touches(&symbol3));
    }

    #[test]
    fn test_touches_true_below() {
        let number1 = Number {
            value: 123,
            row_num: 1,
            col_start: 1,
            col_end: 3,
        };
        let symbol1 = Symbol {
            row_num: 2,
            col_num: 2,
        };
        assert!(number1.touches(&symbol1));

        let symbol2 = Symbol {
            row_num: 2,
            col_num: 0,
        };
        assert!(number1.touches(&symbol2));

        let symbol3 = Symbol {
            row_num: 2,
            col_num: 4,
        };
        assert!(number1.touches(&symbol3));
    }

    #[test]
    fn test_touches_false_far_away() {
        let number1 = Number {
            value: 123,
            row_num: 1,
            col_start: 1,
            col_end: 3,
        };
        let symbol1 = Symbol {
            row_num: 3,
            col_num: 4,
        };
        assert!(!number1.touches(&symbol1));
    }

    #[test]
    fn test_touches_false_same_row() {
        let number1 = Number {
            value: 123,
            row_num: 1,
            col_start: 2,
            col_end: 3,
        };
        let symbol1 = Symbol {
            row_num: 1,
            col_num: 5,
        };
        assert!(!number1.touches(&symbol1));

        let symbol2 = Symbol {
            row_num: 1,
            col_num: 0,
        };
        assert!(!number1.touches(&symbol2));
    }

    #[test]
    fn test_touches_false_above() {
        let number1 = Number {
            value: 123,
            row_num: 1,
            col_start: 2,
            col_end: 3,
        };
        let symbol1 = Symbol {
            row_num: 0,
            col_num: 5,
        };
        assert!(!number1.touches(&symbol1));

        let symbol2 = Symbol {
            row_num: 0,
            col_num: 0,
        };
        assert!(!number1.touches(&symbol2));
    }

    #[test]
    fn test_touches_false_below() {
        let number1 = Number {
            value: 123,
            row_num: 1,
            col_start: 2,
            col_end: 3,
        };
        let symbol1 = Symbol {
            row_num: 2,
            col_num: 5,
        };
        assert!(!number1.touches(&symbol1));

        let symbol2 = Symbol {
            row_num: 2,
            col_num: 0,
        };
        assert!(!number1.touches(&symbol2));
    }

    #[test]
    fn test_process_line_numbers() {
        let mut numbers = Vec::new();
        let mut symbols = Vec::new();
        process_line("467..114..", 0, &mut numbers, &mut symbols);
        assert_eq!(numbers[0].row_num, 0);
        assert_eq!(numbers[0].col_start, 0);
        assert_eq!(numbers[0].col_end, 2);
        assert_eq!(numbers[1].row_num, 0);
        assert_eq!(numbers[1].col_start, 5);
        assert_eq!(numbers[1].col_end, 7);
    }

    #[test]
    fn test_process_line_symbols() {
        let mut numbers = Vec::new();
        let mut symbols = Vec::new();
        process_line("...*......", 1, &mut numbers, &mut symbols);
        assert_eq!(symbols[0].row_num, 1);
        assert_eq!(symbols[0].col_num, 3);
    }

    #[test]
    fn test_process_line_symbols_and_numbers(){
        let mut numbers = Vec::new();
        let mut symbols = Vec::new();
        process_line("467.*.11#.", 0, &mut numbers, &mut symbols);
        assert_eq!(numbers[0].row_num, 0);
        assert_eq!(numbers[0].col_start, 0);
        assert_eq!(numbers[0].col_end, 2);
        assert_eq!(numbers[1].row_num, 0);
        assert_eq!(numbers[1].col_start, 6);
        assert_eq!(numbers[1].col_end, 7);

        assert_eq!(symbols[0].row_num, 0);
        assert_eq!(symbols[0].col_num, 4);
        assert_eq!(symbols[1].row_num, 0);
        assert_eq!(symbols[1].col_num, 8);
    }
}
