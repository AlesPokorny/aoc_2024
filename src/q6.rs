use std::collections::{HashMap, HashSet};
use std::ops::Add;
use std::time::Instant;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn is_out_of_bounds(&self, max_bounds: (i32, i32)) -> bool {
        if self.x < 0 || self.x > max_bounds.0 || self.y < 0 || self.y > max_bounds.1 {
            return true;
        }
        false
    }
}

impl Add<&Direction> for &Position {
    type Output = Position;

    fn add(self, rhs: &Direction) -> Self::Output {
        Position::new(self.x + rhs.x, self.y + rhs.y)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Direction {
    x: i32,
    y: i32,
}

impl Direction {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    fn turn(&self) -> Self {
        if self.x == 0 && self.y == -1 {
            return Self::new(1, 0);
        }
        if self.x == 1 && self.y == 0 {
            return Self::new(0, 1);
        }
        if self.x == 0 && self.y == 1 {
            return Self::new(-1, 0);
        }
        if self.x == -1 && self.y == 0 {
            return Self::new(0, -1);
        }
        panic!("boom")
    }
}

fn find_starting_position(map: &[Vec<char>]) -> (Position, Direction) {
    let starting_position: Position;
    let starting_direction: Direction;
    for (y, row) in map.iter().enumerate() {
        for (x, field) in row.iter().enumerate() {
            if !['.', '#'].contains(field) {
                starting_position = Position::new(x as i32, y as i32);
                starting_direction = match field {
                    '^' => Direction::new(0, -1),
                    '>' => Direction::new(1, 0),
                    'v' => Direction::new(0, 1),
                    '<' => Direction::new(-1, 0),
                    _ => panic!("unexpected starting character!"),
                };

                return (starting_position, starting_direction);
            }
        }
    }
    panic!("Did not find starting position :(")
}

fn part_1(lines: &[String]) -> i64 {
    let map: Vec<Vec<char>> = lines
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let (mut position, mut direction) = find_starting_position(&map);

    let mut visited_fields: HashSet<Position> = HashSet::new();

    let max_y = map.len() - 1;
    let max_x = map[0].len() - 1;

    'outer: loop {
        loop {
            let new_position = &position + &direction;
            if new_position.is_out_of_bounds((max_x as i32, max_y as i32)) {
                break 'outer;
            }
            let field_char = map[new_position.y as usize][new_position.x as usize];
            if field_char != '#' {
                visited_fields.insert(new_position);
                position = new_position;
                break;
            }
            direction = direction.turn();
        }
    }

    visited_fields.len() as i64
}

fn part_2(lines: &[String]) -> i64 {
    let map: Vec<Vec<char>> = lines
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let (starting_position, starting_direction) = find_starting_position(&map);

    let max_y = map.len() - 1;
    let max_x = map[0].len() - 1;

    let mut n_solutions = 0_i64;

    for x in 0..=max_x {
        for y in 0..=max_y {
            let mut new_map = map.clone();
            if new_map[y][x] != '.' {
                continue;
            }
            new_map[y][x] = '#';

            let mut visited_fields: HashMap<Position, Vec<Direction>> = HashMap::new();
            let mut position = starting_position;
            let mut direction = starting_direction;

            'outer: loop {
                loop {
                    let new_position = &position + &direction;
                    if new_position.is_out_of_bounds((max_x as i32, max_y as i32)) {
                        break 'outer;
                    }
                    let field_value = new_map[new_position.y as usize][new_position.x as usize];
                    if field_value != '#' {
                        position = new_position;
                        match visited_fields.get_mut(&position) {
                            Some(direction_vec) => {
                                if direction_vec.contains(&direction) {
                                    n_solutions += 1;
                                    break 'outer;
                                }
                                direction_vec.push(direction);
                            }
                            None => {
                                visited_fields.insert(position, vec![direction]);
                            }
                        }
                        break;
                    }
                    direction = direction.turn();
                }
            }
        }
    }

    n_solutions
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
mod q6_tests {
    use crate::utilities::read_lines;

    use super::{part_1, part_2};

    fn get_lines() -> Vec<String> {
        read_lines("./data/q6_test.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part_1(&get_lines()), 41);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part_2(&get_lines()), 6);
    }
}
