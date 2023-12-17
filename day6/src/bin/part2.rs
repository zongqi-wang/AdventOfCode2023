use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


pub struct Record {
    time: u64,
    distance: u64
}

fn main() {
    let records = parse_time_distance("testinput1.txt").unwrap();
    let ways_to_win = calc_ways_to_win(records);
    println!("Ways to win: {}", ways_to_win);
    
    let records = parse_time_distance("input2.txt").unwrap();
    let ways_to_win = calc_ways_to_win(records);
    println!("Ways to win: {}", ways_to_win);
}

fn calc_ways_to_win(record: Record) -> u64 {
    let mut low = 0;
    let mut high = record.time / 2;
    while low < high {
        let mid = low + (high - low) / 2;
        if mid * (record.time - mid) > record.distance {
            high = mid;
        } else {
            low = mid + 1;
        }
    }
    let num_ways = record.time/2 - low + 1;
    if record.time % 2 != 0 {
        return num_ways * 2
    } else {
        return num_ways * 2 - 1
    }
}

fn parse_time_distance<P: AsRef<Path>>(path: P) -> io::Result<Record> {
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut lines = reader.lines();
    let times_line = lines.next().ok_or(io::Error::new(io::ErrorKind::InvalidData, "Missing times line"))??;
    let distances_line = lines.next().ok_or(io::Error::new(io::ErrorKind::InvalidData, "Missing distances line"))??;

    let time: u64 = times_line.trim_start_matches("Time:").split_whitespace().collect::<String>().parse().unwrap();
    let distance: u64 = distances_line.trim_start_matches("Distance:").split_whitespace().collect::<String>().parse().unwrap();

    Ok(Record { time, distance })
}


#[cfg(test)]
mod test{
    use crate::*;

    #[test]
    fn test_calc_ways_to_win(){
        assert_eq!(calc_ways_to_win(Record{ time: 7, distance: 9 }), 4);
        assert_eq!(calc_ways_to_win(Record{ time: 15, distance: 40 }),8);
        assert_eq!(calc_ways_to_win(Record{ time: 30, distance: 200 }),9);
        assert_eq!(calc_ways_to_win(Record{ time: 71530, distance: 940200 }), 71503);
    }

    #[test]
    fn test_parse_time_distance() {

        let record = parse_time_distance("testinput1.txt").unwrap();
        assert_eq!(record.time, 71530);
        assert_eq!(record.distance, 940200);

        let record = parse_time_distance("input1.txt").unwrap();
        assert_eq!(record.time, 41968894);
        assert_eq!(record.distance, 214178911271055);

    }

}

