use std::collections::HashSet;
use std::hash::Hash;
use std::time::Instant;

const DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Right,
    Direction::Down,
    Direction::Left,
];

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn next_point(&self, direction: Direction, max_x: &i32, max_y: &i32) -> Option<Self> {
        let mut new_x = self.x as i32;
        let mut new_y = self.y as i32;
        (new_x, new_y) = match direction {
            Direction::Up => (new_x, new_y - 1),
            Direction::Right => (new_x + 1, new_y),
            Direction::Down => (new_x, new_y + 1),
            Direction::Left => (new_x - 1, new_y),
        };

        if new_x < 0 || &new_x > max_x || new_y < 0 || &new_y > max_y {
            return None;
        }

        Some(Self::new(new_x as usize, new_y as usize))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Corner(Point);

impl Corner {
    fn new(point: Point) -> Self {
        Self(point)
    }
}

fn field_sizes(
    garden: &Vec<Vec<char>>,
    visited_fields: &mut HashSet<(char, Point)>,
    plant_fields: &mut HashSet<Point>,
    current_point: Point,
    plant: &char,
    max_x: &i32,
    max_y: &i32,
) -> (usize, usize) {
    if visited_fields.contains(&(*plant, current_point)) {
        return (0, 0);
    }
    if plant != &garden[current_point.y][current_point.x] {
        return (0, 1);
    }
    visited_fields.insert((*plant, current_point));
    plant_fields.insert(current_point);

    let mut total_fields = 1_usize;
    let mut total_fences = 0_usize;

    for direction in DIRECTIONS {
        match current_point.next_point(direction, max_x, max_y) {
            Some(new_point) => {
                let (n_fields, n_fences) = field_sizes(
                    garden,
                    visited_fields,
                    plant_fields,
                    new_point,
                    plant,
                    max_x,
                    max_y,
                );
                total_fields += n_fields;
                total_fences += n_fences;
            }
            None => {
                total_fences += 1;
                continue;
            }
        }
    }

    (total_fields, total_fences)
}

fn count_corners(
    plant_fields: HashSet<Point>,
    current_point: Point,
    max_x: &i32,
    max_y: &i32,
) -> usize {
    let mut left_up_corners: HashSet<Corner> = HashSet::new();
    let mut right_up_corners: HashSet<Corner> = HashSet::new();
    let mut left_down_corners: HashSet<Corner> = HashSet::new();
    let mut right_down_corners: HashSet<Corner> = HashSet::new();

    for field in plant_fields.into_iter() {
        left_up_corners.insert(Corner::new(Point::new(field.x, field.y + 1)));
        right_up_corners.insert(Corner::new(Point::new(field.x + 1, field.y + 1)));
        left_down_corners.insert(Corner::new(field));
        right_down_corners.insert(Corner::new(Point::new(field.x + 1, field.y)));
    }

    let mut all_corners: HashSet<Corner> = HashSet::new();

    all_corners.extend(&left_down_corners);
    all_corners.extend(&right_up_corners);
    all_corners.extend(&left_up_corners);
    all_corners.extend(&right_down_corners);

    let mut n_corners: usize = 0;

    for corner in all_corners {
        let in_top_left = left_up_corners.contains(&corner);
        let in_top_right = right_up_corners.contains(&corner);
        let in_right_down = right_down_corners.contains(&corner);
        let in_left_down = left_down_corners.contains(&corner);

        let total =
            in_top_right as i8 + in_right_down as i8 + in_left_down as i8 + in_top_left as i8;

        if total == 1 || total == 3 {
            n_corners += 1;
            continue;
        }

        if total == 2 && ((in_right_down && in_top_left) || (in_top_right && in_left_down)) {
            n_corners += 2;
            continue;
        }
    }

    n_corners
}

fn part_1(lines: &[String]) -> i64 {
    let garden: Vec<Vec<char>> = lines
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();
    let mut visited_fields: HashSet<(char, Point)> = HashSet::new();
    let mut field_mapping: Vec<(char, (usize, usize))> = Vec::new();

    let max_y = (lines.len() - 1) as i32;
    let max_x = (lines[0].len() - 1) as i32;

    for (y, row) in garden.iter().enumerate() {
        for (x, plant) in row.iter().enumerate() {
            let current_point = Point::new(x, y);
            if visited_fields.contains(&(*plant, current_point)) {
                continue;
            }
            let mut plant_fields: HashSet<Point> = HashSet::new();

            field_mapping.push((
                *plant,
                field_sizes(
                    &garden,
                    &mut visited_fields,
                    &mut plant_fields,
                    current_point,
                    plant,
                    &max_x,
                    &max_y,
                ),
            ))
        }
    }

    field_mapping
        .into_iter()
        .map(|(_, (x, y))| x * y)
        .sum::<usize>() as i64
}

fn part_2(lines: &[String]) -> i64 {
    let garden: Vec<Vec<char>> = lines
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();
    let mut visited_fields: HashSet<(char, Point)> = HashSet::new();
    let mut field_mapping: Vec<(char, (usize, usize))> = Vec::new();

    let max_y = (lines.len() - 1) as i32;
    let max_x = (lines[0].len() - 1) as i32;

    for (y, row) in garden.iter().enumerate() {
        for (x, plant) in row.iter().enumerate() {
            let current_point = Point::new(x, y);
            let mut plant_fields: HashSet<Point> = HashSet::new();

            if visited_fields.contains(&(*plant, current_point)) {
                continue;
            }

            let (field_size, _) = field_sizes(
                &garden,
                &mut visited_fields,
                &mut plant_fields,
                current_point,
                plant,
                &max_x,
                &max_y,
            );
            let n_corners = count_corners(plant_fields, current_point, &max_x, &max_y);
            field_mapping.push((*plant, (field_size, n_corners)));
        }
    }

    field_mapping
        .into_iter()
        .map(|(_, (x, y))| x * y)
        .sum::<usize>() as i64
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
mod q12_tests {
    use crate::utilities::read_lines;

    use super::{part_1, part_2};

    fn get_lines() -> Vec<String> {
        read_lines("./data/q12_test.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part_1(&get_lines()), 1930);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part_2(&get_lines()), 1206);
    }
}
