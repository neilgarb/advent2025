use advent2025::util::read_file;

fn main() {
    part1();
    part2();
}

fn part2() {
    let lines = read_file("input/day5.txt");
    let mut ranges: Vec<(i64, i64)> = vec![];
    for line in lines {
        if line == "" {
            break;
        }
        let parts: Vec<&str> = line.split("-").collect();
        let range = (
            parts[0].parse::<i64>().unwrap(),
            parts[1].parse::<i64>().unwrap(),
        );
        ranges.push(range);
        ranges.sort_by(|a, b| a.0.cmp(&b.0));
    }

    for i in 0..ranges.len() {
        let range = ranges[i];
        if range.0 == 0 {
            continue;
        }
        for j in i + 1..ranges.len() {
            if ranges[j].0 >= range.0 && ranges[j].1 <= range.1 && ranges[j].0 != 0 {
                ranges[j].0 = 0;
                ranges[j].1 = 0;
            }
        }
        for j in i + 1..ranges.len() {
            if ranges[j].0 != 0 && ranges[j].0 <= range.1 {
                ranges[i].1 = ranges[j].0 - 1;
                break;
            }
        }
    }

    let mut total = 0;
    for range in ranges {
        if range.0 > 0 && range.1 > 0 && range.1 >= range.0 {
            total += range.1 - range.0 + 1;
        }
    }
    println!("Part 2: {}", total);
}

fn part1() {
    let lines = read_file("input/day5.txt");
    let mut ranges: Vec<(i64, i64)> = vec![];
    let mut ranges_done = false;
    let mut count = 0;
    for line in lines {
        if line == "" {
            ranges_done = true;
            continue;
        }
        if !ranges_done {
            let parts: Vec<&str> = line.split("-").collect();
            ranges.push((
                parts[0].parse::<i64>().unwrap(),
                parts[1].parse::<i64>().unwrap(),
            ));
        } else {
            let num = line.parse::<i64>().unwrap();
            for range in &ranges {
                if num >= range.0 && num <= range.1 {
                    count += 1;
                    break;
                }
            }
        }
    }
    println!("Part 1: {}", count);
}
