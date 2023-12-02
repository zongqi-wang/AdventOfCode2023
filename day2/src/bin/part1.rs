use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

pub struct Game {
    id: u32,
    red: u32,
    green: u32,
    blue: u32,
}

pub trait Contains {
    fn contains(&self, other: &Game) -> bool;
}

impl Contains for Game{
    fn contains(&self, other: &Game) -> bool{
        self.red >= other.red && self.green >= other.green && self.blue >= other.blue
    }
}

fn main() {
    let lines = read_input("input1.txt").unwrap();
    let final_game = Game{ id: 0, red: 12, green: 13, blue: 14 };
    let mut id_sum: u32 = 0;
    for line in lines {
        let game = process_line(&line);
        if final_game.contains(&game){
            id_sum+=game.id;
            println!("{}: {} red, {} green, {} blue", game.id, game.red, game.green, game.blue);
        }
    }
    println!("Sum: {}", id_sum);
}

fn process_line(line: &str) -> Game {
    let re = Regex::new(r"(\d+) (blue|red|green)").unwrap();
    let id_re = Regex::new(r"Game (\d+)").unwrap();
    let mut game = Game { id: 0, blue: 0, red: 0, green: 0 };

    if let Some(cap) = id_re.captures(line) {
        game.id = cap[1].parse().unwrap();
    }

    for part in line.split(';') {
        for cap in re.captures_iter(part) {
            let count: u32 = cap[1].parse().unwrap();
            match &cap[2] {
                "blue" => game.blue = game.blue.max(count),
                "red" => game.red = game.red.max(count),
                "green" => game.green = game.green.max(count),
                _ => {}
            }
        }
    }

    game
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
        assert_eq!(lines.len(), 4);
        assert_eq!(lines[0], "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
        assert_eq!(lines[1], "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue");
        assert_eq!(lines[2], "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red");
        assert_eq!(lines[3], "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red");
        assert_eq!(lines[4], "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");
    }

    
    #[test]
    fn test_parse_game() {
        let line = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";
        let game = process_line(line);
        assert_eq!(game.id, 3);
        assert_eq!(game.blue, 6);
        assert_eq!(game.red, 20);
        assert_eq!(game.green, 13);
    }

    #[test]
    fn test_read_and_parse(){
        let lines: Vec<String> = read_input("testinput1.txt").unwrap();
        let game = process_line(&lines[0]);
        assert_eq!(game.id, 1);
        assert_eq!(game.blue, 6);
        assert_eq!(game.red, 4);
        assert_eq!(game.green, 2);

        let game = process_line(&lines[1]);
        assert_eq!(game.id, 2);
        assert_eq!(game.blue, 4);
        assert_eq!(game.red, 1);
        assert_eq!(game.green, 3);

        let game = process_line(&lines[2]);
        assert_eq!(game.id, 3);
        assert_eq!(game.blue, 6);
        assert_eq!(game.red, 20);
        assert_eq!(game.green, 13);

        let game = process_line(&lines[3]);
        assert_eq!(game.id, 4);
        assert_eq!(game.blue, 15);
        assert_eq!(game.red, 14);
        assert_eq!(game.green, 3);

        let game = process_line(&lines[4]);
        assert_eq!(game.id, 5);
        assert_eq!(game.blue, 2);
        assert_eq!(game.red, 6);
        assert_eq!(game.green, 3);
    }

    #[test]
    fn test_contains_true() {
        let game1 = Game { id: 1, red: 4, green: 2, blue: 6 };
        let game2 = Game { id: 2, red: 3, green: 2, blue: 5 };
        assert!(game1.contains(&game2));
    }

    #[test]
    fn test_contains_false() {
        let game1 = Game { id: 1, red: 4, green: 4, blue: 7 };
        let game2 = Game { id: 2, red: 5, green: 3, blue: 7 };
        assert!(!game1.contains(&game2));
    }

    #[test]
    fn test_contains_equal() {
        let game1 = Game { id: 1, red: 4, green: 2, blue: 6 };
        let game2 = Game { id: 2, red: 4, green: 2, blue: 6 };
        assert!(game1.contains(&game2));
    }

}