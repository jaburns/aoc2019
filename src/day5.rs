const IMI_HALT: i32 = 99;
const IMI_ADD:  i32 = 01;
const IMI_MUL:  i32 = 02;
const IMI_IN:   i32 = 03;
const IMI_OUT:  i32 = 04;
const IMI_JNZ:  i32 = 05;
const IMI_JZ:   i32 = 06;
const IMI_LESS: i32 = 07;
const IMI_EQ:   i32 = 08;

fn get_arg(tape: &[i32], ip: usize, arg: usize) -> i32 {
    let mut arg_digit = tape[ip] / 100;
    for _ in 0..arg {
        arg_digit /= 10;
    }

    if arg_digit % 10 != 0 { tape[ip+arg+1] } else { tape[tape[ip+arg+1] as usize] }
}

pub fn execute_tape(tape: &[i32], inputs: &[i32]) -> Vec<i32> {
    let mut p = Vec::from(tape);
    let mut i = 0usize;
    let mut input_ptr = 0usize;
    let mut outputs = Vec::<i32>::new();

    loop {
        match p[i] % 100 {
            IMI_HALT => break,

            IMI_ADD => { 
                let arg0 = get_arg(&p, i, 0);
                let arg1 = get_arg(&p, i, 1);
                let arg2 = p[i + 3];
                p[arg2 as usize] = arg0 + arg1;
                i += 4;
            },

            IMI_MUL => {
                let arg0 = get_arg(&p, i, 0);
                let arg1 = get_arg(&p, i, 1);
                let arg2 = p[i + 3];
                p[arg2 as usize] = arg0 * arg1;
                i += 4;
            },

            IMI_IN => {
                let input = inputs[input_ptr];
                input_ptr += 1;
                let arg0 = p[i + 1];
                p[arg0 as usize] = input;
                i += 2;
            },

            IMI_OUT => {
                let arg0 = get_arg(&p, i, 0);
                outputs.push(arg0);
                i += 2;
            },

            IMI_JNZ => {
                let arg0 = get_arg(&p, i, 0);
                let arg1 = get_arg(&p, i, 1);
                if arg0 != 0 {
                    i = arg1 as usize;
                } else {
                    i += 3;
                }
            },

            IMI_JZ => {
                let arg0 = get_arg(&p, i, 0);
                let arg1 = get_arg(&p, i, 1);
                if arg0 == 0 {
                    i = arg1 as usize;
                } else {
                    i += 3;
                }
            },

            IMI_LESS => {
                let arg0 = get_arg(&p, i, 0);
                let arg1 = get_arg(&p, i, 1);
                let arg2 = p[i + 3];
                p[arg2 as usize] = if arg0 < arg1 { 1 } else { 0 };
                i += 4;
            },

            IMI_EQ => {
                let arg0 = get_arg(&p, i, 0);
                let arg1 = get_arg(&p, i, 1);
                let arg2 = p[i + 3];
                p[arg2 as usize] = if arg0 == arg1 { 1 } else { 0 };
                i += 4;
            },

            _ => panic!("ABORTED: Encountered unknown opcode {} at location {}", p[i], i)
        }
    }

    outputs
}

pub fn main() {
    let tape: Vec<i32> = std::fs::read_to_string("data/day5.txt").unwrap()
        .split(",")
        .map(|x| x.trim().parse().unwrap())
        .collect();

    let results0 = execute_tape(&tape, &[1]);
    let results1 = execute_tape(&tape, &[5]);

    let result0 = results0[results0.len()-1];
    let result1 = results1[results1.len()-1];

    println!("{} {}", result0, result1);
}