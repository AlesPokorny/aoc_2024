use itertools::Itertools;

fn split_to_vecs_and_sort(lines: Vec<String>) -> (Vec<i32>, Vec<i32>) {
    let mut left_values: Vec<i32> = Vec::with_capacity(lines.len());
    let mut right_values: Vec<i32> = Vec::with_capacity(lines.len());
    for line in lines {
        let values = line.split("   ").collect::<Vec<&str>>();
        left_values.push(values[0].parse::<i32>().unwrap());
        right_values.push(values[1].parse::<i32>().unwrap());
    }
    left_values.sort();
    right_values.sort();
    (left_values, right_values)
}

fn group_by_and_count(input: Vec<i32>) -> Vec<(i32, usize)> {
    input
        .into_iter()
        .group_by(|key| *key)
        .into_iter()
        .map(|(value, group)| (value, group.into_iter().count()))
        .collect()
}

fn part_1(lines: Vec<String>) -> i64 {
    let (left_values, right_values) = split_to_vecs_and_sort(lines);

    left_values
        .into_iter()
        .zip(right_values)
        .map(|(x, y)| (x - y).abs() as i64)
        .sum::<i64>()
}

fn part_2(lines: Vec<String>) -> i64 {
    let (left_values, right_values) = split_to_vecs_and_sort(lines);
    let left_group = group_by_and_count(left_values);
    let right_group = group_by_and_count(right_values);

    left_group
        .into_iter()
        .map(|(value, left_count)| {
            let right_vec = right_group
                .iter()
                .filter(|(right_value, _)| *right_value == value)
                .map(|(_, right_count)| *right_count)
                .collect::<Vec<usize>>();
            if right_vec.len() == 1 {
                ((value as usize) * left_count * right_vec[0]) as i64
            } else {
                0_i64
            }
        })
        .sum()
}

pub fn solution(lines: Vec<String>) {
    println!("Answer part 1: {}", part_1(lines.clone()));
    println!("Answer part 2: {}", part_2(lines));
}

#[cfg(test)]
mod q1_tests {
    use crate::utilities::read_lines;

    use super::{part_1, part_2};

    fn get_lines() -> Vec<String> {
        read_lines("./data/q1_test.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part_1(get_lines()), 11_i64);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part_2(get_lines()), 31_i64);
    }
}
