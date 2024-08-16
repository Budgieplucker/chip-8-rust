#![allow(unused)]

use rand::Rng;
use std::env;
use std::fs::File;
use std::io::Read;

pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;

const START_ADDR: u16 = 0x200;
const RAM_SIZE: usize = 4096;
const NUM_REGS: usize = 16;
const STACK_SIZE: usize = 16;
const NUM_KEYS: usize = 16;
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
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

pub struct Emu {
    pc: u16,
    ram: [u8; RAM_SIZE],
    pub screen: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
    v_reg: [u8; NUM_REGS],
    i_reg: u16,
    sp: u16,
    stack: [u16; STACK_SIZE],
    keys: [bool; NUM_KEYS],
    dt: u8,
    st: u8,
}

impl Emu {
    pub fn new() -> Self {
        let mut new_emu = Emu {
            pc: 0,
            ram: [0; RAM_SIZE],
            screen: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
            v_reg: [0; NUM_REGS],
            i_reg: 0,
            sp: 0,
            stack: [0; STACK_SIZE],
            keys: [false; NUM_KEYS],
            dt: 0,
            st: 0,
        };

        new_emu.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);

        return new_emu;
    }   

    pub fn reset(&mut self) {
        self.pc = 0x200;
        self.ram = [0; RAM_SIZE];
        self.screen = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
        self.v_reg = [0; NUM_REGS];
        self.i_reg = 0;
        self.sp = 0;
        self.stack = [0; STACK_SIZE];
        self.keys = [false; NUM_KEYS];
        self.dt = 0;
        self.st = 0;
        self.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);
    }

    // Looks into ram and finds the 
    pub fn fetch_opcode(&mut self) -> u16 {
        let upper_byte = self.ram[self.pc as usize] as u16;
        let lower_byte = self.ram[(self.pc + 1) as usize] as u16;
        let opcode = (upper_byte << 8) | lower_byte;
        
        // println!("Opcode: {:#2x}", opcode);
        
        self.pc += 2;
        return opcode;
    }

    // Takes a path and loads a file
    pub fn load_rom(&mut self, path: String) {
        // Load the data into a variable and then push it into a vector
        let mut data = File::open(path).expect("unable to open file...");
        let mut buffer = Vec::new();
        data.read_to_end(&mut buffer).unwrap();
        
        // Declare where in memory to put the data
        let start = START_ADDR as usize;
        let end = (START_ADDR as usize ) + buffer.len();
        
        // Load the data into the ram
        self.ram[start..end].copy_from_slice(&mut buffer as &[u8]);
    }

    pub fn pop(&mut self) -> u16 {
        self.sp -= 1;
        return self.stack[self.sp as usize]
    }

    fn push(&mut self, val: u16) {
        self.stack[self.sp as usize] = val;
        self.sp += 1;
    }

    // Decodes and executes instructions
    pub fn execute(&mut self, op: u16) {
        let digit_1 = (op & 0xF000) >> 12;
        let digit_2 = (op & 0x0F00) >> 8;
        let digit_3 = (op & 0x00F0) >> 4;
        let digit_4 = op & 0x000F;

        // Massive match statement to decode and execute opcodes/instructions
        match (digit_1, digit_2, digit_3, digit_4) {
            // NOP
            (0, 0, 0, 0) => return,
            // CLS
            (0, 0, 0xE, 0) => {
                self.screen = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
                println!("CLS");
            },

            // RET
            (0, 0, 0xE, 0xE) => {
                let ret_addr = self.pop();
                self.pc = ret_addr;
                println!("RET");
            },

            // JMP NNN
            (1, _, _, _) => {
                let nnn = op & 0xFFF;
                self.pc = nnn;
                println!("JMP");
            },

            // CALL NNN
            (2, _, _, _) => {
                let nnn = op & 0xFFF;
                self.push(self.pc);
                self.pc = nnn;
                println!("CALL NNN");
            },

            // SKIP VX == NN
            (3, _, _, _) => {
                let x = digit_2 as usize;
                let nn = (op & 0xFF) as u8;
                if self.v_reg[x] == nn {
                    self.pc += 2;
                }
                println!("SKP");
            },

            // SKIP VX != NN
            (4, _, _, _) => {
                let x = digit_2 as usize;
                let nn = (op & 0xFF) as u8;
                if self.v_reg[x] != nn {
                    self.pc += 2;
                }

                println!("SKP");
            },

            // SKIP VX == VY
            (5, _, _, _) => {
                let x = digit_2 as usize;
                let y = digit_3 as usize;
                if self.v_reg[x] == self.v_reg[y] {
                    self.pc += 2;
                }

                println!("SKP");
            },

            // VX = NN
            (6, _, _, _) => {
                let x = digit_2 as usize;
                let nn = (op & 0xFF) as u8;
                self.v_reg[x] = nn;

                println!("VX == NN");
            },

            // VX += NN
            (7, _, _, _) => {
                let x = digit_2 as usize;
                let nn = (op & 0xFF) as u8;
                self.v_reg[x] = self.v_reg[x].wrapping_add(nn);

                println!("VX += NN");
            },

            // VX = VY
            (8, _, _, 0) => {
                let x = digit_2 as usize;
                let y = digit_3 as usize;
                self.v_reg[x] = self.v_reg[y];

                println!("VX = VY");
            },

            // VX |= VY
            (8, _, _, 1) => {
                let x = digit_2 as usize;
                let y = digit_3 as usize;
                self.v_reg[x] |= self.v_reg[y];

                println!("VX |= VY");
            },

            // VX &= VY
            (8, _, _, 2) => {
                let x = digit_2 as usize;
                let y = digit_3 as usize;
                self.v_reg[x] &= self.v_reg[y];

                println!("VX &= VY");
            },

            // VX ^= VY
            (8, _, _, 3) => {
                let x = digit_2 as usize;
                let y = digit_3 as usize;
                self.v_reg[x] ^= self.v_reg[y];

                println!("VX ^= VY");
            },

            // VX += VY
            (8, _, _, 4) => {
                let x = digit_2 as usize;
                let y = digit_3 as usize;

                let (new_vx, carry) = self.v_reg[x].overflowing_add(self.v_reg[y]);
                let new_vf = if carry { 1 } else { 0 };

                self.v_reg[x] = new_vx;
                self.v_reg[0xF] = new_vf;

                println!("VX += VY");
            },

            // VX -= VY
            (8, _, _, 5) => {
                let x = digit_2 as usize;
                let y = digit_3 as usize;

                let (new_vx, borrow) = self.v_reg[x].overflowing_sub(self.v_reg[y]);
                let new_vf = if borrow { 0 } else { 1 };

                self.v_reg[x] = new_vx;
                self.v_reg[0xF] = new_vf;

                println!("VX -= VY");
            },

            // VX >>= 1
            (8, _, _, 6) => {
                let x = digit_2 as usize;
                let lsb = self.v_reg[x] & 1;
                self.v_reg[x] >>= 1;
                self.v_reg[0xF] = lsb;

                println!("VX >>= 1");
            },

            // VX = VY - VX
            (8, _, _, 7) => {
                let x = digit_2 as usize;
                let y = digit_3 as usize;

                let (new_vx, borrow) = self.v_reg[y].overflowing_sub(self.v_reg[x]);
                let new_vf = if borrow { 0 } else { 1 };

                self.v_reg[x] = new_vx;
                self.v_reg[0xF] = new_vf;

                println!("VX = VY - VX");
            },

            // VX <<= 1
            (8, _, _, 0xE) => {
                let x = digit_2 as usize;
                let msb = (self.v_reg[x] >> 7) & 1;
                self.v_reg[x] <<= 1;
                self.v_reg[0xF] = msb;

                println!("VX <<= 1");
            },

            // SKIP VX != VY
            (9, _, _, 0) => {
                let x = digit_2 as usize;
                let y = digit_3 as usize;
                if self.v_reg[x] != self.v_reg[y] {
                    self.pc += 2;
                }

                println!("SKIP VX != VY");
            },

            // I = NNN
            (0xA, _, _, _) => {
                let nnn = op & 0xFFF;
                self.i_reg = nnn;

                println!("I = NNN");
            },

            // JMP V0 + NNN
            (0xB, _, _, _) => {
                let nnn = op & 0xFFF;
                self.pc = (self.v_reg[0] as u16) + nnn;

                println!("JMP V0 + NNN");
            },

            // VX = rand() & NN
            (0xC, _, _, _) => {
                let x = digit_2 as usize;
                let nn = (op & 0xFF) as u8;
                let rng: u8 = rand::thread_rng().gen();
                self.v_reg[x] = rng & nn;

                println!("VX = rand() & NN");
            },

            // DRAW
            (0xD, _, _, _) => {

                println!("DRW");
                // Get the (x, y) coords for our sprite
                let x_coord = self.v_reg[digit_2 as usize] as u16;
                let y_coord = self.v_reg[digit_3 as usize] as u16;
                // The last digit determines how many rows high our sprite is
                let num_rows = digit_4;

                // Keep track if any pixels were flipped
                let mut flipped = false;
                // Iterate over each row of our sprite
                for y_line in 0..num_rows {
                    // Determine which memory address our row's data is stored
                    let addr = self.i_reg + y_line as u16;
                    let pixels = self.ram[addr as usize];
                    // Iterate over each column in our row
                    for x_line in 0..8 {
                        // Use a mask to fetch current pixel's bit. Only flip if a 1
                        if (pixels & (0b1000_0000 >> x_line)) != 0 {
                            // Sprites should wrap around screen, so apply modulo
                            let x = (x_coord + x_line) as usize % SCREEN_WIDTH;
                            let y = (y_coord + y_line) as usize % SCREEN_HEIGHT;

                            // Get our pixel's index in the 1D screen array
                            let idx = x + SCREEN_WIDTH * y;
                            // Check if we're about to flip the pixel and set
                            flipped |= self.screen[idx];
                            self.screen[idx] ^= true;
                        }
                    }
                }
            },
        
            // Default case
            _=>{
                println!("NOP")
            }
        }
    }

}