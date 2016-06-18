#![feature(slice_patterns)]
#![feature(plugin)]
#![plugin(bitfield)]
#![allow(dead_code)]

#[macro_use]
extern crate byteorder;

use std::io::prelude::*;
use std::fs::File;

mod operator;
mod instruction;
mod cpu;
use self::instruction::Instruction;
use self::cpu::Cpu;

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
    let mut cpu = Cpu::new();

    println!("Program length: {} 32-bit values", program.len());

    for i in 0.. {
        println!("{}", program[i].operator());
        cpu.apply(program[i].operator());
        println!("CPU state: {}", cpu)
    }

}
