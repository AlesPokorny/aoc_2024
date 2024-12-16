use std::collections::{HashMap, VecDeque};
use std::{collections::HashSet, hash::Hash, time::Instant};

use itertools::Itertools;

const ALL_DIRECTIONS: [Direction; 4] = [
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Field {
    Wall,
    Space,
    Start,
    End,
    Path,
}

#[derive(Clone, Debug)]
struct Map {
    map: Vec<Vec<Field>>,
    start: Point,
    end: Point,
    walkable_points: HashSet<Point>,
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

impl Map {
    fn new(
        map: Vec<Vec<Field>>,
        start: Point,
        end: Point,
        walkable_points: HashSet<Point>,
    ) -> Self {
        Self {
            map,
            start,
            end,
            walkable_points,
        }
    }

    fn is_walkable(&self, point: &Point) -> bool {
        self.map[point.y][point.x] != Field::Wall && self.map[point.y][point.x] != Field::Start
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Cost(usize);

fn parse_data(lines: &[String]) -> Map {
    let mut walkable_points: HashSet<Point> = HashSet::new();
    let mut map_vec: Vec<Vec<Field>> = Vec::new();
    let mut start = Point::new(0, 0);
    let mut end = Point::new(0, 0);

    for (y, line) in lines.iter().enumerate() {
        let mut row_vec: Vec<Field> = Vec::new();
        for (x, field_char) in line.chars().enumerate() {
            let point = Point::new(x, y);
            let field = match field_char {
                '#' => Field::Wall,
                '.' => {
                    walkable_points.insert(point);
                    Field::Space
                }
                'E' => {
                    walkable_points.insert(point);
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

    Map::new(map_vec, start, end, walkable_points)
}

fn get_cost(prev_direction: &Direction, current_direction: &Direction) -> Cost {
    if prev_direction == current_direction {
        return Cost(1);
    }
    Cost(1001)
}

fn dijkstra(map: &Map) -> HashMap<(Point, Direction), Cost> {
    let mut visited_positions: HashMap<(Point, Direction), Cost> = HashMap::new();

    let mut queue: VecDeque<(Point, Direction, Cost)> = VecDeque::new();

    for direction in ALL_DIRECTIONS {
        let new_position = map.start.move_direction(&direction);
        if !map.is_walkable(&new_position) {
            continue;
        }
        let cost = get_cost(&Direction::Right, &direction);
        queue.push_back((new_position, direction, cost));
    }

    while let Some((point, direction, cost)) = queue.pop_front() {
        if let Some(old_cost) = visited_positions.get(&(point, direction)) {
            if old_cost < &cost {
                continue;
            }
        }

        for new_direction in ALL_DIRECTIONS.iter() {
            let new_point = point.move_direction(new_direction);
            if !map.is_walkable(&new_point) {
                continue;
            }
            let new_cost = Cost(cost.0 + get_cost(&direction, new_direction).0);
            if visited_positions
                .get(&(new_point, *new_direction))
                .unwrap_or(&Cost(usize::MAX))
                > &new_cost
            {
                visited_positions.insert((new_point, *new_direction), new_cost);
                queue.push_back((new_point, *new_direction, new_cost));
            }
        }
    }

    visited_positions
}

fn dijkstra_p2(map: &Map) -> HashMap<(Point, Direction), (Cost, Vec<Point>)> {
    let mut visited_positions: HashMap<(Point, Direction), (Cost, Vec<Point>)> = HashMap::new();

    let mut queue: VecDeque<(Point, Direction, Cost, Vec<Point>)> = VecDeque::new();

    for direction in ALL_DIRECTIONS {
        let new_position = map.start.move_direction(&direction);
        if !map.is_walkable(&new_position) {
            continue;
        }
        let cost = get_cost(&Direction::Right, &direction);
        queue.push_back((new_position, direction, cost, Vec::new()));
    }

    while let Some((point, direction, cost, old_points)) = queue.pop_front() {
        if let Some((old_cost, _)) = visited_positions.get(&(point, direction)) {
            if old_cost < &cost {
                continue;
            }
        }

        for new_direction in ALL_DIRECTIONS.iter() {
            let new_point = point.move_direction(new_direction);
            if !map.is_walkable(&new_point) {
                continue;
            }
            let new_cost = Cost(cost.0 + get_cost(&direction, new_direction).0);
            let mut old_points_to_save = old_points.clone();
            old_points_to_save.push(new_point);

            let saved_option = visited_positions.get(&(new_point, *new_direction));

            if saved_option
                .unwrap_or(&(Cost(usize::MAX), Vec::with_capacity(0)))
                .0
                > new_cost
            {
                visited_positions.insert(
                    (new_point, *new_direction),
                    (new_cost, old_points_to_save.clone()),
                );
                queue.push_back((new_point, *new_direction, new_cost, old_points_to_save));
                continue;
            }

            if saved_option.unwrap().0 == new_cost {
                let mut saved_points = saved_option.unwrap().1.clone();
                saved_points.append(&mut old_points_to_save.clone());

                visited_positions.insert((new_point, *new_direction), (new_cost, saved_points));
                queue.push_back((new_point, *new_direction, new_cost, old_points_to_save));
            }
        }
    }

    visited_positions
}

fn part_1(lines: &[String]) -> i64 {
    let map = parse_data(lines);

    let results = dijkstra(&map);

    ALL_DIRECTIONS
        .into_iter()
        .map(|direction| {
            results
                .get(&(map.end, direction))
                .unwrap_or(&Cost(usize::MAX))
        })
        .min()
        .unwrap()
        .0 as i64
}

fn part_2(lines: &[String]) -> i64 {
    let map = parse_data(lines);

    let points = dijkstra_p2(&map);

    let sorted_stuff: Vec<&(Cost, Vec<Point>)> = ALL_DIRECTIONS
        .into_iter()
        .flat_map(|direction| points.get(&(map.end, direction)))
        .sorted_by_key(|(cost, _)| cost)
        .collect();
    let min_cost = sorted_stuff[0].0;

    sorted_stuff
        .into_iter()
        .filter(|(cost, _)| *cost == min_cost)
        .flat_map(|(_, x)| x)
        .unique()
        .collect::<Vec<&Point>>()
        .len() as i64
        + 2
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
mod q16_tests {
    use crate::utilities::read_lines;

    use super::{part_1, part_2};

    fn get_lines() -> Vec<String> {
        read_lines("./data/q16_test.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part_1(&get_lines()), 11048);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part_2(&get_lines()), 64);
    }
}
