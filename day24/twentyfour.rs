
use std::collections::HashMap;

const BUG: bool = true;
const EMPTY: bool = false;

fn main() {

    let mut adjacent_offsets = HashMap::new();
    adjacent_offsets.insert(1, vec![(-1, 12), (-1, 8), (0, 2), (0, 6)]);
    adjacent_offsets.insert(2, vec![(0, 1), (-1, 8), (0, 3), (0, 7)]);
    adjacent_offsets.insert(3, vec![(0, 2), (-1, 8), (0, 4), (0, 8)]);
    adjacent_offsets.insert(4, vec![(0, 3), (-1, 8), (0, 5), (0, 9)]);
    adjacent_offsets.insert(5, vec![(-1, 14), (-1, 8), (0, 4), (0, 10)]);
    adjacent_offsets.insert(6, vec![(-1, 12), (0, 1), (0, 7), (0, 11)]);
    adjacent_offsets.insert(7, vec![(0, 6), (0, 8), (0, 2), (0, 12)]);
    adjacent_offsets.insert(8, vec![(0, 7), (0, 3), (0, 9), (1, 1), (1, 2), (1, 3), (1, 4), (1, 5)]);
    adjacent_offsets.insert(9, vec![(0, 4), (0, 8), (0, 10), (0, 14)]);
    adjacent_offsets.insert(10, vec![(-1, 14), (0, 5), (0, 9), (0, 15)]);
    adjacent_offsets.insert(11, vec![(-1, 12), (0, 6), (0, 12), (0, 16)]);
    adjacent_offsets.insert(12, vec![(0, 7), (0, 11), (0, 17), (1, 1), (1, 6), (1, 11), (1, 16), (1, 21)]);
    adjacent_offsets.insert(13, vec![]);
    adjacent_offsets.insert(14, vec![(0, 9), (0, 15), (0, 19), (1, 5), (1, 10), (1, 15), (1, 20), (1, 25)]);
    adjacent_offsets.insert(15, vec![(-1, 14), (0, 10), (0, 14), (0, 20)]);
    adjacent_offsets.insert(16, vec![(-1, 12), (0, 11), (0, 17), (0, 21)]);
    adjacent_offsets.insert(17, vec![(0, 12), (0, 18), (0, 22), (0, 16)]);
    adjacent_offsets.insert(18, vec![(0, 17), (0, 23), (0, 19), (1, 21), (1, 22), (1, 23), (1, 24), (1, 25)]);
    adjacent_offsets.insert(19, vec![(0, 14), (0, 18), (0, 20), (0, 24)]);
    adjacent_offsets.insert(20, vec![(-1, 14), (0, 15), (0, 19), (0, 25)]);
    adjacent_offsets.insert(21, vec![(-1, 12), (-1, 18), (0, 22), (0, 16)]);
    adjacent_offsets.insert(22, vec![(0, 17), (-1, 18), (0, 21), (0, 23)]);
    adjacent_offsets.insert(23, vec![(0, 18), (-1, 18), (0, 22), (0, 24)]);
    adjacent_offsets.insert(24, vec![(0, 25), (-1, 18), (0, 19), (0, 23)]);
    adjacent_offsets.insert(25, vec![(-1, 14), (-1, 18), (0, 20), (0, 24)]);

    let mut map = vec![];
    for _n in 0..800 {
        map.push(vec![
            vec![EMPTY, EMPTY, EMPTY, EMPTY, EMPTY],
            vec![EMPTY, EMPTY, EMPTY, EMPTY, EMPTY],
            vec![EMPTY, EMPTY, EMPTY, EMPTY, EMPTY],
            vec![EMPTY, EMPTY, EMPTY, EMPTY, EMPTY],
            vec![EMPTY, EMPTY, EMPTY, EMPTY, EMPTY],
        ]);
    }

    map.insert(400, vec![
        vec![BUG, EMPTY, EMPTY, BUG, EMPTY],
        vec![EMPTY, EMPTY, BUG, EMPTY, EMPTY],
        vec![EMPTY, EMPTY, EMPTY, BUG, BUG],
        vec![EMPTY, EMPTY, EMPTY, BUG, EMPTY],
        vec![BUG, EMPTY, BUG, BUG, BUG],
    ]);

    for n in 0..200 {
        let (new_map, bug_count) = tick(&map, &adjacent_offsets);
        map = new_map;

        println!("Map {} has {} bugs", n, bug_count);
    }
}

fn tick(map: &Vec<Vec<Vec<bool>>>, adjacent_positions: &HashMap<usize, Vec<(i64, usize)>>)
-> (Vec<Vec<Vec<bool>>>, u64) {
    let mut new_map = vec![];
    let mut total_bugs = 0;
    
    for level in 1..map.len() - 1 {
        let mut new_level = vec![];

        for row in 0..map[level].len() {
            let mut new_row = vec![];
            for tile in 0..map[level][row].len() {

                if row == 2 && tile == 2 {
                    new_row.push(EMPTY);
                    continue;
                }

                let adjacent_bugs = count_adjacent_bugs(map, (level, row, tile), adjacent_positions);

                if map[level][row][tile] == BUG {
                    if adjacent_bugs == 1 {
                        new_row.push(BUG);
                        total_bugs += 1;
                    }
                    else {
                        new_row.push(EMPTY);
                    }
                }
                else {
                    if adjacent_bugs == 1 || adjacent_bugs == 2 {
                        new_row.push(BUG);
                        total_bugs += 1;
                    }
                    else {
                        new_row.push(EMPTY);
                    }
                }
            }
            new_level.push(new_row);
        }

        new_map.push(new_level);
    }

    return (new_map, total_bugs);
}

fn count_adjacent_bugs(map: &Vec<Vec<Vec<bool>>>, position: (usize, usize, usize), 
adjacent_positions: &HashMap<usize, Vec<(i64, usize)>>) -> u64 {
    let (level, row, tile) = position;

    let tile_number = (row * 5 + tile) + 1;
    let positions_to_check = adjacent_positions.get(&tile_number).unwrap();

    let mut adjacent_bugs = 0;
    for adjacent_position in positions_to_check {
        let (level_offset, tile_number): (i64, usize) = *adjacent_position;
        let row = (tile_number - 1) / 5;
        let tile = (tile_number - 1) % 5;
        let level = (level as i64 + level_offset) as usize;

        if map[level][row][tile] == BUG {
            adjacent_bugs += 1;
        }
    }

    return adjacent_bugs;
}
