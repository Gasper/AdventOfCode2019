use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;

struct SpaceObject {
    name: String,
    depth: u64,
    orbiters: Vec<SpaceObject>,
}

fn main() {

    let input_file = match File::open("input.txt") {
        Err(_) => panic!("Could not open input.txt"),
        Ok(file) => file,
    };

    let orbit_pairs: Vec<String> = BufReader::new(input_file)
        .lines()
        .map(|line| line.unwrap())
        .collect();
    
    let total_orbits = build_orbit_tree(orbit_pairs);

    println!("Total orbits: {}", total_orbits);
}


impl SpaceObject {
    fn add_orbiter(&mut self, new_orbiter: &String, parent_name: &String) -> Option<u64> {
        if self.name == *parent_name {
            self.orbiters.push(SpaceObject {
                name: (*new_orbiter).clone(),
                depth: self.depth + 1,
                orbiters: Vec::new()
            });

            return Some(self.depth + 1);
        }
        
        for orbiter in self.orbiters.iter_mut() {
            let new_orbiter_depth = orbiter.add_orbiter(new_orbiter, parent_name);
            if new_orbiter_depth.is_some() {
                return new_orbiter_depth;
            }
        }

        return None;
    }
}

fn build_orbit_tree(pairs: Vec<String>) -> u64 {

    let mut depths: HashMap<String, u64> = HashMap::new();
    depths.insert("COM".to_owned(), 0);

    let mut list_to_check = pairs;

    while !list_to_check.is_empty() {
        let mut no_parent: Vec<String> = Vec::new();
        
        for pair in list_to_check {
            let names: Vec<&str> = pair.split(')').collect();
            let parent = (*names[0]).to_owned();
            let orbiter = (*names[1]).to_owned();
            
            if depths.contains_key(&parent) {
                depths.insert(orbiter, depths.get(&parent).unwrap() + 1);
            }
            else {
                no_parent.push(pair);
            }
        }

        list_to_check = no_parent;
    }

    return depths.values().fold(0, |acc, depth| acc + depth);
}
