use advent2025::util::read_file;

fn main() {
    part("Part 1", |num| {
        let num_str = num.to_string();
        if num_str.len() % 2 == 1 {
            return false;
        }
        num_str[..num_str.len() / 2] == num_str[num_str.len() / 2..]
    });

    part("Part 2", |num| {
        let num_str = num.to_string();
        let num_len = num_str.len();
        for i in 1..=num_len / 2 {
            let pat = &num_str[..i];
            if num_str == pat.repeat(num_len / i) {
                return true;
            }
        }
        return false;
    });
}

fn part(name: &str, f: impl Fn(i64) -> bool) {
    let lines = read_file("input/day2.txt");
    let ranges = lines[0]
        .split(',')
        .map(|range| {
            range
                .split('-')
                .map(|num| num.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>();
    let mut cnt = 0;
    ranges.iter().for_each(|range| {
        for i in range[0]..=range[1] {
            if f(i) {
                cnt += i;
            }
        }
    });
    println!("{}: {}", name, cnt);
}
