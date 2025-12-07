use advent2025::util::read_file;
use std::collections::{HashMap, HashSet};

fn main() {
    part1();
    part2();
}

fn part2() {
    let lines = read_file("input/day7.txt");
    let mut worlds: HashMap<usize, usize> = HashMap::new();
    for line in lines {
        for (i, c) in line.chars().enumerate() {
            if c == '.' {
                continue;
            }
            if c == 'S' {
                worlds.insert(i, 1);
            }
            if c == '^' {
                let cnt = worlds.remove(&i);
                if let Some(cnt) = cnt {
                    if i > 0 {
                        let left = worlds.get(&(i - 1));
                        worlds.insert(i - 1, cnt + left.unwrap_or(&0));
                    }
                    if i < line.len() - 1 {
                        let right = worlds.get(&(i + 1));
                        worlds.insert(i + 1, cnt + right.unwrap_or(&0));
                    }
                }
            }
        }
    }
    let mut total = 0;
    for (_, v) in worlds.iter().enumerate() {
        total += v.1;
    }
    println!("Part 2: {}", total);
}

fn part1() {
    let lines = read_file("input/day7.txt");
    let mut beams: HashSet<usize> = HashSet::new();
    let mut splits: i32 = 0;
    for line in lines {
        for (i, c) in line.chars().enumerate() {
            if c == '.' {
                continue;
            }
            if c == 'S' {
                beams.insert(i);
            }
            if c == '^' {
                if beams.contains(&i) {
                    beams.remove(&i);
                    if i < line.len() - 1 {
                        beams.insert(i + 1);
                    }
                    if i > 0 {
                        beams.insert(i - 1);
                    }
                    splits += 1;
                }
            }
        }
    }
    println!("Part 1: {}", splits);
}
