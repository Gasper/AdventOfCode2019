extern crate itertools;

use std::fs::read;
use std::collections::HashMap;
use itertools::Itertools;
use itertools::repeat_n;

mod intcode;

const SCAFFOLD: i64 = 35;
const SPACE: i64 = 46;

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
    while continue_from.is_some() {

        let (ip, output) = intcode::run_program(&mut memory, &input, continue_from.unwrap());      
        continue_from = ip;
        input.clear();

        if !output.is_empty() {
            let pixel = output[0];
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

    println!("Alignment parameters: {}", alignment_parameters(&image));
    print_image(&image);

    // Part 2
    let path = find_path(&image);
    println!("Path: {:?}", &path);

    let (pattern, a, b, c) = find_pattern(&path);
    println!("Pattern: {}", pattern);
    println!("A: {}, B: {}, C: {}", a, b, c);


    let mut memory = intcode::Memory{
        program: input_program.clone(), 
        virtual_memory: HashMap::new(),
        relative_base: 0,
    };
    memory.program[0] = 2;

    let mut continue_from = Some(0);
    let complete_input = format!("{}\n{}\n{}\n{}\nn\n", pattern, a, b, c);
    let mut input_iter: Vec<i64> = complete_input.chars().into_iter().map(|c| c as i64).rev().collect();
    let mut input: Vec<i64> = vec![];
    let mut last_output = 0;
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
            last_output = *output.first().unwrap();
        }
    }

    println!("Dust collected: {}", last_output);
}

// Finding pattern

fn find_pattern(path: &Vec<String>) -> (String, String, String, String) {
    let section_lengths = (1..path.len()/2)
        .cartesian_product(1..path.len()/2)
        .cartesian_product(1..path.len()/2);
    let path_string_full = path.join(",");

    for ((a_len, b_len), c_len) in section_lengths {

        let path_to_test = path.clone();
        let mut total_replaces = 0;

        // A
        
        let mut a_substring = vec![];
        if let Some(a_substring_avilable) = find_available(&path_to_test, a_len) {
            a_substring = a_substring_avilable;
        }
        else {
            continue;
        }

        if a_substring.join(",").len() > 20 {
            continue;
        }
        let (path_to_test, a_replaces) = remove_all(&path_to_test, &a_substring);
        total_replaces += a_replaces.len();

        // B

        let mut b_substring = vec![];
        if let Some(b_substring_avilable) = find_available(&path_to_test, b_len) {
            b_substring = b_substring_avilable;
        }
        else {
            continue;
        }

        if b_substring.join(",").len() > 20 {
            continue;
        }
        let (path_to_test, b_replaces) = remove_all(&path_to_test, &b_substring);
        total_replaces += b_replaces.len();

        // C

        let mut c_substring = vec![];
        if let Some(c_substring_avilable) = find_available(&path_to_test, c_len) {
            c_substring = c_substring_avilable;
        }
        else {
            continue;
        }

        if c_substring.join(",").len() > 20 {
            continue;
        }
        let (path_to_test, c_replaces) = remove_all(&path_to_test, &c_substring);
        total_replaces += c_replaces.len();

        if all_replaced(&path_to_test) && total_replaces < 20 {
            return (build_abc_pattern(&a_replaces, &b_replaces, &c_replaces), 
                a_substring.join(",").replace("R", "R,").replace("L", "L,"), 
                b_substring.join(",").replace("R", "R,").replace("L", "L,"), 
                c_substring.join(",").replace("R", "R,").replace("L", "L,"));
        }
    }

    panic!("No pattern found!");
}

fn build_abc_pattern(a: &Vec<usize>, b: &Vec<usize>, c: &Vec<usize>) -> String {
    let mut pattern: Vec<(usize, char)> = a.into_iter().map(|e| (*e, 'A'))
        .chain(b.into_iter().map(|e| (*e, 'B')))
        .chain(c.into_iter().map(|e| (*e, 'C')))
        .collect();
    
    pattern.sort_by_key(|e| e.0);

    return pattern.into_iter().map(|e| e.1).join(",");
}

fn remove_all(path: &Vec<String>, pattern: &Vec<String>) -> (Vec<String>, Vec<usize>) {
    let mut result = Vec::new();
    let mut i = 0;
    let mut replaces = Vec::new();
    while i < path.len() {
        if path.len() - i >= pattern.len() && path[i..(i+pattern.len())].to_vec() == *pattern {
            i += pattern.len();
            replaces.push(i);
            result.extend(repeat_n(String::from("X"), pattern.len()).into_iter());
        }
        else {
            result.push(path[i].clone());
            i += 1;
        }
    }

    return (result, replaces);
}

fn find_available(path: &Vec<String>, len: usize) -> Option<Vec<String>> {
    let mut i = 0;
    while i < path.len() - len && path[i] == String::from("X") {
        i += 1;
    }

    if i + len >= path.len() {
        return None;
    }

    let candidate = path[i..(i+len)].to_vec();
    if candidate.contains(&String::from("X")) {
        return None;
    }

    return Some(candidate);
}

fn all_replaced(path: &Vec<String>) -> bool {
    return path.into_iter().filter(|p| **p == String::from("X")).count() == path.len();
}

// Building path

fn find_path(image: &Vec<Vec<i64>>) -> Vec<String> {
    let mut path = Vec::new();
    let mut current_position = (26, 26);
    let mut current_direction = 'R';
    let mut relative_direction = 'R';
    loop {
        let (step, position, next_direction) = go_forward(image, current_position, current_direction);
        path.push(format!("{}{}", relative_direction, step));
        current_position = position;

        if let Some(direction) = next_direction {
            relative_direction = get_relative_direction(current_direction, direction);
            current_direction = direction;
        }
        else {
            break;
        }
    }

    return path;
}

fn go_forward(image: &Vec<Vec<i64>>, position: (usize, usize), direction: char) 
    -> (usize, (usize, usize), Option<char>) {
    
    let mut path_length = 0;
    let mut current_position = position;
    while let Some(new_pos) = has_next(image, current_position, direction) {
        current_position = new_pos;
        path_length += 1;
    }

    let next_dir = next_direction(image, current_position, direction);

    return (path_length, current_position, next_dir);
}

fn has_next(image: &Vec<Vec<i64>>, position: (usize, usize), direction: char) -> Option<(usize, usize)> {
    let (x, y) = position;
    match direction {
        'U' => {
            return if y > 0 && image[y-1][x] == SCAFFOLD { Some((x, y-1)) } else { None };
        },
        'D' => {
            return if y < image.len() - 1 && image[y+1][x] == SCAFFOLD { Some((x, y+1)) } else { None };
        },
        'L' => {
            return if x > 0 && image[y][x-1] == SCAFFOLD { Some((x-1, y)) } else { None };
        },
        'R' => {
            return if x < image[y].len() - 1 && image[y][x+1] == SCAFFOLD { Some((x+1, y)) } else { None };
        },
        _ => panic!("Wrong direction code"),
    }
}

fn next_direction(image: &Vec<Vec<i64>>, position: (usize, usize), direction: char) -> Option<char> {
    let (x, y) = position;
    if direction == 'U' || direction == 'D' {
        if x > 0 && image[y][x-1] == SCAFFOLD {
            return Some('L');
        }
        else if x < image[y].len() - 1 && image[y][x+1] == SCAFFOLD {
            return Some('R');
        }
        else {
            return None;
        }
    }
    else {
        if y > 0 && image[y-1][x] == SCAFFOLD {
            return Some('U');
        }
        else if y < image.len() - 1 && image[y+1][x] == SCAFFOLD {
            return Some('D');
        }
        else {
            return None;
        }
    }
}

fn get_relative_direction(current: char, next: char) -> char {
    match current {
        'U' => next,
        'L' => {
            if next == 'U' {
                return 'R';
            }
            else {
                return 'L';
            }
        },
        'R' => {
            if next == 'U' {
                return 'L';
            }
            else {
                return 'R';
            }
        },
        'D' => {
            if next == 'L' {
                return 'R';
            }
            else {
                return 'L';
            }
        },
        _ => panic!("Invalid direction"),
    }
}

fn alignment_parameters(image: &Vec<Vec<i64>>) -> u64 {
    let mut sum = 0;
    for row in 1..image.len() - 1 {
        for col in 1..image[row].len() - 1 {
            if image[row][col] == SCAFFOLD &&
               image[row-1][col] == SCAFFOLD &&
               image[row+1][col] == SCAFFOLD &&
               image[row][col+1] == SCAFFOLD &&
               image[row][col-1] == SCAFFOLD {

                sum += row*col;
            }
        }
    }

    return sum as u64;
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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_replace_vector() {
        
        let path = vec![String::from("R1"), String::from("L1"), String::from("L2"), 
            String::from("L6"), String::from("L1"), String::from("L2")];

        assert_eq!(remove_all(&path, &vec![String::from("L1"), String::from("L2")]),
            vec![String::from("R1"), String::from("L6")]);
    }
}