use advent2025::util::read_file;

fn main() {
    part1();
    part2();
}

fn part2() {
    let lines = read_file("input/day3.txt");

    let mut total = 0;
    let base10: u64 = 10;
    for line in lines {
        if line.is_empty() {
            break;
        }

        let line_len = line.len();
        let mut joltage: u64 = 0;
        let mut prev_index: i32 = -1;
        for i in 0..12 {
            // Find the biggest digit in line[prev_index+1..line_len-(11-i)]
            // Set prev_index to that index
            // Add that digit * 10^(11-i) to joltage
            let mut candidate = line.chars().nth((prev_index + 1) as usize).unwrap();
            let mut set_index = false;
            for j in ((prev_index + 2) as usize)..line_len - (11 - i as usize) {
                if line.chars().nth(j).unwrap() > candidate {
                    candidate = line.chars().nth(j).unwrap();
                    prev_index = j as i32;
                    set_index = true;
                }
            }
            if !set_index {
                prev_index += 1;
            }

            joltage += candidate.to_digit(10).unwrap() as u64 * base10.pow(11 - i);
        }

        total += joltage;
    }

    println!("Part 2: {}", total);
}

fn part1() {
    let lines = read_file("input/day3.txt");
    let mut total = 0;
    for line in lines {
        if line.is_empty() {
            break;
        }
        let mut tens = line.chars().nth(0).unwrap().to_digit(10).unwrap();
        let units = line.chars().nth(1).unwrap().to_digit(10).unwrap();
        let mut joltage = tens * 10 + units;
        for i in 1..line.len() {
            let test_joltage = tens * 10 + line.chars().nth(i).unwrap().to_digit(10).unwrap();
            joltage = joltage.max(test_joltage);
            tens = tens.max(line.chars().nth(i).unwrap().to_digit(10).unwrap());
        }
        total += joltage;
    }
    println!("Part 1: {}", total);
}
