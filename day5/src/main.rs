use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    let result_breakfast = breakfast(input);
    println!("{}", result_breakfast);
    let result_lunch = lunch(input);
    println!("{}", result_lunch);
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
        let c = [left[0], left[1], right[0], right[1]];
        c
    }
    ).collect();

    let mut points: HashMap<(i32, i32), i32> = HashMap::new();

    for [x1, y1, x2, y2] in coordinates {
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

fn lunch(input: &str) -> usize {
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
        let c = [left[0], left[1], right[0], right[1]];
        c
    }
    ).collect();

    let mut points: HashMap<(i32, i32), i32> = HashMap::new();

    for [x1, y1, x2, y2] in coordinates {
        if x1 == x2 {
            for y in y1.min(y2)..=y1.max(y2) {
                *points.entry((x1, y)).or_default() += 1;
            }
        } else if y1 == y2 {
            for x in x1.min(x2)..=x1.max(x2) {
                *points.entry((x, y1)).or_default() += 1;
            }
        } else {
            // we assume the input is clean
            // i.e. all other lines are valid diagonals
            let dx = if x2 - x1 > 0 { 1 } else { -1 };
            let dy = if y2 - y1 > 0 { 1 } else { -1 };
            let mut x = x1;
            let mut y = y1;
            *points.entry((x, y)).or_default() += 1;
            while x != x2 {
                x += dx;
                y += dy;
                *points.entry((x, y)).or_default() += 1;
            }
        }
    }

    points.values().filter(|n| **n > 1).count()
}

#[cfg(test)]
mod tests {
    #[test]
    fn check_breakfast_sample() {
        assert_eq!(super::breakfast(include_str!("sample.txt")), 5);
    }

    #[test]
    fn check_breakfast() {
        assert_eq!(super::breakfast(include_str!("input.txt")), 6005);
    }

    #[test]
    fn check_lunch_sample() {
        assert_eq!(super::lunch(include_str!("sample.txt")), 12);
    }

    #[test]
    fn check_lunch() {
        assert_eq!(super::lunch(include_str!("input.txt")), 23864);
    }
}