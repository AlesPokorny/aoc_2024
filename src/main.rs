#![allow(dead_code)]
#![allow(unused_variables)]

mod q1;
mod q2;
mod q3;
mod q4;
mod q5;
mod q6;
mod q7;
mod utilities;

use crate::q7::solution;
use crate::utilities::read_lines;

fn main() {
    let filename = "./data/q7.txt";
    let lines = read_lines(filename);

    solution(lines)
}
