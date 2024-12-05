use regex::Regex;
use std::time::Instant;

fn part_1(lines: &[String]) -> i64 {
    let mul_re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();

    let mut result = 0_i64;
    for (mul_string, [x_str, y_str]) in mul_re
        .captures_iter(&lines.join(" "))
        .map(|caps| caps.extract())
    {
        result += x_str.parse::<i64>().unwrap() * y_str.parse::<i64>().unwrap();
    }

    result
}

fn part_2(lines: &[String]) -> i64 {
    let mul_re = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)|do\(\)|don't\(\)").unwrap();
    let digits_re = Regex::new(r"([0-9]{1,3}),([0-9]{1,3})").unwrap();

    let mut result = 0_i64;
    let mut can_multiply = true;
    for (mul_string, []) in mul_re
        .captures_iter(&lines.join(" "))
        .map(|caps| caps.extract())
    {
        if mul_string == "do()" {
            can_multiply = true;
            continue;
        }
        if mul_string == "don't()" {
            can_multiply = false;
            continue;
        }

        if can_multiply {
            let Some((_, [x_str, y_str])) =
                digits_re.captures(mul_string).map(|caps| caps.extract())
            else {
                println!("Oops, something went wrong");
                return 0;
            };
            result += x_str.parse::<i64>().unwrap() * y_str.parse::<i64>().unwrap();
        }
    }

    result
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
mod q3_tests {
    use crate::utilities::read_lines;

    use super::{part_1, part_2};

    fn get_lines() -> Vec<String> {
        read_lines("./data/q3_test.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part_1(&get_lines()), 161);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part_2(&get_lines()), 48);
    }
}
