use std::mem;
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
            let raw_instruction = unsafe {
                mem::transmute::<u32, [u8; 4]>(self.platters[0][self.program_counter])
            };
            let instruction = Instruction::new(raw_instruction);
            let operator = instruction.operator();

            //println!("{}", operator);
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
                println!("Array index into {} from platter {} index {}, value {}", a, platter_index, offset, self.platters[platter_index][offset]);
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
                let value = self.registers[b as usize] + self.registers[c as usize];
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

            // 10
            Operator::Output(char_register) => {
                let text = (self.registers[char_register as usize] as u8) as char;
                print!("{}", text);
            }

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
            _ => panic!("Operator not implemented: {}", operator)
        }
    }
}

impl fmt::Display for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
