extern crate pathfinding;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::{HashMap};
use pathfinding::prelude::bfs;

const TILE: char = '.';


fn main() {
    let input_file = match File::open("input.txt") {
        Err(_) => panic!("Could not open input.txt"),
        Ok(file) => file,
    };

    let map: Vec<Vec<char>> = BufReader::new(input_file)
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    let (portals, start, end) = locate_portals(&map);

    println!("Start: {:?}", start);
    println!("End: {:?}", end);
    println!("Portals: {:?}", portals);

    let shortest_path = bfs(&add_level(start, 0), 
        |p| successors(&map, &portals, *p), 
        |p| *p == add_level(end, 0)).unwrap();

    println!("Shortest path: {:?}", shortest_path);
    println!("Shortest path for AA to ZZ is {} steps long", shortest_path.len() - 1);
}

fn locate_portals(map: &Vec<Vec<char>>) -> (HashMap<(usize, usize), (usize, usize)>, (usize, usize), (usize, usize)) {
    let mut label_positions: HashMap<String, (usize, usize)> = HashMap::new();
    let mut portal_mappings: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    let mut start = None;
    let mut end = None;

    for row in 0..map.len() - 1 {
        for column in 0..map[row].len() - 1 {
            if !map[row][column].is_ascii_uppercase() {
                continue;
            }

            let mut label = String::from("");
            label.push(map[row][column]);

            if map[row][column + 1].is_ascii_uppercase() {
                label.push(map[row][column + 1]);
            }
            else if map[row + 1][column].is_ascii_uppercase() {
                label.push(map[row + 1][column]);
            }
            else {
                continue;
            }

            let mut label_position = (0, 0);
            if column > 0 && map[row][column - 1] == TILE {
                label_position = (column - 1, row);
            }
            else if column < map[row].len() - 2 && map[row][column + 2] == TILE {
                label_position = (column + 2, row);
            }
            else if row > 0 && map[row - 1][column] == TILE {
                label_position = (column, row - 1);
            }
            else if row < map.len() - 2 && map[row + 2][column] == TILE {
                label_position = (column, row + 2);
            }
            else {
                panic!("Found label, but no adjecent tiles");
            }

            if let Some(existing_position) = label_positions.get(&label) {
                portal_mappings.insert(*existing_position, label_position);
                portal_mappings.insert(label_position, *existing_position);
            }
            else {
                label_positions.insert(label.clone(), label_position);
            }

            if label.to_string() == "AA" {
                start = Some(label_positions.get(&String::from("AA")).unwrap().clone());
            }
            else if label.to_string() == "ZZ" {
                end = Some(label_positions.get(&String::from("ZZ")).unwrap().clone());
            }
        }
    }

    return (portal_mappings, start.unwrap(), end.unwrap());
}

fn successors(map: &Vec<Vec<char>>, portals: &HashMap<(usize, usize), (usize, usize)>, position: (usize, usize, usize)) 
-> Vec<(usize, usize, usize)> {
        
    fn reachable(tile: char) -> bool {
        return tile == TILE;
    }
    
    let mut successors = Vec::new();
    let (x, y, level) = position;
    if reachable(map[y-1][x]) {
        successors.push((x, y-1, level));
    }
    if reachable(map[y+1][x]) {
        successors.push((x, y+1, level));
    }
    if reachable(map[y][x-1]) {
        successors.push((x-1, y, level));
    }
    if reachable(map[y][x+1]) {
        successors.push((x+1, y, level));
    }

    if let Some(portal_destination) = portals.get(&(x, y)) {

        if is_inner((x, y)) {
            successors.push(add_level(*portal_destination, level + 1));
        }
        else {
            if level > 0 {
                successors.push(add_level(*portal_destination, level - 1));
            }
        }
    }

    return successors;
}

fn is_inner(position: (usize, usize)) -> bool {
    let (x, y) = position;
    return x > 20 && x < 120 && y > 20 && y < 120;
}

fn add_level(position: (usize, usize), level: usize) -> (usize, usize, usize) {
    let (x, y) = position;
    return (x, y, level);
}