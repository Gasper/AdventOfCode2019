
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

struct Reaction {
    product: String,
    quantity: u64,
    inputs: HashMap<String, u64>,
}

struct NanoFactory {
    reactions: HashMap<String, Reaction>,
    surplus: HashMap<String, u64>,
}

fn main() {
    let input_file = match File::open("input.txt") {
        Err(_) => panic!("Could not open input.txt"),
        Ok(file) => file,
    };

    let recipe: Vec<String> = BufReader::new(input_file)
        .lines()
        .map(|line| line.unwrap())
        .collect();

    let mut factory = NanoFactory {
        reactions: HashMap::new(),
        surplus: HashMap::new(),
    };

    for reaction_line in recipe {
        let reaction = parse_reaction(reaction_line);
        factory.reactions.insert(reaction.product.clone(), reaction);
    }

    let required_ore = factory.ore_required_for(String::from("FUEL"), 1);
    println!("Required ORE: {}", required_ore);

    let fuel_for_trillion = factory.fuel_from_ore(1000000000000);
    println!("For trillion ORE pieces we can make: {} FUEL", fuel_for_trillion);
}

impl NanoFactory {
    fn ore_required_for(&mut self, product: String, quantity: u64) -> u64 {
        let simple_materials = self.materials_required_for(product, quantity);

        let mut total_ore = 0;
        for (mat, qty) in simple_materials.iter() {
            let simple_mat_reaction = self.reactions.get(mat).unwrap();
            let (simple_mat_inputs, _surplus) = simple_mat_reaction.inputs_required_for(*qty);

            total_ore += simple_mat_inputs.get("ORE").unwrap();
        }

        self.surplus = HashMap::new();
        return total_ore;
    }

    fn fuel_from_ore(&mut self, available_ore: u64) -> u64 {
        
        let single_fuel = self.ore_required_for(String::from("FUEL"), 1);
        let mut upper_limit = available_ore;
        let mut lower_limit: u64 = 1;

        let mut fuel_possible = 0;
        loop {
            fuel_possible = ((upper_limit + lower_limit) as f64 / 2.0).floor() as u64;
            let ore_needed = self.ore_required_for(String::from("FUEL"), fuel_possible);

            if ore_needed <= available_ore && ore_needed + single_fuel > available_ore {
                break;
            }
            else if ore_needed > available_ore {
                upper_limit = fuel_possible;
            }
            else {
                lower_limit = fuel_possible + 1;
            }
        }

        return fuel_possible;
    }

    fn materials_required_for(&mut self, product: String, quantity: u64) -> HashMap<String, u64> {

        let avilable_surplus = self.take_from_surplus(&product, quantity);
        let product_reaction = self.reactions.get(&product).unwrap();
        let (inputs_required, surplus) = product_reaction.inputs_required_for(quantity - avilable_surplus);
        self.add_surplus(&product, surplus);

        let mut total: HashMap<String, u64> = HashMap::new();

        for (input, input_quantity) in inputs_required.iter() {
            if input == "ORE" {
                total.insert(product.clone(), quantity);
                continue;
            }
            
            let subinputs = self.materials_required_for((*input).clone(), *input_quantity);
            for (subinput, subinput_qty) in subinputs {
                if total.contains_key(&subinput) {
                    let updated_qty = total.get(&subinput).unwrap() + subinput_qty;
                    total.insert(subinput.clone(), updated_qty);
                }
                else {
                    total.insert(subinput.clone(), subinput_qty);
                }
            }
        }

        return total;
    }

    fn add_surplus(&mut self, material: &String, surplus_quantity: u64) {
        if self.surplus.contains_key(material) {
            let updated_qty = self.surplus.get(material).unwrap() + surplus_quantity;
            self.surplus.insert(material.clone(), updated_qty);
        }
        else {
            self.surplus.insert(material.clone(), surplus_quantity);
        } 
    }

    fn take_from_surplus(&mut self, material: &String, needed: u64) -> u64 {
        return match self.surplus.get_mut(material) {
            Some(qty) => {
                if *qty >= needed {
                    *qty = *qty - needed;
                    return needed;
                }
                else {
                    let available = *qty;
                    *qty = 0;
                    return available;
                }
            },
            None => 0,
        };
    }
}

impl Reaction {
    fn inputs_required_for(&self, output_quantity: u64) -> (HashMap<String, u64>, u64) {
        let factor = (output_quantity as f64 / self.quantity as f64).ceil() as u64;
        let surplus = (self.quantity * factor) - output_quantity;

        let mut required_inputs = HashMap::new();
        
        for (input, quantity) in self.inputs.clone() {
            required_inputs.insert(input, quantity * factor);
        }

        return (required_inputs, surplus);
    }
}

fn parse_reaction(reaction_line: String) -> Reaction {

    fn parse_component(comp_string: &str) -> (String, u64) {
        let component: Vec<&str> = comp_string.split(' ').collect();
        return (String::from(component[1]), component[0].parse().unwrap());
    }

    let parts: Vec<&str> = reaction_line.split(" => ").collect();
    
    let inputs_list: Vec<(String, u64)> = parts[0].split(", ")
        .map(|input| parse_component(input)).collect();
    let mut inputs = HashMap::new();
    for input in inputs_list {
        inputs.insert(input.0, input.1);
    }

    let output = parse_component(parts[1]);

    return Reaction {
        product: output.0,
        quantity: output.1,
        inputs: inputs,
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_reaction() {
        let input_line = String::from("2 AB, 3 BC, 4 CA => 1 FUEL");

        let parsed = parse_reaction(input_line);

        assert_eq!(parsed.product, "FUEL");
        assert_eq!(parsed.quantity, 1);
        assert_eq!(parsed.inputs.contains_key("AB"), true);
        assert_eq!(parsed.inputs.contains_key("BC"), true);
        assert_eq!(parsed.inputs.contains_key("CA"), true);
        assert_eq!(*parsed.inputs.get("AB").unwrap(), 2);
        assert_eq!(*parsed.inputs.get("BC").unwrap(), 3);
        assert_eq!(*parsed.inputs.get("CA").unwrap(), 4);
    }

    #[test]
    fn test_example0() {
        let input_lines = vec![String::from("10 ORE => 10 A"),
                                String::from("1 ORE => 1 B"),
                                String::from("7 A, 1 B => 1 C"),
                                String::from("7 A, 1 C => 1 D"),
                                String::from("7 A, 1 D => 1 E"),
                                String::from("7 A, 1 E => 1 FUEL"),];
        
        let mut factory = NanoFactory {
            reactions: HashMap::new(),
            surplus: HashMap::new(),
        };
    
        for reaction_line in input_lines {
            let reaction = parse_reaction(reaction_line);
            factory.reactions.insert(reaction.product.clone(), reaction);
        }

        assert_eq!(factory.ore_required_for(String::from("FUEL"), 1), 31);
    }

    #[test]
    fn test_example1() {
        let input_lines = vec![String::from("9 ORE => 2 A"),
            String::from("8 ORE => 3 B"), String::from("7 ORE => 5 C"),
            String::from("3 A, 4 B => 1 AB"), String::from("5 B, 7 C => 1 BC"),
            String::from("4 C, 1 A => 1 CA"), String::from("2 AB, 3 BC, 4 CA => 1 FUEL"),];
        
        let mut factory = NanoFactory {
            reactions: HashMap::new(),
            surplus: HashMap::new(),
        };
    
        for reaction_line in input_lines {
            let reaction = parse_reaction(reaction_line);
            factory.reactions.insert(reaction.product.clone(), reaction);
        }

        assert_eq!(factory.ore_required_for(String::from("FUEL"), 1), 165);
    }

    #[test]
    fn test_example2() {
        let input_lines = vec![String::from("157 ORE => 5 NZVS"), String::from("165 ORE => 6 DCFZ"),
        String::from("44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL"),
        String::from("12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ"), String::from("179 ORE => 7 PSHF"),
        String::from("177 ORE => 5 HKGWZ"), String::from("7 DCFZ, 7 PSHF => 2 XJWVT"),
        String::from("165 ORE => 2 GPVTF"), String::from("3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT"),];
        
        let mut factory = NanoFactory {
            reactions: HashMap::new(),
            surplus: HashMap::new(),
        };
    
        for reaction_line in input_lines {
            let reaction = parse_reaction(reaction_line);
            factory.reactions.insert(reaction.product.clone(), reaction);
        }

        assert_eq!(factory.ore_required_for(String::from("FUEL"), 1), 13312);
        assert_eq!(factory.fuel_from_ore(1000000000000), 82892753);
    }

    #[test]
    fn test_example3() {
        let input_lines = vec![String::from("2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG"),
        String::from("17 NVRVD, 3 JNWZP => 8 VPVL"), String::from("53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL"),
        String::from("22 VJHF, 37 MNCFX => 5 FWMGM"), String::from("139 ORE => 4 NVRVD"),
        String::from("144 ORE => 7 JNWZP"), String::from("5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC"),
        String::from("5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV"), String::from("145 ORE => 6 MNCFX"),
        String::from("1 NVRVD => 8 CXFTF"), String::from("1 VJHF, 6 MNCFX => 4 RFSQX"), String::from("176 ORE => 6 VJHF"),];
        
        let mut factory = NanoFactory {
            reactions: HashMap::new(),
            surplus: HashMap::new(),
        };
    
        for reaction_line in input_lines {
            let reaction = parse_reaction(reaction_line);
            factory.reactions.insert(reaction.product.clone(), reaction);
        }

        assert_eq!(factory.ore_required_for(String::from("FUEL"), 1), 180697);
        assert_eq!(factory.fuel_from_ore(1000000000000), 5586022);
    }

    #[test]
    fn test_example4() {
        let input_lines = vec![String::from("171 ORE => 8 CNZTR"),
        String::from("7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL"),
        String::from("114 ORE => 4 BHXH"), String::from("14 VRPVC => 6 BMBT"),
        String::from("6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL"),
        String::from("6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT"),
        String::from("15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW"),
        String::from("13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW"),
        String::from("5 BMBT => 4 WPTQ"), String::from("189 ORE => 9 KTJDG"),
        String::from("1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP"), String::from("12 VRPVC, 27 CNZTR => 2 XDBXC"),
        String::from("15 KTJDG, 12 BHXH => 5 XCVML"), String::from("3 BHXH, 2 VRPVC => 7 MZWV"),
        String::from("121 ORE => 7 VRPVC"), String::from("7 XCVML => 6 RJRHP"),
        String::from("5 BHXH, 4 VRPVC => 5 LTCX"),];
        
        let mut factory = NanoFactory {
            reactions: HashMap::new(),
            surplus: HashMap::new(),
        };
    
        for reaction_line in input_lines {
            let reaction = parse_reaction(reaction_line);
            factory.reactions.insert(reaction.product.clone(), reaction);
        }

        assert_eq!(factory.ore_required_for(String::from("FUEL"), 1), 2210736);
        assert_eq!(factory.fuel_from_ore(1000000000000), 460664);
    }
}