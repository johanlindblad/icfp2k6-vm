#![feature(plugin)]
#![plugin(bitfield)]
#![allow(dead_code)]

#[macro_use]
extern crate byteorder;

use std::io::prelude::*;
use std::fs::File;

mod operator;
mod instruction;
use self::instruction::Instruction;

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
    println!("First byte: {}", program[0]);
}
