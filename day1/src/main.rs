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
    fn test_breakfast_sample() {
        assert_eq!(super::breakfast(include_str!("sample.txt")), 7);
    }

    #[test]
    fn test_breakfast() {
        assert_eq!(super::breakfast(include_str!("input.txt")), 1374);
    }

    #[test]
    fn test_lunch_sample() {
        assert_eq!(super::lunch(include_str!("sample.txt")), 5);
    }

    #[test]
    fn test_lunch() {
        assert_eq!(super::lunch(include_str!("input.txt")), 1418);
    }
}
