use std::convert::From;
use std::fs::read;

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

    let input = vec![5];
    let output = run_program(input_program, input);

    println!("{:?}", output);
}

fn get_program(input: String) -> Vec<i64> {
    return input.split(',').map(|c| match (*c).parse::<i64>() {
        Err(_) => panic!("Couldn't parse number {}", c),
        Ok(num) => num,
    }).collect();
}

fn run_program(program_param: Vec<i64>, input_param: Vec<i64>) -> Vec<i64> {

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
mod tests {

    use super::*;

    #[test]
    fn test_parse_instruction() {
        let instr1: Instruction = parse_instruction(1002);
        assert_eq!(instr1.opcode, 2);
        assert_eq!(instr1.par1mode, ParameterMode::PositionMode);
        assert_eq!(instr1.par2mode, ParameterMode::ImmediateMode);
        assert_eq!(instr1.par3mode, ParameterMode::PositionMode);
    }

    #[test]
    fn test_example1() {
        let program = vec![3,9,8,9,10,9,4,9,99,-1,8];
        let input = vec![8];
        assert_eq!(run_program(program, input), vec![1]);
    }

    #[test]
    fn test_example1_negative() {
        let program = vec![3,9,8,9,10,9,4,9,99,-1,8];
        let input = vec![9];
        assert_eq!(run_program(program, input), vec![0]);
    }

    #[test]
    fn test_example2() {
        let program = vec![3,9,7,9,10,9,4,9,99,-1,8];
        let input = vec![7];
        assert_eq!(run_program(program, input), vec![1]);
    }

    #[test]
    fn test_example2_negative() {
        let program = vec![3,9,7,9,10,9,4,9,99,-1,8];
        let input = vec![9];
        assert_eq!(run_program(program, input), vec![0]);
    }

    #[test]
    fn test_example3() {
        let program = vec![3,3,1108,-1,8,3,4,3,99];
        let input = vec![8];
        assert_eq!(run_program(program, input), vec![1]);
    }

    #[test]
    fn test_example3_negative() {
        let program = vec![3,3,1108,-1,8,3,4,3,99];
        let input = vec![9];
        assert_eq!(run_program(program, input), vec![0]);
    }

    #[test]
    fn test_example4() {
        let program = vec![3,3,1107,-1,8,3,4,3,99];
        let input = vec![7];
        assert_eq!(run_program(program, input), vec![1]);
    }

    #[test]
    fn test_example4_negative() {
        let program = vec![3,3,1107,-1,8,3,4,3,99];
        let input = vec![9];
        assert_eq!(run_program(program, input), vec![0]);
    }

    #[test]
    fn test_example5() {
        let program = vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9];
        let input = vec![0];
        assert_eq!(run_program(program, input), vec![0]);
    }

    #[test]
    fn test_example5_negative() {
        let program = vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9];
        let input = vec![22];
        assert_eq!(run_program(program, input), vec![1]);
    }

    #[test]
    fn test_example6() {
        let program = vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1];
        let input = vec![0];
        assert_eq!(run_program(program, input), vec![0]);
    }

    #[test]
    fn test_example6_negative() {
        let program = vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1];
        let input = vec![22];
        assert_eq!(run_program(program, input), vec![1]);
    }

    #[test]
    fn test_example7_longer() {
        let program = vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
        1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
        999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99];
        let input = vec![3];
        assert_eq!(run_program(program, input), vec![999]);
    }

    #[test]
    fn test_example7_longer_1() {
        let program = vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
        1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
        999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99];
        let input = vec![7853];
        assert_eq!(run_program(program, input), vec![1001]);
    }

    #[test]
    fn test_example7_longer_2() {
        let program = vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
        1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
        999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99];
        let input = vec![8];
        assert_eq!(run_program(program, input), vec![1000]);
    }
}