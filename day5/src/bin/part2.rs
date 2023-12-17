use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct Pairings{
    destination_range_start: u64,
    source_range_start: u64,
    range_length: u64,
}

fn main() -> io::Result<()> {
    let file = File::open(Path::new("input1.txt"))?;
    let mut reader = io::BufReader::new(file);
    let initial_seeds = read_initial_seeds(&mut reader)?;

    let mut translated_seeds = initial_seeds.clone();
    for i in 0..7 {
        println!("iteration: {}", i);
        let pairings = read_map_section(&mut reader)?;
        println!("pairings: {:?}", pairings);
        translated_seeds = translate(&mut translated_seeds, &pairings);
        println!("translated_seeds: {:?}", translated_seeds);
    }
    println!("\n\nfinal seeds: {:?}", translated_seeds.iter().min().unwrap());
    Ok(())
}

fn read_initial_seeds(reader: &mut io::BufReader<File>) -> io::Result<Vec<(u64, u64)>> {
    let mut line = String::new();
    reader.read_line(&mut line)?;

    let seeds: Vec<u64> = line.trim()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();

    let mut initial_seeds:Vec<(u64, u64)> = Vec::new();
    for i in 0..seeds.len()/2 {
        initial_seeds.push((seeds[i*2], seeds[i*2+1]));
    }

    println!("initial_seeds: {:?}", initial_seeds);
    reader.read_line(&mut line)?;
    Ok(initial_seeds)
}

fn read_map_section(reader: &mut io::BufReader<File>) -> io::Result<Vec<Pairings>> {
    let mut pairings : Vec<Pairings> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            break;
        }
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() != 3 {
            continue;
        }
        let destination_range_start = parts[0].parse::<u64>();
        let source_range_start = parts[1].parse::<u64>();
        let range_length = parts[2].parse::<u64>();

        if destination_range_start.is_err() || source_range_start.is_err() || range_length.is_err() {
            continue;
        }

        let parts: Vec<u64> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
        let pairing = Pairings {
            destination_range_start: parts[0],
            source_range_start: parts[1],
            range_length: parts[2]
        };
        pairings.push(pairing);

    }

    Ok(pairings)
}

fn translate(initial_seeds: &mut Vec<(u64, u64)>, mapping: &Vec<Pairings>) -> Vec<(u64, u64)> {
    let mut translated_seeds:Vec<(u64, u64)> = Vec::new();
    let mut found: bool;
    for (seed_start, seed_range) in &mut *initial_seeds {
        found = false;
        for pair in mapping {

            if *seed_start >= pair.source_range_start && *seed_start < (pair.source_range_start + pair.range_length) {
                if *seed_start + *seed_range > pair.source_range_start + pair.range_length {
                    let translated_seed = *seed_start - pair.source_range_start + pair.destination_range_start;
                    let translated_seed_range = *seed_range - (pair.source_range_start + pair.range_length - *seed_start);
                    translated_seeds.push((translated_seed, translated_seed_range));
                    let reminder = *seed_range - translated_seed_range;
                    let new_seed_start = *seed_start + reminder;
                    initial_seeds.push((new_seed_start, reminder));
                    found = true;
                    break;
                } else {
                    let translated_seed = *seed_start - pair.source_range_start + pair.destination_range_start;
                    translated_seeds.push((translated_seed, *seed_range));
                    found = true;
                    break;
                }
            }
        }
        if !found {
            translated_seeds.push((*seed_start, *seed_range));
        }
    }

    translated_seeds
}
#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_read_initial_seeds() {
        let file = File::open(Path::new("testinput1.txt")).unwrap();
        let mut reader = io::BufReader::new(file);
        let seeds = read_initial_seeds(&mut reader).unwrap();
        assert_eq!(seeds, vec![(79, 14), (55, 13)]);
    }

    #[test]
    fn test_translate() {
        let initial_seeds:Vec<(u64, u64)> = vec![(79, 14), (55, 13)];
        let mapping = vec![
            Pairings {
                destination_range_start: 50,
                source_range_start: 98,
                range_length: 2,
            },
            Pairings {
                destination_range_start: 52,
                source_range_start: 50,
                range_length: 48,
            },
        ];

        let translated_seeds = translate(&mut initial_seeds, &mapping);

        assert_eq!(translated_seeds, vec![(81,14),(57,13)]);
    }
}
