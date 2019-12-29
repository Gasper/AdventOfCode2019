extern crate itertools;

use std::fs::read;
use std::collections::HashMap;
use itertools::Itertools;
use itertools::repeat_n;

mod intcode;

fn main() {

    let raw_input = match read("input.txt") {
        Err(_) => panic!("Can't read input.txt!"),
        Ok(file) => file,
    };

    let input_string = String::from_utf8_lossy(&raw_input);
    let input_program = get_program(input_string.to_string());

    let mut memory = intcode::Memory{
        program: input_program.clone(), 
        virtual_memory: HashMap::new(),
        relative_base: 0,
    };

    let mut image = vec![];
    let mut current_row = vec![];

    let mut continue_from = Some(0);
    let mut input = vec![];
    let mut last_output = 0;

    let complete_input = vec!["NOT T T", "AND A T", "AND B T", "AND C T", "NOT T J", "AND D J", "OR H T", "OR E T", "AND T J", "RUN\n"];
    let complete_input: Vec<String> = complete_input.into_iter().map(|s| String::from(s)).collect();
    let mut input_iter: Vec<i64> = complete_input.join("\n").chars().into_iter().map(|c| c as i64).rev().collect();

    while continue_from.is_some() {

        let (ip, output) = intcode::run_program(&mut memory, &input, continue_from.unwrap());      
        continue_from = ip;
        input.clear();

        if output.is_empty() {
            if !input_iter.is_empty() {
                input = vec![input_iter.pop().unwrap()];
            }
        }
        else {
            let pixel = output[0];
            last_output = pixel;

            if pixel == 10 {
                image.push(current_row);
                current_row = vec![];
            }
            else {
                current_row.push(pixel);
            }
        }
    }
    image.pop();

    print_image(&image);
    println!("Hull damage: {}", last_output);
}


fn print_image(image: &Vec<Vec<i64>>) {
    for row in image {
        let row_strings: Vec<String> = row.into_iter().map(|x| (*x as u8 as char).to_string()).collect();
        println!("{}", row_strings.join(""));
    }
}

fn get_program(input: String) -> Vec<i64> {
    return input.split(',').map(|c| match (*c).parse::<i64>() {
        Err(_) => panic!("Couldn't parse number {}", c),
        Ok(num) => num,
    }).collect();
}
