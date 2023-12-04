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
    symbol: char,
    row_num: i32,
    col_num: i32,
    numbers_touching: Vec<u32>
}

pub trait Touches {
    fn touches(&self, sym: &mut Symbol) -> bool;
}

impl Touches for Number {
    fn touches(&self, sym: &mut Symbol) -> bool {
        if self.row_num == sym.row_num
            || self.row_num + 1 == sym.row_num
            || sym.row_num + 1 == self.row_num
        {
            if self.col_start - 1 <= sym.col_num  && self.col_end + 1 >= sym.col_num {
                sym.numbers_touching.push(self.value);
                return true;
            }
            return false;
        }
        return false;
    }
}

fn main() {
    let lines = read_input("input2.txt").unwrap();
    let mut numbers: Vec<Number> = Vec::new();
    let mut symbols: Vec<Symbol> = Vec::new();
    let mut row = 0;

    for line in lines {
        process_line(line.as_str(), row, &mut numbers, &mut symbols);
        row += 1;
    }

    let mut sum = 0;

    for num in numbers {
        for sym in &mut symbols {
            num.touches(sym); 
        }
    }

    for sym in symbols {
        if sym.symbol == '*' && sym.numbers_touching.len() == 2{
            let product = sym.numbers_touching[0] * sym.numbers_touching[1];   
            println!("Symbol: row {} col {} first number {} second number {} product {}", sym.row_num, sym.col_num, sym.numbers_touching[0], sym.numbers_touching[1], product);
            sum += product;
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
        symbols.push(Symbol { symbol: symbol_match.as_str().chars().next().unwrap(), row_num: row, col_num: symbol_match.start() as i32, numbers_touching: Vec::new()});
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
        assert_eq!(symbols[0].symbol, '*');
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

    #[test]
    fn test_process_line_numbers_rouching(){
        let number1 = Number {
            value: 123,
            row_num: 1,
            col_start: 1,
            col_end: 3,
        };
        let number2 = Number {
            value: 234,
            row_num: 2,
            col_start: 1,
            col_end: 3,
        };
        let number3 = Number {
            value: 345,
            row_num: 3,
            col_start: 1,
            col_end: 3,
        };

        let number4 = Number {
            value: 456,
            row_num: 4,
            col_start: 1,
            col_end: 3,
        };
        let number5 = Number {
            value: 567,
            row_num: 4,
            col_start: 1,
            col_end: 3,
        };
        let mut symbol1 = Symbol {
            symbol: '*',
            row_num: 3,
            col_num: 4,
            numbers_touching: Vec::new()
        };

        assert!(!number1.touches(&mut symbol1));
        assert!(number2.touches(&mut symbol1));
        assert!(number3.touches(&mut symbol1));
        assert!(number4.touches(&mut symbol1));
        assert!(number5.touches(&mut symbol1));

        assert_eq!(symbol1.numbers_touching.len(), 4);
        assert_eq!(symbol1.numbers_touching[0], 234);
        assert_eq!(symbol1.numbers_touching[1], 345);
        assert_eq!(symbol1.numbers_touching[2], 456);
        assert_eq!(symbol1.numbers_touching[3], 567);

    }
}
