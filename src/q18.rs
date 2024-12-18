use pathfinding::prelude::bfs;
use std::{collections::HashSet, hash::Hash, time::Instant};

const ALL_DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Right,
    Direction::Down,
    Direction::Left,
];

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn move_direction(self, direction: &Direction) -> Self {
        match direction {
            Direction::Up => Self::new(self.x, self.y - 1),
            Direction::Right => Self::new(self.x + 1, self.y),
            Direction::Down => Self::new(self.x, self.y + 1),
            Direction::Left => Self::new(self.x - 1, self.y),
        }
    }
}

#[derive(PartialEq, Eq)]
enum Field {
    Safe,
    Corrupted,
}

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

struct Map {
    map: Vec<Vec<Field>>,
    size: usize,
}

impl Map {
    fn new(map: Vec<Vec<Field>>, size: usize) -> Self {
        Self { map, size }
    }

    fn let_memory_fall(&mut self, falling_memory: &mut Vec<Point>, nanoseconds: usize) {
        for _ in 0..nanoseconds {
            let point = falling_memory.pop().unwrap();
            self.map[point.y][point.x] = Field::Corrupted;
        }
    }

    fn get_safe_neighbors(&self, point: &Point) -> Vec<Point> {
        let x = point.x as i32;
        let y = point.y as i32;

        let mut output: Vec<Point> = Vec::new();

        for direction in ALL_DIRECTIONS {
            let (new_x, new_y) = match direction {
                Direction::Up => (x, y - 1),
                Direction::Right => (x + 1, y),
                Direction::Down => (x, y + 1),
                Direction::Left => (x - 1, y),
            };

            if new_x >= 0
                && new_x <= self.size as i32
                && new_y >= 0
                && new_y <= self.size as i32
                && self.map[new_y as usize][new_x as usize] == Field::Safe
            {
                output.push(Point::new(new_x as usize, new_y as usize))
            }
        }

        output
    }

    fn find_shortest_path(&self) -> Option<Vec<Point>> {
        bfs(
            &Point::new(0, 0),
            |point| self.get_safe_neighbors(point),
            |&point| Point::new(self.size, self.size) == point,
        )
    }
}

fn parse_data(lines: &[String]) -> Vec<Point> {
    lines
        .iter()
        .map(|line| {
            let line_vec = line
                .split(",")
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            Point::new(line_vec[0], line_vec[1])
        })
        .rev()
        .collect()
}

fn create_map(size: usize) -> Map {
    let map = (0..=size)
        .map(|_| (0..=size).map(|_| Field::Safe).collect())
        .collect();
    Map::new(map, size)
}

fn part_1(lines: &[String]) -> usize {
    let mut map = create_map(70);
    let mut falling_memory = parse_data(lines);

    map.let_memory_fall(&mut falling_memory, 1024);

    let result = map.find_shortest_path();

    result.unwrap().len() - 1
}

fn part_2(lines: &[String]) -> Point {
    let mut map = create_map(70);
    let mut falling_memory = parse_data(lines);

    map.let_memory_fall(&mut falling_memory, 1024);

    let shortest_path = map.find_shortest_path().unwrap();
    let mut path_points: HashSet<Point> = HashSet::with_capacity(shortest_path.len());

    for point in shortest_path {
        path_points.insert(point);
    }

    loop {
        let last_point = *falling_memory.last().unwrap();
        map.let_memory_fall(&mut falling_memory, 1);
        if !path_points.contains(&last_point) {
            continue;
        }

        match map.find_shortest_path() {
            Some(new_shortest_path) => {
                path_points = HashSet::with_capacity(new_shortest_path.len());
                for point in new_shortest_path {
                    path_points.insert(point);
                }
            }
            None => return last_point,
        }
    }
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
    println!("result: {:?}", result_2);
    println!("duration: {:?}", end - start);
}

#[cfg(test)]
mod q18_tests {
    use crate::utilities::read_lines;

    use super::{part_1, part_2, Point};

    fn get_lines() -> Vec<String> {
        read_lines("./data/q18_test.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part_1(&get_lines()), 22);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part_2(&get_lines()), Point::new(6, 1));
    }
}
