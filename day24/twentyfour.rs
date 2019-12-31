
use std::collections::HashSet;

const BUG: bool = true;
const EMPTY: bool = false;

fn main() {

    let mut map = vec![
        vec![BUG, EMPTY, EMPTY, BUG, EMPTY],
        vec![EMPTY, EMPTY, BUG, EMPTY, EMPTY],
        vec![EMPTY, EMPTY, EMPTY, BUG, BUG],
        vec![EMPTY, EMPTY, EMPTY, BUG, EMPTY],
        vec![BUG, EMPTY, BUG, BUG, BUG],
    ];

    let mut biodiversities = HashSet::new();

    loop {
        let new_map = tick(&map);
        let new_map_bio = count_biodiversity(&new_map);

        if biodiversities.contains(&new_map_bio) {
            println!("Biodiversity of repeated map is {}", new_map_bio);
            break;
        }

        biodiversities.insert(new_map_bio);
        map = new_map;
    }
}

fn tick(map: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut new_map = vec![];
    
    for row in 0..map.len() {
        let mut new_row = vec![];
        for tile in 0..map[row].len() {

            let mut adjecent_bugs = 0;
            if row > 0 && map[row-1][tile] == BUG {
                adjecent_bugs += 1;
            }

            if row < map.len() - 1 && map[row+1][tile] == BUG {
                adjecent_bugs += 1;
            }

            if tile > 0 && map[row][tile-1] == BUG {
                adjecent_bugs += 1;
            }

            if tile < map[row].len() - 1 && map[row][tile+1] == BUG {
                adjecent_bugs += 1;
            }

            if map[row][tile] == BUG {
                if adjecent_bugs == 1 {
                    new_row.push(BUG);
                }
                else {
                    new_row.push(EMPTY);
                }
            }
            else {
                if adjecent_bugs == 1 || adjecent_bugs == 2 {
                    new_row.push(BUG);
                }
                else {
                    new_row.push(EMPTY);
                }
            }
        }

        new_map.push(new_row);
    }

    return new_map;
}

fn count_biodiversity(map: &Vec<Vec<bool>>) -> u64 {

    let mut biodiversity = 0;
    let mut i = 0;
    for row in map {
        for tile in row {
            if *tile == BUG {
                biodiversity += 2u64.pow(i);
            }
            i += 1;
        }
    }

    return biodiversity;
}

fn print_image(map: &Vec<Vec<bool>>) {

    for row in map {
        let row_string: Vec<String> = row.into_iter().map(|c| {
            if *c == BUG {
                return String::from("#");
            }
            else {
                return String::from(".");
            }
        }).collect();

        println!("{}", row_string.join(""));
    }
}