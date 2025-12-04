use advent2025::util::read_file;

fn main() {
    part1();
    part2();
}

fn part2() {
    let mut lines = read_file("input/day4.txt");
    let mut total = 0;
    loop {
        let count = count_rolls(&mut lines, true);
        if count == 0 {
            break;
        }
        total += count;
    }
    println!("Part 2: {}", total);
}

fn part1() {
    let mut lines = read_file("input/day4.txt");
    let count = count_rolls(&mut lines, false);
    println!("Part 1: {}", count);
}

fn count_rolls(lines: &mut Vec<String>, remove: bool) -> u32 {
    let mut count = 0;
    for y in 0..lines.len() {
        for x in 0..lines[y].len() {
            let char = lines[y].chars().nth(x).unwrap();
            if char != '@' {
                continue;
            }
            let mut total = 0;
            for i in -1i32..=1 {
                for j in -1i32..=1 {
                    if i == 0 && j == 0 {
                        continue;
                    }
                    if y as i32 + i < 0
                        || y as i32 + i >= lines.len() as i32
                        || x as i32 + j < 0
                        || x as i32 + j >= lines[0].len() as i32
                        || lines[(y as i32 + i) as usize]
                            .chars()
                            .nth((x as i32 + j) as usize)
                            .unwrap()
                            != '@'
                    {
                        continue;
                    }
                    total += 1;
                }
            }
            if total < 4 {
                count += 1;
                if remove {
                    lines[y].replace_range(x..x + 1, ".");
                }
            }
        }
    }
    count
}
