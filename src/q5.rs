use rayon::prelude::*;
use std::{collections::HashMap, time::Instant};

fn split_parts(lines: &[String]) -> (Vec<&String>, Vec<&String>) {
    let mut is_ordering_part = true;

    let mut ordering_part: Vec<&String> = Vec::new();
    let mut check_part: Vec<&String> = Vec::new();

    for line in lines {
        if line.is_empty() {
            is_ordering_part = false;
            continue;
        }

        if is_ordering_part {
            ordering_part.push(line);
        } else {
            check_part.push(line);
        }
    }

    (ordering_part, check_part)
}

fn format_input(lines: &[String]) -> (HashMap<usize, Vec<usize>>, Vec<Vec<usize>>) {
    let (ordering_part, check_part) = split_parts(lines);

    (
        format_ordering_part(ordering_part),
        format_check_part(check_part),
    )
}

fn format_ordering_part(ordering_part: Vec<&String>) -> HashMap<usize, Vec<usize>> {
    let mut ordering_map: HashMap<usize, Vec<usize>> = HashMap::new();

    for line in ordering_part {
        let split_line = line.split("|").collect::<Vec<&str>>();
        let number_before = split_line[0].parse::<usize>().unwrap();
        let number_after = split_line[1].parse::<usize>().unwrap();

        match ordering_map.get_mut(&number_before) {
            Some(vec_values) => {
                vec_values.push(number_after);
            }
            None => {
                ordering_map.insert(number_before, vec![number_after]);
            }
        }
    }
    ordering_map
}

fn format_check_part(check_part: Vec<&String>) -> Vec<Vec<usize>> {
    let mut formatted: Vec<Vec<usize>> = Vec::new();

    for line in check_part {
        let formatted_line: Vec<usize> = line
            .split(",")
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        formatted.push(formatted_line);
    }

    formatted
}

fn check_if_line_ok(line: &[usize], ordering_map: &HashMap<usize, Vec<usize>>) -> bool {
    let line_len = line.len();

    for (j, value) in line.iter().rev().enumerate() {
        let after_values = match ordering_map.get(value) {
            Some(value) => value,
            None => continue,
        };

        for before_value in line[0..line_len - j].iter() {
            if after_values.contains(before_value) {
                return false;
            }
        }
    }
    true
}

fn part_1(lines: &[String]) -> i64 {
    let (ordering_map, check_part) = format_input(lines);

    let mut middle_values: Vec<usize> = Vec::with_capacity(check_part.len());

    check_part
        .into_par_iter()
        .map(|line| {
            if check_if_line_ok(&line, &ordering_map) {
                line[line.len() / 2]
            } else {
                0_usize
            }
        })
        .collect_into_vec(&mut middle_values);

    middle_values.iter().sum::<usize>() as i64
}

fn part_2(lines: &[String]) -> i64 {
    let (ordering_map, check_part) = format_input(lines);

    let mut middle_values: Vec<usize> = Vec::with_capacity(check_part.len());

    check_part
        .into_par_iter()
        .map(|mut line| {
            let line_len = line.len();
            let mut n_iterations_for_fix = 0_usize;
            let mut is_broken = true;

            while is_broken {
                'values_loop: for (i, value) in line.iter().rev().enumerate() {
                    let after_values = match ordering_map.get(value) {
                        Some(value) => value,
                        None => continue,
                    };

                    for (j, before_value) in line[0..line_len - i].iter().enumerate() {
                        if after_values.contains(before_value) {
                            line.swap(line_len - i - 1, j);
                            n_iterations_for_fix += 1;
                            is_broken = true;
                            break 'values_loop;
                        }
                    }
                    is_broken = false;
                }

                if !is_broken {
                    break;
                }
            }
            if n_iterations_for_fix > 0 {
                line[line.len() / 2]
            } else {
                0_usize
            }
        })
        .collect_into_vec(&mut middle_values);
    middle_values.iter().sum::<usize>() as i64
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
mod q5_tests {
    use crate::utilities::read_lines;

    use super::{part_1, part_2};

    fn get_lines() -> Vec<String> {
        read_lines("./data/q5_test.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part_1(&get_lines()), 143);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part_2(&get_lines()), 123);
    }
}
