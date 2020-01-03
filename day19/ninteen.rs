extern crate itertools;

use std::fs::read;
use std::collections::HashMap;
use itertools::Itertools;

mod intcode;

fn main() {
    let raw_input = match read("input.txt") {
        Err(_) => panic!("Can't read input.txt!"),
        Ok(file) => file,
    };

    let input_string = String::from_utf8_lossy(&raw_input);
    let input_program = get_program(input_string.to_string());

    let mut beam_detection = HashMap::new();
    let mut all_positions: Vec<(i64, i64)> = (0..100).cartesian_product(0..100).collect();

    let mut count: i64 = 0;
    while !all_positions.is_empty() {

        let mut memory = intcode::Memory{
            program: input_program.clone(), 
            virtual_memory: HashMap::new(),
            relative_base: 0,
        };

        let mut continue_from = Some(0);
        let next = all_positions.pop().unwrap();
        let mut input = vec![next.1, next.0];

        while continue_from.is_some() {

            let (ip, output) = intcode::run_program(&mut memory, &input, continue_from.unwrap());      
            continue_from = ip;

            if output.is_empty() {
                input.pop();
            }
            else {
                assert_eq!(output.len(), 1);
                let sensor = output.first().unwrap();
                beam_detection.insert(next, *sensor);
                count += sensor;
            }
        }
    }

    println!("Detected {} positions with beam", count);
    paint_screen(beam_detection);
}

fn get_program(input: String) -> Vec<i64> {
    return input.split(',').map(|c| match (*c).parse::<i64>() {
        Err(_) => panic!("Couldn't parse number {}", c),
        Ok(num) => num,
    }).collect();
}

fn paint_screen(tiles: HashMap<(i64, i64), i64>) -> Vec<String> {
    const cols: usize = 100;
    const rows: usize = 100;
    let mut big_field: [char; rows*cols] = [' '; rows*cols];
    for (location, color) in &tiles {
        let y = location.1;
        let x = location.0;

        big_field[((y  * cols as i64) + (x as i64)) as usize] = match *color {
            0 => '.',
            1 => '#',
            _ => '?',
        };
    }

    let mut row_strings = vec![];
    for i in 0..rows {
        let a: Vec<char> = (big_field[(i*cols)..((i+1)*cols)]).to_vec();
        let s: Vec<String> = a.into_iter().map(|x| x.to_string()).collect();
        let row_string = s.join("");
        println!("{:?}", row_string);
        row_strings.push(row_string);
    }

    return row_strings;
}