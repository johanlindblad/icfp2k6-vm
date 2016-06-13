use operator::Operator;

use std::fmt;

bitfield!{pub Instruction,
    operator: 4,
    orthography_register: 3,
    other: 13,
    registers: [4; 3],
}

impl Instruction {
    fn operator(&self) -> Operator {
        match self.get_operator() {
            0 => Operator::ConditionalMove(self.get_registers()),
            1 => Operator::ArrayIndex(self.get_registers()),
            2 => Operator::Addition(self.get_registers()),
            13 => Operator::Orthography(self.get_orthography_register(), 0 as u32),
            op => panic!("Operator {} not implemented", op)
        }
    }

    fn orthography_value(&self) {
        //self.get_other() as u32 &
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.operator())
    }
}
