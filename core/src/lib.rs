#![allow(unused_variables)]

use rand::random;
use std::{error::Error, io};

pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;

const MEM_SIZE: usize = 4096;
const V_REGS: usize = 16;
const STACK_SIZE: usize = 16;
const NUM_KEYS: usize = 16;

const START_ADDR: u16 = 0x200;

const FONTSET_SIZE: usize = 80;
const FONTSET: [u8; FONTSET_SIZE] = [
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
    memory: [u8; MEM_SIZE],
    display: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
    pc: u16,
    stack: [u16; STACK_SIZE],
    sp: u16,
    index_register: u16,
    variable_registers: [u8; V_REGS],
    delay_timer: u8,
    sound_timer: u8,
    keys: [bool; NUM_KEYS],
}
impl Cpu {
    pub fn setup_cpu() -> Self {
        let mut cpu = Cpu {
            memory: [0; MEM_SIZE],
            display: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
            pc: START_ADDR,
            stack: [0; STACK_SIZE],
            sp: 0,
            index_register: 0,
            variable_registers: [0; V_REGS],
            delay_timer: 0,
            sound_timer: 0,
            keys: [false; NUM_KEYS],
        };
        cpu.memory[..FONTSET_SIZE].copy_from_slice(&FONTSET);
        cpu
    }

    pub fn reset(&mut self) {
        self.memory = [0; MEM_SIZE];
        self.display = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
        self.pc = START_ADDR;
        self.stack = [0; STACK_SIZE];
        self.sp = 0;
        self.index_register = 0;
        self.variable_registers = [0; V_REGS];
        self.delay_timer = 0;
        self.sound_timer = 0;
        self.keys = [false; NUM_KEYS];
        self.memory[..FONTSET_SIZE].copy_from_slice(&FONTSET);
    }

    pub fn tick(&mut self) {
        let op = self.fetch();
        self.execute(op);
    }

    fn fetch(&mut self) -> u16 {
        let op: u16 = (((self.memory[self.pc as usize]) as u16) << 8)
            | self.memory[(self.pc + 1) as usize] as u16;
        self.pc += 2;
        op
    }

    fn execute(&mut self, op: u16) {
        let nibble1 = (op & 0xF000) >> 12;
        let nibble2 = (op & 0x0F00) >> 8;
        let nibble3 = (op & 0x00F0) >> 4;
        let nibble4 = op & 0x000F;

        match (nibble1, nibble2, nibble3, nibble4) {
            // NOOP
            (0, 0, 0, 0) => return,
            // CLEAR SCREEN
            (0, 0, 0xE, 0) => self.clear_screen(),
            // RETURN FROM SUBROUTINE
            (0, 0, x, y) => {
                self.pc = self.pop();
            }
            // JUMP
            (1, _, _, _) => {
                self.pc = op & 0xFFF;
            }
            // CALL SUBROUTINE
            (2, _, _, _) => {
                let nnn = op & 0xFFF;
                self.push(self.pc);
                self.pc = nnn;
            }
            // SKIP IF VX == 0xNN
            (3, _, _, _) => {
                let nn = op & 0xFF;
                if self.variable_registers[nibble2 as usize] == nn as u8 {
                    self.pc += 2;
                }
            }
            // SKIP IF VX != 0xNN
            (4, _, _, _) => {
                let nn = op & 0xFF;
                if self.variable_registers[nibble2 as usize] != nn as u8 {
                    self.pc += 2;
                }
            }
            // SKIP IF VX == VY
            (5, _, _, 0) => {
                if self.variable_registers[nibble2 as usize]
                    == self.variable_registers[nibble3 as usize]
                {
                    self.pc += 2;
                }
            }
            // VX = 0xNN
            (6, _, _, _) => {
                let nn = op & 0xFF;
                self.variable_registers[nibble2 as usize] = nn as u8;
            }
            // VX += NN
            (7, _, _, _) => {
                let nn = op & 0xFF;
                self.variable_registers[nibble2 as usize] =
                    self.variable_registers[nibble2 as usize].wrapping_add(nn as u8);
            }
            // VX = VY
            (8, _, _, 0) => {
                self.variable_registers[nibble2 as usize] =
                    self.variable_registers[nibble3 as usize];
            }
            // VX |= VY
            (8, _, _, 1) => {
                self.variable_registers[nibble2 as usize] |=
                    self.variable_registers[nibble3 as usize];
            }
            // VX &= VY
            (8, _, _, 2) => {
                self.variable_registers[nibble2 as usize] &=
                    self.variable_registers[nibble3 as usize];
            }
            // VX ^= VY
            (8, _, _, 3) => {
                self.variable_registers[nibble2 as usize] ^=
                    self.variable_registers[nibble3 as usize];
            }
            // VX += VY
            (8, _, _, 4) => {
                let (vx, carry) = self.variable_registers[nibble2 as usize]
                    .overflowing_add(self.variable_registers[nibble3 as usize]);
                let vf = if carry { 1 } else { 0 };

                self.variable_registers[nibble2 as usize] = vx;
                self.variable_registers[0xF] = vf;
            }
            // VX -= VY
            (8, _, _, 5) => {
                let (vx, carry) = self.variable_registers[nibble2 as usize]
                    .overflowing_sub(self.variable_registers[nibble3 as usize]);
                let vf = if carry { 0 } else { 1 };

                self.variable_registers[nibble2 as usize] = vx;
                self.variable_registers[0xF] = vf;
            }
            // VX >>= 1
            (8, _, _, 6) => {
                let vf = self.variable_registers[nibble2 as usize] & 1;
                self.variable_registers[nibble2 as usize] >>= 1;
                self.variable_registers[0xF] = vf;
            }
            // VY - VX
            (8, _, _, 7) => {
                let (vx, carry) = self.variable_registers[nibble3 as usize]
                    .overflowing_sub(self.variable_registers[nibble2 as usize]);
                let vf = if carry { 0 } else { 1 };

                self.variable_registers[nibble2 as usize] = vx;
                self.variable_registers[0xF] = vf;
            }
            // VX <<= 1
            (8, _, _, 0xE) => {
                let vf = (self.variable_registers[nibble2 as usize] >> 7) & 1;
                self.variable_registers[nibble2 as usize] <<= 1;
                self.variable_registers[0xF] = vf;
            }
            // SKIP if VX != VY
            (9, _, _, 0) => {
                if self.variable_registers[nibble2 as usize]
                    != self.variable_registers[nibble3 as usize]
                {
                    self.pc += 2;
                }
            }
            // I = 0xNNN
            (0xA, _, _, _) => {
                self.index_register = op & 0xFFF;
            }
            // JMP TO V0 + NNN
            (0xB, _, _, _) => {
                let nnn = op & 0xFFF;
                self.pc = (self.variable_registers[0] as u16) + nnn;
            }
            // VX = RAND & NN
            (0xC, _, _, _) => {
                let rand: u8 = random();
                let nn = (op & 0xFF) as u8;
                self.variable_registers[nibble2 as usize] = rand & nn;
            }
            // Display
            (0xD, _, _, _) => {
                let x = self.variable_registers[nibble2 as usize];
                let y = self.variable_registers[nibble3 as usize];

                let width: u8 = 8;
                let height = nibble4;

                // If VF is set: pixel is flipped
                let mut flipped = false;
            }
            (_, _, _, _) => unimplemented!("Opcode not implemented: {op}"),
        }
    }

    pub fn tick_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            if self.sound_timer == 1 {
                // TODO: BEEP
            }
            self.sound_timer -= 1;
        }
    }

    fn clear_screen(&mut self) {
        self.display = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
    }

    fn load_rom(&self, path: &str) -> Result<(), io::Error> {
        //TODO: load ROM into memory at location 512
        todo!()
    }

    fn push(&mut self, val: u16) {
        self.stack[self.sp as usize] = val;
        self.sp += 1;
    }
    fn pop(&mut self) -> u16 {
        self.sp -= 1;
        self.stack[self.sp as usize]
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut cpu = Cpu::setup_cpu();
    cpu.load_rom("TODO")?;

    Ok(())
}
