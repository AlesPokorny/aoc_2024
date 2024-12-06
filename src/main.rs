#![allow(dead_code)]
#![allow(unused_variables)]

mod q1;
mod q2;
mod q3;
mod q4;
mod q5;
mod q6;
mod utilities;

use crate::q6::solution;
use crate::utilities::read_lines;

fn main() {
    let filename = "./data/q6.txt";
    let lines = read_lines(filename);

    solution(lines)
}
