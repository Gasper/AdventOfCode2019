extern crate itertools;

use std::fs::read;
use std::collections::HashMap;
use itertools::Itertools;

mod intcode;

const SHIP_SIZE: i64 = 100;

const NO_PULL: i64 = 0;
const PULL: i64 = 1;

#[derive(PartialEq, Eq)]
enum Mode {
    FindNewEdge,
    CheckPoints,
}

fn main() {
    let raw_input = match read("input.txt") {
        Err(_) => panic!("Can't read input.txt!"),
        Ok(file) => file,
    };

    let input_string = String::from_utf8_lossy(&raw_input);
    let input_program = get_program(input_string.to_string());

    let mut points_to_check: Vec<(i64, i64)> = vec![];
    let mut current_edge = (5, 8);
    let mut current_mode: Mode = Mode::FindNewEdge;
    
    'main_loop: loop {
        let (x, y) = current_edge;

        if current_mode == Mode::FindNewEdge {
            points_to_check = vec![(x + 1, y + 1)];
        }

        let mut memory = intcode::Memory{
            program: input_program.clone(), 
            virtual_memory: HashMap::new(),
            relative_base: 0,
        };

        let mut continue_from = Some(0);
        let next = points_to_check.pop().unwrap();
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

                if current_mode == Mode::FindNewEdge {
                    if *sensor == PULL {
                        current_edge = (x + 1, y + 1);
                    }
                    else {
                        current_edge = (x, y + 1);
                    }

                    println!("New edge: {:?}", current_edge);

                    if x < SHIP_SIZE {
                        current_mode = Mode::FindNewEdge;
                    }
                    else {
                        current_mode = Mode::CheckPoints;

                        let (new_x, new_y) = current_edge;
                        points_to_check = generate_points(new_x, new_y);
                    }
                }
                else {
                    if points_to_check.is_empty() {
                        println!("The first fit for Santa's ship is X:{}, Y: {}", x-SHIP_SIZE+1, y);
                        break 'main_loop;
                    }

                    if *sensor == NO_PULL {
                        current_mode = Mode::FindNewEdge;
                    }
                }
            }
        }
    }
}

fn generate_points(x: i64, y: i64) -> Vec<(i64, i64)> {
    let mut points = vec![];
    for y_offset in 0..SHIP_SIZE {
        points.push((x - SHIP_SIZE + 1, y + y_offset));
    }

    for x_offset in 0..(SHIP_SIZE - 1) {
        points.push((x - SHIP_SIZE + 2 + x_offset, y));
    }

    return points;
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