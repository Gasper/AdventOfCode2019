use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::f64;

fn main() {
    let input_file = match File::open("input.txt") {
        Err(_) => panic!("Could not open input.txt"),
        Ok(file) => file,
    };

    let asteorid_field: Vec<String> = BufReader::new(input_file)
        .lines()
        .map(|line| line.unwrap())
        .collect();

    let positions = &asteroid_positions(asteorid_field);

    let mut max_visible = 0;
    let mut max_asteroid = (0, 0);

    for position in positions.into_iter() {
        let visible_current = visible_asteroids(*position, positions);
        if visible_current.len() > max_visible {
            max_visible = visible_current.len();
            max_asteroid = *position;
        }
    }

    println!("Max visible asteroids: {}", max_visible);
    println!("From coordinates: {:?}", max_asteroid);

    // Part 2
    let visible_from_station = visible_asteroids(max_asteroid, positions);
    let bet_direction = find_bet_direction(&visible_from_station);
    let bet_asteroid = closest_in_direction(positions, max_asteroid, bet_direction);
    println!("Bet direction: {:?}", bet_direction);
    println!("Bet on asteroid: {:?}", bet_asteroid);
}

fn asteroid_positions(asteorid_field: Vec<String>) -> Vec<(i64, i64)> {
    let mut asteroids = Vec::new();

    for field_line in asteorid_field.into_iter().enumerate() {
        let line_asteroids: Vec<(i64, i64)> = (*field_line.1).match_indices("#")
                                            .map(|(index, _)| (field_line.0 as i64, index as i64))
                                            .collect();
        asteroids.extend(line_asteroids);
    }

    return asteroids;
}

fn direction_to_asteroid(first: (i64, i64), second: (i64, i64)) -> (f64, f64) {
    let length = (((second.0 - first.0) as f64).powi(2) + 
                    ((second.1 - first.1) as f64).powi(2)).sqrt();

    return ((second.0 - first.0) as f64 / length, (second.1 - first.1) as f64 / length);
}

fn visible_asteroids(first: (i64, i64), other: &Vec<(i64, i64)>) -> Vec<(f64, f64)> {
    let mut covered_angles: Vec<(f64, f64)> = Vec::new();

    for asteroid in other.into_iter().filter(|a| *(*a) != first) {
        let angle = direction_to_asteroid(first, *asteroid);
        let mut to_add = Vec::new();
        match covered_angles.clone().into_iter().find(|a| directions_eq(angle, *a)) {
            Some(_) => {},
            None => to_add.push(angle),
        };

        covered_angles.append(&mut to_add);
    }

    return covered_angles;
}

fn directions_eq(first: (f64, f64), second: (f64, f64)) -> bool {
    return (first.0 - second.0).abs() < 1e-12 &&
           (first.1 - second.1).abs() < 1e-12;
}

// Part 2
fn find_bet_direction(direction_vectors: &Vec<(f64, f64)>) -> (f64, f64) {
    let mut clockwise_directions = (*direction_vectors).clone();
    clockwise_directions.sort_by(|a, b| {
        
        let mut angle1 = a.1.atan2(-a.0).to_degrees();
        if angle1 < 0.0 {
            angle1 = 360.0 + angle1;
        }

        let mut angle2 = b.1.atan2(-b.0).to_degrees();
        if angle2 < 0.0 {
            angle2 = 360.0 + angle2;
        }
        
        return angle1.partial_cmp(&angle2).unwrap();
    });

    return clockwise_directions[199];
}

fn distance_between(asteorid1: (i64, i64), asteorid2: (i64, i64)) -> f64 {
    return (((asteorid2.0 - asteorid1.0) as f64).powi(2) + 
            ((asteorid2.1 - asteorid1.1) as f64).powi(2)).sqrt();
}

fn closest_in_direction(positions: &Vec<(i64, i64)>, laser: (i64, i64), direction: (f64, f64)) -> (i64, i64) {
    let right_direction: Vec<(i64, i64)> = positions.clone().into_iter().filter(|p| {
        let asteorid_direction = direction_to_asteroid(laser, *p);
        return directions_eq(asteorid_direction, direction);
    }).collect();

    let mut closest_dist = f64::INFINITY;
    let mut closest_asteroid = (0i64, 0i64);
    for asteroid in right_direction {
        if distance_between(laser, asteroid) < closest_dist {
            closest_dist = distance_between(laser, asteroid);
            closest_asteroid = asteroid.clone();
        }
    }

    return closest_asteroid;
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    #[ignore]
    fn test_bet_direction() {
        let directions = vec![(-1.0, 0.0), (-1.0, -1.0), (0.0, -1.0), (1.0, -1.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0), (-1.0, 1.0), ];
        assert_eq!(find_bet_direction(&directions), (-1.0, 0.0));
    }

    #[test]
    fn test_asteroid_positions() {
        let example1 = vec![String::from(".#..#"),
                            String::from("....."),
                            String::from("#####"),
                            String::from("....#"),
                            String::from("...##")];
        
        let positions = vec![(0, 1), (0, 4), (2, 0), (2, 1), (2, 2), 
            (2, 3), (2, 4), (3, 4), (4, 3), (4, 4)];                            

        assert_eq!(asteroid_positions(example1), positions);
    }

    #[test]
    fn test_direction() {
        assert!(directions_eq(direction_to_asteroid((0, 1), (0, 4)), (0f64, 1f64)));
        assert!(directions_eq(direction_to_asteroid((0, 0), (3, -3)), ((1f64/2f64.sqrt()), -1f64/2f64.sqrt())));
        assert!(directions_eq(direction_to_asteroid((0, 0), (3, -3)), direction_to_asteroid((0, 0), (4, -4))));
        assert!(directions_eq(direction_to_asteroid((0, 0), (-2, 4)), direction_to_asteroid((0, 0), (-4, 8))));
        assert!(directions_eq(direction_to_asteroid((3, 0), (-1, 0)), direction_to_asteroid((3, 0), (-4, 0))));
    }

    #[test]
    fn test_visible_asteroids() {
        let positions = vec![(0, 1), (0, 4), (2, 0), (2, 1), (2, 2), 
            (2, 3), (2, 4), (3, 4), (4, 3), (4, 4)];  
        
        assert_eq!(visible_asteroids((0, 1), &positions).len(), 7);
        assert_eq!(visible_asteroids((0, 4), &positions).len(), 7);
        assert_eq!(visible_asteroids((2, 0), &positions).len(), 6);
        assert_eq!(visible_asteroids((2, 1), &positions).len(), 7);
        assert_eq!(visible_asteroids((2, 2), &positions).len(), 7);
        assert_eq!(visible_asteroids((2, 3), &positions).len(), 7);
        assert_eq!(visible_asteroids((2, 4), &positions).len(), 5);
        assert_eq!(visible_asteroids((3, 4), &positions).len(), 7);
        assert_eq!(visible_asteroids((4, 3), &positions).len(), 8);
        assert_eq!(visible_asteroids((4, 4), &positions).len(), 7);
    }

    #[test]
    fn test_example1() {
        let field = vec![String::from("......#.#."),
                        String::from("#..#.#...."),
                        String::from("..#######."),
                        String::from(".#.#.###.."),
                        String::from(".#..#....."),
                        String::from("..#....#.#"),
                        String::from("#..#....#."),
                        String::from(".##.#..###"),
                        String::from("##...#..#."),
                        String::from(".#....####")];
        
        let positions = &asteroid_positions(field);

        let mut max_visible = 0;
        let mut max_asteroid = (0, 0);

        for position in positions.into_iter() {
            let visible_current = visible_asteroids(*position, positions);
            if visible_current.len() > max_visible {
                max_visible = visible_current.len();
                max_asteroid = *position;
            }
        }

        assert_eq!(max_visible, 33);
        assert_eq!(max_asteroid, (8, 5));
        
    }

    #[test]
    fn test_example2() {
        let field = vec![String::from(".#..#..###"),
                        String::from("####.###.#"),
                        String::from("....###.#."),
                        String::from("..###.##.#"),
                        String::from("##.##.#.#."),
                        String::from("....###..#"),
                        String::from("..#.#..#.#"),
                        String::from("#..#.#.###"),
                        String::from(".##...##.#"),
                        String::from(".....#.#..")];
        
        let positions = &asteroid_positions(field);

        let mut max_visible = 0;
        let mut max_asteroid = (0, 0);

        for position in positions.into_iter() {
            let visible_current = visible_asteroids(*position, positions);
            if visible_current.len() > max_visible {
                max_visible = visible_current.len();
                max_asteroid = *position;
            }
        }

        assert_eq!(max_visible, 41);
        assert_eq!(max_asteroid, (3, 6));
        
    }

    #[test]
    fn test_example3() {
        let field = vec![String::from(".#..##.###...#######"),
                        String::from("##.############..##."),
                        String::from(".#.######.########.#"),
                        String::from(".###.#######.####.#."),
                        String::from("#####.##.#.##.###.##"),
                        String::from("..#####..#.#########"),
                        String::from("####################"),
                        String::from("#.####....###.#.#.##"),
                        String::from("##.#################"),
                        String::from("#####.##.###..####.."),
                        String::from("..######..##.#######"),
                        String::from("####.##.####...##..#"),
                        String::from(".#####..#.######.###"),
                        String::from("##...#.##########..."),
                        String::from("#.##########.#######"),
                        String::from(".####.#.###.###.#.##"),
                        String::from("....##.##.###..#####"),
                        String::from(".#.#.###########.###"),
                        String::from("#.#.#.#####.####.###"),
                        String::from("###.##.####.##.#..##")];
        
        let positions = &asteroid_positions(field);

        let mut max_visible = 0;
        let mut max_asteroid = (0, 0);

        for position in positions.into_iter() {
            let visible_current = visible_asteroids(*position, positions);
            if visible_current.len() > max_visible {
                max_visible = visible_current.len();
                max_asteroid = *position;
            }
        }

        assert_eq!(max_visible, 210);
        assert_eq!(max_asteroid, (13, 11));
        
    }
}