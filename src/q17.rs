use std::time::Instant;

#[derive(Clone, Debug)]
struct Computer {
    a: usize,
    b: usize,
    c: usize,
    program: Vec<usize>,
    intruction_pointer: usize,
    output: Vec<usize>,
}

impl Computer {
    fn new(a: usize, b: usize, c: usize, program: Vec<usize>) -> Self {
        Self {
            a,
            b,
            c,
            program,
            intruction_pointer: 0_usize,
            output: Vec::new(),
        }
    }

    fn increase_pointer(&mut self) {
        self.intruction_pointer += 2;
    }

    fn get_operand(&self) -> usize {
        self.program[self.intruction_pointer + 1]
    }

    fn get_combo_operand(&self, literal_operand: usize) -> usize {
        match literal_operand {
            1..=3 => literal_operand,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            7 => panic!("invalid program"),
            _ => panic!("impossibru"),
        }
    }

    fn adv(&mut self, operand: usize) {
        self.a /= 2_usize.pow(self.get_combo_operand(operand) as u32);
        self.intruction_pointer += 2;
    }

    fn bxl(&mut self, operand: usize) {
        self.b ^= operand;
        self.intruction_pointer += 2;
    }

    fn bst(&mut self, operand: usize) {
        self.b = self.get_combo_operand(operand) % 8;
        self.intruction_pointer += 2;
    }

    fn jnz(&mut self, operand: usize) {
        if self.a == 0 {
            self.intruction_pointer += 2;
            return;
        }
        self.intruction_pointer = operand;
    }

    fn bxc(&mut self) {
        self.b ^= self.c;
        self.intruction_pointer += 2;
    }

    fn out(&mut self, operand: usize) {
        self.output.push(self.get_combo_operand(operand) % 8);
        self.intruction_pointer += 2;
    }

    fn bdv(&mut self, operand: usize) {
        self.b = self.a / 2_usize.pow(self.get_combo_operand(operand) as u32);
        self.intruction_pointer += 2;
    }

    fn cdv(&mut self, operand: usize) {
        self.c = self.a / 2_usize.pow(self.get_combo_operand(operand) as u32);
        self.intruction_pointer += 2;
    }

    fn get_output(&self) -> String {
        self.output
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }

    fn run(&mut self) {
        loop {
            match self.program.get(self.intruction_pointer) {
                Some(0) => self.adv(self.get_operand()),
                Some(1) => self.bxl(self.get_operand()),
                Some(2) => self.bst(self.get_operand()),
                Some(3) => self.jnz(self.get_operand()),
                Some(4) => self.bxc(),
                Some(5) => self.out(self.get_operand()),
                Some(6) => self.bdv(self.get_operand()),
                Some(7) => self.cdv(self.get_operand()),
                None => break,
                _ => panic!("oopsies"),
            }
        }
    }
}

fn parse_register(line: &str) -> usize {
    line.split(" ").last().unwrap().parse::<usize>().unwrap()
}

fn parse_data(lines: &[String]) -> Computer {
    let register_a = parse_register(&lines[0]);
    let register_b = parse_register(&lines[1]);
    let register_c = parse_register(&lines[2]);
    let program: Vec<usize> = lines[4]
        .split(" ")
        .last()
        .unwrap()
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    Computer::new(register_a, register_b, register_c, program)
}

fn part_1(lines: &[String]) -> String {
    let mut computer = parse_data(lines);

    computer.run();

    computer.get_output()
}

fn get_registry_value(offsets: &[usize]) -> usize {
    let mut value = 0;
    let len_offsets = offsets.len();
    for (i, offset) in offsets.iter().enumerate() {
        value += offset * 8_usize.pow((len_offsets - i) as u32);
    }
    value
}

fn part_2(lines: &[String]) -> usize {
    // solution by hand = ((((((((((((((((5*8)+6)*8+0)*8+0)*8+6)*8+4)*8+4)*8+6)*8+7)+0)*8+4)*8+0)*8+2)*8+5)*8+0)*8+5)*8+2
    let computer = parse_data(lines);

    let mut offset = 0;
    let mut offsets: Vec<usize> = Vec::new();
    let max_idx = computer.program.len() - 1;

    loop {
        let mut test_computer = computer.clone();
        let registry_value = get_registry_value(&offsets) + offset;
        test_computer.a = registry_value;
        test_computer.run();

        let offets_len = offsets.len();
        let a = &computer.program[(max_idx - offets_len)..=max_idx];

        if test_computer.output == computer.program[(max_idx - offets_len)..=max_idx] {
            offsets.push(offset);
            offset = 0;
            if offsets.len() - 1 == max_idx {
                break;
            }
            continue;
        }

        offset += 1;
        if offset > 7 {
            let prev_offset = offsets.pop().unwrap();
            offsets.push(prev_offset + 1);
            offset = 0;
        }
    }

    get_registry_value(&offsets) / 8
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
mod q16_tests {
    use crate::utilities::read_lines;

    use super::{part_1, part_2};

    fn get_lines() -> Vec<String> {
        read_lines("./data/q17_test.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part_1(&get_lines()), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part_2(&get_lines()), 117440);
    }
}
