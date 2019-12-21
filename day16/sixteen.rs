extern crate itertools;

use std::fs::read;
use itertools::{join, repeat_n};

fn main() {
    let raw_input = match read("input.txt") {
        Err(_) => panic!("Can't read input.txt!"),
        Ok(file) => file,
    };

    let input_string = String::from_utf8_lossy(&raw_input);
    let input_digits = &parse_input(String::from(input_string));

    let big_vector: Vec<i64> = input_digits.into_iter()
        .cycle()
        .take(input_digits.len() * 10000)
        .map(|digit| *digit)
        .collect();
    
    let iteration_result = calculate_n_iterations(&big_vector, 100);
    let first_seven: usize = join(&input_digits[0..7], "").parse().unwrap();   
        
    println!("Eight numbers at {}, are: {:?}", first_seven, iteration_result[(first_seven)..(first_seven + 8)].to_vec());
    
}

fn calculate_n_iterations(input: &Vec<i64>, n: i64) -> Vec<i64> {
    let mut current_input = input.clone();
    for _ in 0..n {
        current_input = calculate_iteration(&current_input);
    }

    return current_input.clone();
}


fn calculate_iteration(input: &Vec<i64>) -> Vec<i64> {
    let mut output = repeat_n(0, input.len()).collect();
    calculate_bottom(input, &mut output);
    return output;
}

fn calculate_bottom(input: &Vec<i64>, output: &mut Vec<i64>) {
    let middle_point = input.len() / 2;

    output[input.len() - 1] = input[input.len() - 1];
    for (i, num) in input.into_iter().rev().enumerate().take(middle_point).skip(1) {
        output[input.len() - 1 - i] = ((output[input.len() - i] + num) % 10).abs();
    }

}

fn parse_input(input: String) -> Vec<i64> {
    return input.chars()
        .map(|num| num as i64 - '0' as i64)
        .collect();
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_parse_input() {
        assert_eq!(parse_input(String::from("5434534")), vec![5,4,3,4,5,3,4]);
    }

    #[test]
    fn test_iterator() {
        let nums: Vec<i64> = get_pattern(1, vec![0, 1, 0, -1]).take(8).collect();
        assert_eq!(nums, vec![1, 0, -1, 0, 1, 0, -1, 0]);

        let nums: Vec<i64> = get_pattern(2, vec![0, 1, 0, -1]).take(8).collect();
        assert_eq!(nums, vec![1, 1, 0, 0, -1, -1, 0, 0]);

        let nums: Vec<i64> = get_pattern(3, vec![0, 1, 0, -1]).take(8).collect();
        assert_eq!(nums, vec![1, 1, 1, 0, 0, 0, -1, -1]);

        let nums: Vec<i64> = get_pattern(4, vec![0, 1, 0, -1]).take(8).collect();
        assert_eq!(nums, vec![1, 1, 1, 1, 0, 0, 0, 0]);
    }

    #[test]
    fn test_calculate_iteration() {
        let input1 = vec![1,2,3,4,5,6,7,8];
        assert_eq!(calculate_iteration(&input1)[4..8].to_vec(), vec![6i64,1,5,8]);

        let input2 = vec![4,8,2,2,6,1,5,8];
        assert_eq!(calculate_iteration(&input2)[4..8].to_vec(), vec![0i64,4,3,8]);

        let input3 = vec![3,4,0,4,0,4,3,8];
        assert_eq!(calculate_iteration(&input3)[4..8].to_vec(), vec![5i64,5,1,8]);

        let input3 = vec![0,3,4,1,5,5,1,8];
        assert_eq!(calculate_iteration(&input3)[4..8].to_vec(), vec![9i64,4,9,8]);
    }

    #[test]
    #[ignore]
    fn test_example1() {
        let input1 = parse_input(String::from("80871224585914546619083218645595"));
        let after_100 = calculate_n_iterations(&input1, 100);
        
        assert_eq!(after_100[0..8].to_vec(), vec![2,4,1,7,6,1,7,6]);
    }

    #[test]
    #[ignore]
    fn test_example2() {
        let input1 = parse_input(String::from("19617804207202209144916044189917"));
        let after_100 = calculate_n_iterations(&input1, 100);
        
        assert_eq!(after_100[0..8].to_vec(), vec![7,3,7,4,5,4,1,8]);
    }

    #[test]
    #[ignore]
    fn test_example3() {
        let input1 = parse_input(String::from("69317163492948606335995924319873"));
        let after_100 = calculate_n_iterations(&input1, 100);
        
        assert_eq!(after_100[0..8].to_vec(), vec![5,2,4,3,2,1,3,3]);
    }

    #[test]
    fn test_into_number() {
        let digits = vec![0,3,0,3,6,7,3,2,5,7,7,2];
        assert_eq!(join(&digits[0..7], ""), String::from("0303673"));
    }

    #[test]
    fn test_build_bottom() {
        let input3 = vec![3,4,0,4,0,4,3,8];
        let mut output = vec![0,0,0,0,0,0,0,0];

        calculate_bottom(&input3, &mut output);

        assert_eq!(output, vec![0,0,0,0,5,5,1,8]);
    }

    #[test]
    fn test_build_top() {
        let input3 = vec![3,4,0,4,0,4,3,8];
        let mut output = vec![0,0,0,0,0,0,0,0];

        calculate_top(&input3, &mut output);

        assert_eq!(output, vec![0,3,4,0,0,0,0,0]);
    }

    #[test]
    fn test_example_2_1() {
        let inp = &vec![0,3,0,3,6,7,3,2,5,7,7,2,1,2,9,4,4,0,6,3,4,9,1,5,6,5,4,7,4,6,6,4];
        let big_vector: Vec<i64> = inp.into_iter()
            .cycle()
            .take(inp.len() * 10000)
            .map(|digit| *digit)
            .collect();

        let c = calculate_n_iterations(&big_vector, 100);
        let first_seven: usize = join(&inp[0..7], "").parse().unwrap();   
        
        assert_eq!(c[(first_seven)..(first_seven + 8)].to_vec(), vec![8,4,4,6,2,0,2,6]);
    }

    #[test]
    fn test_example_2_2() {
        let inp = &vec![0,2,9,3,5,1,0,9,6,9,9,9,4,0,8,0,7,4,0,7,5,8,5,4,4,7,0,3,4,3,2,3];
        let big_vector: Vec<i64> = inp.into_iter()
            .cycle()
            .take(inp.len() * 10000)
            .map(|digit| *digit)
            .collect();

        let c = calculate_n_iterations(&big_vector, 100);
        let first_seven: usize = join(&inp[0..7], "").parse().unwrap();   
        
        assert_eq!(c[(first_seven)..(first_seven + 8)].to_vec(), vec![7,8,7,2,5,2,7,0]);
    }

    #[test]
    fn test_example_2_3() {
        let inp = &vec![0,3,0,8,1,7,7,0,8,8,4,9,2,1,9,5,9,7,3,1,1,6,5,4,4,6,8,5,0,5,1,7];
        let big_vector: Vec<i64> = inp.into_iter()
            .cycle()
            .take(inp.len() * 10000)
            .map(|digit| *digit)
            .collect();

        let c = calculate_n_iterations(&big_vector, 100);
        let first_seven: usize = join(&inp[0..7], "").parse().unwrap();   
        
        assert_eq!(c[(first_seven)..(first_seven + 8)].to_vec(), vec![5,3,5,5,3,7,3,1]);
    }
}