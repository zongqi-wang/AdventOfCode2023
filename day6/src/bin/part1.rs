use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


pub struct Record {
    time: u32,
    distance: u32
}

fn main() {
    let records = parse_time_distance("input1.txt").unwrap();

    let mut num_ways = 0;
    for record in records {
        println!("Time: {}, Distance: {}", record.time, record.distance);
        let ways_to_win = calc_ways_to_win(record);
        println!("Ways to win: {}", ways_to_win);
        if num_ways == 0 {
            num_ways = ways_to_win;
        } else {
            num_ways *= ways_to_win;
        }

        println!("Total Ways: {}", num_ways);
    }
}

fn calc_ways_to_win(record: Record) -> u32 {
    let mut num_ways = 0;
    let mut time = (record.time)/ 2;
    while time * (record.time - time) > record.distance {
        num_ways += 1;
        time -= 1;
    }
    if record.time % 2 != 0 {
        return num_ways * 2
    } else {
        return num_ways * 2 - 1
    }
}

fn parse_time_distance<P: AsRef<Path>>(path: P) -> io::Result<Vec<Record>> {
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut lines = reader.lines();
    let times_line = lines.next().ok_or(io::Error::new(io::ErrorKind::InvalidData, "Missing times line"))??;
    let distances_line = lines.next().ok_or(io::Error::new(io::ErrorKind::InvalidData, "Missing distances line"))??;

    let times: Vec<_> = times_line.trim_start_matches("Time:").split_whitespace().map(|s| s.parse().unwrap()).collect();
    let distances: Vec<_> = distances_line.trim_start_matches("Distance:").split_whitespace().map(|s| s.parse().unwrap()).collect();

    if times.len() != distances.len() {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Mismatched times and distances"));
    }

    let pairs = times.into_iter().zip(distances.into_iter()).map(|(time, distance)| Record { time, distance }).collect();
    Ok(pairs)
}


#[cfg(test)]
mod test{
    use crate::*;

    #[test]
    fn test_calc_ways_to_win(){
        assert_eq!(calc_ways_to_win(Record{ time: 7, distance: 9 }), 4);
        assert_eq!(calc_ways_to_win(Record{ time: 15, distance: 40 }),8);
        assert_eq!(calc_ways_to_win(Record{ time: 30, distance: 200 }),9);
    }

}