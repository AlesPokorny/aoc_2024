
use std::time::Instant;

#[derive(Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, Copy)]
enum Entity {
    Box,
    Wall,
    Robot,
    EmptySpace,
}

struct Map {
    size: (usize, usize),
    map: Vec<Vec<Entity>>,
    robot_position: Point,
}

impl Point {
    fn new(x: usize, y:usize) -> Self {
        Self { x, y }
    }

    fn move_direction(&self, direction: &Direction) -> Self {
        match direction {
            Direction::Up => Self::new(self.x, self.y - 1),
            Direction::Right => Self::new(self.x + 1, self.y),
            Direction::Down => Self::new(self.x, self.y + 1),
            Direction::Left => Self::new(self.x - 1, self.y),
        }
    }
}

impl Map {
    fn new(size: (usize, usize), map: Vec<Vec<Entity>>, robot_position: Point) -> Self {
        Self { size, map, robot_position }
    }

    fn print(&self) {
        for row in self.map.iter() {
            for entity in row {
                let char_to_print = match entity {
                    Entity::Box => 'O',
                    Entity::Wall => '#',
                    Entity::Robot => '@',
                    Entity::EmptySpace => '.',
                };
                print!("{}", char_to_print);
            }
            print!("\n");
        }
    }

    fn move_robot(&mut self, new_point: &Point) {
        self.map[new_point.y][new_point.x] = Entity::Robot;
        self.map[self.robot_position.y][self.robot_position.x] = Entity::EmptySpace;

        self.robot_position = *new_point;
    }

    fn move_box(&mut self, old_point: &Point, new_point: &Point) {
        self.map[new_point.y][new_point.x] = Entity::Box;
        self.map[self.robot_position.y][self.robot_position.x] = Entity::EmptySpace;
    }

    fn move_boxes(&mut self, point: &Point, direction: &Direction) -> bool {
        let new_point = point.move_direction(direction);

        match self.get_entity_at_point(&new_point) {
            Entity::Box => {
                if self.move_boxes(&new_point, direction) {
                    self.move_box(point, &new_point);
                    return true;
                }
                return false;
            },
            Entity::Wall => {
                false
            },
            Entity::Robot => {
                panic!("Are they multiplying");
            },
            Entity::EmptySpace => {
                self.move_box(point, &new_point);
                true
            },
        }


    }

    fn bust_a_move(&mut self, direction: Direction) {
        let new_point = self.robot_position.move_direction(&direction);

        match self.get_entity_at_point(&new_point) {
            &Entity::Box => {
                if self.move_boxes(&new_point, &direction) {
                    self.move_robot(&new_point);
                }
            },
            &Entity::Wall => {
                return
            },
            &Entity::Robot => {
                panic!("Robotception")
            },
            &Entity::EmptySpace => {
                self.move_robot(&new_point);
            },
        }

    }

    fn get_entity_at_point(&self, point: &Point) -> &Entity {
        &self.map[point.y][point.x]
    }

    fn count_boxes(&self) -> usize {
        let mut result = 0_usize;
        for (y, row) in self.map.iter().enumerate() {
            for (x, entity) in row.iter().enumerate() {
                match entity {
                    Entity::Box => result += y * 100 + x,
                    _ => continue,
                }
            }
        }

        result
    }

    fn expand_map(&mut self) {
        let mut expanded_map: Vec<Vec<Entity>> = Vec::new();

        for (y, row) in self.map.iter().enumerate() {
            let mut new_row: Vec<Entity> = Vec::new();
            for (x, entity) in row.iter().enumerate() {
                match entity {
                    Entity::Robot => {
                        new_row.push(Entity::Robot);
                        new_row.push(Entity::EmptySpace);
                    },
                    Entity::Box => {
                        new_row.push(Entity::Box);
                        new_row.push(Entity::Box);
                    },
                    Entity::Wall => {
                        new_row.push(Entity::Wall);
                        new_row.push(Entity::Wall);
                    },
                    Entity::EmptySpace => {
                        new_row.push(Entity::EmptySpace);
                        new_row.push(Entity::EmptySpace);
                    },
                }
            }
            expanded_map.push(new_row);
        }
        self.map = expanded_map;
        self.robot_position = Point::new(self.robot_position.x * 2, self.robot_position.y)

    }
}

fn parse_data(lines: &[String]) -> (Map, Vec<Direction>) {
    let mut map_part = true;
    let mut robot_position = Point::new(0, 0);
    let max_x = lines[0].len();
    let max_y = lines.len();
    let mut map_vec: Vec<Vec<Entity>> = Vec::with_capacity(max_y);
    let mut directions: Vec<Direction> = Vec::new();

    for (y, row) in lines.iter().enumerate() {
        if row == "" {
            map_part = false;
            continue;
        }
        let mut map_row: Vec<Entity> = Vec::with_capacity(max_x);
        for (x, space) in row.chars().enumerate() {
            if map_part {
                let entity = match space {
                    '#' => Entity::Wall,
                    '.' => Entity::EmptySpace,
                    'O' => Entity::Box,
                    '@' => {
                        robot_position = Point::new(x, y);
                        Entity::Robot
                    },
                    _ => panic!("AAAAAA"),
                };
                map_row.push(entity);
                continue;
            }

            let direction = match space {
                'v' => Direction::Down,
                '>' => Direction::Right,
                '^' => Direction::Up,
                '<' => Direction::Left,
                _ => panic!("Another aaaaaa"),
            };
            directions.push(direction)
        }
        if map_part {
            map_vec.push(map_row);
        }
    }

    let map = Map::new((max_x, max_y), map_vec, robot_position);

    (map, directions)


}


fn part_1(lines: &[String]) -> i64 {
    let (mut map, directions) = parse_data(lines);

    for direction in directions {
        map.bust_a_move(direction);
    }


    map.count_boxes() as i64
}

fn part_2(lines: &[String]) -> i64 {
    let (mut map, directions) = parse_data(lines);
    map.expand_map();

    let a = map.map[map.robot_position.y][map.robot_position.x];

    for direction in directions {
        map.bust_a_move(direction);
    }

    map.print();


    map.count_boxes() as i64
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
mod q15_tests {
    use crate::utilities::read_lines;

    use super::{part_1, part_2};

    fn get_lines() -> Vec<String> {
        read_lines("./data/q15_test.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part_1(&get_lines()), 12);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part_2(&get_lines()), 0);
    }
}
