extern crate byteorder;

use std::io::prelude::*;
use std::fs::File;
use std::io::Cursor;

use byteorder::{BigEndian, ReadBytesExt};

fn read_program(path: &str) -> Vec<u32> {
    let mut file = File::open(path).unwrap();

    let mut bytes: Vec<u8> = Vec::new();
    file.read_to_end(&mut bytes).unwrap();
    let mut buffer = Cursor::new(&bytes[..]);
    let mut program: Vec<u32> = Vec::new();

    while let Ok(u32_value) = buffer.read_u32::<BigEndian>() {
        program.push(u32_value);
    }

    program
}

fn main() {
    let program = read_program("../cmu.um");

    println!("Program length: {} 32-bit values", program.len());
    println!("First byte: {}", program[0]);
}
