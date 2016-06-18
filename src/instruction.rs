use operator::Operator;

use std::fmt;

bitfield!{pub Instruction,
    operator: 4,
    orthography_register: 3,
    other: 16,
    registers: [3; 3],
}

impl Instruction {
    pub fn operator(&self) -> Operator {
        match self.get_operator() {
            0 => Operator::ConditionalMove(self.get_registers()),
            1 => Operator::ArrayIndex(self.get_registers()),
            2 => Operator::ArrayAmendment(self.get_registers()),
            3 => Operator::Addition(self.get_registers()),
            5 => Operator::Division(self.get_registers()),
            6 => Operator::NotAnd(self.get_registers()),
            13 => Operator::Orthography(self.get_orthography_register(), self.get_orthography_value()),
            op => panic!("Operator {} not implemented", op)
        }
    }

    fn get_orthography_value(&self) -> u32 {
        (self.get_other() as u32) << 9 &
        (self.get_registers()[0] as u32) << 6 &
        (self.get_registers()[1] as u32) << 3 &
        self.get_registers()[2] as u32
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.operator())
    }
}
