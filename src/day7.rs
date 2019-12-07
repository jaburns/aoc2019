use crate::vm::{IntCodeMachine,RunResult};
use permutohedron::Heap;

fn run_forward_amplifier_circuit(tape: &Vec<i32>, phase_seq: &[i32; 5]) -> i32 {
    let a = IntCodeMachine::run_all(&tape, &[phase_seq[0], 0]).pop().unwrap();
    let b = IntCodeMachine::run_all(&tape, &[phase_seq[1], a]).pop().unwrap();
    let c = IntCodeMachine::run_all(&tape, &[phase_seq[2], b]).pop().unwrap();
    let d = IntCodeMachine::run_all(&tape, &[phase_seq[3], c]).pop().unwrap();
    let e = IntCodeMachine::run_all(&tape, &[phase_seq[4], d]).pop().unwrap();
    e
}

fn run_feedback_amplifier_circuit(tape: &Vec<i32>, phase_seq: &[i32; 5]) -> i32 {
    let mut signal = 0i32;
    let mut machine_index = 0usize;
    let mut machines = [
        IntCodeMachine::new(tape),
        IntCodeMachine::new(tape),
        IntCodeMachine::new(tape),
        IntCodeMachine::new(tape),
        IntCodeMachine::new(tape)
    ];

    for i in 0..machines.len() {
        machines[i].run();
        machines[i].provide_input(phase_seq[i]);
    }

    loop {
        match machines[machine_index].run() {
            RunResult::RequiresInput => machines[machine_index].provide_input(signal),
            RunResult::Halted => break,
            RunResult::ProvidingOutput(_) => panic!("Expected input or halt")
        }

        match machines[machine_index].run() {
            RunResult::RequiresInput => panic!("Expected output or halt"),
            RunResult::Halted => break,
            RunResult::ProvidingOutput(x) => {
                signal = x;
                machine_index = (machine_index + 1) % 5
            }
        }
    }

    signal
}

fn find_best_thruster_signal(tape: &Vec<i32>, phases: &[i32;5], run: fn(&Vec<i32>, &[i32;5]) -> i32) -> i32 {
    let mut mut_phases = [0i32; 5];
    mut_phases.copy_from_slice(phases);

    let mut heap = Heap::new(&mut mut_phases);
    let mut best_signal = 0i32;
    while let Some(perm) = heap.next_permutation() {
        let new_signal = run(tape, perm);
        if new_signal > best_signal { best_signal = new_signal };
    }
    best_signal
}

pub fn main() {
    let tape: Vec<i32> = std::fs::read_to_string("data/day7.txt").unwrap()
        .split(",")
        .map(|x| x.trim().parse().unwrap())
        .collect();

    let result0 = find_best_thruster_signal(&tape, &[0,1,2,3,4], run_forward_amplifier_circuit);
    let result1 = find_best_thruster_signal(&tape, &[5,6,7,8,9], run_feedback_amplifier_circuit);

    println!("{} {}", result0, result1);
}