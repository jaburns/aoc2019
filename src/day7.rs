use crate::day5::execute_tape;

fn run_amplifiers(tape: &Vec<i32>, phase_seq: &[i32; 5]) -> i32 {
    let a = execute_tape(&tape, &[phase_seq[0], 0]).pop().unwrap();
    let b = execute_tape(&tape, &[phase_seq[1], a]).pop().unwrap();
    let c = execute_tape(&tape, &[phase_seq[2], b]).pop().unwrap();
    let d = execute_tape(&tape, &[phase_seq[3], c]).pop().unwrap();
    let e = execute_tape(&tape, &[phase_seq[4], d]).pop().unwrap();
    e
}

pub fn main() {
    let tape: Vec<i32> = std::fs::read_to_string("data/day7.txt").unwrap()
        .split(",")
        .map(|x| x.trim().parse().unwrap())
        .collect();

    // for all permutations of [0,1,2,3,4]
    let result0 = run_amplifiers(&tape, &[0,1,2,3,4]);

    let result1 = 0i32;
    println!("{} {}", result0, result1);
}