extern crate pathfinding;
extern crate itertools;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::{HashMap, VecDeque, HashSet};
use pathfinding::prelude::bfs;
use itertools::Itertools;

// Determined, using simple, unoptimized version
const UPPER_BOUND: usize = 6070;

#[derive(Clone, Debug, PartialEq, Eq)]
struct PossiblePath {
    keys: Vec<char>,
    current_positions: Vec<(usize, usize)>,
    current_keys: Vec<char>,
    remaining: HashMap<char, (usize, usize)>,
    path_length: usize,
}

fn main() {
    let input_file = match File::open("input-2.txt") {
        Err(_) => panic!("Could not open input.txt"),
        Ok(file) => file,
    };

    let map: Vec<Vec<char>> = BufReader::new(input_file)
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    let key_positions_no_start = find_keys(&map);
    
    let starting = PossiblePath {
        keys: vec!['@'],
        current_positions: vec![(39, 39), (41, 39), (39, 41), (41, 41)],
        current_keys: vec!['@', '@', '@', '@'],
        remaining: key_positions_no_start.clone(),
        path_length: 0,
    };
    
    let mut key_positions = vec![
        key_positions_no_start.clone(), 
        key_positions_no_start.clone(),
        key_positions_no_start.clone(),
        key_positions_no_start.clone()];
    
    &key_positions[0].insert('@', starting.current_positions[0]);
    &key_positions[1].insert('@', starting.current_positions[1]);
    &key_positions[2].insert('@', starting.current_positions[2]);
    &key_positions[3].insert('@', starting.current_positions[3]);

    let key_graphs = vec![
        build_key_graph(&map, &key_positions[0]),
        build_key_graph(&map, &key_positions[1]),
        build_key_graph(&map, &key_positions[2]),
        build_key_graph(&map, &key_positions[3])];

    let mut checked_paths: HashMap<String, usize> = HashMap::new();
    let mut continuations: VecDeque<PossiblePath> = VecDeque::new();
    let starting_options = starting.possible_continuations(&key_graphs);
    continuations.extend(starting_options);

    let mut shortest = UPPER_BOUND;
    let mut shortest_path = starting;
    while let Some(next_path) = continuations.pop_front() {
        if let Some(len) = checked_paths.get(&unique_path_pattern(&next_path.keys, &next_path.current_keys)) {
            if *len <= next_path.path_length {
                continue;
            }
        }
        checked_paths.insert(unique_path_pattern(&next_path.keys, &next_path.current_keys), next_path.path_length);

        let next_continuations = next_path.possible_continuations(&key_graphs);

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


fn build_key_graph(map: &Vec<Vec<char>>, key_positions: &HashMap<char, (usize, usize)>)
-> HashMap<char, HashMap<char, (usize, HashSet<char>, HashSet<char>)>> {
    
    let doors_on_map = find_doors(map);

    let mut shortest_paths = HashMap::new();
    for (key, key_position) in key_positions {

        let mut key_connections = HashMap::new();

        for (key_target, key_position_target) in key_positions {
            if *key == *key_target {
                continue;
            }

            if let Some(target_path) = bfs(key_position, 
                |p| successors(map, *p), |p| *p == *key_position_target) {
                
                let key_doors = doors_on_path(&target_path, &doors_on_map);
                let key_gratis_keys = keys_on_path(&target_path, &key_positions);
    
                // Minus 1, because start node is also included
                key_connections.insert(*key_target, (target_path.len() - 1, key_doors, key_gratis_keys));
            }
        }

        if !key_connections.is_empty() {
            shortest_paths.insert(*key, key_connections);
        }
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

fn unique_path_pattern(keys: &Vec<char>, current_keys: &Vec<char>) -> String {
    let mut ends = current_keys.clone();
    ends.sort();
    
    let mut start: Vec<char> = keys.clone(); 
    start.sort();
    let prefix: String = start.into_iter().unique().filter(|k| !ends.contains(k)).collect();
    return format!("{}/{:?}", prefix, ends);
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

    fn possible_continuations(&self, graphs: &Vec<HashMap<char, HashMap<char, (usize, HashSet<char>, HashSet<char>)>>>) 
    -> Vec<PossiblePath> {
        let mut continuations = Vec::new();
        let starting = &self.current_keys;

        for (key, position) in self.remaining.clone() {
            for i in 0..4 {
                if !&graphs[i][&starting[i]].contains_key(&key) {
                    continue;
                }

                let (length, required, gratis_keys) = &graphs[i][&starting[i]][&key];

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
                    new_continuation.current_positions[i] = position;
                    new_continuation.current_keys[i] = key;

                    if new_continuation.path_length < UPPER_BOUND {
                        continuations.push(new_continuation);
                    }
                }
            }
        }
        
        return continuations;
    }
}

fn find_keys(map: &Vec<Vec<char>>) -> HashMap<char, (usize, usize)> {
    let mut keys = HashMap::new();
    for key in ('a' as u8)..=('z' as u8) {
        if let Some(position) = location_of(map, key as char) {
            keys.insert(key as char, position);
        }
    }

    return keys;
}

fn find_doors(map: &Vec<Vec<char>>) -> HashMap<char, (usize, usize)> {
    let mut doors = HashMap::new();
    for door in ('A' as u8)..('Z' as u8) {
        if let Some(position) = location_of(map, door as char) {
            let door_lower: char = (door as char).to_lowercase().to_string().chars().next().unwrap();
            doors.insert(door_lower, position);
        }
    }

    return doors;
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