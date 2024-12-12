#![allow(dead_code)]
#![allow(unused_variables)]

mod q1;
mod q10;
mod q11;
mod q12;
mod q2;
mod q3;
mod q4;
mod q5;
mod q6;
mod q7;
mod q8;
mod q9;
mod utilities;

use crate::q12::solution;
use crate::utilities::read_lines;

fn main() {
    let filename = "./data/q12.txt";
    let lines = read_lines(filename);

    solution(lines)
}
