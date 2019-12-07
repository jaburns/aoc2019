const IMI_HALT: usize = 99;
const IMI_ADD:  usize =  1;
const IMI_MUL:  usize =  2;

fn execute_tape(tape: &Vec<usize>, noun: usize, verb: usize) -> usize {
    let mut p = Vec::from(tape.as_slice());
    let mut i = 0usize;

    p[1] = noun;
    p[2] = verb;

    loop {
        if p[i] == IMI_HALT { break }

        if p[i] == IMI_ADD { 
            let val_ina = p[p[i+1]];
            let val_inb = p[p[i+2]];
            let addr_out = p[i+3];
            p[addr_out] = val_ina + val_inb;
        }

        if p[i] == IMI_MUL {
            let val_ina = p[p[i+1]];
            let val_inb = p[p[i+2]];
            let addr_out = p[i+3];
            p[addr_out] = val_ina * val_inb;
        }

        i += 4;
    }

    p[0]
}

fn search_for_result(tape: &Vec<usize>, result: usize) -> (usize, usize) {
    let mut noun = 0usize;
    loop {
        for verb in 0..noun {
            let test = execute_tape(&tape, noun, verb);
            if test == result {
                return (noun, verb)
            }
        }
        noun += 1;
    }
}

pub fn day2() {
    let tape: Vec<usize> = std::fs::read_to_string("data/day2.txt").unwrap()
        .split(",")
        .map(|x| x.trim().parse().unwrap())
        .collect();

    let result0 = execute_tape(&tape, 12, 2);
    let (result1_noun, result1_verb) = search_for_result(&tape, 19690720);
    let result1 = 100 * result1_noun + result1_verb;

    println!("{} {}", result0, result1);
}