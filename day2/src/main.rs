fn main() {
    let input = include_str!("input.txt");
    let result_breakfast = breakfast(input);
    println!("breakfast: {}", result_breakfast);

    let result_lunch = lunch(input);
    println!("lunch: {}", result_lunch);
}

fn breakfast(input: &str) -> i32 {

    let mut distance = 0;
    let mut depth = 0;

    for line in input.trim().split('\n') {
        let mut parts = line.split(' ');
        let command = parts.next().unwrap();
        let value: i32 = parts.next().unwrap().parse().unwrap();
        match command {
            "forward" => {
                distance += value;
            }
            "down" => {
                depth += value;
            }
            "up" => {
                depth -= value;
            }
            _ => panic!(),
        }
    }
    distance * depth
}

fn lunch(input: &str) -> i32 {
    let mut distance = 0;
    let mut depth = 0;
    let mut aim = 0;

    for line in input.trim().split('\n') {
        let mut parts = line.split(' ');
        let command = parts.next().unwrap();
        let value: i32 = parts.next().unwrap().parse().unwrap();
        match command {
            "forward" => {
                distance += value;
                depth += aim * value;
            }
            "down" => {
                aim += value;
            }
            "up" => {
                aim -= value;
            }
            _ => panic!(),
        }
    }
    distance * depth
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_breakfast_sample() {
        assert_eq!(super::breakfast(include_str!("sample.txt")), 150);
    }

    #[test]
    fn test_breakfast() {
        assert_eq!(super::breakfast(include_str!("input.txt")), 1451208);
    }

    #[test]
    fn test_lunch_sample() {
        assert_eq!(super::lunch(include_str!("sample.txt")), 900);
    }

    #[test]
    fn test_lunch() {
        assert_eq!(super::lunch(include_str!("input.txt")), 1620141160);
    }
}
