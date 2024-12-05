fn safe_check(levels: Vec<i64>) -> bool {
    let mut prev_value = 0_i64;
    let mut increasing: bool = true;
    for (i, level) in levels.into_iter().enumerate() {
        if i == 0 {
            prev_value = level;
        } else if (1..=3).contains(&(level - prev_value).abs()) {
            if i == 1 {
                increasing = prev_value < level;
            } else if increasing != (level > prev_value) {
                return false;
            }
            prev_value = level;
            continue;
        } else {
            return false;
        }
    }
    true
}

fn load_reports(lines: Vec<String>) -> Vec<Vec<i64>> {
    lines
        .into_iter()
        .map(|le_string| {
            le_string
                .split(" ")
                .map(|x| x.parse::<i64>().unwrap())
                .collect()
        })
        .collect()
}

fn part_1(lines: Vec<String>) -> i64 {
    let reports: Vec<Vec<i64>> = load_reports(lines);

    let mut safe_counter = 0_i64;
    for levels in reports {
        if safe_check(levels) {
            safe_counter += 1;
        }
    }

    safe_counter
}

fn part_2(lines: Vec<String>) -> i64 {
    let reports: Vec<Vec<i64>> = load_reports(lines);

    let mut safe_counter = 0_i64;
    for levels in reports {
        if safe_check(levels.clone()) {
            safe_counter += 1;
        } else {
            for i in 0..levels.len() {
                let mut new_levels = levels.clone();
                new_levels.remove(i);
                if safe_check(new_levels) {
                    safe_counter += 1;
                    break;
                }
            }
        }
    }
    safe_counter
}

pub fn solution(lines: Vec<String>) {
    println!("Answer part 1: {}", part_1(lines.clone()));
    println!("Answer part 2: {}", part_2(lines));
}

#[cfg(test)]
mod q2_tests {
    use crate::utilities::read_lines;

    use super::{part_1, part_2};

    fn get_lines() -> Vec<String> {
        read_lines("./data/q2_test.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part_1(get_lines()), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part_2(get_lines()), 4);
    }
}
