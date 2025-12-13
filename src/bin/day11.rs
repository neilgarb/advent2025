use std::collections::{HashMap, VecDeque};

use advent2025::util::read_file;

fn main() {
    part1();
    part2();
}

fn part2() {
    let lines = read_file("input/day11.txt");
    let mut conns: HashMap<String, Vec<String>> = HashMap::new();
    for line in lines {
        if line.is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.split(" ").collect();
        let from = parts[0][..parts[0].len() - 1].to_string();

        let has_from = conns.get(&from);
        let mut v: Vec<String> = match has_from {
            Some(v) => v.clone(),
            None => vec![],
        };

        for to in parts[1..].iter() {
            v.push(to.to_string());
        }

        conns.insert(from, v);
    }

    let to_fft = count_paths(&conns, "svr".to_string(), "fft".to_string());
    let to_dac = count_paths(&conns, "fft".to_string(), "dac".to_string());
    let to_out = count_paths(&conns, "dac".to_string(), "out".to_string());

    println!("Part 2: {}", to_fft * to_dac * to_out);
}

fn part1() {
    let lines = read_file("input/day11.txt");
    let mut conns: HashMap<String, Vec<String>> = HashMap::new();
    for line in lines {
        let parts: Vec<&str> = line.split(" ").collect();
        let from = parts[0][..parts[0].len() - 1].to_string();

        let has_from = conns.get(&from);
        let mut v: Vec<String> = match has_from {
            Some(v) => v.clone(),
            None => vec![],
        };

        for to in parts[1..].iter() {
            v.push(to.to_string());
        }

        conns.insert(from, v);
    }

    println!(
        "Part 1: {}",
        count_paths(&conns, "you".to_string(), "out".to_string())
    );
}

#[derive(Debug)]
struct Q {
    node: String,
    count: usize,
}

fn count_paths(conns: &HashMap<String, Vec<String>>, from: String, to: String) -> usize {
    let mut queue: VecDeque<Q> = VecDeque::new();
    queue.push_front(Q {
        node: from.clone(),
        count: 1,
    });

    let mut res: usize = 0;

    while let Some(q) = queue.pop_front() {
        if q.node == to {
            res += q.count;
        }

        for t in conns.get(&q.node).unwrap_or(&vec![]) {
            // Check if already in the queue. If so, just increase by one.
            let mut found = false;
            for qq in queue.iter_mut() {
                if qq.node == *t {
                    qq.count += q.count;
                    found = true;
                }
            }
            // Otherwise, insert.
            if !found {
                queue.push_back(Q {
                    node: t.clone(),
                    count: q.count,
                });
            }
        }
    }

    return res;
}
