use crate::vm::IntCodeMachine;
use permutohedron::Heap;

fn run_amplifier_circuit(tape: &Vec<i32>, phase_seq: &[i32; 5]) -> i32 {
    let a = IntCodeMachine::run_all(&tape, &[phase_seq[0], 0]).pop().unwrap();
    let b = IntCodeMachine::run_all(&tape, &[phase_seq[1], a]).pop().unwrap();
    let c = IntCodeMachine::run_all(&tape, &[phase_seq[2], b]).pop().unwrap();
    let d = IntCodeMachine::run_all(&tape, &[phase_seq[3], c]).pop().unwrap();
    let e = IntCodeMachine::run_all(&tape, &[phase_seq[4], d]).pop().unwrap();
    e
}

fn find_best_thruster_signal(tape: &Vec<i32>) -> i32 {
    let mut phases = [0i32,1,2,3,4];
    let mut heap = Heap::new(&mut phases);
    let mut best_signal = 0i32;
    while let Some(perm) = heap.next_permutation() {
        let new_signal = run_amplifier_circuit(tape, perm);
        if new_signal > best_signal { best_signal = new_signal };
    }
    best_signal
}

pub fn main() {
    let tape: Vec<i32> = std::fs::read_to_string("data/day7.txt").unwrap()
        .split(",")
        .map(|x| x.trim().parse().unwrap())
        .collect();

    let result0 = find_best_thruster_signal(&tape);
    let result1 = 0i32;

    println!("{} {}", result0, result1);
}