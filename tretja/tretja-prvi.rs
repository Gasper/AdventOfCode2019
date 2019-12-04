extern crate itertools;

use std::cmp;
use itertools::Itertools;

fn main() {

    let wire1 = parse_wire("R1000,U564,L752,D449,R783,D938,L106,U130,R452,U462,R861,U654,L532,D485,R761,U336,L648,U671,L618,U429,R122,D183,L395,U662,R900,U644,L168,D778,L268,U896,L691,D852,L987,U462,R346,U103,R688,U926,R374,D543,R688,D682,R992,D140,L379,D245,L423,D504,R957,U937,L67,D560,L962,U275,R688,D617,L778,U581,R672,D402,R3,U251,R593,U897,L866,U189,L8,D5,R761,U546,R594,D880,L318,U410,L325,U564,L889,U688,L472,D146,R317,D314,L229,U259,R449,D630,L431,U4,R328,D727,R298,D558,R81,D508,L160,U113,L994,U263,L193,D631,R881,D608,L924,U447,R231,U885,L157,D739,R656,D121,R704,U437,L710,D207,R150,U406,R816,U683,R496,D715,L899,U757,L579,D684,L85,D354,R198,D411,R818,U772,L910,U493,R38,D130,L955,U741,R744,D224,L485,U201,L903,D904,R748,U288,R34,U673,R503,D931,L190,U547,L83,D341,R459,U114,L758,U220,L506,U444,L472,D941,L68,D910,R415,U668,L957,U709,R817,U116,R699,D424,R548,D285,R347,U396,R791,U62,L785,D360,L628,U415,L568,D429,R154,D840,L865,U181,L106,D564,L452,U156,L967,D421,R41,U500,L316,D747,R585,D858,L809,U402,L484,U752,R319,D563,R273,U84,R53,U874,L849,U90,R194,D969,R907,D625,L298,D984,R744,U172,R537,D177,L14,D921,L156,U133,R429,D787,R688,U894,L154,U192,R663,D225,L781,U426,R623,D60,L723,D995,R814,D195,L951,D594,R994,D543,L893,U781,R899,U85,R270,U303,R256,U977,R894,U948,R270,D301,L874,D388,R290,U986,L660,D741,L25,U381,R814,D150,R578,D529,R550,D176,R221,D653,R529,U83,R351,D462,R492,U338,R611,D5,L137,D547,R305,U356,R83,D880,R522,U681,R353,D54,R910,U774,L462,U48,L511,U750,R98,U455,R585,D579,L594".to_owned());
    let wire2 = parse_wire("L1003,U936,R846,U549,L824,D684,R944,U902,R177,U875,L425,U631,L301,U515,L790,D233,R49,U408,L184,D103,R693,D307,L557,D771,L482,D502,R759,D390,L378,U982,L430,U337,L970,U400,R829,U212,L92,D670,R741,D566,L797,U477,L377,U837,R19,U849,L21,D870,L182,U414,L586,U768,L637,U135,R997,U405,L331,D256,L22,D46,L504,D660,L757,U676,L360,D499,R180,D723,L236,U78,R218,U523,L71,D60,L485,U503,L352,D969,R747,U831,L285,D859,L245,D517,L140,U463,L895,U284,L546,U342,R349,D438,R816,U21,L188,U482,L687,D903,L234,U15,L758,D294,R789,D444,L498,D436,L240,D956,L666,U686,R978,D827,R919,U108,R975,D35,R475,U59,L374,U24,L26,D497,R454,D388,R180,D561,R80,D433,R439,D818,R962,D912,R247,U972,R948,D807,R867,D946,R725,U395,R706,U187,L17,U332,L862,D660,L70,U608,R223,D506,R592,U357,R520,D149,L572,D800,L570,D358,R648,U174,R520,U153,L807,U92,R840,U560,L938,D599,R972,D539,R385,D495,L26,D894,L907,D103,L494,U51,L803,D620,L68,D226,R947,U210,R864,D755,L681,D520,L867,D577,R378,D741,L91,D294,L289,D531,L301,U638,L496,U83,L278,D327,R351,D697,L593,U331,R91,D967,R419,D327,R78,U304,R462,D2,L656,D700,L27,D29,L598,U741,L349,D957,R161,U688,R326,D798,L263,U45,L883,U982,R116,D835,L878,U253,L232,D732,R639,D408,R997,D867,R726,D258,L65,D600,L315,U783,L761,U606,R67,D949,L475,U542,L231,U279,L950,U649,L670,D870,L264,U958,R748,D365,R252,D129,R754,U27,R571,D690,L671,U143,L750,U303,L412,U24,L443,D550,R826,U699,L558,U543,L881,D204,R248,D192,R813,U316,L76,D78,R523,U716,L422,D793,R684,D175,L347,D466,L219,D140,L803,U433,R96".to_owned());

    let intersects = wire_intersects(wire1, wire2);
    let closest_intersect = closest_intersect(intersects);

    println!("Closest intersect is {}", closest_intersect);
}  

#[derive(PartialEq)]
#[derive(Clone)]
#[derive(Debug)]
enum WireDirection {
    Horizontal,
    Vertical,
}

#[derive(Clone)]
#[derive(Debug)]
struct Line {
    x1: i64, y1: i64,
    x2: i64, y2: i64,
    direction: WireDirection,
}

struct Wire {
    vertical_segments: Vec<Line>,
    horizontal_segments: Vec<Line>,
}

impl Line {
    fn intersection(&self, other: Line) -> Result<(i64, i64), String> {
        let (vertical, horizontal) = match self.direction {
            WireDirection::Horizontal => (&other, self),
            WireDirection::Vertical => (self, &other),
        };

        if self.direction != other.direction    &&
            vertical.x1 >= cmp::min(horizontal.x1, horizontal.x2) &&
            vertical.x1 <= cmp::max(horizontal.x1, horizontal.x2) &&
            horizontal.y1 >= cmp::min(vertical.y1, vertical.y2)  &&
            horizontal.y1 <= cmp::max(vertical.y1, vertical.y2) {
                
            return Ok((vertical.x1, horizontal.y1));
        }
        else {
            return Err("No intersect between these lines.".to_owned());
        }
    }
} 

fn parse_wire(path: String) -> Wire {
    let mut x = 0i64;
    let mut y = 0i64;
    let mut horizontal_segments = Vec::new();
    let mut vertical_segments = Vec::new();

    for segment in path.split(',') {

        let (direction, distance_string) = segment.split_at(1);
        let distance: i64 = match distance_string.parse() {
            Ok(number) => number,
            Err(_) => panic!("Unable to parse the distance {}", distance_string),
        };

        let x1 = x;
        let y1 = y;

        match direction {
            "U" => {
                y += distance;
                vertical_segments.push(Line {x1: x1, y1: y1, x2: x, y2: y, direction: WireDirection::Vertical});
            },
            "D" => {
                y -= distance;
                vertical_segments.push(Line {x1: x1, y1: y1, x2: x, y2: y, direction: WireDirection::Vertical});
            }
            "L" => {
                x -= distance;
                horizontal_segments.push(Line {x1: x1, y1: y1, x2: x, y2: y, direction: WireDirection::Horizontal});
            }
            "R" => {
                x += distance;
                horizontal_segments.push(Line {x1: x1, y1: y1, x2: x, y2: y, direction: WireDirection::Horizontal});
            }
            _ => panic!("Invalid direction {}", direction),
        };
    }

    return Wire{horizontal_segments: horizontal_segments,
                vertical_segments: vertical_segments};

}

fn wire_intersects(wire1: Wire, wire2: Wire) -> Vec<(i64, i64)> {

    let intersects1: Vec<Result<(i64, i64), String>> = wire1.horizontal_segments.into_iter()
        .cartesian_product(wire2.vertical_segments)
        .map(|(line1, line2)| line1.intersection(line2))
        .filter(|intersection| intersection.is_ok())
        .collect();

    let intersects2: Vec<Result<(i64, i64), String>> = wire2.horizontal_segments.into_iter()
        .cartesian_product(wire1.vertical_segments)
        .map(|(line1, line2)| line1.intersection(line2))
        .filter(|intersection| intersection.is_ok())
        .collect();

    return intersects1.into_iter()
        .merge(intersects2)
        .map(|intersect| intersect.unwrap())
        .collect();
}

fn closest_intersect(intersects: Vec<(i64, i64)>) -> i64 {
    let mut sorted_intersects: Vec<i64> = intersects
        .into_iter()
        .map(|(x, y)| x.abs() + y.abs())
        .filter(|dist| dist > &0i64)
        .collect();

    sorted_intersects.sort();

    return sorted_intersects[0];
}


#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_intersection() -> Result<(), String> {
        let line1 = Line {x1: 0, y1: 0, x2: 0, y2: 8, direction: WireDirection::Vertical};
        let line2 = Line {x1: 0, y1: 0, x2: 8, y2: 0, direction: WireDirection::Horizontal};
        assert_eq!(line1.intersection(line2)?, (0, 0));

        // The same as previous, but direction of one line is reversed
        let line1 = Line {x1: 0, y1: 0, x2: 0, y2: 8, direction: WireDirection::Vertical};
        let line2 = Line {x1: 8, y1: 0, x2: 0, y2: 0, direction: WireDirection::Horizontal};
        assert_eq!(line1.intersection(line2)?, (0, 0));

        // Example 1 from aoc
        let line1 = Line {x1: 3, y1: 2, x2: 3, y2: 6, direction: WireDirection::Vertical};
        let line2 = Line {x1: 2, y1: 3, x2: 6, y2: 3, direction: WireDirection::Horizontal};
        assert_eq!(line1.intersection(line2)?, (3, 3));

        // Example 2 from aoc
        let line1 = Line {x1: 6, y1: 3, x2: 6, y2: 7, direction: WireDirection::Vertical};
        let line2 = Line {x1: 3, y1: 5, x2: 8, y2: 5, direction: WireDirection::Horizontal};
        assert_eq!(line1.intersection(line2)?, (6, 5));

        // Debug example 1
        let line1 = Line { x1: 6, y1: 3, x2: 2, y2: 3, direction: WireDirection::Horizontal };
        let line2 = Line { x1: 3, y1: 5, x2: 3, y2: 2, direction: WireDirection::Vertical };
        assert_eq!(line2.intersection(line1)?, (3, 3));

        return Ok(());
    }

    #[test]
    fn test_wire_parsing() {
        let test_path = "R3,U3,L2,D1".to_owned();
        let wire = parse_wire(test_path);

        assert_eq!(wire.horizontal_segments[0].x1, 0);
        assert_eq!(wire.horizontal_segments[0].y1, 0);
        assert_eq!(wire.horizontal_segments[0].x2, 3);
        assert_eq!(wire.horizontal_segments[0].y2, 0);

        assert_eq!(wire.vertical_segments[0].x1, 3);
        assert_eq!(wire.vertical_segments[0].y1, 0);
        assert_eq!(wire.vertical_segments[0].x2, 3);
        assert_eq!(wire.vertical_segments[0].y2, 3);

        assert_eq!(wire.horizontal_segments[1].x1, 3);
        assert_eq!(wire.horizontal_segments[1].y1, 3);
        assert_eq!(wire.horizontal_segments[1].x2, 1);
        assert_eq!(wire.horizontal_segments[1].y2, 3);

        assert_eq!(wire.vertical_segments[1].x1, 1);
        assert_eq!(wire.vertical_segments[1].y1, 3);
        assert_eq!(wire.vertical_segments[1].x2, 1);
        assert_eq!(wire.vertical_segments[1].y2, 2);
    }

    #[test]
    fn test_wire_intersects() {
        let wire1 = parse_wire("R8,U5,L5,D3".to_owned());
        let wire2 = parse_wire("U7,R6,D4,L4".to_owned());

        assert_eq!(wire_intersects(wire1, wire2), vec![(0, 0), (3, 3), (6, 5)]);
    }

    #[test]
    fn test_closest_intersect() {
        let intersects = vec![(0, 0), (9, 1), (1, 9), (13, 2), (34, 3)];

        assert_eq!(closest_intersect(intersects), 10);
    }

    #[test]
    fn test_example1() {
        let wire1 = parse_wire("R75,D30,R83,U83,L12,D49,R71,U7,L72".to_owned());
        let wire2 = parse_wire("U62,R66,U55,R34,D71,R55,D58,R83".to_owned());
        let wire_intersects = wire_intersects(wire1, wire2);

        assert_eq!(closest_intersect(wire_intersects), 159);
    }

    #[test]
    fn test_example2() {
        let wire1 = parse_wire("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51".to_owned());
        let wire2 = parse_wire("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7".to_owned());
        let wire_intersects = wire_intersects(wire1, wire2);

        assert_eq!(closest_intersect(wire_intersects), 135);
    }
}