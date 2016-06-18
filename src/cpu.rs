use operator::Operator;
use std::fmt;

#[derive(Debug)]
pub struct Cpu {
    registers: [u32; 8]
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            registers: [0; 8]
        }
    }

    // TODO: Figure out how to make bitfield copyable if instruction
    // would be preferable over operator
    pub fn apply(&mut self, operator: Operator) {
        match operator {
            // 0
            Operator::ConditionalMove([a, b, c]) => {
                if a != b && c > 0 {
                    self.registers[a as usize] = self.registers[b as usize];
                }
            },

            // 2
            Operator::ArrayAmendment([a, b, c]) => {
            },

            // 3
            Operator::Addition([a, b, c]) => {
                let value = self.registers[b as usize] + self.registers[c as usize];
                self.registers[a as usize] = value;
            },

            // 5
            Operator::Division([a, b, c]) => {
                if self.registers[c as usize] > 0 {
                    let value = self.registers[b as usize] / self.registers[c as usize];
                    self.registers[a as usize] = value;
                }
            },

            // 6
            Operator::NotAnd([a, b, c]) => {
                let value = (! self.registers[b as usize]) & (! self.registers[c as usize]);
                self.registers[a as usize] = value;
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
