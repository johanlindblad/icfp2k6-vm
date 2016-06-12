#![feature(plugin)]
#![allow(dead_code)]
#![plugin(bitfield)]

extern crate byteorder;

use std::io::prelude::*;
use std::fs::File;

bitfield!{Instruction,
    operator: 4,
    orthography_a: 3,
    other: 13,
    register_a: 4,
    register_b: 4,
    register_c: 4
}

fn read_program(path: &str) -> Vec<Instruction> {
    let mut file = File::open(path).unwrap();

    let mut program: Vec<Instruction> = Vec::new();
    let mut temp: [u8; 4] = [0; 4];

    while let Ok(4) = file.read(&mut temp) {
        let instruction = Instruction::new(temp);
        program.push(instruction);
    }

    program
}

fn main() {
    let program = read_program("../cmu.um");

    println!("Program length: {} 32-bit values", program.len());
    println!("First byte: {}", program[0].get_operator());
}
