use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

fn parse_data(lines: &[String]) -> (HashSet<String>, Vec<String>) {
    let mut available_patterns: HashSet<String> = HashSet::new();
    let mut display_designs: Vec<String> = Vec::new();

    let mut is_first_part = true;

    for line in lines {
        if line.is_empty() {
            is_first_part = false;
            continue;
        }
        if is_first_part {
            available_patterns = HashSet::from_iter(line.split(", ").map(|x| x.to_owned()));
            continue;
        }
        display_designs.push(line.to_owned());
    }

    (available_patterns, display_designs)
}

fn is_design_possible(
    available_patterns: &HashSet<String>,
    cache: &mut HashSet<String>,
    design: &str,
) -> bool {
    if cache.contains(design) {
        return true;
    }

    for pattern in available_patterns {
        let pattern_len = pattern.len();
        let design_len = design.len();
        if pattern == design {
            return true;
        }
        if pattern_len <= design_len {
            let len_diff = design_len - pattern_len;
            if design[len_diff..design_len] == *pattern {
                let new_design = &design[0..len_diff];
                let is_possible = is_design_possible(available_patterns, cache, new_design);
                if is_possible {
                    cache.insert(new_design.to_owned());
                    return true;
                }
            }
        }
    }

    false
}

fn get_n_designs(
    available_patterns: &HashSet<String>,
    cache: &mut HashMap<String, usize>,
    design: &str,
) -> usize {
    if let Some(n_designs) = cache.get(design) {
        return *n_designs;
    }

    let mut n_designs = 0;

    for pattern in available_patterns {
        let pattern_len = pattern.len();
        let design_len = design.len();
        if pattern == design {
            n_designs += 1;
        }
        if pattern_len <= design_len {
            let len_diff = design_len - pattern_len;
            if design[len_diff..design_len] == *pattern {
                let new_design = &design[0..len_diff];
                let n_new_designs = get_n_designs(available_patterns, cache, new_design);
                if n_new_designs > 0 {
                    cache.insert(new_design.to_owned(), n_new_designs);
                    n_designs += n_new_designs;
                }
            }
        }
    }

    n_designs
}

fn part_1(lines: &[String]) -> usize {
    let (available_patterns, display_designs) = parse_data(lines);
    let mut cache: HashSet<String> = HashSet::new();

    let mut n_possible_designs = 0;

    for desired_design in display_designs {
        if is_design_possible(&available_patterns, &mut cache, &desired_design) {
            n_possible_designs += 1;
        }
    }

    n_possible_designs
}

fn part_2(lines: &[String]) -> usize {
    let (available_patterns, display_designs) = parse_data(lines);
    let mut cache: HashMap<String, usize> = HashMap::new();

    let mut n_possible_designs = 0;

    for desired_design in display_designs {
        n_possible_designs += get_n_designs(&available_patterns, &mut cache, &desired_design);
    }

    n_possible_designs
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
mod q19_tests {
    use crate::utilities::read_lines;

    use super::{part_1, part_2};

    fn get_lines() -> Vec<String> {
        read_lines("./data/q19_test.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part_1(&get_lines()), 6);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part_2(&get_lines()), 16);
    }
}
