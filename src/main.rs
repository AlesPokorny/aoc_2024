#![allow(dead_code)]
#![allow(unused_variables)]

mod q1;
mod q2;
mod q3;
mod q4;
mod q5;
mod q6;
mod q7;
mod q8;
mod utilities;

use crate::q8::solution;
use crate::utilities::read_lines;

fn main() {
    let filename = "./data/q8.txt";
    let lines = read_lines(filename);

    solution(lines)
}
