extern crate pathfinding;
extern crate itertools;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::{HashMap, VecDeque, HashSet};
use pathfinding::prelude::bfs;
use itertools::Itertools;

#[derive(Clone, Debug, PartialEq, Eq)]
struct PossiblePath {
    keys: Vec<char>,
    current_position: (usize, usize),
    current_key: char,
    remaining: HashMap<char, (usize, usize)>,
    path_length: usize,
}

fn main() {
    let input_file = match File::open("input.txt") {
        Err(_) => panic!("Could not open input.txt"),
        Ok(file) => file,
    };

    let map: Vec<Vec<char>> = BufReader::new(input_file)
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();
    
    let mut key_positions = find_keys(&map);
    println!("Key pos: {:?}", key_positions);
    
    let mut starting = PossiblePath {
        keys: vec!['@'],
        current_position: (0, 0),
        current_key: '@',
        remaining: key_positions.clone(),
        path_length: 0,
    };
    starting.current_position = location_of(&map, '@').unwrap();
    &key_positions.insert('@', starting.current_position);

    let key_graph = build_key_graph(&map, key_positions);

    let mut checked_paths: HashMap<String, usize> = HashMap::new();
    let mut continuations: VecDeque<PossiblePath> = VecDeque::new();
    let starting_options = starting.possible_continuations(&key_graph);
    continuations.extend(starting_options);

    let mut shortest = 6070;
    let mut shortest_path = starting;
    while let Some(next_path) = continuations.pop_front() {
        if let Some(len) = checked_paths.get(&unique_path_pattern(&next_path.keys, next_path.current_key)) {
            if *len <= next_path.path_length {
                continue;
            }
        }
        checked_paths.insert(unique_path_pattern(&next_path.keys, next_path.current_key), next_path.path_length);

        let next_continuations = next_path.possible_continuations(&key_graph);

        if next_continuations.is_empty() {
            if next_path.has_all_keys() && next_path.path_length < shortest {
                println!("New shortest: {}", next_path.path_length);
                shortest = next_path.path_length;
                shortest_path = next_path;
            }
        }
        else {
            for candidate in next_continuations.into_iter() {
                continuations.push_back(candidate);
            }
        }
    }

    println!("Shortest path: {}", shortest);   
    println!("Keys: {:?}", shortest_path);
}


fn build_key_graph(map: &Vec<Vec<char>>, key_positions: HashMap<char, (usize, usize)>)
-> HashMap<char, HashMap<char, (usize, HashSet<char>, HashSet<char>)>> {
    
    let doors_on_map = find_doors(map);

    let mut shortest_paths = HashMap::new();
    for (key, key_position) in &key_positions {

        let mut key_connections = HashMap::new();

        for (key_target, key_position_target) in &key_positions {
            if *key == *key_target {
                continue;
            }

            let target_path = bfs(key_position, 
                |p| successors(map, *p),
                |p| *p == *key_position_target).unwrap();
            
            let key_doors = doors_on_path(&target_path, &doors_on_map);
            let key_gratis_keys = keys_on_path(&target_path, &key_positions);

            key_connections.insert(*key_target, (target_path.len() - 1, key_doors, key_gratis_keys));
        }

        shortest_paths.insert(*key, key_connections);
    }

    return shortest_paths;
}

fn doors_on_path(path: &Vec<(usize, usize)>, door_positions: &HashMap<char, (usize, usize)>) -> HashSet<char> {
    let mut keys = HashSet::new();
    for position in path.into_iter() {
        for (key, pos) in door_positions {
            if *pos == *position {
                keys.insert(*key);
            }
        }
    }
    return keys;
}

fn keys_on_path(path: &Vec<(usize, usize)>, key_positions: &HashMap<char, (usize, usize)>) -> HashSet<char> {
    let mut keys = HashSet::new();
    for position in path.into_iter().skip(1) {
        for (key, pos) in key_positions {
            if *pos == *position {
                keys.insert(*key);
            }
        }
    }
    return keys;
}


fn successors(map: &Vec<Vec<char>>, position: (usize, usize)) -> Vec<(usize, usize)> {
        
    fn reachable(tile: char) -> bool {
        return tile != '#';
    }
    
    let mut successors = Vec::new();
    let (x, y) = position;
    if reachable(map[y-1][x]) {
        successors.push((x, y-1));
    }
    if reachable(map[y+1][x]) {
        successors.push((x, y+1));
    }
    if reachable(map[y][x-1]) {
        successors.push((x-1, y));
    }
    if reachable(map[y][x+1]) {
        successors.push((x+1, y));
    }

    return successors;
}

fn unique_path_pattern(keys: &Vec<char>, destination: char) -> String {
    let mut start: Vec<char> = keys.clone();    
    start.sort();
    let prefix: String = start.into_iter().unique().filter(|k| *k != destination).collect();
    return format!("{}/{}", prefix, destination);
}

impl PossiblePath {

    fn has_all_keys(&self) -> bool {
        return self.remaining.is_empty();
    }

    fn can_open_all_doors(&self, doors: &HashSet<char>, gratis_keys: &HashSet<char>) -> bool {
        for door in doors {
            if !self.keys.contains(door) && !gratis_keys.contains(door) {
                return false;
            }
        }
        return true;
    }

    fn possible_continuations(&self, graph: &HashMap<char, HashMap<char, (usize, HashSet<char>, HashSet<char>)>>) 
    -> Vec<PossiblePath> {
        let mut continuations = Vec::new();
        let starting = self.current_key;

        for (key, position) in self.remaining.clone() {
            let (length, required, gratis_keys) = &graph[&starting][&key];

            if self.can_open_all_doors(required, gratis_keys) {
                let mut new_continuation = self.clone();
                new_continuation.path_length += length;
                
                for gratis_key in gratis_keys.into_iter() {
                    if !new_continuation.keys.contains(&gratis_key) {
                        new_continuation.keys.push(*gratis_key);
                    }
                    
                    new_continuation.remaining.remove(gratis_key);    
                }

                new_continuation.remaining.remove(&key);
                new_continuation.current_position = position;
                new_continuation.current_key = key;

                if new_continuation.path_length < 6070 {
                    continuations.push(new_continuation);
                }
            }
        }
        
        return continuations;
    }
}

fn find_keys(map: &Vec<Vec<char>>) -> HashMap<char, (usize, usize)> {
    let mut keys = HashMap::new();
    for key in ('a' as u8)..('z' as u8 + 1) {
        if let Some(position) = location_of(map, key as char) {
            keys.insert(key as char, position);
        }
    }

    return keys;
}

fn find_doors(map: &Vec<Vec<char>>) -> HashMap<char, (usize, usize)> {
    let mut keys = HashMap::new();
    for key in ('A' as u8)..('Z' as u8 + 1) {
        if let Some(position) = location_of(map, key as char) {
            let door_lower: char = (key as char).to_lowercase().to_string().chars().next().unwrap();
            keys.insert(door_lower, position);
        }
    }

    return keys;
}

fn location_of(map: &Vec<Vec<char>>, item: char) -> Option<(usize, usize)> {
    for (y, row) in map.into_iter().enumerate() {
        match row.into_iter().position(|tile| *tile == item) {
            Some(x) => return Some((x, y)),
            None => {},
        };
    }

    return None;
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_reachable() {
        fn reachable(tile: char, keys: &Vec<char>) -> bool {
            return tile == '.' 
                || tile == '@' 
                || (tile != '#' && tile.is_uppercase() && keys.contains(&tile.to_lowercase().last().unwrap()));
        }

        let keys = vec!['a', 'c', 'g'];
        assert!(reachable('A', &keys));
        assert!(!reachable('#', &keys));
        assert!(!reachable('F', &keys));
        assert!(reachable('G', &keys));
    }

    #[test]
    fn test_hash_map() {
        let i: HashMap<char, HashMap<char, usize>> = HashMap::new();
    }
}