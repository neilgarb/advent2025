use advent2025::util::read_file;

fn main() {
    part1();
    part2();
}

fn part2() {
    let lines = read_file("input/day6.txt");
    let oper_line = lines.iter().last().unwrap();
    let mut tot: i64 = 0;

    for i in 0..oper_line.len() {
        let c = oper_line.chars().nth(i).unwrap();
        if c != '+' && c != '*' {
            continue;
        }

        // find the next char index
        let mut n = oper_line.len();
        for j in i + 1..oper_line.len() {
            let c = oper_line.chars().nth(j).unwrap();
            if c == '+' || c == '*' {
                n = j - 1;
                break;
            }
        }

        let mut sub: i64 = match c {
            '+' => 0,
            '*' => 1,
            _ => panic!("unknown oper {}", c),
        };

        for col in i..n {
            let mut operand = "".to_string();
            for row in 0..lines.len() - 1 {
                match lines[row].chars().nth(col) {
                    Some(cc) if cc != ' ' => operand = operand + cc.to_string().as_str(),
                    _ => continue,
                };
            }
            match c {
                '+' => sub += operand.parse::<i64>().unwrap(),
                '*' => sub *= operand.parse::<i64>().unwrap(),
                _ => panic!("unknown oper {}", c),
            };
        }

        tot += sub;
    }

    println!("Part 2: {}", tot);
}

fn part1() {
    let lines = read_file("input/day6.txt");
    let opers: Vec<&str> = lines[lines.len() - 1].split_whitespace().collect();
    let mut tot = 0;

    for i in 0..opers.len() {
        let oper = opers[i];
        let mut sub: i64 = match oper {
            "+" => 0,
            "*" => 1,
            _ => panic!("unknown oper {}", oper),
        };
        lines[..lines.len() - 1].iter().for_each(|l| {
            let parts: Vec<&str> = l.split_whitespace().collect();
            match oper {
                "+" => sub = sub + parts[i].parse::<i64>().unwrap(),
                "*" => sub = sub * parts[i].parse::<i64>().unwrap(),
                _ => panic!("unknown oper {}", oper),
            };
        });
        tot += sub;
    }
    println!("Part 1: {}", tot);
}
