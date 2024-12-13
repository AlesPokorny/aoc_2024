use std::time::Instant;

#[derive(Clone, Copy, Debug)]
struct Button {
    x: usize,
    y: usize,
}

impl Button {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Copy, Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Copy, Debug)]
struct Machine {
    button_a: Button,
    button_b: Button,
    prize: Point,
}

impl Machine {
    fn new(button_a: Button, button_b: Button, prize: Point) -> Self {
        Self {
            button_a,
            button_b,
            prize,
        }
    }

    fn solve(&self, offset: usize) -> Option<usize> {
        let a = self.button_a.x;
        let b = self.button_a.y;
        let c = self.prize.x + offset;
        let d = self.button_b.x;
        let e = self.button_b.y;
        let f = self.prize.y + offset;

        let le_bottom = (a * e) as i64 - (b * d) as i64;
        if a == 0 || le_bottom == 0 {
            return None;
        }
        let le_top = (a * f) as i64 - (b * c) as i64;

        let n_presses_b = le_top as f64 / le_bottom as f64;
        if n_presses_b.fract() != 0.0 || n_presses_b < 0.0 {
            return None;
        }
        let n_presses_a = (c as f64 - d as f64 * n_presses_b) / a as f64;
        if n_presses_a.fract() != 0.0 || n_presses_a < 0.0 {
            return None;
        }

        Some(n_presses_a as usize * 3 + n_presses_b as usize)
    }
}

fn parse_line(line: &str, split_str: &str) -> (usize, usize) {
    let clean_line = line.replace(" ", "");
    let split_line: Vec<&str> = clean_line.split(",").collect();
    let x = split_line[0]
        .split(split_str)
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let y = split_line[1]
        .split(split_str)
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap();

    (x, y)
}

fn parse_lines(lines: &[String]) -> Vec<Machine> {
    let mut button_a = Button::new(0, 0);
    let mut button_b = Button::new(0, 0);
    let mut prize = Point::new(0, 0);
    let mut my_machines: Vec<Machine> = Vec::new();

    for (i, line) in lines.iter().enumerate() {
        let modulo = i % 4;
        if modulo == 3 {
            my_machines.push(Machine::new(button_a, button_b, prize));
            continue;
        }
        if modulo == 2 {
            let (x, y) = parse_line(line, "=");
            prize = Point::new(x, y);
            continue;
        }

        let (x, y) = parse_line(line, "+");

        if modulo == 0 {
            button_a = Button::new(x, y);
            continue;
        }

        if modulo == 1 {
            button_b = Button::new(x, y);
            continue;
        }
    }
    my_machines.push(Machine::new(button_a, button_b, prize));
    my_machines
}

fn part_1(lines: &[String]) -> i64 {
    let games = parse_lines(lines);

    games
        .into_iter()
        .map(|game| game.solve(0).unwrap_or_default())
        .sum::<usize>() as i64
}

fn part_2(lines: &[String]) -> i64 {
    let games = parse_lines(lines);

    games
        .into_iter()
        .map(|game| game.solve(10000000000000).unwrap_or_default())
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
mod q13_tests {
    use crate::utilities::read_lines;

    use super::{part_1, part_2};

    fn get_lines() -> Vec<String> {
        read_lines("./data/q13_test.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part_1(&get_lines()), 480);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part_2(&get_lines()), 875318608908);
    }
}
