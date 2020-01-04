extern crate num_bigint;
extern crate num_traits;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;
use num_bigint::{BigInt};
use num_traits::{One, ToPrimitive};

fn main() {

    let input_file = match File::open("input.txt") {
        Err(_) => panic!("Could not open input.txt"),
        Ok(file) => file,
    };

    let instructions: Vec<String> = BufReader::new(input_file)
        .lines().map(|line| line.unwrap()).collect();

    let deck = 0..10007;
    let shuffled = run_instructions(deck.collect(), &instructions);

    let mut position = 0;
    for card in shuffled {
        if card == 2019 {
            println!("Card on place 2019 is {}", position);
        }
        position += 1;
    }

    let original_position = backtrack_instructions(2496, 10007, &instructions, 1);
    println!("Original position of the 2496 card: {}", original_position);

    // Part 2
    let original_position = backtrack_instructions(2020, 119315717514047, &instructions, 101741582076661);
    println!("Original position of the card in the big deck: {}", original_position);
}

fn run_instructions(deck: Vec<i128>, instructions: &Vec<String>) -> Vec<i128> {
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

fn parse_instruction(instruction: &String) -> (String, i128) {
    if instruction.starts_with("deal with increment") {
        let (_start, parameter) = instruction.split_at(20);
        return (String::from("deal with increment"), parameter.parse().unwrap());
    }
    else if instruction.starts_with("cut") {
        let (_start, parameter) = instruction.split_at(4);
        return (String::from("cut"), parameter.parse().unwrap());
    }
    else if instruction.starts_with("deal into new stack") {
        return (String::from("deal into new stack"), 0);
    }
    else {
        panic!("No or unknown instruction");
    }
}

fn new_stack(deck: Vec<i128>) -> Vec<i128> {
    return deck.into_iter().rev().collect();
}

fn cut(deck: Vec<i128>, at: i128) -> Vec<i128> {
    let at_normalized: usize;
    if at >= 0 {
        at_normalized = at as usize;
    }
    else {
        at_normalized = (deck.len() as i128 + at) as usize;
    }

    let (upper, lower) = deck.split_at(at_normalized);
    return lower.into_iter().map(|c| *c).chain(upper.into_iter().map(|c| *c)).collect();
}

fn deal_with_increment(deck: Vec<i128>, increment: usize) -> Vec<i128> {
    let deck_len = deck.len();
    let mut positions = HashMap::with_capacity(deck_len);

    let mut current_position = 0;
    for card in deck {
        positions.insert(current_position, card);
        current_position = (current_position + increment) % deck_len;
    }

    return (0..deck_len).map(|n| positions[&n]).collect();
}

// Part 2

fn backtrack_instructions(card_at: i128, deck_size: i128, instructions: &Vec<String>, repeat: i128) -> i128 {
    let mut params_temp: (i128, i128) = (1, 0);

    for instruction_raw in instructions.into_iter() {
        let (instruction, parameter) = parse_instruction(instruction_raw);

        let operation = match instruction.as_str() {
            "deal with increment" => (parameter, 0),
            "cut" => (1, -parameter),
            "deal into new stack" => (-1, -1),
            _ => panic!("Unknwon instruction: {}", instruction),
        };

        params_temp = combine_lcfs(params_temp, operation, deck_size);
    }

    let a_repeated = pow_mod(params_temp.0, repeat, deck_size);
    params_temp = (
        a_repeated,
        div_mod(params_temp.1 * (1-a_repeated), 1-params_temp.0, deck_size),
    );

    let inversed = div_mod(card_at - params_temp.1, params_temp.0, deck_size);
    return mod_n(inversed, deck_size);
}

fn combine_lcfs(a: (i128, i128), b: (i128, i128), m: i128) -> (i128, i128) {
    return (
        mod_n(a.0 * b.0, m),
        mod_n(a.1 * b.0 + b.1, m),
    );
}

fn div_mod(a: i128, b: i128, m: i128) -> i128 {
    let tx: BigInt = One::one();
    let big_mod: BigInt = tx * m;
    
    let temp_bigint: BigInt = One::one();
    let multiplied = (temp_bigint * a * pow_mod(b, m-2, m)).modpow(&One::one(), &big_mod);
    return multiplied.to_i128().unwrap();
}

fn pow_mod(a: i128, pow: i128, m: i128) -> i128 {
    if pow == 0 {
        return 1;
    }

    if is_even(pow) {
        let t = pow_mod(a, pow/2, m);
        return mod_n((mod_n(t, m) as u128 * mod_n(t, m) as u128) as i128, m) as i128;
    }
    else {
        let t = pow_mod(a, (pow-1)/2, m);
        let temp_bigint: BigInt = One::one();
        let multiplied = (temp_bigint * t * t * a) % m;
        return multiplied.to_i128().unwrap();
    }
}

fn mod_n(a: i128, m: i128) -> i128 {
    if a < 0 {
        return m - (a.abs() % m);
    }
    else {
        return a % m;
    }
}

fn is_even(a: i128) -> bool {
    return a & 1 == 0;
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
        
        assert_eq!(run_instructions(deck, &instr), vec![0, 3, 6, 9, 2, 5, 8, 1, 4, 7]);
    }

    #[test]
    fn test_example2() {
        let deck = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let instr = vec![String::from("cut 6"),
        String::from("deal with increment 7"), String::from("deal into new stack")];
        
        assert_eq!(run_instructions(deck, &instr), vec![3, 0, 7, 4, 1, 8, 5, 2, 9, 6]);
    }

    #[test]
    fn test_example3() {
        let deck = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let instr = vec![String::from("deal with increment 7"), 
        String::from("deal with increment 9"), String::from("cut -2")];
        
        assert_eq!(run_instructions(deck, &instr), vec![6, 3, 0, 7, 4, 1, 8, 5, 2, 9]);
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
        
        assert_eq!(run_instructions(deck, &instr), vec![9, 2, 5, 8, 1, 4, 7, 0, 3, 6]);
    }

    #[test]
    fn test_backtrack_example2() {
        let deck = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let instr = vec![String::from("cut 6"),
        String::from("deal with increment 7"), String::from("deal into new stack")];
        
        let mut shuffled = run_instructions(deck.clone(), &instr);
        for i in 0..16 {
            shuffled = run_instructions(shuffled.clone(), &instr);
        }

        for i in 0..11 {
            let original = backtrack_instructions(i, 11, &instr, 17);
            assert_eq!(shuffled[i as usize], deck[original as usize]);
        }
    }

    #[test]
    fn test_backtrack_example3() {
        let deck = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let instr = vec![String::from("deal with increment 7"),
        String::from("deal with increment 9"), String::from("cut -2")];
        
        let mut shuffled = run_instructions(deck.clone(), &instr);
        for i in 0..13 {
            shuffled = run_instructions(shuffled.clone(), &instr);
        }

        for i in 0..11 {
            let original = backtrack_instructions(i, 11, &instr, 14);
            assert_eq!(shuffled[i as usize], deck[original as usize]);
        }
    }

    #[test]
    fn test_backtrack_example4() {
        let deck = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
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
        
        let mut shuffled = run_instructions(deck.clone(), &instr);
        for i in 0..58 {
            shuffled = run_instructions(shuffled.clone(), &instr);
        }


        for i in 0..11 {
            let original = backtrack_instructions(i, 11, &instr, 59);
            assert_eq!(shuffled[i as usize], deck[original as usize]);
        }
    }

    #[test]
    fn test_power_mod() {
        assert_eq!(mod_n(4_i128.pow(6), 11), 4);
        assert_eq!(pow_mod(4, 6, 11), 4);
        assert_eq!(pow_mod(-95, 12, 11), 5);
    }

    #[test]
    fn test_div_mod() {
        assert_eq!(div_mod(8, -95, 11), 2);
        assert_eq!(div_mod(134, 8, 11), 3);
        assert_eq!(div_mod(-111, -9, 11), 5);
    }
}