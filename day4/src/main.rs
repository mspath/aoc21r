
fn main() {
    let input = include_str!("input.txt");
    let result_breakfast = breakfast(input);
    println!("breakfast: {}", result_breakfast);
}

#[derive(PartialEq)]
#[derive(Debug,Copy,Clone)]
enum BallotType { Uncalled, Called }

#[derive(Debug)]
struct Ball {
    number: usize,
    ballot: BallotType,
}

#[derive(Debug)]
struct BingoCard {
    ballots: Vec<Ball>
}

fn new_bingo_card(numbers: &Vec<usize>) -> BingoCard {
    let balls = numbers.iter().map(|n| {
        Ball { number: *n, ballot: BallotType::Uncalled }
    }).collect();
    BingoCard { ballots: balls }
}

fn new_bingo_card_with_number(old: &BingoCard, number: usize) -> BingoCard {
    let balls = old.ballots.iter().map(|n| {
        if n.number == number {
            Ball { number: number, ballot: BallotType::Called }
        } else {
            Ball { number: n.number, ballot: n.ballot }
        }
    }).collect();
    BingoCard { ballots: balls }
}

fn count_matches(card: &BingoCard, row: &Vec<i32>) -> usize {
    let balls: Vec<&Ball> = row.iter().map(|i| card.ballots.get(*i as usize).unwrap()).collect();
    let matches = balls.iter().filter(|b| b.ballot == BallotType::Called).count();
    matches
}

fn pretty_print_card(card: &BingoCard) {
    for i in 0..25 {
        let b = &card.ballots[i];
        if b.ballot == BallotType::Called {
            print!("{}-", b.number)
        }
        else {
            print!("X-")
        }
        if i % 5 == 4 {
            println!("");
        }
    }
}

fn add_unmarked(card: &BingoCard) -> usize {
    let unmarked = card.ballots.iter().fold(0, |acc, b| {
        let s = if b.ballot == BallotType::Uncalled { b.number } else { 0 };
        acc + s
    });
    unmarked
}

fn check_bingo(card: &BingoCard) -> bool {
    let row_1 = vec![0, 1, 2, 3, 4];
    let row_2 = vec![5, 6, 7, 8, 9];
    let row_3 = vec![10, 11, 12, 13, 14];
    let row_4 = vec![15, 16, 17, 18, 19];
    let row_5 = vec![20, 21, 22, 23, 24];
    let col_1 = vec![0, 5, 10, 15, 20];
    let col_2 = vec![1, 6, 11, 16, 21];
    let col_3 = vec![2, 7, 12, 17, 22];
    let col_4 = vec![3, 8, 13, 18, 23];
    let col_5 = vec![4, 9, 14, 19, 24];
    let lines = vec![row_1, row_2, row_3, row_4, row_5, col_1, col_2, col_3, col_4, col_5];
    let bingo = lines.iter().any(|l| count_matches(card, l) == 5);
    if bingo {
        pretty_print_card(card);
        println!("{}", add_unmarked(card));
    }
    bingo
}

fn breakfast(input: &str) -> i32 {

    let (numbers_str, boards_str) = input.split_once("\n\n").unwrap();

    let numbers: Vec<usize> = numbers_str
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect();
    println!("{:?}", numbers);

    let boards: Vec<&str> = boards_str.split("\n\n").collect();
    let boards: Vec<Vec<&str>> = boards.iter()
        .map(|s| s.split_whitespace().collect())
        .collect();
    let boards: Vec<Vec<usize>> = boards
        .iter()
        .map(|bb| { bb.iter().map(|s| s.parse().unwrap()).collect() })
        .collect();
    let mut bingo_cards: Vec<BingoCard> = boards.iter().map (|bb| {
        new_bingo_card(bb)
    }).collect();
    numbers.iter().try_for_each(|n| {
        bingo_cards = bingo_cards.iter().map(|bc| new_bingo_card_with_number(bc, *n)).collect();
        let bingo = bingo_cards.iter().any(|c| check_bingo(c));
        if bingo { println!("{}", n); }
        if bingo { None } else { Some(()) }
    });
    0
}

fn lunch(input: &str) -> i32 {
    0
}