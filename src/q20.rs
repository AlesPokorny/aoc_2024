use std::{hash::Hash, time::Instant};

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

    fn move_direction(&self, direction: &Direction, max_size: (usize, usize)) -> Option<Self> {
        let mut new_x = self.x as i32;
        let mut new_y = self.y as i32;

        match direction {
            Direction::Up => new_y -= 1,
            Direction::Right => new_x += 1,
            Direction::Down => new_y += 1,
            Direction::Left => new_x -= 1,
        };

        if new_x < 0 || new_x > max_size.0 as i32 || new_y < 0 || new_y > max_size.1 as i32 {
            None
        } else {
            Some(Self::new(new_x as usize, new_y as usize))
        }
    }

    fn get_distance(&self, point: &Point) -> usize {
        self.x.abs_diff(point.x) + self.y.abs_diff(point.y)
    }

    fn get_point_between(&self, point: &Point) -> Self {
        let new_x = if self.x == point.x {
            self.x
        } else {
            self.x.max(point.x) - 1
        };
        let new_y = if self.y == point.y {
            self.y
        } else {
            self.y.max(point.y) - 1
        };
        Self::new(new_x, new_y)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Field {
    Wall,
    Space,
    Start,
    End,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn is_opposite(&self, prev_direction: Option<Direction>) -> bool {
        if prev_direction.is_none() {
            return false;
        }
        match self {
            Self::Up => prev_direction.unwrap() == Direction::Down,
            Self::Right => prev_direction.unwrap() == Direction::Left,
            Self::Down => prev_direction.unwrap() == Direction::Up,
            Self::Left => prev_direction.unwrap() == Direction::Right,
        }
    }
}

#[derive(Clone, Debug)]
struct Map {
    map: Vec<Vec<Field>>,
    start: Point,
    end: Point,
    walls: Vec<Point>,
    size: (usize, usize),
}

impl Map {
    fn new(
        map: Vec<Vec<Field>>,
        start: Point,
        end: Point,
        walls: Vec<Point>,
        size: (usize, usize),
    ) -> Self {
        Self {
            map,
            start,
            end,
            walls,
            size,
        }
    }

    fn get_field_at_point(&self, point: &Point) -> Field {
        self.map[point.y][point.x]
    }

    fn get_next_path_points(&self, point: &Point) -> Vec<Point> {
        let mut points: Vec<Point> = Vec::new();

        for direction in ALL_DIRECTIONS {
            let new_point = point.move_direction(&direction, self.size).unwrap();
            if self.is_wall(&new_point) {
                continue;
            }

            points.push(new_point);
        }

        points
    }

    fn follow_path(&self) -> Vec<Point> {
        let mut points: Vec<Point> = Vec::new();
        let mut point = self.start;
        let mut prev_direction: Option<Direction> = None;
        points.push(point);

        'outer: loop {
            for direction in ALL_DIRECTIONS {
                if direction.is_opposite(prev_direction) {
                    continue;
                }

                let new_point = match point.move_direction(&direction, self.size) {
                    Some(new_point) => new_point,
                    None => continue,
                };
                let field = self.get_field_at_point(&new_point);
                if field == Field::Space {
                    prev_direction = Some(direction);
                    point = new_point;
                    points.push(new_point);
                    break;
                }
                if field == Field::End {
                    points.push(new_point);
                    break 'outer;
                }
            }
        }

        points
    }

    fn is_wall(&self, point: &Point) -> bool {
        self.get_field_at_point(point) == Field::Wall
    }
}

fn parse_data(lines: &[String]) -> Map {
    let mut walls: Vec<Point> = Vec::new();
    let mut map_vec: Vec<Vec<Field>> = Vec::new();
    let mut start = Point::new(0, 0);
    let mut end = Point::new(0, 0);
    let size = (lines.len() - 1, lines[0].len() - 1);

    for (y, line) in lines.iter().enumerate() {
        let mut row_vec: Vec<Field> = Vec::new();
        for (x, field_char) in line.chars().enumerate() {
            let point = Point::new(x, y);
            let field = match field_char {
                '#' => {
                    walls.push(point);
                    Field::Wall
                }
                '.' => Field::Space,
                'E' => {
                    end = point;
                    Field::End
                }
                'S' => {
                    start = point;
                    Field::Start
                }
                _ => panic!("oopsies"),
            };
            row_vec.push(field);
        }
        map_vec.push(row_vec);
    }

    Map::new(map_vec, start, end, walls, size)
}

fn part_1(lines: &[String]) -> usize {
    let map = parse_data(lines);
    let path = map.follow_path();

    let path_len = path.len();
    let min_shortcut_len = 100;
    let mut cheating_paths_lengths: Vec<usize> = Vec::new();

    for (i, point_1) in path[0..(path_len - min_shortcut_len)].iter().enumerate() {
        for (j, point_2) in path[(i + min_shortcut_len)..path_len].iter().enumerate() {
            let manhattan_distance = point_1.get_distance(point_2);
            if manhattan_distance > 2 {
                continue;
            }
            if j + min_shortcut_len - manhattan_distance >= min_shortcut_len {
                cheating_paths_lengths.push(j + min_shortcut_len - manhattan_distance);
            }
        }
    }

    cheating_paths_lengths
        .into_iter()
        .filter(|x| *x >= min_shortcut_len)
        .collect::<Vec<usize>>()
        .len()
}

fn part_2(lines: &[String]) -> usize {
    let map = parse_data(lines);
    let path = map.follow_path();
    let path_len = path.len();

    let min_shortcut_len = 100;
    let max_shortcut = 20;

    let mut cheating_paths_lengths: Vec<usize> = Vec::new();

    for (i, point_1) in path[0..(path_len - min_shortcut_len)].iter().enumerate() {
        for (j, point_2) in path[(i + min_shortcut_len)..path_len].iter().enumerate() {
            let manhattan_distance = point_1.get_distance(point_2);
            if manhattan_distance > max_shortcut {
                continue;
            }
            if j + min_shortcut_len - manhattan_distance >= min_shortcut_len {
                cheating_paths_lengths.push(j + min_shortcut_len - manhattan_distance);
            }
        }
    }

    // println!("{:?}", cheating_paths_lengths);
    cheating_paths_lengths
        .into_iter()
        .filter(|x| *x >= min_shortcut_len)
        .collect::<Vec<usize>>()
        .len()
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

    use super::{part_1, part_2};

    fn get_lines() -> Vec<String> {
        read_lines("./data/q20_test.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part_1(&get_lines()), 22);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part_2(&get_lines()), 285);
    }
}
