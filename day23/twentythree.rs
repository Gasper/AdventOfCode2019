use std::fs::read;
use std::collections::{HashMap, VecDeque};

mod intcode;

const NO_DATA: i64 = -1;

struct Computer {
    memory: intcode::Memory,
    address: i64,
    input_queue: VecDeque<i64>,
    input: Vec<i64>,
    output_queue: VecDeque<i64>,
    ip: Option<usize>,
    is_idle: bool,
}

fn main() {

    let raw_input = match read("input.txt") {
        Err(_) => panic!("Can't read input.txt!"),
        Ok(file) => file,
    };

    let input_string = String::from_utf8_lossy(&raw_input);
    let input_program = get_program(input_string.to_string());

    let mut network: Vec<Computer> = vec![];
    for i in 0..50 {
        let computer = Computer {
            memory: intcode::Memory {
                program: input_program.clone(), 
                virtual_memory: HashMap::new(),
                relative_base: 0,
            },
            address: i,
            input_queue: VecDeque::new(),
            input: vec![i],
            output_queue: VecDeque::new(),
            ip: Some(0),
            is_idle: false,
        };

        network.push(computer);
    }

    let mut nat_delivered_y = 0;
    let mut nat_x = 0;
    let mut nat_y = 0;
    let mut current_pc = 0;
    loop {
        let mut computer = &mut network[current_pc];
        let (ip, output) = intcode::run_program(&mut computer.memory, &computer.input, computer.ip.unwrap());
        computer.ip = ip;
        computer.input.clear();

        if output.is_empty() {
            let next_input_value = computer.read_input();
            computer.input.push(next_input_value);

            if next_input_value == NO_DATA {
                computer.is_idle = true;
            }
        }
        else {
            assert_eq!(output.len(), 1);
            computer.output_queue.push_back(*output.first().unwrap());
        }

        if computer.output_queue.len() == 3 {
            let destination = computer.output_queue.pop_front().unwrap();
            let x = computer.output_queue.pop_front().unwrap();
            let y = computer.output_queue.pop_front().unwrap();

            if destination == 255 {
                nat_x = x;
                nat_y = y;
            }
            else {
                let destination_computer = &mut network[destination as usize];
                destination_computer.input_queue.push_back(x);
                destination_computer.input_queue.push_back(y);
                destination_computer.is_idle = false;
                println!("Sending X:{} Y:{} to {}", x, y, destination);
            }
        }

        let computers_idle: bool = (&network).into_iter().map(|c| c.is_idle).fold(true, |a, b| a && b);
        if computers_idle && nat_y != 0 {
            let computer_zero = &mut network[0];
            computer_zero.input_queue.push_back(nat_x);
            computer_zero.input_queue.push_back(nat_y);

            if nat_y == nat_delivered_y {
                println!("NAT delivered Y {} twice!", nat_y);
                break;
            }

            nat_delivered_y = nat_y;

            for computer in network.iter_mut() {
                (*computer).is_idle = false;
            }
        }
 
        current_pc = (current_pc + 1) % network.len();
    }
}

impl Computer {
    fn read_input(&mut self) -> i64 {
        if self.input_queue.len() > 0 {
            return self.input_queue.pop_front().unwrap();
        }
        else {
            return NO_DATA;
        }
    }
}

fn get_program(input: String) -> Vec<i64> {
    return input.split(',').map(|c| match (*c).parse::<i64>() {
        Err(_) => panic!("Couldn't parse number {}", c),
        Ok(num) => num,
    }).collect();
}