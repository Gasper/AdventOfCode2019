use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use std::collections::HashMap;

fn main() {

    let input_file = match File::open("input.txt") {
        Err(_) => panic!("Could not open input.txt"),
        Ok(file) => file,
    };

    let instructions: Vec<String> = BufReader::new(input_file)
        .lines().map(|line| line.unwrap()).collect();

    let deck = (0..10007);
    let shuffled = run_instructions(deck.collect(), instructions);

    let mut position = 0;
    for card in shuffled {
        if card == 2019 {
            println!("Card on place 2019 is {}", position);
        }
        position += 1;
    }
}

fn run_instructions(deck: Vec<i64>, instructions: Vec<String>) -> Vec<i64> {
    let mut deck_temp = deck.clone();
    for instruction_raw in instructions {
        let (instruction, parameter) = parse_instruction(instruction_raw);

        deck_temp = match instruction.as_str() {
            "deal with increment" => deal_with_increment(deck_temp, parameter as usize),
            "cut" => cut(deck_temp, parameter),
            "deal into new stack" => new_stack(deck_temp),
            _ => panic!("Unknwon instruction: {}", instruction),
        };
    }

    return deck_temp;
}

fn parse_instruction(instruction: String) -> (String, i64) {
    if instruction.starts_with("deal with increment") {
        let (start, parameter) = instruction.split_at(20);
        return (String::from("deal with increment"), parameter.parse().unwrap());
    }
    else if instruction.starts_with("cut") {
        let (start, parameter) = instruction.split_at(4);
        return (String::from("cut"), parameter.parse().unwrap());
    }
    else if instruction.starts_with("deal into new stack") {
        return (instruction, 0);
    }
    else {
        panic!("No or unknown instruction");
    }
}

fn new_stack(deck: Vec<i64>) -> Vec<i64> {
    return deck.into_iter().rev().collect();
}

fn cut(deck: Vec<i64>, at: i64) -> Vec<i64> {
    let mut at_normalized: usize = 0;
    if at >= 0 {
        at_normalized = at as usize;
    }
    else {
        at_normalized = (deck.len() as i64 + at) as usize;
    }

    let (upper, lower) = deck.split_at(at_normalized);
    return lower.into_iter().map(|c| *c).chain(upper.into_iter().map(|c| *c)).collect();
}

fn deal_with_increment(deck: Vec<i64>, increment: usize) -> Vec<i64> {
    let deck_len = deck.len();
    let mut positions = HashMap::with_capacity(deck_len);

    let mut current_position = 0;
    for card in deck {
        positions.insert(current_position, card);
        current_position = (current_position + increment) % deck_len;
    }

    return (0..deck_len).map(|n| positions[&n]).collect();
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_new_stack() {
        let deck = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        
        assert_eq!(new_stack(deck), vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
    }

    #[test]
    fn test_cut() {
        let deck = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        
        assert_eq!(cut(deck, 3), vec![3, 4, 5, 6, 7, 8, 9, 0, 1, 2]);
    }

    #[test]
    fn test_cut_negative() {
        let deck = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        
        assert_eq!(cut(deck, -4), vec![6, 7, 8, 9, 0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_deal_with_increment() {
        let deck = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        
        assert_eq!(deal_with_increment(deck, 3), vec![0, 7, 4, 1, 8, 5, 2, 9, 6, 3]);
    }

    #[test]
    fn test_example1() {
        let deck = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let instr = vec![String::from("deal with increment 7"),
        String::from("deal into new stack"), String::from("deal into new stack")];
        
        assert_eq!(run_instructions(deck, instr), vec![0, 3, 6, 9, 2, 5, 8, 1, 4, 7]);
    }

    #[test]
    fn test_example2() {
        let deck = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let instr = vec![String::from("cut 6"),
        String::from("deal with increment 7"), String::from("deal into new stack")];
        
        assert_eq!(run_instructions(deck, instr), vec![3, 0, 7, 4, 1, 8, 5, 2, 9, 6]);
    }

    #[test]
    fn test_example3() {
        let deck = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let instr = vec![String::from("deal with increment 7"), 
        String::from("deal with increment 9"), String::from("cut -2")];
        
        assert_eq!(run_instructions(deck, instr), vec![6, 3, 0, 7, 4, 1, 8, 5, 2, 9]);
    }

    #[test]
    fn test_example4() {
        let deck = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let instr = vec![String::from("deal into new stack"),
        String::from("cut -2"),
        String::from("deal with increment 7"),
        String::from("cut 8"),
        String::from("cut -4"),
        String::from("deal with increment 7"),
        String::from("cut 3"),
        String::from("deal with increment 9"),
        String::from("deal with increment 3"),
        String::from("cut -1")];
        
        assert_eq!(run_instructions(deck, instr), vec![9, 2, 5, 8, 1, 4, 7, 0, 3, 6]);
    }
}