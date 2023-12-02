# Advent of Code 2023

## Intro
This is my attempt to learn and use rust. Projects are sturctured such that each day is a new cargo package, and source codes are stored in `day/src/bin/`. These can be run with cargo run `cargo run --bin part1`.

## day 1 - Trebuchet?!

### part 1
Given a string, find the first and last digit to form a two digit number and return the sum.
My approach is to use simply run through each array once and check if each `char` is a digit. If so, record and move on. At the end of parsing, construct the 2 digit number and push onto a vector.

### part 2
Similar to part 1 but now strings can be considered numbers too, such that "one" == 1. 
This problem took considerably longer. I am using a regex approach to find all such texts and numbers, and record their digits. However, regex only finds "one" given the string "oneight", and will fail this exercise. So, I had to reerse each string, and the regex to find the patterns again in the reverse order and combine the results. 

## day 2 - Cube Conundrum
### part 1
Given a string, parse the string into a game where the game has the max number of red, green, and blue cubes appeared in the string.
I wrote a parser with regex again to parse each string into a `Game` struct. This `Game` struct also has a `contains` trait function that returns true iff all types of its cubes >= another Game. 

### part 2
Same as part1, except easier. I just need the sum of all "power" of the `Game`s from the input. "power" is defined by `Game.red` * `Game.blue` * `Game.green`
