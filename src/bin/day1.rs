use advent2025::util::read_file;

fn main() {
    part1();
    part2();
}

fn part1() {
    let lines = read_file("input/day1.txt");
    let mut cur: i32 = 50;
    let mut cnt = 0;
    for line in lines {
        let amt = line[1..].parse::<i32>().unwrap();
        if line.starts_with('L') {
            cur = (cur - amt) % 100;
        } else if line.starts_with('R') {
            cur = (cur + amt) % 100;
        }
        if cur == 0 {
            cnt += 1;
        }
    }
    println!("Part 1: {}", cnt);
}

fn part2() {
    let lines = read_file("input/day1.txt");
    let mut cur: i32 = 50;
    let mut cnt = 0;
    for line in lines {
        let amt = line[1..].parse::<i32>().unwrap();
        if line.starts_with('L') {
            let mut chg = 0;
            if cur - amt <= 0 {
                chg = (amt - cur) / 100;
                if cur != 0 {
                    chg += 1;
                }
            }

            cnt += chg;
            cur = (cur - amt) % 100;
            if cur < 0 {
                cur = cur + 100;
            }
        } else if line.starts_with('R') {
            let chg = (cur + amt) / 100;

            cnt += chg;
            cur = (cur + amt) % 100;
            if cur < 0 {
                cur = cur + 100;
            }
        }
    }
    println!("Part 2: {}", cnt);
}
