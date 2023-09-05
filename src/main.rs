#![allow(unused_variables)]

use std::{error::Error, io, path::Path};

const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;

const FONTSET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

struct Cpu {
    memory: [u8; 4096],
    display: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
    pc: u16,
    stack: Vec<u8>,
    index_register: u16,
    delay_timer: u8,
    sound_timer: u8,
    variable_registers: [u8; 16],
}
impl Cpu {
    fn setup_cpu() -> Self {
        Cpu {
            memory: [0; 4096],
            display: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
            pc: 0,
            stack: vec![],
            index_register: 0,
            delay_timer: 0,
            sound_timer: 0,
            variable_registers: [0; 16],
        }
    }
    fn load_rom(&self, path: &str) -> Result<(), io::Error> {
        //TODO: load ROM into memory at location 512
        todo!()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut cpu = Cpu::setup_cpu();
    cpu.load_rom("TODO")?;

    Ok(())
}
