pub mod defs {
    pub const I_HALT: i32 = 99;
    pub const I_ADD:  i32 = 01;
    pub const I_MUL:  i32 = 02;
    pub const I_IN:   i32 = 03;
    pub const I_OUT:  i32 = 04;
    pub const I_JNZ:  i32 = 05;
    pub const I_JZ:   i32 = 06;
    pub const I_LESS: i32 = 07;
    pub const I_CMP:  i32 = 08;
}

pub mod vm {
    use crate::intcode::defs::*;

    #[derive(PartialEq, Eq, Debug, Clone, Copy)]
    pub enum RunResult {
        RequiresInput,
        ProvidingOutput(i32),
        Halted,
    }

    pub struct IntCodeMachine {
        tape: Vec<i32>,
        ip: usize,
        last_result: Option<RunResult>,
        input_address: usize,
    }

    impl IntCodeMachine {
        pub fn new(init_tape: &[i32]) -> IntCodeMachine {
            IntCodeMachine {
                tape: Vec::from(init_tape),
                ip: 0,
                last_result: None,
                input_address: 0,
            }
        }

        pub fn provide_input(&mut self, input: i32) {
            self.tape[self.input_address] = input;
        }

        pub fn run(&mut self) -> RunResult {
            if self.last_result == Some(RunResult::Halted) {
                panic!("Cannot continue from halted state");
            }

            loop {
                match self.tape[self.ip] % 100 {
                    I_HALT => {
                        self.last_result = Some(RunResult::Halted);
                        break
                    },

                    I_ADD => { 
                        let arg0 = self.get_arg(0);
                        let arg1 = self.get_arg(1);
                        let arg2 = self.tape[self.ip + 3];
                        self.tape[arg2 as usize] = arg0 + arg1;
                        self.ip += 4
                    },

                    I_MUL => {
                        let arg0 = self.get_arg(0);
                        let arg1 = self.get_arg(1);
                        let arg2 = self.tape[self.ip + 3];
                        self.tape[arg2 as usize] = arg0 * arg1;
                        self.ip += 4
                    },

                    I_IN => {
                        self.input_address = self.tape[self.ip + 1] as usize;
                        self.last_result = Some(RunResult::RequiresInput);
                        self.ip += 2;
                        break
                    },

                    I_OUT => {
                        let arg0 = self.get_arg(0);
                        self.last_result = Some(RunResult::ProvidingOutput(arg0));
                        self.ip += 2;
                        break
                    },

                    I_JNZ => {
                        let arg0 = self.get_arg(0);
                        let arg1 = self.get_arg(1);
                        if arg0 != 0 {
                            self.ip = arg1 as usize
                        } else {
                            self.ip += 3
                        }
                    },

                    I_JZ => {
                        let arg0 = self.get_arg(0);
                        let arg1 = self.get_arg(1);
                        if arg0 == 0 {
                            self.ip = arg1 as usize
                        } else {
                            self.ip += 3
                        }
                    },

                    I_LESS => {
                        let arg0 = self.get_arg(0);
                        let arg1 = self.get_arg(1);
                        let arg2 = self.tape[self.ip + 3];
                        self.tape[arg2 as usize] = if arg0 < arg1 { 1 } else { 0 };
                        self.ip += 4
                    },

                    I_CMP => {
                        let arg0 = self.get_arg(0);
                        let arg1 = self.get_arg(1);
                        let arg2 = self.tape[self.ip + 3];
                        self.tape[arg2 as usize] = if arg0 == arg1 { 1 } else { 0 };
                        self.ip += 4
                    },

                    _ => panic!("ABORTED: Encountered unknown opcode {} at location {}", self.tape[self.ip], self.ip)
                }
            }

            self.last_result.unwrap()
        }

        fn get_arg(&self, arg: usize) -> i32 {
            let mut arg_digit = self.tape[self.ip] / 100;
            for _ in 0..arg {
                arg_digit /= 10;
            }

            if arg_digit % 10 != 0 { self.tape[self.ip+arg+1] } else { self.tape[self.tape[self.ip+arg+1] as usize] }
        }

        pub fn run_all(tape: &[i32], inputs: &[i32]) -> Vec<i32> {
            let mut vm = IntCodeMachine::new(tape);
            let mut input_ptr = 0usize;
            let mut outputs = Vec::<i32>::new();

            loop {
                match vm.run() {
                    RunResult::Halted => break,
                    RunResult::ProvidingOutput(x) => outputs.push(x),
                    RunResult::RequiresInput => { 
                        vm.provide_input(inputs[input_ptr]);
                        input_ptr += 1
                    }
                }
            }

            outputs
        }
    }
}

pub mod assembler {
    use std::collections::HashMap;
    use crate::intcode::defs::*;

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
        pub internal_labels: Vec<(String,u32)>,
    }

    const INSTRUCTIONS: [InstructionDef; 11] = [
        InstructionDef { name: "halt", opcode: I_HALT, inargs: 0, outargs: 0 },
        InstructionDef { name: "add",  opcode: I_ADD,  inargs: 2, outargs: 1 },
        InstructionDef { name: "mul",  opcode: I_MUL,  inargs: 2, outargs: 1 },
        InstructionDef { name: "in",   opcode: I_IN,   inargs: 0, outargs: 1 },
        InstructionDef { name: "out",  opcode: I_OUT,  inargs: 1, outargs: 0 },
        InstructionDef { name: "jnz",  opcode: I_JNZ,  inargs: 2, outargs: 0 },
        InstructionDef { name: "jz",   opcode: I_JZ,   inargs: 2, outargs: 0 },
        InstructionDef { name: "less", opcode: I_LESS, inargs: 2, outargs: 1 },
        InstructionDef { name: "cmp",  opcode: I_CMP,  inargs: 2, outargs: 1 },
        InstructionDef { name: "dd",   opcode: -1,     inargs: 0, outargs: 0 },
        InstructionDef { name: "fill", opcode: -1,     inargs: 0, outargs: 0 },
    ];

    fn parse_label(labels: &HashMap<String,i32>, arg: &str) -> (i32, bool) {
        if arg.starts_with("$") {
            (0, false)
        } else if arg.starts_with("&") {
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
            let arg = &parsed.words[1];
            let arg_val = match arg.parse::<i32>() {
                Ok(x) => x,
                Err(_) => {
                    let (arg_val, _) = parse_label(labels, arg);
                    arg_val
                }
            };

            return match parsed.def.name {
                "dd"   => vec![arg_val],
                "fill" => vec![arg_val; parsed.size as usize],
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
            result.push(parse_label(labels, &parsed.words[word_i]).0);
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

        let size = match words[0].as_str() {
            "dd"   => 1, 
            "fill" => words[2].parse::<u32>().unwrap(),
            _      => 1 + ins.inargs + ins.outargs,
        };

        let mut internal_labels = Vec::<(String,u32)>::new();
        for i in 1..words.len() {
            if words[i].starts_with("$") {
                internal_labels.push((String::from(&words[i][1..]), i as u32));
            }
        }

        ParsedInstruction { size: size, def: ins, words: words, internal_labels: internal_labels }
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

                for label in &parsed.internal_labels {
                    address_labels.insert(label.0.clone(), (cur_address + label.1) as i32);
                }

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
}