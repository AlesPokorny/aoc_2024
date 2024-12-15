use std::time::Instant;

#[derive(Clone, Copy, Debug)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Entity {
    Box,
    Wall,
    Robot,
    EmptySpace,
    BoxLeft,
    BoxRight,
}

struct Map {
    map: Vec<Vec<Entity>>,
    robot_position: Point,
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

    fn to_left(self) -> Self {
        Self::new(self.x - 1, self.y)
    }

    fn to_right(self) -> Self {
        Self::new(self.x + 1, self.y)
    }
}

impl Map {
    fn new(map: Vec<Vec<Entity>>, robot_position: Point) -> Self {
        Self {
            map,
            robot_position,
        }
    }

    fn print(&self) {
        for row in self.map.iter() {
            for entity in row {
                let char_to_print = match entity {
                    Entity::Box => 'O',
                    Entity::Wall => '#',
                    Entity::Robot => '@',
                    Entity::EmptySpace => '.',
                    Entity::BoxLeft => '[',
                    Entity::BoxRight => ']',
                };
                print!("{}", char_to_print);
            }
            println!();
        }
    }

    fn move_robot(&mut self, new_point: &Point) {
        self.map[new_point.y][new_point.x] = Entity::Robot;
        self.map[self.robot_position.y][self.robot_position.x] = Entity::EmptySpace;

        self.robot_position = *new_point;
    }

    fn move_box(&mut self, old_point: &Point, new_point: &Point) {
        self.map[new_point.y][new_point.x] = self.map[old_point.y][old_point.x];
        self.map[old_point.y][old_point.x] = Entity::EmptySpace;
    }

    fn move_boxes(&mut self, point: &Point, direction: &Direction) -> bool {
        let new_point = point.move_direction(direction);

        match self.get_entity_at_point(&new_point) {
            Entity::Box | Entity::BoxLeft | Entity::BoxRight => {
                if self.move_boxes(&new_point, direction) {
                    self.move_box(point, &new_point);
                    return true;
                }
                false
            }
            Entity::Wall => false,
            Entity::Robot => {
                panic!("Are they multiplying");
            }
            Entity::EmptySpace => {
                self.move_box(point, &new_point);
                true
            }
        }
    }

    fn bust_a_move(&mut self, direction: Direction) {
        let new_point = self.robot_position.move_direction(&direction);

        match self.get_entity_at_point(&new_point) {
            Entity::Box => {
                if self.move_boxes(&new_point, &direction) {
                    self.move_robot(&new_point);
                }
            }
            Entity::Wall => (),
            Entity::Robot => {
                panic!("Robotception")
            }
            Entity::EmptySpace => {
                self.move_robot(&new_point);
            }
            _ => panic!(),
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
                    }
                    Entity::Box => {
                        new_row.push(Entity::BoxLeft);
                        new_row.push(Entity::BoxRight);
                    }
                    Entity::Wall => {
                        new_row.push(Entity::Wall);
                        new_row.push(Entity::Wall);
                    }
                    Entity::EmptySpace => {
                        new_row.push(Entity::EmptySpace);
                        new_row.push(Entity::EmptySpace);
                    }
                    _ => panic!(),
                }
            }
            expanded_map.push(new_row);
        }
        self.map = expanded_map;
        self.robot_position = Point::new(self.robot_position.x * 2, self.robot_position.y)
    }

    fn bust_a_horizontal_move(&mut self, direction: Direction) {
        if ![Direction::Left, Direction::Right].contains(&direction) {
            panic!();
        }
        let new_point = self.robot_position.move_direction(&direction);

        match self.get_entity_at_point(&new_point) {
            Entity::BoxLeft | Entity::BoxRight => {
                if self.move_boxes(&new_point, &direction) {
                    self.move_robot(&new_point);
                }
            }
            &Entity::Wall => (),
            &Entity::Robot => {
                panic!("Robotception")
            }
            &Entity::EmptySpace => {
                self.move_robot(&new_point);
            }
            _ => panic!(),
        }
    }

    fn bust_a_vertical_move(&mut self, direction: Direction) {
        let old_point = self.robot_position;
        let new_point = self.robot_position.move_direction(&direction);

        match self.get_entity_at_point(&new_point) {
            Entity::Box => panic!(),
            Entity::Wall => (),
            Entity::Robot => panic!("Robotception"),
            Entity::EmptySpace => self.move_robot(&new_point),
            Entity::BoxLeft => {
                if self.can_move_big_box(&new_point, &direction, &Entity::BoxLeft) {
                    self.make_big_moves(&new_point, &direction, &Entity::BoxLeft);
                    self.move_robot(&new_point);
                }
            }
            Entity::BoxRight => {
                if self.can_move_big_box(&new_point, &direction, &Entity::BoxRight) {
                    self.make_big_moves(&new_point, &direction, &Entity::BoxRight);
                    self.move_robot(&new_point);
                }
            }
        }
    }

    fn can_move_big_box(&self, point: &Point, direction: &Direction, entity: &Entity) -> bool {
        let one_new_point = point.move_direction(direction);
        let other_new_point = match entity {
            Entity::BoxLeft => one_new_point.to_right(),
            Entity::BoxRight => one_new_point.to_left(),
            _ => panic!(),
        };

        let mut can_move = true;

        for new_point in [one_new_point, other_new_point] {
            match self.get_entity_at_point(&new_point) {
                Entity::Box => panic!(),
                Entity::Wall => return false,
                Entity::Robot => panic!("Robotception"),
                Entity::EmptySpace => can_move &= true,
                Entity::BoxLeft => {
                    can_move &= self.can_move_big_box(&new_point, direction, &Entity::BoxLeft)
                }
                Entity::BoxRight => {
                    can_move &= self.can_move_big_box(&new_point, direction, &Entity::BoxRight)
                }
            }
        }
        can_move
    }

    fn make_big_moves(&mut self, point: &Point, direction: &Direction, entity: &Entity) {
        let one_new_point = point.move_direction(direction);
        let other_new_point = match entity {
            Entity::BoxLeft => one_new_point.to_right(),
            Entity::BoxRight => one_new_point.to_left(),
            _ => panic!(),
        };
        for new_point in [one_new_point, other_new_point] {
            match self.get_entity_at_point(&new_point) {
                Entity::Box => panic!(),
                Entity::Wall => panic!("I thought we could do it"),
                Entity::Robot => panic!("Robotception"),
                Entity::EmptySpace => continue,
                Entity::BoxLeft => self.make_big_moves(&new_point, direction, &Entity::BoxLeft),
                Entity::BoxRight => self.make_big_moves(&new_point, direction, &Entity::BoxRight),
            }
        }
        self.move_big_box(point, &one_new_point, entity);
    }

    fn move_big_box(&mut self, old_point: &Point, new_point: &Point, entity: &Entity) {
        self.map[new_point.y][new_point.x] = self.map[old_point.y][old_point.x];
        self.map[old_point.y][old_point.x] = Entity::EmptySpace;
        if entity == &Entity::BoxLeft {
            let new_point = new_point.to_right();
            let old_point = old_point.to_right();
            self.map[new_point.y][new_point.x] = self.map[old_point.y][old_point.x];
            self.map[old_point.y][old_point.x] = Entity::EmptySpace;
        } else {
            let new_point = new_point.to_left();
            let old_point = old_point.to_left();
            self.map[new_point.y][new_point.x] = self.map[old_point.y][old_point.x];
            self.map[old_point.y][old_point.x] = Entity::EmptySpace;
        }
    }

    fn count_big_boxes(&self) -> usize {
        let mut result = 0_usize;
        for (y, row) in self.map.iter().enumerate() {
            for (x, entity) in row.iter().enumerate() {
                match entity {
                    Entity::BoxLeft => result += y * 100 + x,
                    _ => continue,
                }
            }
        }

        result
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
        if row.is_empty() {
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
                    }
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

    let map = Map::new(map_vec, robot_position);

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

    for direction in directions {
        match direction {
            Direction::Left | Direction::Right => map.bust_a_horizontal_move(direction),
            _ => map.bust_a_vertical_move(direction),
        }
    }

    map.count_big_boxes() as i64
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
        assert_eq!(part_1(&get_lines()), 10092);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part_2(&get_lines()), 0);
    }
}
