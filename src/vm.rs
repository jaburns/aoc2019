const IMI_HALT: i32 = 99;
const IMI_ADD:  i32 = 01;
const IMI_MUL:  i32 = 02;
const IMI_IN:   i32 = 03;
const IMI_OUT:  i32 = 04;
const IMI_JNZ:  i32 = 05;
const IMI_JZ:   i32 = 06;
const IMI_LESS: i32 = 07;
const IMI_EQ:   i32 = 08;

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
                IMI_HALT => {
                    self.last_result = Some(RunResult::Halted);
                    break
                },

                IMI_ADD => { 
                    let arg0 = self.get_arg(0);
                    let arg1 = self.get_arg(1);
                    let arg2 = self.tape[self.ip + 3];
                    self.tape[arg2 as usize] = arg0 + arg1;
                    self.ip += 4
                },

                IMI_MUL => {
                    let arg0 = self.get_arg(0);
                    let arg1 = self.get_arg(1);
                    let arg2 = self.tape[self.ip + 3];
                    self.tape[arg2 as usize] = arg0 * arg1;
                    self.ip += 4
                },

                IMI_IN => {
                    self.input_address = self.tape[self.ip + 1] as usize;
                    self.last_result = Some(RunResult::RequiresInput);
                    self.ip += 2;
                    break
                },

                IMI_OUT => {
                    let arg0 = self.get_arg(0);
                    self.last_result = Some(RunResult::ProvidingOutput(arg0));
                    self.ip += 2;
                    break
                },

                IMI_JNZ => {
                    let arg0 = self.get_arg(0);
                    let arg1 = self.get_arg(1);
                    if arg0 != 0 {
                        self.ip = arg1 as usize
                    } else {
                        self.ip += 3
                    }
                },

                IMI_JZ => {
                    let arg0 = self.get_arg(0);
                    let arg1 = self.get_arg(1);
                    if arg0 == 0 {
                        self.ip = arg1 as usize
                    } else {
                        self.ip += 3
                    }
                },

                IMI_LESS => {
                    let arg0 = self.get_arg(0);
                    let arg1 = self.get_arg(1);
                    let arg2 = self.tape[self.ip + 3];
                    self.tape[arg2 as usize] = if arg0 < arg1 { 1 } else { 0 };
                    self.ip += 4
                },

                IMI_EQ => {
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