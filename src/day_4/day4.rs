use itertools::Itertools;
use regex::Regex;
use std::collections::HashSet;

const INPUT: &str = include_str!("./input.txt");

fn read_input(input: &str) -> Vec<Vec<String>> {
    let mut passphrases: Vec<Vec<String>> = Vec::new();
    let re = Regex::new(r"(\w+)").unwrap();
    for v in input.split('\n') {
        let row = re
            .captures_iter(v)
            .map(|c| c[0].to_owned())
            .collect::<Vec<String>>();
        passphrases.push(row)
    }
    passphrases
}

pub fn run() {
    println!("=== Day 2 ===");
    part1(INPUT);
    part2(INPUT);
    println!("=============");
    println!();
}

fn part1(input: &str) -> u32 {
    let passphrases = read_input(input);
    let mut valids = 0u32;
    for passphrase in passphrases.iter() {
        if passphrase.is_empty() {
            continue;
        }
        let passphrase_unique: HashSet<String> = HashSet::from_iter(passphrase.iter().cloned());
        if passphrase.len() == passphrase_unique.len() {
            valids += 1;
        }
    }
    println!("Day 4 part 1, solution: {}", valids);
    valids
}

fn part2(input: &str) -> u32 {
    let passphrases = read_input(input);
    let mut valids = 0u32;
    for passphrase in passphrases.iter() {
        if passphrase.is_empty() {
            continue;
        }
        let passphrase_sorted: Vec<String> = passphrase
            .iter()
            .map(|s| s.chars().sorted().collect::<String>())
            .collect();
        let passphrase_unique = passphrase_sorted.iter().unique();
        if passphrase_unique.count() == passphrase.len() {
            valids += 1;
        }
    }
    println!("Day 4 part 1, solution: {}", valids);
    valids
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works_with_test_input() {
        assert_eq!(part1("aa bb cc dd ee\n"), 1);
        assert_eq!(part1("aa bb cc dd aa\n"), 0);
        assert_eq!(part1("aa bb cc dd aaa\n"), 1);
    }

    #[test]
    fn part1_works_with_puzzle_input() {
        assert_eq!(part1(INPUT), 451);
    }

    #[test]
    fn part2_works_with_test_input() {
        assert_eq!(part2("abcde fghij"), 1);
        assert_eq!(part2("abcde xyz ecdab"), 0);
        assert_eq!(part2("a ab abc abd abf abj"), 1);
        assert_eq!(part2("iiii oiii ooii oooi oooo"), 1);
        assert_eq!(part2("oiii ioii iioi iiio"), 0);
    }

    #[test]
    fn part2_works_with_puzzle_input() {
        assert_eq!(part2(INPUT), 223);
    }
}
