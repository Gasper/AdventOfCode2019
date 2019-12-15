use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;

fn main() {

    let input_file = match File::open("input.txt") {

        Err(why) => panic!("Could not open file: {}", why.description()),
        
        Ok(file) => file,
    };

    let mut modules_mass = Vec::new();

    for line in BufReader::new(input_file).lines() {
        if let Ok(mass_string) = line {
            
            let mass: f64 = match mass_string.parse() {
                Err(_) => panic!("{} is not an unsigned number", mass_string),
                Ok(float_mass) => float_mass,
            };

            modules_mass.push(mass);
        }
    }

    let mut total_fuel = 0f64;
    for module_mass in modules_mass.iter() {
        total_fuel = total_fuel + total_for_module(*module_mass);
    }

    println!("Total fuel required: {}", total_fuel);
}

fn calculate_module_fuel(module_mass: f64) -> f64 {
    return (module_mass / 3.0).floor() - 2.0
}

fn calculate_extra_fuel(total_fuel: f64) -> f64 {
    let mut total_extra_fuel = 0f64;
    let mut extra_fuel = total_fuel;
    while extra_fuel >= 0.0 {
        extra_fuel = (extra_fuel / 3.0).floor() - 2.0;

        if extra_fuel > 0.0 {
            total_extra_fuel += extra_fuel;
        }
    }

    return total_extra_fuel;
}

fn total_for_module(mass: f64) -> f64 {
    let fuel = calculate_module_fuel(mass);
    let extra = calculate_extra_fuel(fuel);
    return fuel + extra;
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_extra_fuel() {
        assert_eq!(calculate_extra_fuel(654.0), 312.0);
    }

    #[test]
    fn test_example1() {
        assert_eq!(total_for_module(14.0), 2.0)
    }

    #[test]
    fn test_example2() {
        assert_eq!(total_for_module(1969.0), 966.0)
    }

    #[test]
    fn test_example3() {
        assert_eq!(total_for_module(100756.0), 50346.0)
    }
}