use std::collections::HashMap;
use std::time::Instant;

fn parse_data(lines: &[String]) -> Vec<usize> {
    lines[0]
        .split(" ")
        .map(|x| x.parse::<usize>().unwrap())
        .collect()
}

fn let_magic_happen(
    cache: &mut HashMap<(usize, usize), usize>,
    stone: usize,
    iteration: usize,
    max_iteration: usize,
) -> usize {
    let iterations_left = max_iteration - iteration;

    if iterations_left == 0 {
        return 1;
    }

    if let Some(answer) = cache.get(&(stone, iterations_left)) {
        return *answer;
    }
    let mut answer: usize;

    if stone == 0 {
        answer = let_magic_happen(cache, 1, iteration + 1, max_iteration);
    } else {
        let stone_len = stone.ilog10() + 1;

        if stone_len % 2 == 0 {
            let half_stone_len = stone_len / 2;
            let value = 10_usize.pow(half_stone_len);
            let stone_1 = stone / value;
            let stone_2 = stone % value;
            answer = let_magic_happen(cache, stone_1, iteration + 1, max_iteration);

            answer += let_magic_happen(cache, stone_2, iteration + 1, max_iteration);
        } else {
            answer = let_magic_happen(cache, stone * 2024, iteration + 1, max_iteration);
        }
    }
    cache.insert((stone, iterations_left), answer);
    answer
}

fn part_1(lines: &[String]) -> i64 {
    let stones = parse_data(lines);
    let mut cache: HashMap<(usize, usize), usize> = HashMap::new();
    let mut answer = 0;
    for stone in stones {
        answer += let_magic_happen(&mut cache, stone, 0, 25);
    }

    answer as i64
}

fn part_2(lines: &[String]) -> i64 {
    let stones = parse_data(lines);

    let mut cache: HashMap<(usize, usize), usize> = HashMap::new();

    let mut answer = 0;
    for stone in stones {
        answer += let_magic_happen(&mut cache, stone, 0, 75);
    }

    answer as i64
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
mod q11_tests {
    use crate::utilities::read_lines;

    use super::{part_1, part_2};

    fn get_lines() -> Vec<String> {
        read_lines("./data/q11_test.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part_1(&get_lines()), 55312);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part_2(&get_lines()), 81);
    }
}
