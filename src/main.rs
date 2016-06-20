#![feature(slice_patterns)]
#![feature(plugin)]
#![plugin(bitfield)]
#![allow(dead_code)]

use std::io::prelude::*;
use std::fs::File;

mod operator;
mod instruction;
mod cpu;
use self::cpu::Cpu;

fn read_program(path: &str) -> Vec<u32> {
    let mut file = File::open(path).unwrap();

    let mut program: Vec<u32> = Vec::new();

    let mut temp: [u8; 4] = [0; 4];

    while let Ok(4) = file.read(&mut temp) {
        let instruction = ((temp[0] as u32) << 24) + ((temp[1] as u32) << 16) + ((temp[2] as u32) << 8) + (temp[3] as u32);

        program.push(instruction);
    }

    program
}

fn main() {
    let program = read_program("../sandmark.umz");
    let mut cpu = Cpu::new(program.into_boxed_slice());


    cpu.run();
    /*println!("Program length: {} 32-bit values", program.len());

    for i in 0.. {
        println!("{}", program[i].operator());
        cpu.apply(program[i].operator());
        println!("CPU state: {}", cpu)
    }
*/
}
