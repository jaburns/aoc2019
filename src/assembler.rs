use std::collections::HashMap;

#[derive(PartialEq, Eq, Debug)]
enum InArg {
    Immediate(i32),
    Address(String),
}

#[derive(PartialEq, Eq, Debug)]
struct OutArg(String);

#[derive(PartialEq, Eq, Debug)]
enum Instruction {
    Halt, 
    Add(InArg, InArg, OutArg),
    Mul(InArg, InArg, OutArg),
    In(OutArg),
    Out(InArg),
    Jnz(InArg, InArg),
    Jz(InArg, InArg),
    Less(InArg, InArg, OutArg),
    Cmp(InArg, InArg, OutArg),
    Dd(InArg),
}

fn parse_in_arg(arg: &str) -> InArg {
    match arg.parse::<i32>() {
        Ok(x) => InArg::Immediate(x),
        Err(_) => InArg::Address(String::from(arg)),
    }
}

fn parse_instruction(text: &str) -> (i32, Instruction) {
    let no_commas = text.replace(",", " ");
    let words: Vec<&str> = no_commas.split_whitespace().collect();

    match words[0] {
        "halt" => (1, Instruction::Halt),
        "add"  => (4, Instruction::Add(parse_in_arg(words[1]), parse_in_arg(words[2]), OutArg(String::from(words[3])))),
        "mul"  => (4, Instruction::Mul(parse_in_arg(words[1]), parse_in_arg(words[2]), OutArg(String::from(words[3])))),
        "in"   => (2, Instruction::In(OutArg(String::from(words[1])))),
        "out"  => (2, Instruction::Out(parse_in_arg(words[1]))),
        "jnz"  => (3, Instruction::Jnz(parse_in_arg(words[1]), parse_in_arg(words[2]))),
        "jz"   => (3, Instruction::Jz(parse_in_arg(words[1]), parse_in_arg(words[2]))),
        "less" => (4, Instruction::Less(parse_in_arg(words[1]), parse_in_arg(words[2]), OutArg(String::from(words[3])))),
        "cmp"  => (4, Instruction::Cmp(parse_in_arg(words[1]), parse_in_arg(words[2]), OutArg(String::from(words[3])))),
        "dd"   => (1, Instruction::Dd(parse_in_arg(words[1]))),
        _ => panic!("Unexpected instruction")
    }
}

fn arg_is_immediate(arg: &InArg, place: u32) -> i32 {
    match arg {
        InArg::Immediate(_) => 10i32.pow(place + 2),
        InArg::Address(_) => 0,
    }
}

fn assemble_first_value(instruction: &Instruction) -> i32 {
    match instruction {
        Instruction::Halt          => 99,
        Instruction::Add(a, b, _)  => 1 + arg_is_immediate(a,0) + arg_is_immediate(b,1),
        Instruction::Mul(a, b, _)  => 2 + arg_is_immediate(a,0) + arg_is_immediate(b,1),
        Instruction::In(_)         => 3,
        Instruction::Out(a)        => 4 + arg_is_immediate(a,0),
        Instruction::Jnz(a, _)     => 5 + arg_is_immediate(a,0) + 1000,
        Instruction::Jz(a, _)      => 6 + arg_is_immediate(a,0) + 1000,
        Instruction::Less(a, b, _) => 7 + arg_is_immediate(a,0) + arg_is_immediate(b,1),
        Instruction::Cmp(a, b, _)  => 8 + arg_is_immediate(a,0) + arg_is_immediate(b,1),
        Instruction::Dd(_)         => 0,
    }
}

fn assemble_inarg(labels: &HashMap<String,i32>, arg: &InArg) -> i32 {
    match arg {
        InArg::Immediate(x) => *x,
        InArg::Address(x) => labels[x],
    }
}

fn assemble_outarg(labels: &HashMap<String,i32>, arg: &OutArg) -> i32 {
    let OutArg(x) = arg;
    labels[x]
}

fn assemble_instruction(labels: &HashMap<String,i32>, instruction: &Instruction) -> Vec<i32> {
    match instruction {
        Instruction::Halt          => vec![assemble_first_value(instruction)],
        Instruction::Add(a, b, o)  => vec![assemble_first_value(instruction), assemble_inarg(labels,a), assemble_inarg(labels,b), assemble_outarg(labels,o)],
        Instruction::Mul(a, b, o)  => vec![assemble_first_value(instruction), assemble_inarg(labels,a), assemble_inarg(labels,b), assemble_outarg(labels,o)],
        Instruction::In(o)         => vec![assemble_first_value(instruction), assemble_outarg(labels,o)],
        Instruction::Out(a)        => vec![assemble_first_value(instruction), assemble_inarg(labels,a)],
        Instruction::Jnz(a, b)     => vec![assemble_first_value(instruction), assemble_inarg(labels,a), assemble_inarg(labels,b)],
        Instruction::Jz(a, b)      => vec![assemble_first_value(instruction), assemble_inarg(labels,a), assemble_inarg(labels,b)],
        Instruction::Less(a, b, o) => vec![assemble_first_value(instruction), assemble_inarg(labels,a), assemble_inarg(labels,b), assemble_outarg(labels,o)],
        Instruction::Cmp(a, b, o)  => vec![assemble_first_value(instruction), assemble_inarg(labels,a), assemble_inarg(labels,b), assemble_outarg(labels,o)],
        Instruction::Dd(a)         => vec![assemble_inarg(labels,a)],
    }
}

pub fn assemble(path: &str, debug: bool) -> Vec<i32> {
    let source: Vec<String> = std::fs::read_to_string(path).unwrap()
        .replace(":", ":\n")
        .lines()
        .map(|x| String::from(x.trim()))
        .filter(|x| x.len() > 0 && !x.starts_with(";"))
        .collect();

    let mut address_labels = HashMap::<String,i32>::new();
    let mut cur_address = 0i32;
    let mut instructions = Vec::<Instruction>::new();

    for line in source {
        if line.ends_with(":") {
            address_labels.insert(String::from(line.replace(":", "")), cur_address);
        } else {
            let (size, instruction) = parse_instruction(&line);
            instructions.push(instruction);
            cur_address += size;
        }
    }

    if debug { println!("") };

    let mut output = Vec::<i32>::new();

    for ins in instructions {
        let addr = output.len();
        let asm = assemble_instruction(&address_labels, &ins);
        if debug { println!("{} : {:?} : {:?}", addr, ins, asm) };
        output.extend(asm);
    }
    
    if debug {
        println!("");
        println!("{:?}", output);
        println!("");
    }

    output
}