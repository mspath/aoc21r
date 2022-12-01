fn main() {
    let input = include_str!("sample.txt");
    let result_breakfast = breakfast(input);
    println!("breakfast: {}", result_breakfast);

    let result_lunch = lunch(input);
    println!("lunch: {}", result_lunch);
}

// this finds the bits necessary to store an int
// (we need this since the values out of range need to be discarded)
fn count_bits(n: i32) -> i32 {
    if n == 0 {
        return 0
    } else {
        return 1 + count_bits(n >> 1)
    }
}

fn breakfast(input: &str) -> i32 {

    let report: Vec<i32> = input
        .split('\n')
        .map(|line| i32::from_str_radix(&line, 2)
        .unwrap())
        .collect();

    let base: i32 = 2;

    // biggest value
    let max = report.iter().max().unwrap();
    println!("max: {}", max);
    // bits needed to store it
    let max_bits = count_bits(*max);
    println!("bits: {}", max_bits);

    let mut gamma: i32 = 0;
    let mut epsilon: i32 = 0;

    for i in 0..max_bits {
        let flag = &base.pow(i as u32);
        let mut count = 0;

        for m in &report {
            if m & flag == *flag {
                count = count + 1;
            }
        }
        if count > report.len() / 2 {
            gamma = gamma + flag;
        } else {
            epsilon = epsilon + flag;
        }
    }
    
    gamma * epsilon
}

fn count_ones_at_position(input: &Vec<i32>, pos: i32) -> i32 {
    let base: i32 = 2;
    let flag = base.pow(pos as u32);
    let mut count = 0;

    for m in input {
        if m & flag == flag {
            count = count + 1;
        }
    }

    count
}

fn lunch(input: &str) -> i32 {

    let report: Vec<i32> = input
        .split('\n')
        .map(|line| i32::from_str_radix(&line, 2)
        .unwrap())
        .collect();

    // biggest value
    let max = report.iter().max().unwrap();
    println!("max: {}", max);
    // bits needed to store it
    let max_bits = count_bits(*max);
    println!("bits: {}", max_bits);

    let mut bits = max_bits.clone();

    while bits > 0 {
        bits = bits - 1;
        println!("ones at position {}: {}", bits, count_ones_at_position(&report, bits));
    }

    return 230;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_breakfast_sample() {
        assert_eq!(super::breakfast(include_str!("sample.txt")), 198);
    }

    #[test]
    fn test_breakfast() {
        assert_eq!(super::breakfast(include_str!("input.txt")), 3277364);
    }

    #[test]
    fn test_lunch_sample() {
        assert_eq!(super::lunch(include_str!("sample.txt")), 230);
    }

    #[test]
    fn test_lunch() {
        assert_eq!(super::lunch(include_str!("input.txt")), 5736383);
    }
}
