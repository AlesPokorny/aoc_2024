use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::ops::Sub;
use std::time::Instant;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Self {
            x: x as i32,
            y: y as i32,
        }
    }
    fn new_from_i32(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    fn is_valid(&self, max_x: usize, max_y: usize) -> bool {
        self.x >= 0 && self.y >= 0 && self.x <= max_x as i32 && self.y <= max_y as i32
    }
}

struct Distance {
    x: i32,
    y: i32,
}

impl Distance {
    fn new(x: usize, y: usize) -> Self {
        Self {
            x: x as i32,
            y: y as i32,
        }
    }
    fn new_from_i32(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Sub for &Position {
    type Output = Distance;

    fn sub(self, rhs: Self) -> Self::Output {
        Distance::new(
            self.x.abs_diff(rhs.x) as usize,
            self.y.abs_diff(rhs.y) as usize,
        )
    }
}

fn get_slope(point_1: &Position, point_2: &Position) -> f32 {
    (point_1.y - point_2.y) as f32 / (point_1.x - point_2.x) as f32
}

fn get_y(point_1: &Position, x: i32, slope: f32) -> i32 {
    let y = point_1.y as f32 + (x - point_1.x) as f32 * slope;
    y.round() as i32
}

fn get_antinodes(
    point_1: &Position,
    point_2: &Position,
    closest_only: bool,
    max_x: usize,
    max_y: usize,
) -> Vec<Position> {
    let distance = point_1 - point_2;
    let x_small: i32;
    let x_large: i32;

    if point_1.x < point_2.x {
        x_small = point_1.x - distance.x;
        x_large = point_2.x + distance.x;
    } else {
        x_small = point_2.x - distance.x;
        x_large = point_1.x + distance.x;
    }

    let slope = get_slope(point_1, point_2);

    if closest_only {
        return vec![
            Position::new_from_i32(x_small, get_y(point_1, x_small, slope)),
            Position::new_from_i32(x_large, get_y(point_1, x_large, slope)),
        ];
    }

    let mut antinodes: Vec<Position> = Vec::new();

    for direction in [-1, 1] {
        let mut x = if direction == -1 { x_small } else { x_large };
        loop {
            let pos = Position::new_from_i32(x, get_y(point_1, x, slope));
            if pos.is_valid(max_x, max_y) {
                antinodes.push(pos);
                x += distance.x * direction;
            } else {
                break;
            }
        }
    }
    antinodes.push(*point_1);
    antinodes.push(*point_2);

    antinodes
}

fn parse_data(lines: &[String]) -> HashMap<char, Vec<Position>> {
    let mut antennas: HashMap<char, Vec<Position>> = HashMap::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, field) in line.chars().enumerate() {
            if field != '.' {
                match antennas.get_mut(&field) {
                    Some(v) => v.push(Position::new(x, y)),
                    None => {
                        antennas.insert(field, vec![Position::new(x, y)]);
                    }
                }
            }
        }
    }

    antennas
}

fn part_1(lines: &[String]) -> i64 {
    let antennas = parse_data(lines);
    let max_y = lines.len() - 1;
    let max_x = lines[0].len() - 1;

    let mut antinodes: HashSet<Position> = HashSet::new();

    for (_, positions) in antennas.iter() {
        let n_positions = positions.len();

        for pos_vec in positions.iter().combinations(2) {
            let pos_1 = pos_vec[0];
            let pos_2 = pos_vec[1];
            for antinode in get_antinodes(pos_1, pos_2, true, max_x, max_y) {
                if antinode.is_valid(max_x, max_y) {
                    antinodes.insert(antinode);
                }
            }
        }
    }

    antinodes.len() as i64
}

fn part_2(lines: &[String]) -> i64 {
    let antennas = parse_data(lines);
    let max_y = lines.len() - 1;
    let max_x = lines[0].len() - 1;

    let mut antinodes: HashSet<Position> = HashSet::new();

    for (_, positions) in antennas.iter() {
        let n_positions = positions.len();

        for pos_vec in positions.iter().combinations(2) {
            let pos_1 = pos_vec[0];
            let pos_2 = pos_vec[1];
            for antinode in get_antinodes(pos_1, pos_2, false, max_x, max_y) {
                antinodes.insert(antinode);
            }
        }
    }

    antinodes.len() as i64
}

pub fn solution(lines: Vec<String>) {
    println!("Part 1:");
    let start = Instant::now();
    let result_1 = part_1(&lines);
    let end = Instant::now();
    println!("result: {}", result_1);
    println!("duration: {:?}", end - start);

    println!("Part 2:");
    let start = Instant::now();
    let result_2 = part_2(&lines);
    let end = Instant::now();
    println!("result: {}", result_2);
    println!("duration: {:?}", end - start);
}

#[cfg(test)]
mod q8_tests {
    use crate::utilities::read_lines;

    use super::{part_1, part_2};

    fn get_lines() -> Vec<String> {
        read_lines("./data/q8_test.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part_1(&get_lines()), 14);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part_2(&get_lines()), 34);
    }
}
