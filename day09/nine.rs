use std::convert::From;
use std::fs::read;
use std::collections::HashMap;

const FINISH: i64 = 99;
const ADD: i64 = 1;
const MULTIPLY: i64 = 2;
const INPUT: i64 = 3;
const OUTPUT: i64 = 4;
const JMP_TRUE: i64 = 5;
const JMP_FALSE: i64 = 6;
const LESS_THAN: i64 = 7;
const EQUALS: i64 = 8;
const ADJUST_BASE: i64 = 9;

#[derive(PartialEq, Debug)]
enum ParameterMode {
    PositionMode,
    ImmediateMode,
    RelativeMode,
}

impl From<i64> for ParameterMode {
    fn from(number: i64) -> Self {
        match number {
            0 => ParameterMode::PositionMode,
            1 => ParameterMode::ImmediateMode,
            2 => ParameterMode:: RelativeMode,
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
    
    let diagnostic_input = vec![1];
    let (ip, output) = run_program(&input_program, &diagnostic_input, 0);
    
    if ip.is_none() {
        println!("Output was: {:?}", output);
    }
    else {
        panic!("Program was missing some input");
    }
}

fn get_program(input: String) -> Vec<i64> {
    return input.split(',').map(|c| match (*c).parse::<i64>() {
        Err(_) => panic!("Couldn't parse number {}", c),
        Ok(num) => num,
    }).collect();
}

fn run_program(program: &Vec<i64>, input_param: &Vec<i64>, ip: usize) -> (Option<usize>, Vec<i64>) {

    let mut program = program.clone();
    let mut input = input_param.clone();
    let mut output = Vec::<i64>::new();
    let mut virtual_memory = HashMap::new();

    let mut relative_base: usize = 0;
    let mut pic: usize = ip;
    while program[pic] != FINISH {

        let instruction = parse_instruction(program[pic]);

        match instruction.opcode {
            ADD => {
                let (param1, param2, dest) = load_three_params(&program, &virtual_memory, pic, relative_base, instruction);

                write_memory(&mut program, &mut virtual_memory, dest as usize, param1 + param2);
                pic += 4;
            },
            MULTIPLY => {
                let (param1, param2, dest) = load_three_params(&program, &virtual_memory, pic, relative_base, instruction);

                write_memory(&mut program, &mut virtual_memory, dest as usize, param1 * param2);
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

                let position1 = read_memory(&program, &virtual_memory, pic + 1);
                let dest = match instruction.par1mode {
                    ParameterMode::PositionMode => read_memory(&program, &virtual_memory, position1 as usize),
                    ParameterMode::ImmediateMode => position1,
                    ParameterMode::RelativeMode => (relative_base as i64 + position1),
                };

                write_memory(&mut program, &mut virtual_memory, dest as usize, input_number);
                pic += 2;
            },
            OUTPUT => {
                let param1 = load_one_param(&program, &virtual_memory, pic, relative_base, instruction);
                output.push(param1);
                pic += 2;
            },
            JMP_TRUE => {
                let (param1, param2) = load_two_params(&program, &virtual_memory, pic, relative_base, instruction);

                if param1 != 0 {
                    pic = param2 as usize;
                }
                else {
                    pic += 3;
                }
            },
            JMP_FALSE => {
                let (param1, param2) = load_two_params(&program, &virtual_memory, pic, relative_base, instruction);

                if param1 == 0 {
                    pic = param2 as usize;
                }
                else {
                    pic += 3;
                }
            },
            LESS_THAN => {
                let (param1, param2, dest) = load_three_params(&program, &virtual_memory, pic, relative_base, instruction);

                if param1 < param2 {
                    write_memory(&mut program, &mut virtual_memory, dest as usize, 1);
                }
                else {
                    write_memory(&mut program, &mut virtual_memory, dest as usize, 0);
                }

                pic += 4;
            },
            EQUALS => {
                let (param1, param2, dest) = load_three_params(&program, &virtual_memory, pic, relative_base, instruction);

                if param1 == param2 {
                    write_memory(&mut program, &mut virtual_memory, dest as usize, 1);
                }
                else {
                    write_memory(&mut program, &mut virtual_memory, dest as usize, 0);
                }

                pic += 4;
            }
            ADJUST_BASE => {
                let param1 = load_one_param(&program, &virtual_memory, pic, relative_base, instruction);
                relative_base = (relative_base as i64 + param1) as usize;
                
                pic += 2;
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

fn load_one_param(program: &Vec<i64>, virtual_memory: &HashMap<usize, i64>, pic: usize, relative_base: usize, instruction: Instruction) -> i64 {
    let position1 = read_memory(&program, &virtual_memory, pic + 1);
    return match instruction.par1mode {
        ParameterMode::PositionMode => read_memory(&program, &virtual_memory, position1 as usize),
        ParameterMode::ImmediateMode => position1,
        ParameterMode::RelativeMode => read_memory(&program, &virtual_memory, (relative_base as i64 + position1) as usize),
    };
}

fn load_two_params(program: &Vec<i64>, virtual_memory: &HashMap<usize, i64>, pic: usize, relative_base: usize, instruction: Instruction) -> (i64, i64) {
    let position1 = read_memory(&program, &virtual_memory, pic + 1);
    let param1 = match instruction.par1mode {
        ParameterMode::PositionMode => read_memory(&program, &virtual_memory, position1 as usize),
        ParameterMode::ImmediateMode => position1,
        ParameterMode::RelativeMode => read_memory(&program, &virtual_memory, (relative_base as i64 + position1) as usize),
    };

    let position2 = read_memory(&program, &virtual_memory, pic + 2);
    let param2 = match instruction.par2mode {
        ParameterMode::PositionMode => read_memory(&program, &virtual_memory, position2 as usize),
        ParameterMode::ImmediateMode => position2,
        ParameterMode::RelativeMode => read_memory(&program, &virtual_memory, (relative_base as i64 + position2) as usize),
    };

    return (param1, param2);
}

fn load_three_params(program: &Vec<i64>, virtual_memory: &HashMap<usize, i64>, pic: usize, relative_base: usize, instruction: Instruction) -> (i64, i64, i64) {
    let position1 = read_memory(&program, &virtual_memory, pic + 1);
    let param1 = match instruction.par1mode {
        ParameterMode::PositionMode => read_memory(&program, &virtual_memory, position1 as usize),
        ParameterMode::ImmediateMode => position1,
        ParameterMode::RelativeMode => read_memory(&program, &virtual_memory, (relative_base as i64 + position1) as usize),
    };

    let position2 = read_memory(&program, &virtual_memory, pic + 2);
    let param2 = match instruction.par2mode {
        ParameterMode::PositionMode => read_memory(&program, &virtual_memory, position2 as usize),
        ParameterMode::ImmediateMode => position2,
        ParameterMode::RelativeMode => read_memory(&program, &virtual_memory, (relative_base as i64 + position2) as usize),
    };

    let position3 = read_memory(&program, &virtual_memory, pic + 3);
    let param3 = match instruction.par3mode {
        ParameterMode::PositionMode => position3,
        ParameterMode::ImmediateMode => position3,
        ParameterMode::RelativeMode =>  (relative_base as i64 + position3),
    };

    return (param1, param2, param3);
}

fn read_memory(program: &Vec<i64>, virtual_memory: &HashMap<usize, i64>, location: usize) -> i64 {
    if location < program.len() {
        return program[location];
    }
    else {
        return match virtual_memory.get(&location) {
            Some(value) => *value,
            None => 0,
        };
    }
}

fn write_memory(program: &mut Vec<i64>, virtual_memory: &mut HashMap<usize, i64>, location: usize, value: i64) {

    if location < program.len() {
        program[location] = value;
    } else {
        virtual_memory.insert(location, value);
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_large_numbers() {
        let input = vec![];
        let mut program = vec![1102,34915192,34915192,7,4,7,99,0];

        assert_eq!(run_program(&mut program, &input, 0), (None, vec![1219070632396864]));
    }

    #[test]
    fn test_self_copy() {
        let input = vec![];
        let mut program = vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99];

        assert_eq!(run_program(&mut program, &input, 0), (None, vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99]));
    }

    #[test]
    
    fn test_large_number() {
        let input = vec![];
        let mut program = vec![104,1125899906842624,99];

        assert_eq!(run_program(&mut program, &input, 0), (None, vec![1125899906842624]));
    }
}