use std::{hash::Hash, ops::Add, time::Instant};

use itertools::Itertools;

const DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Right,
    Direction::Down,
    Direction::Left,
];

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Add<&Direction> for Point {
    type Output = Option<Self>;

    fn add(self, rhs: &Direction) -> Self::Output {
        let (x, y) = match rhs {
            Direction::Up => (self.x as i32, self.y as i32 - 1),
            Direction::Right => (self.x as i32 + 1, self.y as i32),
            Direction::Down => (self.x as i32, self.y as i32 + 1),
            Direction::Left => (self.x as i32 - 1, self.y as i32),
        };

        if x < 0 || y < 0 {
            return None;
        }
        Some(Self::new(x as usize, y as usize))
    }
}

fn parse_data(lines: &[String]) -> (Vec<Vec<u32>>, Vec<Point>) {
    let mut starting_points: Vec<Point> = Vec::new();
    let map = lines
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, point)| {
                    let digit = point.to_digit(10).unwrap();
                    if digit == 0 {
                        starting_points.push(Point::new(x, y));
                    }
                    digit
                })
                .collect::<Vec<u32>>()
        })
        .collect();

    (map, starting_points)
}

fn search_for_hikes(map: &Vec<Vec<u32>>, current_point: Point, current_height: &u32) -> Vec<Point> {
    let mut ends: Vec<Point> = Vec::new();
    for direction in DIRECTIONS.iter() {
        let new_point = match current_point + direction {
            Some(point) => point,
            None => continue,
        };

        match map.get(new_point.y) {
            Some(row) => match row.get(new_point.x) {
                Some(height) => {
                    if current_height + 1 == *height {
                        if *height == 9 {
                            ends.push(new_point);
                        }

                        ends.extend(search_for_hikes(map, new_point, height));
                    }
                }
                None => continue,
            },
            None => continue,
        }
    }

    ends
}

fn part_1(lines: &[String]) -> i64 {
    let (map, starting_points) = parse_data(lines);

    let mut n_solutions = 0_usize;
    for starting_point in starting_points {
        let all_ends = search_for_hikes(&map, starting_point, &0);
        n_solutions += all_ends.into_iter().unique().collect::<Vec<Point>>().len();
    }

    n_solutions as i64
}

fn part_2(lines: &[String]) -> i64 {
    let (map, starting_points) = parse_data(lines);

    let mut n_solutions = 0_usize;
    for starting_point in starting_points {
        let all_ends = search_for_hikes(&map, starting_point, &0);
        n_solutions += all_ends.into_iter().collect::<Vec<Point>>().len();
    }

    n_solutions as i64
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
mod q10_tests {
    use crate::utilities::read_lines;

    use super::{part_1, part_2};

    fn get_lines() -> Vec<String> {
        read_lines("./data/q10_test.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part_1(&get_lines()), 36);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part_2(&get_lines()), 81);
    }
}
