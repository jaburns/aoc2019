const IMI_HALT:   i32 = 99;
const IMI_ADD:    i32 =  1;
const IMI_MUL:    i32 =  2;
const IMI_INPUT:  i32 =  3;
const IMI_OUTPUT: i32 =  4;

fn arg_is_immediate(instruction: i32, arg: i32) -> bool {
    let mut ret = instruction / 100;
    for _ in 0..arg {
        ret /= 10;
    }
    ret % 10 != 0
}

fn execute_tape(tape: &[i32], inputs: &[i32]) -> Vec<i32> {
    let mut p = Vec::from(tape);
    let mut i = 0usize;
    let mut input_ptr = 0usize;
    let mut outputs = Vec::<i32>::new();

    loop {
        match p[i] % 100 {
            IMI_HALT => break,

            IMI_ADD => { 
                let arg0 = if arg_is_immediate(p[i], 0) { p[i+1] } else { p[p[i+1] as usize] };
                let arg1 = if arg_is_immediate(p[i], 1) { p[i+2] } else { p[p[i+2] as usize] };
                let addr_out = p[i+3];
                p[addr_out as usize] = arg0 + arg1;
                i += 4;
            },

            IMI_MUL => {
                let arg0 = if arg_is_immediate(p[i], 0) { p[i+1] } else { p[p[i+1] as usize] };
                let arg1 = if arg_is_immediate(p[i], 1) { p[i+2] } else { p[p[i+2] as usize] };
                let addr_out = p[i+3];
                p[addr_out as usize] = arg0 * arg1;
                i += 4;
            },

            IMI_INPUT => {
                let input = inputs[input_ptr];
                input_ptr += 1;
                let addr_out = p[i+1];
                p[addr_out as usize] = input;
                i += 2;
            },

            IMI_OUTPUT => {
                let arg0 = if arg_is_immediate(p[i], 0) { p[i+1] } else { p[p[i+1] as usize] };
                outputs.push(arg0);
                i += 2;
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

    let result0 = execute_tape(&tape, &[1]);

    for i in 0..result0.len() {
        println!("{}", result0[i]);
    }
}