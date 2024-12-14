use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::time::Instant;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Velocity {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Robot {
    position: Point,
    speed: Velocity,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Velocity {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Robot {
    fn new(point: Point, speed: Velocity) -> Self {
        Self {
            position: point,
            speed,
        }
    }

    fn position_in_n_seconds(&mut self, n_seconds: i32, max_x: i32, max_y: i32) -> Self {
        let mut new_x = (self.position.x + self.speed.x * n_seconds) % (max_x + 1);
        new_x = if new_x < 0 { new_x + max_x + 1 } else { new_x };
        let mut new_y = (self.position.y + self.speed.y * n_seconds) % (max_y + 1);
        new_y = if new_y < 0 { new_y + max_y + 1 } else { new_y };
        let new_point = Point::new(new_x, new_y);
        self.position = new_point;
        *self
    }

    fn get_quadrant(&self, max_x: i32, max_y: i32) -> Option<Quadrant> {
        let mid_x = max_x / 2;
        let mid_y = max_y / 2;

        match self.position.x.cmp(&mid_x) {
            Ordering::Greater => match self.position.y.cmp(&mid_y) {
                Ordering::Less => Some(Quadrant::TopLeft),
                Ordering::Greater => Some(Quadrant::BottomLeft),
                Ordering::Equal => None,
            },
            Ordering::Less => match self.position.y.cmp(&mid_y) {
                Ordering::Less => Some(Quadrant::TopRight),
                Ordering::Greater => Some(Quadrant::BottomRight),
                Ordering::Equal => None,
            },
            Ordering::Equal => None,
        }
    }
}

enum Quadrant {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

fn parse_data(lines: &[String]) -> Vec<Robot> {
    lines
        .iter()
        .map(|line| {
            let re = Regex::new(r"-?\d+").unwrap();
            let numbers: Vec<i32> = re
                .find_iter(line)
                .map(|captures| captures.as_str().parse::<i32>().unwrap())
                .collect();
            Robot::new(
                Point::new(numbers[0], numbers[1]),
                Velocity::new(numbers[2], numbers[3]),
            )
        })
        .collect()
}

fn count_robots_in_quadrant(robots: &Vec<Robot>, max_x: i32, max_y: i32) -> i32 {
    let mut top_left = 0;
    let mut top_right = 0;
    let mut bot_right = 0;
    let mut bot_left = 0;

    for robot in robots {
        match robot.get_quadrant(max_x, max_y) {
            Some(Quadrant::TopLeft) => top_left += 1,
            Some(Quadrant::TopRight) => top_right += 1,
            Some(Quadrant::BottomLeft) => bot_left += 1,
            Some(Quadrant::BottomRight) => bot_right += 1,
            None => continue,
        }
    }
    top_left * top_right * bot_right * bot_left
}

fn part_1(lines: &[String]) -> i64 {
    let robots = parse_data(lines);

    let max_x = 100;
    let max_y = 102;
    let n_seconds = 100;

    let moved_robots: Vec<Robot> = robots
        .clone()
        .into_iter()
        .map(|mut robot| robot.position_in_n_seconds(n_seconds, max_x, max_y))
        .collect();

    count_robots_in_quadrant(&moved_robots, max_x, max_y) as i64
}

fn generate_image(robots: &Vec<Robot>, max_x: i32, max_y: i32, iteration: i32) {
    let mut imgbuf: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> =
        image::ImageBuffer::new(max_x as u32 + 1, max_y as u32 + 1);
    for pixel in imgbuf.pixels_mut() {
        *pixel = image::Rgb([254, 254, 254]);
    }

    for robot in robots {
        let pixel = imgbuf.get_pixel_mut(robot.position.x as u32, robot.position.y as u32);
        *pixel = image::Rgb([0, 0, 0]);
    }

    imgbuf.save(format!("pictures/{}.png", iteration)).unwrap();
}

fn part_2(lines: &[String]) -> i64 {
    let robots = parse_data(lines);
    let mut robots_map: HashSet<Vec<Robot>> = HashSet::new();
    robots_map.insert(robots.clone());

    let max_x = 100;
    let max_y = 102;

    for n_seconds in 1..=150000 {
        let moved_robots: Vec<Robot> = robots
            .clone()
            .into_iter()
            .map(|mut robot| robot.position_in_n_seconds(n_seconds, max_x, max_y))
            .collect();
        generate_image(&moved_robots, max_x, max_y, n_seconds);

        match robots_map.insert(moved_robots) {
            true => continue,
            false => {
                println!("{}", n_seconds);
                break;
            }
        }
    }

    0
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
mod q14_tests {
    use crate::utilities::read_lines;

    use super::{part_1, part_2};

    fn get_lines() -> Vec<String> {
        read_lines("./data/q14_test.txt")
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
