use crate::assembler::assemble;
use crate::vm::IntCodeMachine;

pub fn main() {
    let mut digits: Vec<i32> = std::fs::read_to_string("data/day8.txt").unwrap()
        .trim()
        .chars()
        .map(|x| x.to_digit(10).unwrap() as i32)
        .collect();

    digits.push(99);

    let tape = assemble("intcode/day8.asm", false);

    let result0 = IntCodeMachine::run_all(&tape, &digits).pop().unwrap();
    let result1 = 0i32;

    println!("{} {}", result0, result1);
}