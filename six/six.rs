use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;

fn main() {

    let input_file = match File::open("input.txt") {
        Err(_) => panic!("Could not open input.txt"),
        Ok(file) => file,
    };

    let orbit_pairs: Vec<String> = BufReader::new(input_file)
        .lines()
        .map(|line| line.unwrap())
        .collect();
    
    let orbit_map: HashMap<String, String> = build_orbit_map(orbit_pairs);
    let distance_to_santa = distance_between(orbit_map, "YOU".to_owned(), "SAN".to_owned());

    println!("Distance between us and Santa is: {}", distance_to_santa);
}

fn build_orbit_map(pairs: Vec<String>) -> HashMap<String, String> {

    let mut map: HashMap<String, String> = HashMap::new();
    map.insert("COM".to_owned(), "".to_owned());

    let mut list_to_check = pairs;

    while !list_to_check.is_empty() {
        let mut no_parent: Vec<String> = Vec::new();
        
        for pair in list_to_check {
            let names: Vec<&str> = pair.split(')').collect();
            let parent = (*names[0]).to_owned();
            let orbiter = (*names[1]).to_owned();
            
            if map.contains_key(&parent) {
                map.insert(orbiter, parent.clone());
            }
            else {
                no_parent.push(pair);
            }
        }

        list_to_check = no_parent;
    }

    return map;
}

fn distance_between(orbit_map: HashMap<String, String>, first: String, second: String) -> u64 {

    let mut first_path = path_to_center(&orbit_map, first);
    let mut second_path = path_to_center(&orbit_map, second);

    while !first_path.is_empty() && !second_path.is_empty() &&
          first_path.last() == second_path.last() {
        
        first_path.pop();
        second_path.pop();
    }

    return (first_path.len() + second_path.len()) as u64;
}

/// Constructs a path from given space object to the center of mass (COM).
/// 
/// The path is constructed using a given orbit map. It starts from the object that
/// our given object is orbiting around and ends at the center of mass.
/// If the map is not a tree, or if root node is not "COM", this function will fail.
/// 
/// # Examples
/// 
/// ```
/// using std::collections::HashMap;
/// 
/// let orbit_map = HashMap::new();
/// orbit_map.insert("COM".to_owned(), "A".to_owned());
/// orbit_map.insert("A".to_owned(), "B".to_owned());
/// orbit_map.insert("B".to_owned(), "C".to_owned());
/// orbit_map.insert("C".to_owned(), "D".to_owned());
/// 
/// assert_eq!(path_to_center(orbit_map, "D".to_owned()), vec!["COM".to_owned(), "A".to_owned(), "B".to_owned(), "C".to_owned()]);
/// ```
fn path_to_center(orbit_map: &HashMap<String, String>, object: String) -> Vec<String> {
    let mut path = Vec::new();
    let mut next_object = object;
    while next_object != "COM" {
        next_object = (*orbit_map).get(&next_object).unwrap().clone();
        path.push(next_object.clone());
    }

    return path;
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_example1() {
        let pairs = vec!["COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L", "K)YOU", "I)SAN"];
        let pairs = pairs.into_iter().map(|p| p.to_owned()).collect();

        let map = build_orbit_map(pairs);
        assert_eq!(distance_between(map, "YOU".to_owned(), "SAN".to_owned()), 4);
    }
}