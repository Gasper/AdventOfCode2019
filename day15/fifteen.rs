extern crate pathfinding;

use std::fs::read;
use std::collections::{HashMap, VecDeque};
use pathfinding::prelude::dijkstra_all;

mod intcode;

// Droid
const NORTH: i64 = 1;
const SOUTH: i64 = 2;
const WEST: i64 = 3;
const EAST: i64 = 4;

const HIT_WALL: i64 = 0;
const MOVED: i64 = 1;
const FOUND_OXYGEN: i64 = 2;

#[derive(Clone)]
struct Path {
    path: Vec<(i64, i64)>,
    current: usize,
}

struct Droid {
    motion_graph: HashMap<(i64, i64), (i64, i64)>,
    tile_types: HashMap<(i64,i64), i64>,
    unchecked_tiles: VecDeque<(i64, i64)>,
    position: (i64, i64),
    memory: intcode::Memory,
}

fn main() {
    let raw_input = match read("input.txt") {
        Err(_) => panic!("Can't read input.txt!"),
        Ok(file) => file,
    };

    let input_string = String::from_utf8_lossy(&raw_input);
    let input_program = get_program(input_string.to_string());
    
    let mut droid = Droid {
        motion_graph: HashMap::new(),
        tile_types: HashMap::new(),
        unchecked_tiles: VecDeque::new(),
        position: (0, 0),
        memory: intcode::Memory{
            program: input_program.clone(), 
            virtual_memory: HashMap::new(),
            relative_base: 0,
        },
    };

    droid.add_all_adjecent();

    let mut oxygen_system_position = None;
    let mut continue_from = Some(0);
    let mut input = vec![];
    let mut current_path = droid.path_to_next();
    while continue_from.is_some() && current_path.is_some() {

        let (ip, output) = intcode::run_program(&mut droid.memory, &input, continue_from.unwrap());      
        continue_from = ip;
        input.clear();

        if output.is_empty() {
            // If there's no output, we're making a move
            if current_path.as_ref().map(|p| p.is_complete()).unwrap() {
                droid.add_all_adjecent();
                current_path = droid.path_to_next();
            }

            if current_path.is_some() {
                let next_move = current_path.as_mut().map(|p| p.next_move()).unwrap();
                input.push(next_move);
            }
        }
        else {
            assert_eq!(output.len(), 1);
            let sensor = output.first().unwrap();

            match *sensor {
                HIT_WALL => {
                    // Don't change the position, and instead just calculate path to next unchecked tile
                    droid.tile_types.insert(current_path.as_ref().map(|p| p.current_position()).unwrap(), *sensor);
                    current_path = droid.path_to_next();
                },
                MOVED => {
                    // Was able to move to next position
                    droid.tile_types.insert(droid.position, *sensor);
                    droid.position = current_path.as_ref().map(|p| p.current_position()).unwrap();
                },
                FOUND_OXYGEN => {
                    droid.tile_types.insert(droid.position, *sensor);
                    droid.position = current_path.as_ref().map(|p| p.current_position()).unwrap();
                    oxygen_system_position = Some(droid.position);
                },
                _ => panic!("Unexpected output."),
            };
        }
    }

    let path_to_oxygen = droid.path_to_root(oxygen_system_position.unwrap());
    println!("Path to oxygen system is {} moves long:", path_to_oxygen.len() - 1);
    
    // Part 2

    let result = dijkstra_all(&oxygen_system_position.unwrap(),
        |position| droid.adjecent_empty_tiles(*position));
    
    let furthest_point = result.into_iter()
        .map(|(tile, result)| (tile, result.1))
        .fold(((0, 0), 0), |a, b| {
            if a.1 > b.1 {
                return a;
            } else {
                return b;
            }
        });
    println!("Furthest point: {:?}", furthest_point);

    paint_screen(droid.tile_types);
}

impl Droid {
    fn add_all_adjecent(&mut self) {
        let (current_x, current_y) = self.position;
        self.add_adjacent((current_x, current_y + 1));
        self.add_adjacent((current_x - 1, current_y));
        self.add_adjacent((current_x + 1, current_y));
        self.add_adjacent((current_x, current_y - 1));      
    }

    fn add_adjacent(&mut self, adjacent_tile: (i64, i64)) {
        if !self.motion_graph.contains_key(&adjacent_tile) {
            self.motion_graph.insert(adjacent_tile, self.position);
            self.unchecked_tiles.push_back(adjacent_tile);
        }
    }

    fn path_to_next(&mut self) -> Option<Path> {
        let mut current_to_root = self.path_to_root(self.position);
        
        let next = match self.unchecked_tiles.pop_front() {
            Some(position) => position,
            None => return None,
        };

        let mut next_to_root = self.path_to_root(next);
        
        let mut common_parent = (0, 0);
        while current_to_root.last() == next_to_root.last() {
            common_parent = current_to_root.pop().unwrap();
            next_to_root.pop();
        }

        return Some(Path {
            path: current_to_root.into_iter()
            .chain(vec![common_parent].into_iter())
            .chain(next_to_root.into_iter().rev())
            .collect(),
            current: 0,
        });
    }

    fn path_to_root(&self, starting_point: (i64, i64)) -> Vec<(i64, i64)> {
        let mut path = Vec::new();
        let mut current = starting_point;
        while current != (0, 0) {
            path.push(current);
            current = self.motion_graph[&current];
        }

        path.push((0, 0));
        return path;
    }

    fn adjecent_empty_tiles(&self, tile: (i64, i64)) -> Vec<((i64, i64), i64)> {
        let (tile_x, tile_y) = tile;
        let mut adjecent_empty = Vec::new();
        if self.tile_types.get(&(tile_x + 1, tile_y)) != Some(&HIT_WALL) {
            adjecent_empty.push(((tile_x + 1, tile_y), 1));
        }
        
        if self.tile_types.get(&(tile_x - 1, tile_y)) != Some(&HIT_WALL) {
            adjecent_empty.push(((tile_x - 1, tile_y), 1));
        }

        if self.tile_types.get(&(tile_x, tile_y + 1)) != Some(&HIT_WALL) {
            adjecent_empty.push(((tile_x, tile_y + 1), 1));
        }
        
        if self.tile_types.get(&(tile_x, tile_y - 1)) != Some(&HIT_WALL) {
            adjecent_empty.push(((tile_x, tile_y - 1), 1));
        }

        return adjecent_empty;
    }
}

impl Path {
    fn next_move(&mut self) -> i64 {
        let (current_x, current_y) = self.path[self.current];
        let (next_x, next_y) = self.path[self.current + 1];
        self.current += 1;

        if current_y < next_y {
            return NORTH;
        }
        else if current_y > next_y {
            return SOUTH;
        }
        else if current_x < next_x {
            return WEST;
        }
        else if current_x > next_x {
            return EAST;
        }
        else {
            panic!("Two steps on the path are the same.");
        }
    }

    fn is_complete(&self) -> bool {
        return self.current + 1 == self.path.len();
    }

    fn current_position(&self) -> (i64, i64) {
        return self.path[self.current];
    }

    fn previous_position(&self) -> (i64, i64) {
        return self.path[self.current - 1];
    }
}

fn get_program(input: String) -> Vec<i64> {
    return input.split(',').map(|c| match (*c).parse::<i64>() {
        Err(_) => panic!("Couldn't parse number {}", c),
        Ok(num) => num,
    }).collect();
}

fn paint_screen(tiles: HashMap<(i64, i64), i64>,) {
    const cols: usize = 300;
    const rows: usize = 50;
    let mut big_field: [char; rows*cols] = [' '; rows*cols];
    for (location, color) in &tiles {
        let y = location.1 + 20;
        let x = location.0 + 50;
        if *color == FOUND_OXYGEN {
            println!("Should be at: {:?}", location);
        }

        big_field[((y  * cols as i64) + (x as i64)) as usize] = match *color {
            HIT_WALL => '#',
            MOVED => '.',
            FOUND_OXYGEN => 'X',
            _ => '?',
        };
    }

    big_field[((20  * cols as i64) + (50 as i64)) as usize] = 'S';

    for i in 0..rows {
        let a: Vec<char> = (big_field[(i*cols)..((i+1)*cols)]).to_vec();
        let s: Vec<String> = a.into_iter().map(|x| x.to_string()).collect();
        println!("{:?}", s.join(""));
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_path_planning() {
        let mut graph = HashMap::new();
        graph.insert((1, 1), (0, 0));
        graph.insert((2, 2), (1, 1));
        
        graph.insert((3, 2), (2, 2));
        graph.insert((4, 2), (3, 2));

        graph.insert((1, 2), (2, 2));
        graph.insert((0, 2), (1, 2));

        let mut unchecked = VecDeque::new();
        unchecked.push_back((0, 2));

        let mut droid = Droid {
            motion_graph: graph,
            tile_types: HashMap::new(),
            position: (4, 2),
            unchecked_tiles: unchecked,
            memory: intcode::Memory {
                program: Vec::new(),
                virtual_memory: HashMap::new(),
                relative_base: 0,
            },
        };

        let path = droid.path_to_next();

        assert_eq!(path.path, vec![(4,2), (3,2), (2,2), (1,2), (0,2)]);

    }
}