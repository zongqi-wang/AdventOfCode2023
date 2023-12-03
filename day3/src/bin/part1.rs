use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let lines = read_input("testinput1.txt").unwrap();
    for line in lines{
        println!("{}", line);
    }
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
}