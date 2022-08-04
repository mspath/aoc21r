use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    let result_breakfast = breakfast(input);
    println!("{}", result_breakfast);
}

fn breakfast(input: &str) -> usize {
    let coordinates: Vec<[i32; 4]> = input
    .trim()
    .split('\n')
    .into_iter()
    .map(|line| {
        let mut parts = line.split(" -> ");
        let left: Vec<i32> = parts.next()
            .unwrap()
            .split(',')
            .map(|n| n.parse().unwrap())
            .collect();
        let right: Vec<i32> = parts.next()
            .unwrap()
            .split(',')
            .map(|n| n.parse().unwrap())
            .collect();
        //let c = vec![left[0], left[1], right[0], right[1]];
        let c = [left[0], left[1], right[0], right[1]];
        c
    }
    ).collect();

    let horizontal_vertical: Vec<[i32; 4]> = coordinates.into_iter()
        .filter(|c|c[0] == c[2] || c[1] == c[3])
        .collect();
    println!("{:?}", horizontal_vertical.len());

    let mut points: HashMap<(i32, i32), i32> = HashMap::new();

    for [x1, y1, x2, y2] in horizontal_vertical {
        if x1 == x2 {
            for y in y1.min(y2)..=y1.max(y2) {
                *points.entry((x1, y)).or_default() += 1;
            }
        } else if y1 == y2 {
            for x in x1.min(x2)..=x1.max(x2) {
                *points.entry((x, y1)).or_default() += 1;
            }
        }
    }

    points.values().filter(|n| **n > 1).count()
}
