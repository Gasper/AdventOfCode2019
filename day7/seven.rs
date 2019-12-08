extern crate itertools;

use std::convert::From;
use std::fs::read;
use itertools::Itertools;
use itertools::concat;

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

struct Amplifier {
    memory: Vec<i64>,
    instruction_pointer: usize,
    input: Vec<i64>,
}

fn main() {
    let raw_input = match read("input.txt") {
        Err(_) => panic!("Can't read input.txt!"),
        Ok(file) => file,
    };

    let input_string = String::from_utf8_lossy(&raw_input);
    let input_program = get_program(input_string.to_string());

    let mut max_signal: i64 = 0;
    for phase_sequence in (5..10).permutations(5) {
        let signal = run_amplifier_chain(&input_program, phase_sequence);

        if signal > max_signal {
            max_signal = signal;
        }
    }

    println!("Max possible signal is {}", max_signal);
}

fn run_amplifier_chain(program: &Vec<i64>, amplifier_phases: Vec<i64>) -> i64 {
    
    let mut amplifiers: [Amplifier; 5] = [
        Amplifier{memory: program.clone(), instruction_pointer: 0, input: vec![0, amplifier_phases[0]]},
        Amplifier{memory: program.clone(), instruction_pointer: 0, input: vec![amplifier_phases[1]]},
        Amplifier{memory: program.clone(), instruction_pointer: 0, input: vec![amplifier_phases[2]]},
        Amplifier{memory: program.clone(), instruction_pointer: 0, input: vec![amplifier_phases[3]]},
        Amplifier{memory: program.clone(), instruction_pointer: 0, input: vec![amplifier_phases[4]]},
    ];

    let mut finished: bool = false;
    let mut next_program: usize = 0;
    while !finished {
        let current_program = next_program;
        next_program = (next_program + 1) % amplifiers.len();

        let current_amplifier = &mut amplifiers[current_program];
        let (pic, output) = run_program(&mut current_amplifier.memory, 
                &current_amplifier.input, current_amplifier.instruction_pointer);

        let new_input = vec![output, amplifiers[next_program].input.clone()];
        amplifiers[next_program].input = concat(new_input);

        match pic {
            Some(pointer) => {
                amplifiers[current_program].instruction_pointer = pointer;
                amplifiers[current_program].input = vec![];
            },
            None => {
                if current_program == 4 {
                    finished = true;
                }
            }
        }
    }

    return amplifiers[0].input[0];
}

fn get_program(input: String) -> Vec<i64> {
    return input.split(',').map(|c| match (*c).parse::<i64>() {
        Err(_) => panic!("Couldn't parse number {}", c),
        Ok(num) => num,
    }).collect();
}

fn run_program(program: &mut Vec<i64>, input_param: &Vec<i64>, ip: usize) -> (Option<usize>, Vec<i64>) {

    let mut input = input_param.clone();
    let mut output = Vec::<i64>::new();

    let mut pic: usize = ip;
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
                    None => {
                        // If there is no input available, switch to different program
                        return (Some(pic), output);
                    },
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

    return (None, output);
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
        let phases = vec![9,8,7,6,5];
        let program = vec![3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,
        27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5];

        assert_eq!(run_amplifier_chain(&program, phases), 139629729);
    }

    #[test]
    fn test_example2() {
        let phases = vec![9,7,8,5,6];
        let program = vec![3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,
        -5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,
        53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10];

        assert_eq!(run_amplifier_chain(&program, phases), 18216);
    }
}