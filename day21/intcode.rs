use std::collections::HashMap;

pub const FINISH: i64 = 99;
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

pub struct Memory {
    pub program: Vec<i64>,
    pub virtual_memory: HashMap<usize, i64>,
    pub relative_base: usize,
}

pub fn run_program(memory: &mut Memory, input_param: &Vec<i64>, ip: usize) -> (Option<usize>, Vec<i64>) {

    let mut input = input_param.clone();

    let mut pic: usize = ip;
    while memory.program[pic] != FINISH {

        let instruction = parse_instruction(memory.program[pic]);

        match instruction.opcode {
            ADD => {
                let (param1, param2, dest) = memory.load_three_params(pic, memory.relative_base, instruction);

                memory.write_memory(dest as usize, param1 + param2);
                pic += 4;
            },
            MULTIPLY => {
                let (param1, param2, dest) = memory.load_three_params(pic, memory.relative_base, instruction);

                memory.write_memory(dest as usize, param1 * param2);
                pic += 4;
            },
            INPUT => {
                let input_number: i64 = match input.pop() {
                    Some(num) => num,
                    None => {
                        // If there is no input available, switch to different program
                        return (Some(pic), vec![]);
                    },
                };

                let position1 = memory.read_memory(pic + 1);
                let dest = match instruction.par1mode {
                    ParameterMode::PositionMode => position1,
                    ParameterMode::ImmediateMode => position1,
                    ParameterMode::RelativeMode => (memory.relative_base as i64 + position1),
                };

                memory.write_memory(dest as usize, input_number);
                pic += 2;
            },
            OUTPUT => {
                let param1 = memory.load_one_param(pic, memory.relative_base, instruction);
                pic += 2;

                return (Some(pic), vec![param1]);
            },
            JMP_TRUE => {
                let (param1, param2) = memory.load_two_params(pic, memory.relative_base, instruction);

                if param1 != 0 {
                    pic = param2 as usize;
                }
                else {
                    pic += 3;
                }
            },
            JMP_FALSE => {
                let (param1, param2) = memory.load_two_params(pic, memory.relative_base, instruction);

                if param1 == 0 {
                    pic = param2 as usize;
                }
                else {
                    pic += 3;
                }
            },
            LESS_THAN => {
                let (param1, param2, dest) = memory.load_three_params(pic, memory.relative_base, instruction);

                if param1 < param2 {
                    memory.write_memory(dest as usize, 1);
                }
                else {
                    memory.write_memory(dest as usize, 0);
                }

                pic += 4;
            },
            EQUALS => {
                let (param1, param2, dest) = memory.load_three_params(pic, memory.relative_base, instruction);

                if param1 == param2 {
                    memory.write_memory(dest as usize, 1);
                }
                else {
                    memory.write_memory(dest as usize, 0);
                }

                pic += 4;
            }
            ADJUST_BASE => {
                let param1 = memory.load_one_param(pic, memory.relative_base, instruction);
                memory.relative_base = (memory.relative_base as i64 + param1) as usize;
                
                pic += 2;
            }
            _ => panic!("Unknown opcode: {}", instruction.opcode),
        };

    }

    return (None, vec![]);
}

fn parse_instruction(code: i64) -> Instruction {
    return Instruction {
        opcode: code % 100,
        par1mode: ParameterMode::from((code / 100) % 10),
        par2mode: ParameterMode::from((code / 1000) % 10),
        par3mode: ParameterMode::from((code / 10000) % 10),
    };
}

impl Memory {
    fn read_memory(&self, location: usize) -> i64 {
        if location < self.program.len() {
            return self.program[location];
        }
        else {
            return match self.virtual_memory.get(&location) {
                Some(value) => *value,
                None => 0,
            };
        }
    }

    fn write_memory(&mut self, location: usize, value: i64) {

        if location < self.program.len() {
            self.program[location] = value;
        } else {
            self.virtual_memory.insert(location, value);
        }
    }

    fn load_one_param(&self, pic: usize, relative_base: usize, instruction: Instruction) -> i64 {
        let position1 = self.read_memory(pic + 1);
        return match instruction.par1mode {
            ParameterMode::PositionMode => self.read_memory(position1 as usize),
            ParameterMode::ImmediateMode => position1,
            ParameterMode::RelativeMode => self.read_memory((relative_base as i64 + position1) as usize),
        };
    }
    
    fn load_two_params(&self, pic: usize, relative_base: usize, instruction: Instruction) -> (i64, i64) {
        let position1 = self.read_memory(pic + 1);
        let param1 = match instruction.par1mode {
            ParameterMode::PositionMode => self.read_memory(position1 as usize),
            ParameterMode::ImmediateMode => position1,
            ParameterMode::RelativeMode => self.read_memory((relative_base as i64 + position1) as usize),
        };
    
        let position2 = self.read_memory(pic + 2);
        let param2 = match instruction.par2mode {
            ParameterMode::PositionMode => self.read_memory(position2 as usize),
            ParameterMode::ImmediateMode => position2,
            ParameterMode::RelativeMode => self.read_memory((relative_base as i64 + position2) as usize),
        };
    
        return (param1, param2);
    }
    
    fn load_three_params(&self, pic: usize, relative_base: usize, instruction: Instruction) -> (i64, i64, i64) {
        let position1 = self.read_memory(pic + 1);
        let param1 = match instruction.par1mode {
            ParameterMode::PositionMode => self.read_memory(position1 as usize),
            ParameterMode::ImmediateMode => position1,
            ParameterMode::RelativeMode => self.read_memory((relative_base as i64 + position1) as usize),
        };
    
        let position2 = self.read_memory(pic + 2);
        let param2 = match instruction.par2mode {
            ParameterMode::PositionMode => self.read_memory(position2 as usize),
            ParameterMode::ImmediateMode => position2,
            ParameterMode::RelativeMode => self.read_memory((relative_base as i64 + position2) as usize),
        };
    
        let position3 = self.read_memory(pic + 3);
        let param3 = match instruction.par3mode {
            ParameterMode::PositionMode => position3,
            ParameterMode::ImmediateMode => position3,
            ParameterMode::RelativeMode =>  (relative_base as i64 + position3),
        };
    
        return (param1, param2, param3);
    }
}