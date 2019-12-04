use std::fs::read;

const FINISH: u64 = 99;
const ADD: u64 = 1;
const MULTIPLY: u64 = 2;

fn main() {
    let raw_input = match read("input.txt") {
        Err(_) => panic!("Can't read input.txt!"),
        Ok(file) => file,
    };

    let input = String::from_utf8_lossy(&raw_input);
    let input_program = get_program(input.to_string());

    'outer: for noun in 0..100 {
        for verb in 0..100 {
            let mut program = input_program.clone();
            program[1] = noun;
            program[2] = verb;

            let program = run_program(program);

            if program[0] == 19690720 {
                println!("Noun: {}, Verb: {}", noun, verb);
                println!("Result: {}", 100 * noun + verb);
                break 'outer;
            }
        }
    }
}

fn get_program(input: String) -> Vec<u64> {
    return input.split(',').map(|c| match (*c).parse::<u64>() {
        Err(_) => panic!("Couldn't parse number {}", c),
        Ok(num) => num,
    }).collect();
}

fn run_program(program_param: Vec<u64>) -> Vec<u64> {

    let mut program = program_param.clone();
    let mut pic = 0;
    while program[pic] != FINISH {
        let op1_pos = program[pic + 1] as usize;
        let op2_pos = program[pic + 2] as usize;
        let result_pos = program[pic + 3] as usize;

        program[result_pos] = match program[pic] {
            ADD => program[op1_pos] + program[op2_pos],
            MULTIPLY => program[op1_pos] * program[op2_pos],
            _ => panic!("Invalid command: {}", program[pic])
        };

        pic += 4;
    }

    return program;
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example1() {
        let program = vec![1, 0, 0, 0, 99];
        let expected = vec![2, 0, 0, 0, 99];
        
        assert_eq!(run_program(program), expected);
    }

    #[test]
    fn test_example2() {
        let program = vec![2,3,0,3,99];
        let expected = vec![2,3,0,6,99];
        
        assert_eq!(run_program(program), expected);
    }

    #[test]
    fn test_example3() {
        let program = vec![2,4,4,5,99,0];
        let expected = vec![2,4,4,5,99,9801];
        
        assert_eq!(run_program(program), expected);
    }

    #[test]
    fn test_example4() {
        let program = vec![1,1,1,4,99,5,6,0,99];
        let expected = vec![30,1,1,4,2,5,6,0,99];
        
        assert_eq!(run_program(program), expected);
    }

    #[test]
    fn test_get_program() {
        assert_eq!(get_program("1,1,1,4,99,5,6,0,99".to_string()), vec![1,1,1,4,99,5,6,0,99]);
        assert_eq!(get_program("1,1,1,4,99".to_string()), vec![1,1,1,4,99]);
    }

    #[test]
    #[should_panic]
    fn test_get_program_panic_float() {
        get_program("1,1,1,4,99,5,6,0.45,99".to_string());
    }

    #[test]
    #[should_panic]
    fn test_get_program_panic_char() {
        get_program("1,1,1,4,99,5,6,dg,99".to_string());
    }
}