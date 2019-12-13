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

struct Memory {
    program: Vec<i64>,
    virtual_memory: HashMap<usize, i64>,
    relative_base: usize,
}

// Arcade

const EMPTY: i64 = 0;
const WALL: i64 = 1;
const BLOCK: i64 = 2;
const PADDLE: i64 = 3; 
const BALL: i64 = 4;

struct Arcade {
    painted_positions: HashMap<(i64, i64), i64>,
    score: i64,
    ball_x: i64,
    ball_y: i64,
    paddle_x: i64,
    memory: Memory,
}

fn main() {
    let raw_input = match read("input.txt") {
        Err(_) => panic!("Can't read input.txt!"),
        Ok(file) => file,
    };

    let input_string = String::from_utf8_lossy(&raw_input);
    let input_program = get_program(input_string.to_string());
    
    let mut arcade = Arcade {
        painted_positions: HashMap::new(),
        score: 0, 
        ball_x: 0,
        ball_y: 0,
        paddle_x: 0,
        memory: Memory{
            program: input_program.clone(), 
            virtual_memory: HashMap::new(),
            relative_base: 0,
        },
    };

    // Insert coin
    arcade.memory.write_memory(0, 2);

    let mut continue_from = Some(0);
    let mut input = vec![];
    let mut ouput_tile = vec![];
    while continue_from.is_some() {

        let (ip, output) = run_program(&mut arcade.memory, &input, continue_from.unwrap());
        
        continue_from = ip;

        if continue_from.is_some() {
            input = vec![];
            if output.is_empty() {
                // Program stopped and there is no output: 
                if arcade.ball_x < arcade.paddle_x {
                    input = vec![-1];
                } else if arcade.ball_x > arcade.paddle_x {
                    input = vec![1];
                } else {
                    input = vec![0];
                }
            }
            else if ouput_tile.len() < 2 {
                ouput_tile.push(output[0]);
            }
            else if ouput_tile.len() == 2 {
                arcade.process_output(ouput_tile[0], ouput_tile[1], output[0]);
                ouput_tile = vec![];
            }
        }
    }
    
    println!("SCORE: {}", arcade.score);

    paint_screen(arcade.painted_positions);
}

impl Arcade {
    fn process_output(&mut self, x: i64, y: i64, tile_type: i64) {
        if x == -1 && y == 0 {
            self.score = tile_type;
        }
        else {
            self.painted_positions.insert((x, y), tile_type);

            if tile_type == BALL {
                self.ball_x = x;
                self.ball_y = y;
            }
            else if tile_type == PADDLE {
                self.paddle_x = x;
            }
        }
    }

    fn count_block_tiles(&self) -> i64 {
        return self.painted_positions.values().filter(|tile| **tile == BLOCK).count() as i64;
    }
}

fn get_program(input: String) -> Vec<i64> {
    return input.split(',').map(|c| match (*c).parse::<i64>() {
        Err(_) => panic!("Couldn't parse number {}", c),
        Ok(num) => num,
    }).collect();
}

fn run_program(memory: &mut Memory, input_param: &Vec<i64>, ip: usize) -> (Option<usize>, Vec<i64>) {

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

fn paint_screen(painted: HashMap<(i64, i64), i64>) {
    const cols: usize = 45;
    const rows: usize = 23;
    let mut big_field: [char; rows*cols] = [' '; rows*cols];
    for (location, color) in &painted {
        let y = location.1;
        let x = location.0;
        if *color != EMPTY {
            big_field[((y  * cols as i64) + (x as i64)) as usize] = '#';
        }
        else {
            big_field[((y  * cols as i64) + (x as i64)) as usize] = '.';
        }
    }

    for i in 0..rows {
        let a: Vec<char> = (big_field[(i*cols)..((i+1)*cols)]).to_vec();
        let s: Vec<String> = a.into_iter().map(|x| x.to_string()).collect();
        println!("{:?}", s.join(""));
    }
}

#[cfg(test)]
mod test {

    use super::*;
}