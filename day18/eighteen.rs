extern crate pathfinding;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::{HashMap, VecDeque, HashSet};
use pathfinding::prelude::dijkstra;

#[derive(Clone, Debug)]
struct PossiblePath {
    keys: Vec<char>,
    current_position: (usize, usize),
    remaining: HashMap<char, (usize, usize)>,
    path_length: usize,
}

fn main() {
    let input_file = match File::open("input3.txt") {
        Err(_) => panic!("Could not open input.txt"),
        Ok(file) => file,
    };

    let map: Vec<Vec<char>> = BufReader::new(input_file)
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();
    
    let key_positions = find_keys(&map);
    println!("Key pos: {:?}", key_positions);
    
    let mut starting = PossiblePath {
        keys: Vec::new(),
        current_position: (0, 0),
        remaining: key_positions,
        path_length: 0,
    };
    starting.current_position = location_of(&map, '@').unwrap();
    let starting_options = starting.possible_continuations(&map);

    let mut checked_paths: HashMap<String, usize> = HashMap::new();
    let mut continuations: VecDeque<PossiblePath> = VecDeque::new();
    continuations.extend(starting_options);
    let mut shortest = 6070;
    while let Some(next_path) = continuations.pop_front() {
        if let Some(len) = checked_paths.get(&unique_path_pattern(&next_path.keys)) {
            if *len < next_path.path_length {
                continue;
            }
        }
        checked_paths.insert(unique_path_pattern(&next_path.keys), next_path.path_length);

        let mut next_continuations = next_path.possible_continuations(&map);

        if next_continuations.is_empty() {
            if next_path.has_all_keys() && next_path.path_length < shortest {
                shortest = next_path.path_length;
            }
        }
        else {
            println!("more cont: {}", continuations.len());
            continuations.extend(next_continuations.into_iter().filter(|c| c.path_length < shortest));
        }
    }

    println!("Shortest path: {}", shortest);   
    // 6616 
}

fn unique_path_pattern(keys: &Vec<char>) -> String {
    let mut start: Vec<char> = keys[0..(keys.len() - 1)].to_vec();
    start.sort();
    let prefix: String = start.into_iter().collect();
    return format!("{}/{}", prefix, *keys.last().unwrap());
}

impl PossiblePath {
    fn successors(&self, map: &Vec<Vec<char>>, position: (usize, usize)) -> Vec<((usize, usize), usize)> {
        
        fn reachable(tile: char, keys: &Vec<char>) -> bool {
            return tile == '.' 
                || tile == '@' 
                || tile.is_lowercase()
                || (tile != '#' && tile.is_uppercase() && keys.contains(&tile.to_lowercase().last().unwrap()));
        }
        
        let mut successors = Vec::new();
        let (x, y) = position;
        if reachable(map[y-1][x], &(self.keys)) {
            successors.push(((x, y-1), 1));
        }
        if reachable(map[y+1][x], &(self.keys)) {
            successors.push(((x, y+1), 1));
        }
        if reachable(map[y][x-1], &(self.keys)) {
            successors.push(((x-1, y), 1));
        }
        if reachable(map[y][x+1], &(self.keys)) {
            successors.push(((x+1, y), 1));
        }

        return successors;
    }

    fn has_all_keys(&self) -> bool {
        return self.remaining.is_empty();
    }

    fn keys_on_path(&self, path: Vec<(usize, usize)>) -> Vec<char> {
        let mut keys = Vec::new();
        for position in path.into_iter().skip(1) {
            for (key, pos) in &self.remaining {
                if *pos == position {
                    keys.push(*key);
                }
            }
        }
        return keys;
    }

    fn possible_continuations(&self, map: &Vec<Vec<char>>) -> Vec<PossiblePath> {
        let mut continuations = Vec::new();
        //println!("looking for paths with these keys: {:?}", self.keys);
        for (key, position) in self.remaining.clone() {
            let path = dijkstra(&(self.current_position), 
                |pos| self.successors(map, *pos),
                |pos| *pos == position);
            
            if let Some((working_path, length)) = path {
                let new_keys = &self.keys_on_path(working_path);
                let mut new_continuation = self.clone();
                new_continuation.path_length += length;
                for key in new_keys {
                    new_continuation.remaining.remove(key);
                }
                new_continuation.keys.extend(new_keys);
                new_continuation.current_position = position;

                if new_continuation.path_length < 6070 {
                    continuations.push(new_continuation);
                }
                
            }
            else {
                //println!("Nothing for: {}, {:?}", key, position);
            }
        }
        
        continuations.sort_by_key(|c| c.path_length);
        if continuations.is_empty() {
            return vec![];
        }
        else if continuations.len() == 1 {
            return vec![continuations[0].clone()];
        }
        else {
            return vec![continuations[0].clone(), continuations[1].clone()];
        }
    }
}

fn find_keys(map: &Vec<Vec<char>>) -> HashMap<char, (usize, usize)> {
    let mut keys = HashMap::new();
    for key in ('a' as u8)..('z' as u8) {
        if let Some(position) = location_of(map, key as char) {
            keys.insert(key as char, position);
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