extern crate itertools;

use std::convert::From;
use std::fs::read;
use itertools::Itertools;

const FINISH: i64 = 99;
const ADD: i64 = 1;
const MULTIPLY: i64 = 2;
const INPUT: i64 = 3;
const OUTPUT: i64 = 4;
const JMP_TRUE: i64 = 5;
const JMP_FALSE: i64 = 6;
const LESS_THAN: i64 = 7;
const EQUALS: i64 = 8;

#[derive(PartialEq, Debug)]
enum ParameterMode {
    PositionMode,
    ImmediateMode,
}

impl From<i64> for ParameterMode {
    fn from(number: i64) -> Self {
        match number {
            0 => ParameterMode::PositionMode,
            1 => ParameterMode::ImmediateMode,
            _ => panic!("Invalid parameter mode"),
        }
    }
}

struct Instruction {
    opcode: i64,
    par1mode: ParameterMode,
    par2mode: ParameterMode,
    par3mode: ParameterMode,
}

fn main() {
    let raw_input = match read("input.txt") {
        Err(_) => panic!("Can't read input.txt!"),
        Ok(file) => file,
    };

    let input_string = String::from_utf8_lossy(&raw_input);
    let input_program = get_program(input_string.to_string());

    let mut max_signal: i64 = 0;
    for phase_sequence in (0..5).permutations(5) {
        let signal = run_amplifier_chain(&input_program, phase_sequence);

        if signal > max_signal {
            max_signal = signal;
        }
    }

    println!("Max possible signal is {}", max_signal);
}

fn run_amplifier_chain(program: &Vec<i64>, amplifier_phases: Vec<i64>) -> i64 {
    
    let mut signal_strength: i64 = 0;
    for phase in amplifier_phases {
        let input = vec![signal_strength, phase];
        let output = run_program(&program, input);

        signal_strength = *output.last().unwrap();
    }

    return signal_strength;
}

// From Day 5
fn get_program(input: String) -> Vec<i64> {
    return input.split(',').map(|c| match (*c).parse::<i64>() {
        Err(_) => panic!("Couldn't parse number {}", c),
        Ok(num) => num,
    }).collect();
}

fn run_program(program_param: &Vec<i64>, input_param: Vec<i64>) -> Vec<i64> {

    let mut program = program_param.clone();
    let mut input = input_param.clone();
    let mut output = Vec::<i64>::new();

    let mut pic: usize = 0;
    while program[pic] != FINISH {

        let instruction = parse_instruction(program[pic]);

        match instruction.opcode {
            ADD => {
                let (param1, param2) = load_params(&program, pic, instruction);

                let dest: usize = program[pic + 3] as usize;
                program[dest] = param1 + param2;
                pic += 4;
            },
            MULTIPLY => {
                let (param1, param2) = load_params(&program, pic, instruction);

                let dest: usize = program[pic + 3] as usize;
                program[dest] = param1 * param2;
                pic += 4;
            },
            INPUT => {
                let input_number: i64 = match input.pop() {
                    Some(num) => num,
                    None => panic!("Tried to read input number, but no input was available"),
                };

                let dest: usize = program[pic + 1] as usize;
                program[dest] = input_number;
                pic += 2;
            },
            OUTPUT => {
                let param1 = match instruction.par1mode {
                    ParameterMode::PositionMode => program[program[pic + 1] as usize],
                    ParameterMode::ImmediateMode => program[pic + 1],
                };

                output.push(param1);
                pic += 2;
            },
            JMP_TRUE => {
                let (param1, param2) = load_params(&program, pic, instruction);

                if param1 != 0 {
                    pic = param2 as usize;
                }
                else {
                    pic += 3;
                }
            },
            JMP_FALSE => {
                let (param1, param2) = load_params(&program, pic, instruction);

                if param1 == 0 {
                    pic = param2 as usize;
                }
                else {
                    pic += 3;
                }
            },
            LESS_THAN => {
                let (param1, param2) = load_params(&program, pic, instruction);
                let dest = program[pic + 3] as usize;

                if param1 < param2 {
                    program[dest] = 1;
                }
                else {
                    program[dest] = 0;
                }

                pic += 4;
            },
            EQUALS => {
                let (param1, param2) = load_params(&program, pic, instruction);
                let dest = program[pic + 3] as usize;

                if param1 == param2 {
                    program[dest] = 1;
                }
                else {
                    program[dest] = 0;
                }

                pic += 4;
            }
            _ => panic!("Unknown opcode: {}", instruction.opcode),
        };

    }

    return output;
}

fn parse_instruction(code: i64) -> Instruction {
    return Instruction {
        opcode: code % 100,
        par1mode: ParameterMode::from((code / 100) % 10),
        par2mode: ParameterMode::from((code / 1000) % 10),
        par3mode: ParameterMode::from((code / 10000) % 10),
    };
}

fn load_params(program: &Vec<i64>, pic: usize, instruction: Instruction) -> (i64, i64) {
    let param1 = match instruction.par1mode {
        ParameterMode::PositionMode => program[program[pic + 1] as usize],
        ParameterMode::ImmediateMode => program[pic + 1],
    };

    let param2 = match instruction.par2mode {
        ParameterMode::PositionMode => program[program[pic + 2] as usize],
        ParameterMode::ImmediateMode => program[pic + 2],
    };

    return (param1, param2);
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_example1() {
        let phases = vec![4,3,2,1,0];
        let program = vec![3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0];

        assert_eq!(run_amplifier_chain(&program, phases), 43210);
    }

    #[test]
    fn test_example2() {
        let phases = vec![0,1,2,3,4];
        let program = vec![3,23,3,24,1002,24,10,24,1002,23,-1,23,
        101,5,23,23,1,24,23,23,4,23,99,0,0];

        assert_eq!(run_amplifier_chain(&program, phases), 54321);
    }

    #[test]
    fn test_example3() {
        let phases = vec![1,0,4,3,2];
        let program = vec![3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,
        1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0];

        assert_eq!(run_amplifier_chain(&program, phases), 65210);
    }
}