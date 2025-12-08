use std::collections::HashMap;

use advent2025::util::read_file;

fn main() {
    part1();
    part2();
}

type Point = (i64, i64, i64);

#[derive(Debug, Copy, Clone)]
struct Edge {
    a: Point,
    b: Point,
    distance: f64,
}

fn part2() {
    let lines = read_file("input/day8.txt");

    let mut points: Vec<Point> = vec![];
    for l in lines {
        points.push({
            let parts: Vec<i64> = l.split(",").map(|p| p.parse().unwrap()).collect();
            (parts[0], parts[1], parts[2])
        });
    }

    let mut edges: Vec<Edge> = vec![];
    for i in 0..points.len() {
        for j in i..points.len() {
            if i == j {
                continue;
            }
            let d = distance(points[i], points[j]);
            edges.push(Edge {
                a: points[i],
                b: points[j],
                distance: d,
            });
        }
    }

    edges.sort_by(|a, b| a.distance.total_cmp(&b.distance));

    let mut circuit_index = 1;
    let mut circuit_map: HashMap<Point, i32> = HashMap::new();
    let mut answer = 0;
    for i in 0..edges.len() {
        let e = edges[i];
        let circuit_a = circuit_map.get(&e.a);
        let circuit_b = circuit_map.get(&e.b);
        if circuit_a.is_none() && circuit_b.is_none() {
            circuit_map.insert(e.a, circuit_index);
            circuit_map.insert(e.b, circuit_index);
            circuit_index += 1;
        } else if circuit_a.is_some() && circuit_b.is_none() {
            circuit_map.insert(e.b, *circuit_a.unwrap());
        } else if circuit_b.is_some() && circuit_a.is_none() {
            circuit_map.insert(e.a, *circuit_b.unwrap());
        } else {
            let a = *circuit_a.unwrap();
            let mut b_points: Vec<Point> = vec![];
            for (k, v) in circuit_map.iter() {
                if *v == *circuit_b.unwrap() {
                    b_points.push(*k);
                }
            }
            for p in b_points {
                circuit_map.insert(p, a);
            }
        }
        if circuit_map.len() == points.len() {
            let mut init = 0;
            let mut all_same = true;
            for (_, v) in circuit_map.iter() {
                if init == 0 {
                    init = *v
                } else if *v != init {
                    all_same = false;
                    break;
                }
            }

            if all_same {
                answer = e.a.0 * e.b.0;
                break;
            }
        }
    }

    println!("Part 2: {}", answer);
}

fn part1() {
    let lines = read_file("input/day8.txt");

    let mut points: Vec<Point> = vec![];
    for l in lines {
        points.push({
            let parts: Vec<i64> = l.split(",").map(|p| p.parse().unwrap()).collect();
            (parts[0], parts[1], parts[2])
        });
    }

    let mut edges: Vec<Edge> = vec![];
    for i in 0..points.len() {
        for j in i..points.len() {
            if i == j {
                continue;
            }
            let d = distance(points[i], points[j]);
            edges.push(Edge {
                a: points[i],
                b: points[j],
                distance: d,
            });
        }
    }

    edges.sort_by(|a, b| a.distance.total_cmp(&b.distance));

    let mut circuit_index = 1;
    let mut circuit_map: HashMap<Point, i32> = HashMap::new();
    for i in 0..1000 {
        let e = edges[i];
        let circuit_a = circuit_map.get(&e.a);
        let circuit_b = circuit_map.get(&e.b);
        if circuit_a.is_none() && circuit_b.is_none() {
            circuit_map.insert(e.a, circuit_index);
            circuit_map.insert(e.b, circuit_index);
            circuit_index += 1;
        } else if circuit_a.is_some() && circuit_b.is_none() {
            circuit_map.insert(e.b, *circuit_a.unwrap());
            continue;
        } else if circuit_b.is_some() && circuit_a.is_none() {
            circuit_map.insert(e.a, *circuit_b.unwrap());
            continue;
        } else {
            let a = *circuit_a.unwrap();
            let mut b_points: Vec<Point> = vec![];
            for (k, v) in circuit_map.iter() {
                if *v == *circuit_b.unwrap() {
                    b_points.push(*k);
                }
            }
            for p in b_points {
                circuit_map.insert(p, a);
            }
        }
    }

    let mut circuit_counts: HashMap<i32, i32> = HashMap::new();
    for (_, v) in circuit_map.iter() {
        let cur = circuit_counts.get(v);
        match cur {
            Some(cur) => circuit_counts.insert(*v, *cur + 1),
            None => circuit_counts.insert(*v, 1),
        };
    }

    let mut counts: Vec<i32> = vec![];
    for (_, v) in circuit_counts.iter() {
        counts.push(*v);
    }

    counts.sort_by(|a, b| a.cmp(b).reverse());

    println!("Part 1: {}", counts[0] * counts[1] * counts[2]);
}

fn distance(a: Point, b: Point) -> f64 {
    ((a.0 - b.0).pow(2) as f64 + (a.1 - b.1).pow(2) as f64 + (a.2 - b.2).pow(2) as f64).sqrt()
}
