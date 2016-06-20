use operator::Operator;
use std::fmt;
use instruction::Instruction;

#[derive(Debug)]
pub struct Cpu {
    registers: [u32; 8],
    platters: Vec<Box<[u32]>>,
    program_counter: usize,
    halted: bool
}

impl Cpu {
    pub fn new(program: Box<[u32]>) -> Cpu {
        let mut platters = Vec::new();
        platters.push(program);


        Cpu {
            registers: [0; 8],
            platters: platters,
            program_counter: 0,
            halted: true
        }
    }

    pub fn run(&mut self) {
        self.halted = false;

        while self.halted == false {

            let raw_instruction = self.platters[0][self.program_counter];
            let instruction_array = [
                (raw_instruction >> 24) as u8,
                ((raw_instruction >> 16) & 255) as u8,
                ((raw_instruction >> 8) & 255) as u8,
                ((raw_instruction) & 255) as u8
            ];

            let instruction = Instruction::new(instruction_array);
            let operator = instruction.operator();

            //println!("{}", operator);
            //println!("{:b}", raw_instruction);
            //println!("{:?}", instruction_array);
            self.program_counter += 1;
            //println!("Finger: {}", self.program_counter);
            self.apply(operator);
            //println!("{:?}", self.registers);
        }
    }

    // TODO: Figure out how to make bitfield copyable if instruction
    // would be preferable over operator
    fn apply(&mut self, operator: Operator) {
        match operator {
            // 0
            Operator::ConditionalMove([a, b, c]) => {
                if self.registers[c as usize] > 0 {
                    self.registers[a as usize] = self.registers[b as usize];
                }
            },

            // 1
            Operator::ArrayIndex([a, b, c]) => {
                let platter_index = self.registers[b as usize] as usize;
                let offset = self.registers[c as usize] as usize;
                self.registers[a as usize] = self.platters[platter_index][offset];
            },

            // 2
            Operator::ArrayAmendment([a, b, c]) => {
                let platter_index = self.registers[a as usize] as usize;
                let ref mut platter = self.platters[platter_index];
                let offset = self.registers[b as usize] as usize;
                let value = self.registers[c as usize];
                platter[offset] = value;
            },

            // 3
            Operator::Addition([a, b, c]) => {
                let value = self.registers[b as usize].wrapping_add(self.registers[c as usize]);
                self.registers[a as usize] = value;
            },

            // 4
            Operator::Multiplication([a, b, c]) => {
                // 254
                let value = self.registers[b as usize].wrapping_mul(self.registers[c as usize]) % (!0);
                self.registers[a as usize] = value;
            },

            // 5
            Operator::Division([a, b, c]) => {
                let value = self.registers[b as usize] / self.registers[c as usize];
                self.registers[a as usize] = value;
            },

            // 6
            Operator::NotAnd([a, b, c]) => {
                let value = ! (self.registers[b as usize] & self.registers[c as usize]);
                self.registers[a as usize] = value;
            },

            // 7
            Operator::Halt => {
                self.halted = true
            },

            // 8
            Operator::Allocation(index_register, capacity_register) => {
                let capacity = self.registers[capacity_register as usize];
                self.platters.push(vec![0; capacity as usize].into_boxed_slice());
                self.registers[index_register as usize] = (self.platters.len() as u32) - 1;
            },

            // 9
            Operator::Abandonment(index_register) => {
                let index = self.registers[index_register as usize];

                if index == 0 {
                    panic!("Trying to abandon platter 0");
                }

                self.platters[index as usize] = Box::new([]);
            },

            // 10
            Operator::Output(char_register) => {
                let text = (self.registers[char_register as usize] as u8) as char;
                print!("{}", text);
            },

            Operator::Input(char_register) => {
                panic!("{}", "Input not implemented");
            },

            // 12
            Operator::LoadProgram(b, c) => {
                let array_index = self.registers[b as usize] as usize;
                let offset = self.registers[c as usize] as usize;

                if array_index > 0 {
                    self.platters[0] = self.platters[array_index].to_owned();
                }

                self.program_counter = offset;
            },

            // 13
            Operator::Orthography(register_number, value) => {
                self.registers[register_number as usize] = value
            }
        }
    }
}

impl fmt::Display for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
