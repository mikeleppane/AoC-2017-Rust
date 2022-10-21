use itertools::Itertools;
use regex::Regex;

#[allow(dead_code)]
const TEST_INPUT: &str = include_str!("./test_input.txt");
#[allow(dead_code)]
const TEST_INPUT_2: &str = include_str!("./test_input_2.txt");
const INPUT: &str = include_str!("./input.txt");

fn read_input(input: &str) -> Vec<Vec<u32>> {
    let mut sheet: Vec<Vec<u32>> = Vec::new();
    let re = Regex::new(r"(\d+)").unwrap();
    for v in input.split('\n') {
        let row = re
            .captures_iter(v)
            .map(|c| c[0].parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        sheet.push(row)
    }
    sheet
}

pub fn run() {
    println!("=== Day 2 ===");
    part1(INPUT);
    part2(INPUT);
    println!("=============");
    println!();
}

fn part1(input: &str) -> u32 {
    let sheet = read_input(input);
    let mut diffs = 0u32;
    for row in sheet.iter() {
        if !row.is_empty() {
            diffs += row.iter().max().unwrap() - row.iter().min().unwrap();
        }
    }
    println!("Day 2 part 1, solution: {}", diffs);
    diffs
}

fn part2(input: &str) -> u32 {
    let sheet = read_input(input);
    let mut divs = 0u32;
    for row in sheet.iter() {
        'outer: for (index, first) in row.iter().sorted().enumerate() {
            for second in row.iter().sorted().collect::<Vec<_>>()[(index + 1)..].iter() {
                if *second % first == 0 {
                    divs += *second / first;
                    break 'outer;
                }
            }
        }
    }
    println!("Day 2 part 2, solution: {}", divs);
    divs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works_with_test_input() {
        assert_eq!(part1(TEST_INPUT), 18);
    }

    #[test]
    fn part1_works_with_puzzle_input() {
        assert_eq!(part1(INPUT), 48357);
    }

    #[test]
    fn part2_works_with_test_input() {
        assert_eq!(part2(TEST_INPUT_2), 9);
    }

    #[test]
    fn part2_works_with_puzzle_input() {
        assert_eq!(part2(INPUT), 351);
    }
}
