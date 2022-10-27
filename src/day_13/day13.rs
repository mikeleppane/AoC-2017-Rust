use std::collections::HashMap;
use std::str::FromStr;

#[allow(dead_code)]
const INPUT: &str = include_str!("./input.txt");
#[allow(dead_code)]
const TEST_INPUT: &str = include_str!("./test_input.txt");

#[derive(Clone, PartialEq)]
enum Direction {
    Up,
    Down,
}

#[derive(Clone, PartialEq)]
struct Scanner {
    direction: Direction,
    position: u16,
    layers: u16,
}

impl Scanner {
    fn new(direction: Direction, position: u16, layers: u16) -> Self {
        Self {
            direction,
            position,
            layers,
        }
    }
}

fn read_input(input: &str) -> HashMap<u16, Scanner> {
    let mut scanner_map: HashMap<u16, Scanner> = HashMap::new();
    for line in input.split('\n') {
        let mut parts = line.split(':');
        let (depth, layers) = (parts.next().unwrap().trim(), parts.next().unwrap().trim());
        scanner_map.insert(
            u16::from_str(depth).unwrap(),
            Scanner::new(Direction::Down, 1u16, u16::from_str(layers).unwrap()),
        );
    }
    scanner_map
}

pub fn run() {
    println!("=== Day 13 ===");
    part1(INPUT);
    part2(INPUT);
    println!("=============");
    println!();
}

fn part1(input: &str) -> u16 {
    let mut scanner_map = read_input(input);
    let levels = *(scanner_map.keys().max().unwrap());
    let mut caughts: Vec<u16> = Vec::new();
    for (packet_place, _n) in (0..=levels).enumerate() {
        if scanner_map.contains_key(&(packet_place as u16))
            && matches!(scanner_map[&(packet_place as u16)].position, 1)
        {
            caughts.push(packet_place as u16);
        }
        for (_, scanner) in scanner_map.iter_mut() {
            if scanner.direction == Direction::Down && scanner.position < scanner.layers {
                scanner.position += 1;
                continue;
            }
            if scanner.direction == Direction::Down && scanner.position == scanner.layers {
                scanner.position -= 1;
                scanner.direction = Direction::Up;
                continue;
            }
            if scanner.direction == Direction::Up && scanner.position > 1 {
                scanner.position -= 1;
                continue;
            }
            if scanner.direction == Direction::Up && scanner.position == 1 {
                scanner.position += 1;
                scanner.direction = Direction::Down;
                continue;
            }
        }
    }
    let mut severity = 0u16;
    for caught in caughts {
        severity += caught * scanner_map[&caught].layers;
    }
    println!("Day 13 part 1, solution: {}", severity);
    severity
}

fn part2(input: &str) -> u32 {
    let mut scanner_map = read_input(input);
    let levels = *(scanner_map.keys().max().unwrap());
    let mut has_caught = false;
    let mut rounds = 0u32;
    loop {
        let mut scanner_map_cache = scanner_map.clone();
        for (packet_place, _n) in (0..=levels).enumerate() {
            if scanner_map_cache.contains_key(&(packet_place as u16))
                && matches!(scanner_map_cache[&(packet_place as u16)].position, 1)
            {
                has_caught = true;
                break;
            }
            for (_, scanner) in scanner_map_cache.iter_mut() {
                if scanner.direction == Direction::Down && scanner.position < scanner.layers {
                    scanner.position += 1;
                    continue;
                }
                if scanner.direction == Direction::Down && scanner.position == scanner.layers {
                    scanner.position -= 1;
                    scanner.direction = Direction::Up;
                    continue;
                }
                if scanner.direction == Direction::Up && scanner.position > 1 {
                    scanner.position -= 1;
                    continue;
                }
                if scanner.direction == Direction::Up && scanner.position == 1 {
                    scanner.position += 1;
                    scanner.direction = Direction::Down;
                    continue;
                }
            }
        }
        if !has_caught {
            break;
        }
        has_caught = false;
        rounds += 1;
        for (_, scanner) in scanner_map.iter_mut() {
            if scanner.direction == Direction::Down && scanner.position < scanner.layers {
                scanner.position += 1;
                continue;
            }
            if scanner.direction == Direction::Down && scanner.position == scanner.layers {
                scanner.position -= 1;
                scanner.direction = Direction::Up;
                continue;
            }
            if scanner.direction == Direction::Up && scanner.position > 1 {
                scanner.position -= 1;
                continue;
            }
            if scanner.direction == Direction::Up && scanner.position == 1 {
                scanner.position += 1;
                scanner.direction = Direction::Down;
                continue;
            }
        }
    }
    println!("Day 13 part 2, solution: {}", rounds);
    rounds
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_eq;

    #[test]
    fn part1_works_with_test_input() {
        assert_eq!(part1(TEST_INPUT), 24);
    }

    #[test]
    fn part1_works_with_puzzle_input() {
        assert_eq!(part1(INPUT), 1928);
    }

    #[test]
    fn part2_works_with_test_input() {
        assert_eq!(part2(TEST_INPUT), 10);
    }

    #[test]
    fn part2_works_with_puzzle_input() {
        assert_eq!(part2(INPUT), 3830344);
    }
}
