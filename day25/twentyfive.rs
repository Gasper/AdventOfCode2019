
#[macro_use] extern crate text_io;

use std::fs::read;
use std::collections::HashMap;

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

    let mut input_stream = vec![].into_iter();
    let mut continue_from = Some(0);
    let mut input = vec![];
    while continue_from.is_some() {

        let (ip, output) = intcode::run_program(&mut memory, &input, continue_from.unwrap());      
        continue_from = ip;
        input.clear();

        if !output.is_empty() {
            print!("{}", *output.first().unwrap() as u8 as char);
        }
        else {
            if let Some(next_char) = input_stream.next() {
                input.push(next_char);
            }
            else {
                let line: String = read!("{}\n");
                let mut temp_input_stream: Vec<i64> = line.chars().map(|c| c as i64).collect();
                temp_input_stream.push(10);
                input_stream = temp_input_stream.into_iter();
            }
        }
    }
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
