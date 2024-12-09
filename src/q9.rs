use std::time::Instant;

fn parse_data(lines: &[String]) -> Vec<Vec<Option<usize>>> {
    lines[0]
        .chars()
        .enumerate()
        .map(|(i, entry)| {
            let n_spaces = entry.to_digit(10).unwrap();
            if i % 2 == 0 {
                let id = i / 2;
                (0..n_spaces)
                    .map(|_| Some(id))
                    .collect::<Vec<Option<usize>>>()
            } else {
                (0..n_spaces).map(|_| None).collect::<Vec<Option<usize>>>()
            }
        })
        .collect()
}

fn part_1(lines: &[String]) -> i64 {
    let mut drive: Vec<Option<usize>> = parse_data(lines).into_iter().flatten().collect();

    let mut i = 0_usize;
    while let Some(bit) = drive.get(i) {
        if bit.is_none() {
            loop {
                let stuff_from_the_end = drive.pop().unwrap();
                if stuff_from_the_end.is_some() {
                    drive[i] = stuff_from_the_end;
                    break;
                }
            }
        }
        i += 1;
    }

    drive
        .iter()
        .enumerate()
        .map(|(i, value)| match value {
            Some(x) => x * i,
            None => 0,
        })
        .sum::<usize>() as i64
}

fn part_2(lines: &[String]) -> i64 {
    let mut drive: Vec<Vec<Option<usize>>> = parse_data(lines);

    let drive_len = drive.len();

    for i in 0..drive_len {
        let file_index = drive_len - i - 1;

        let file = &drive[file_index];
        let file_len = file.len();
        if file_len > 0 && file[0].is_some() {
            for j in 0..(drive_len - i) {
                let possible_slot = &drive[j];
                let n_slots = possible_slot.iter().filter(|x| x.is_none()).count();

                if n_slots >= file_len {
                    for (k, file_bit) in drive[file_index].clone().into_iter().enumerate() {
                        if let Some((l, _)) = drive[j]
                            .clone()
                            .iter()
                            .enumerate()
                            .find(|(_, x)| x.is_none())
                        {
                            drive[j][l] = file_bit;
                        }
                    }
                    drive[file_index] = (0..file_len).map(|_| None).collect();
                    break;
                }
            }
        }
    }

    drive
        .into_iter()
        .flatten()
        .enumerate()
        .map(|(i, value)| match value {
            Some(x) => x * i,
            None => 0,
        })
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
mod q9_tests {
    use crate::utilities::read_lines;

    use super::{part_1, part_2};

    fn get_lines() -> Vec<String> {
        read_lines("./data/q9_test.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part_1(&get_lines()), 1928);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part_2(&get_lines()), 2858);
    }
}
