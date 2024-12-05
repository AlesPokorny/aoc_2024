use std::time::Instant;

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn add_step(&self, step: usize) -> Self {
        Self {
            x: self.x + step,
            y: self.y + step,
        }
    }

    fn in_bounds(&self, max_x: usize, max_y: usize) -> bool {
        if self.x > max_x || self.y > max_y {
            return false;
        }
        true
    }

    fn get_diag_points(&self) -> [Self; 4] {
        [
            Self {
                x: self.x - 1,
                y: self.y - 1,
            },
            Self {
                x: self.x + 1,
                y: self.y - 1,
            },
            Self {
                x: self.x - 1,
                y: self.y + 1,
            },
            Self {
                x: self.x + 1,
                y: self.y + 1,
            },
        ]
    }
}

fn transpose(lines: &[String]) -> Vec<String> {
    assert!(!lines.is_empty());
    (0..lines[0].len())
        .map(|i| {
            lines
                .iter()
                .map(|inner| inner.chars().rev().nth(i).unwrap())
                .collect::<String>()
        })
        .collect()
}

fn count_xmas(line: &String, backwards: bool) -> i64 {
    let xmas = if backwards { "SAMX" } else { "XMAS" };
    line.as_bytes()
        .windows(xmas.len())
        .filter(|&w| w == xmas.as_bytes())
        .count() as i64
}

fn part_1(lines: &Vec<String>) -> i64 {
    let mut total_xmas = 0_i64;
    for line in lines {
        total_xmas += count_xmas(line, false);
        total_xmas += count_xmas(line, true);
    }
    let transposed_lines = transpose(lines);
    for line in &transposed_lines {
        total_xmas += count_xmas(line, false);
        total_xmas += count_xmas(line, true);
    }

    // damn diagonals
    let max_x = lines[0].len() - 1;
    let max_y = lines.len() - 1;

    let mut starting_points: Vec<Point> = (0..=max_x).map(|x| Point::new(x, 0)).collect();
    starting_points.append(
        &mut (1..=max_x)
            .map(|y| Point::new(0, y))
            .collect::<Vec<Point>>(),
    );

    for starting_point in starting_points {
        let mut diag_chars: Vec<char> = Vec::new();
        let mut transposed_diag_chars: Vec<char> = Vec::new();

        let mut step = 0_usize;
        loop {
            let point = starting_point.add_step(step);
            if !point.in_bounds(max_x, max_y) {
                break;
            }
            diag_chars.push(lines[point.x].chars().nth(point.y).unwrap());
            transposed_diag_chars.push(transposed_lines[point.x].chars().nth(point.y).unwrap());
            step += 1;
        }
        let diag_string: String = diag_chars.into_iter().collect();
        let transposed_diag_string: String = transposed_diag_chars.into_iter().collect();

        total_xmas += count_xmas(&diag_string, false);
        total_xmas += count_xmas(&diag_string, true);
        total_xmas += count_xmas(&transposed_diag_string, false);
        total_xmas += count_xmas(&transposed_diag_string, true);
    }

    total_xmas
}

fn part_2(lines: &[String]) -> i64 {
    let mut total_x_mas = 0_i64;
    let max_x = lines[0].len() - 1;
    let max_y = lines.len() - 1;

    for (y, line) in lines[1..=max_y].iter().enumerate() {
        for (x, char) in line[1..=max_x].chars().enumerate() {
            let a_point = Point::new(x + 1, y + 1);
            if char == 'A' {
                let points = a_point.get_diag_points();
                if points.iter().any(|point| !point.in_bounds(max_x, max_y)) {
                    continue;
                }

                let letters: Vec<char> = points
                    .iter()
                    .map(|point| lines[point.y].chars().nth(point.x).unwrap())
                    .collect();

                if letters.iter().filter(|letter| **letter == 'M').count() == 2
                    && letters.iter().filter(|letter| **letter == 'S').count() == 2
                    && letters[0] != letters[3]
                {
                    total_x_mas += 1;
                }
            }
        }
    }

    total_x_mas
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
mod q4_tests {
    use crate::utilities::read_lines;

    use super::{part_1, part_2};

    fn get_lines() -> Vec<String> {
        read_lines("./data/q4_test.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part_1(&get_lines()), 18);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part_2(&get_lines()), 9);
    }
}
