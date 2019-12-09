use std::collections::HashMap;

#[derive(Debug)]
struct InstructionDef {
    pub name: &'static str,
    pub opcode: i32,
    pub inargs: u32,
    pub outargs: u32,
}

#[derive(Debug)]
struct ParsedInstruction {
    pub size: u32,
    pub def: &'static InstructionDef,
    pub words: Vec<String>,
}

const INSTRUCTIONS: [InstructionDef; 11] = [
    InstructionDef { name: "halt", opcode: 99, inargs: 0, outargs: 0 },
    InstructionDef { name: "add",  opcode:  1, inargs: 2, outargs: 1 },
    InstructionDef { name: "mul",  opcode:  2, inargs: 2, outargs: 1 },
    InstructionDef { name: "in",   opcode:  3, inargs: 0, outargs: 1 },
    InstructionDef { name: "out",  opcode:  4, inargs: 1, outargs: 0 },
    InstructionDef { name: "jnz",  opcode:  5, inargs: 2, outargs: 0 },
    InstructionDef { name: "jz",   opcode:  6, inargs: 2, outargs: 0 },
    InstructionDef { name: "less", opcode:  7, inargs: 2, outargs: 1 },
    InstructionDef { name: "cmp",  opcode:  8, inargs: 2, outargs: 1 },
    InstructionDef { name: "dd",   opcode: -1, inargs: 0, outargs: 0 },
    InstructionDef { name: "zero", opcode: -1, inargs: 0, outargs: 0 },
];

fn parse_label(labels: &HashMap<String,i32>, arg: &str) -> (i32, bool) {
    if arg.starts_with("&") {
        (labels[&arg[1..]], true)
    } else {
        (labels[arg], false)
    }
}

fn immediate_flag_for_index(word_i: usize) -> i32 {
    10i32.pow(1 + word_i as u32)
}

fn assemble_parsed_instruction(labels: &HashMap<String,i32>, parsed: &ParsedInstruction) -> Vec<i32> {
    if parsed.def.opcode < 0 {
        return match parsed.def.name {
            "dd"   => {
                let arg = &parsed.words[1];
                vec![match arg.parse::<i32>() {
                    Ok(x) => x,
                    Err(_) => {
                        let (arg_val, _) = parse_label(labels, arg);
                        arg_val
                    }
                }]
            }
            "zero" => vec![0; parsed.size as usize],
            _ => panic!(),
        };
    }

    let mut result = Vec::<i32>::new();
    let mut word_i = 1usize;
    let mut op_flags = 0i32;

    for _ in 0..parsed.def.inargs {
        let arg = &parsed.words[word_i];

        result.push(match arg.parse::<i32>() {
            Ok(x) => {
                op_flags += immediate_flag_for_index(word_i);
                x
            },
            Err(_) => {
                let (arg_val, imm) = parse_label(labels, arg);
                if imm { op_flags += immediate_flag_for_index(word_i) };
                arg_val
            }
        });

        word_i += 1
    }

    for _ in 0..parsed.def.outargs {
        let (arg_val, imm) = parse_label(labels, &parsed.words[word_i]);
        result.push(arg_val);
        if imm { op_flags += immediate_flag_for_index(word_i) };
        word_i += 1
    }

    result.insert(0, op_flags + parsed.def.opcode);

    result
}

fn parse_instruction_text(text: &str) -> ParsedInstruction {
    let no_commas = String::from(text).replace(",", " ");
    let words: Vec<String> = no_commas.split_whitespace().map(|x| String::from(x)).collect();

    let ins: &InstructionDef = INSTRUCTIONS
        .iter()
        .find(|x| x.name == words[0])
        .unwrap();

    if ins.opcode < 0 {
        return match words[0].as_str() {
            "dd"   => ParsedInstruction { size: 1, def: ins, words: words },
            "zero" => ParsedInstruction { size: words[1].parse::<u32>().unwrap(), def: ins, words: words },
            _ => panic!(),
        };
    }

    ParsedInstruction { size: 1 + ins.inargs + ins.outargs, def: ins, words: words }
}

pub fn assemble(path: &str, debug: bool) -> Vec<i32> {
    let source: Vec<String> = std::fs::read_to_string(path).unwrap()
        .replace(":", ":\n")
        .lines()
        .map(|x| String::from(x.trim()))
        .filter(|x| x.len() > 0 && !x.starts_with(";"))
        .collect();

    let mut address_labels = HashMap::<String,i32>::new();
    let mut cur_address = 0u32;
    let mut instructions = Vec::<ParsedInstruction>::new();

    for line in source {
        if line.ends_with(":") {
            address_labels.insert(String::from(line.replace(":", "")), cur_address as i32);
        } else {
            let parsed = parse_instruction_text(&line);
            cur_address += parsed.size;
            instructions.push(parsed);
        }
    }

    let mut output = Vec::<i32>::new();

    for ins in instructions {
        let addr = output.len();
        let asm = assemble_parsed_instruction(&address_labels, &ins);
        if debug { println!("{} : {:?} : {:?}", addr, ins.words, asm) };
        output.extend(asm);
    }
    
    if debug {
        println!("");
        println!("{:?}", output);
        println!("");
    }

    output
}