use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec;
use regex::Regex;

struct Card{
    winning_matches: u32,
    copies: u32
}

fn main() {
    let lines = read_input("input2.txt").unwrap();
    let mut sum: u32 = 0;

    let mut card_list:vec::Vec<Card> = vec::Vec::new();

    for line in lines {
        // 1. process line
        let card = process_line(&line);
        card_list.push(card);
    }
    // 2. iterate over the cards and clone them
    for i in 0..card_list.len() {
        for j in 1..(card_list[i].winning_matches+1) as usize {
            if i+j >= card_list.len() {
                break;
            }
            card_list[i+j].copies += card_list[i].copies;
        }
    }

    // 3. iterate over the cards and sum the copies
    for card in card_list {
        sum += card.copies;
    }
    println!("Sum: {}", sum);
}


fn process_line(line: &str) -> Card{
    let re = Regex::new(r"Card\s+(\d+):\s+((?:\d+\s*)+)\|\s+((?:\d+\s*)+)").unwrap();
    let caps = re.captures(line).unwrap();

    let game_id: u32 = caps[1].parse().unwrap();
    let winning_numbers: Vec<u32> = caps[2].split_whitespace().map(|s| s.parse().unwrap()).collect();
    let player_numbers: Vec<u32> = caps[3].split_whitespace().map(|s| s.parse().unwrap()).collect();

    let mut matches: u32 = 0;
    for number in &player_numbers{
        if winning_numbers.contains(number) {
            matches += 1;
        }
    }
    Card {
        winning_matches: matches,
        copies: 1
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
        let lines: Vec<String> = read_input("testinput2.txt").unwrap();
        assert_eq!(lines.len(), 5);
        assert_eq!(lines[0], "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");
        assert_eq!(lines[1], "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19");
        assert_eq!(lines[2], "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1");
        assert_eq!(lines[3], "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83");
        assert_eq!(lines[4], "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36");
    }

    #[test]
    fn validate_real_input_read() {
        let lines: Vec<String> = read_input("input1.txt").unwrap();
        assert_eq!(lines.len(), 206);
        assert_eq!(lines[0], "Card   1: 58 68  1 21 88 37 66 61 23 25 | 63 95 45 43 79 64 29 87  8 70 84 34 91 67  3 76 27 24 28 62 13 54 19 93  7");
    }

    #[test]
    fn test_process_line() {
        let line = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let game = process_line(line);
        assert_eq!(game.game_id, 1);
        assert_eq!(game.winning_numbers, vec![41, 48, 83, 86, 17]);
        assert_eq!(game.player_numbers, vec![83, 86, 6, 31, 17, 9, 48, 53]);

        let line = "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19";
        let game = process_line(line);
        assert_eq!(game.game_id, 2);
        assert_eq!(game.winning_numbers, vec![13, 32, 20, 16, 61]);
        assert_eq!(game.player_numbers, vec![61, 30, 68, 82, 17, 32, 24, 19]);

        let line = "Card  15: 35 40 13 30 75 31 21 58 66 95 | 41 89 99 21 27 18  1 70 10 12 95 23 61 35 40 98 24 42 64 81 78 11 26 28 96";
        let game = process_line(line);
        assert_eq!(game.game_id, 15);
        assert_eq!(game.winning_numbers, vec![35, 40, 13, 30, 75, 31, 21, 58, 66, 95]);
        assert_eq!(game.player_numbers, vec![41, 89, 99, 21, 27, 18, 1, 70, 10, 12, 95, 23, 61, 35, 40, 98, 24, 42, 64, 81, 78, 11, 26, 28, 96]);
    }

    #[test]
    fn test_clone_cards() {
        let mut card_map:HashMap<u32, u32> = HashMap::new();
        let game = Game {
            game_id: 1,
            winning_numbers: vec![41, 48, 83, 86, 17],
            player_numbers: vec![83, 86, 6, 31, 17, 9, 48, 53]
        };

        let game2 = Game {
            game_id: 2,
            winning_numbers: vec![13, 32, 20, 16, 61],
            player_numbers: vec![61, 30, 68, 82, 17, 32, 24, 19]
        };

        let game3 = Game {
            game_id: 3,
            winning_numbers: vec![1, 21, 53, 59, 44],
            player_numbers: vec![69, 82, 63, 72, 16, 21, 14, 1]
        };

        let game4 = Game {
            game_id: 4,
            winning_numbers: vec![41, 92, 73, 84, 69],
            player_numbers: vec![59, 84, 76, 51, 58, 5, 54, 83]
        };

        let game5 = Game {
            game_id: 5,
            winning_numbers: vec![87, 83, 26, 28, 32],
            player_numbers: vec![88, 30, 70, 12, 93, 22, 82, 36]
        };

        let game6 = Game {
            game_id: 6,
            winning_numbers: vec![87, 83, 26, 28, 32],
            player_numbers: vec![74, 77, 10, 23, 35, 67, 36, 11]
        };

        clone_cards(&mut card_map, &game, 4);
        assert_eq!(card_map.len(), 4);
        assert_eq!(card_map[&1], 1);
        assert_eq!(card_map[&2], 1);
        assert_eq!(card_map[&3], 1);
        assert_eq!(card_map[&4], 1);


        clone_cards(&mut card_map, &game2, 2);
        assert_eq!(card_map.len(), 4);
        assert_eq!(card_map[&1], 1);
        assert_eq!(card_map[&2], 2);
        assert_eq!(card_map[&3], 2);
        assert_eq!(card_map[&4], 2);

        clone_cards(&mut card_map, &game3, 2);
        assert_eq!(card_map.len(), 5);
        assert_eq!(card_map[&1], 2);
        assert_eq!(card_map[&2], 2);
        assert_eq!(card_map[&3], 3);
        assert_eq!(card_map[&4], 3);
        assert_eq!(card_map[&5], 1);

        clone_cards(&mut card_map, &game4, 1);

        assert_eq!(card_map.len(), 6);
        assert_eq!(card_map[&1], 2);
        assert_eq!(card_map[&2], 2);
        assert_eq!(card_map[&3], 3);
        assert_eq!(card_map[&4], 4);
        assert_eq!(card_map[&5], 2);
        assert_eq!(card_map[&6], 1);

        clone_cards(&mut card_map, &game5, 0);
        assert_eq!(card_map.len(), 7);
        assert_eq!(card_map[&1], 2);
        assert_eq!(card_map[&2], 2);
        assert_eq!(card_map[&3], 3);
        assert_eq!(card_map[&4], 4);
        assert_eq!(card_map[&5], 3);
        assert_eq!(card_map[&6], 2);
        assert_eq!(card_map[&7], 1);

        clone_cards(&mut card_map, &game6, 0);
        assert_eq!(card_map.len(), 8);
        assert_eq!(card_map[&1], 1);
        assert_eq!(card_map[&2], 2);
        assert_eq!(card_map[&3], 4);
        assert_eq!(card_map[&4], 8);
        assert_eq!(card_map[&5], 3);
        assert_eq!(card_map[&6], 3);
        assert_eq!(card_map[&7], 2);
        assert_eq!(card_map[&8], 1);

    }
}