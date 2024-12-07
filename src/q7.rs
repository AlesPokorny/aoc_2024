use itertools::iproduct;
use rayon::prelude::*;
use std::time::Instant;

fn get_all_combinations(n_values: usize, operators: &[u8]) -> Vec<Vec<u8>> {
    let mut combinations: Vec<Vec<u8>> = vec![vec![]];

    for _ in 0..n_values {
        combinations = iproduct!(combinations.iter(), operators.iter())
            .map(|(v, x)| {
                let mut v1 = v.clone();
                v1.push(*x);
                v1
            })
            .collect();
    }
    combinations
}

fn check_if_between_bounds(values: &Vec<usize>, target_value: usize) -> bool {
    let n_ones = values.iter().filter(|x| **x == 1).count();

    let min_possible_value: usize = values.iter().sum::<usize>() - n_ones;
    let mut max_possible_value = 1_usize;

    for value in values {
        if *value == 1 {
            max_possible_value += value;
        } else {
            max_possible_value *= value;
        }
    }

    if target_value < (min_possible_value - n_ones) || target_value > max_possible_value {
        return false;
    }
    true
}

fn is_possible(operators: &[u8], values: Vec<usize>, target_value: usize) -> bool {
    let n_values = values.len() - 1;
    let combinations = get_all_combinations(n_values, operators);

    for combination in combinations {
        let mut possible_answer = values[0];
        for (value, operator) in values[1..=n_values].iter().zip(combination) {
            if operator == 0 {
                possible_answer += value;
            } else if operator == 1 {
                possible_answer *= value;
            } else if operator == 2 {
                let mut answer_string = possible_answer.to_string();
                answer_string.push_str(&value.to_string());
                possible_answer = answer_string.parse::<usize>().unwrap();
            }
        }
        if possible_answer == target_value {
            return true;
        }
    }
    false
}

fn part_1(lines: &[String]) -> i64 {
    let mut results: Vec<usize> = Vec::new();
    let operators: [u8; 2] = [0, 1];

    lines
        .into_par_iter()
        .map(|line| {
            let all_values: Vec<Vec<usize>> = line
                .split(": ")
                .map(|first_split| {
                    first_split
                        .split(" ")
                        .map(|second_split| second_split.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>()
                })
                .collect();

            let target_value = all_values[0][0];
            let values = all_values[1].clone();

            if !check_if_between_bounds(&values, target_value) {
                return 0;
            }

            if is_possible(&operators, values, target_value) {
                return target_value;
            }
            0
        })
        .collect_into_vec(&mut results);
    results.into_iter().sum::<usize>() as i64
}

fn part_2(lines: &[String]) -> i64 {
    let mut results: Vec<usize> = Vec::new();
    let operators: [u8; 3] = [0, 1, 2];

    lines
        .into_par_iter()
        .map(|line| {
            let all_values: Vec<Vec<usize>> = line
                .split(": ")
                .map(|first_split| {
                    first_split
                        .split(" ")
                        .map(|second_split| second_split.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>()
                })
                .collect();

            let target_value = all_values[0][0];
            let values = all_values[1].clone();

            if is_possible(&operators, values, target_value) {
                return target_value;
            }
            0
        })
        .collect_into_vec(&mut results);
    results.into_iter().sum::<usize>() as i64
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
mod q7_tests {
    use crate::utilities::read_lines;

    use super::{part_1, part_2};

    fn get_lines() -> Vec<String> {
        read_lines("./data/q7_test.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part_1(&get_lines()), 3749);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part_2(&get_lines()), 11387);
    }
}
