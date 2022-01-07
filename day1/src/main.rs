fn main() {
    let input = include_str!("input.txt");
    let result_breakfast = breakfast(input);
    println!("breakfast: {}", result_breakfast);

    let result_lunch = lunch(input);
    println!("lunch: {}", result_lunch);
}

fn breakfast(input: &str) -> i32 {
    let measurements: Vec<i32> = input
        .trim()
        .split('\n')
        .map(|l| l.parse::<i32>()
        .unwrap())
        .collect();
    let mut counter = 0;
    for window in measurements.windows(2) {
        if window[1] > window[0] {
            counter += 1;
        }
    }
    counter
}

fn lunch(input: &str) -> i32 {
    let measurements: Vec<i32> = input
        .trim()
        .split('\n')
        .map(|l| l.parse::<i32>()
        .unwrap())
        .collect();
    let mut counter = 0;
    for window in measurements.windows(4) {
        if window[3] > window[0] {
            counter += 1;
        }
    }
    counter
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_one_sample() {
        assert_eq!(super::breakfast(include_str!("sample.txt")), 7);
    }

    #[test]
    fn part_one() {
        assert_eq!(super::breakfast(include_str!("input.txt")), 1374);
    }

    #[test]
    fn part_two_sample() {
        assert_eq!(super::lunch(include_str!("sample.txt")), 5);
    }

    #[test]
    fn part_two() {
        assert_eq!(super::lunch(include_str!("input.txt")), 1418);
    }
}
