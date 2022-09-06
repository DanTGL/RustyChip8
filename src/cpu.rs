use rand::random;

const WIDTH: usize = 64;
const HEIGHT: usize = 32;
const PROGRAM_START: usize = 0x200;

const FONT: [u8; 80] = [
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
pub struct CPU {
    pub registers: [u8; 16],
    pub status: u8,
    pub program_counter: u16,
    pub reg_i: u16,
    pub memory: [u8; 4096],
    pub stack: Vec<u8>,
    pub framebuffer: [u8; WIDTH * HEIGHT],
    pub vram_dirty: bool,
    pub sound_timer: u8,
    pub delay_timer: u8,
    pub prev_keypad: u16,
}

impl CPU {
    pub fn new() -> Self {
        let mut mem = [0; 4096];
        mem[0..FONT.len()].copy_from_slice(&FONT);
        
        CPU {
            registers: [0; 16],
            status: 0,
            program_counter: PROGRAM_START as u16,
            reg_i: 0,
            memory: mem,
            stack: vec![],
            framebuffer: [0; WIDTH * HEIGHT],
            vram_dirty: true,
            sound_timer: 0,
            delay_timer: 0,
            prev_keypad: 0,
        }
    }

    fn call_subroutine(&mut self, op1: u8, op2: u8) {
        self.stack.push((self.program_counter & 0xFF) as u8);
        self.stack.push(((self.program_counter >> 8) & 0xFF) as u8);
        self.program_counter = ((op1 as u16) & 0xF) << 8 | (op2 as u16);
    }

    fn ret_subroutine(&mut self) {
        self.program_counter = (self.stack.pop().unwrap() as u16) << 8 | (self.stack.pop().unwrap() as u16);
    }

    pub fn load_program(&mut self, program: Vec<u8>) {
        for (idx,val) in program.into_iter().enumerate() {
            self.memory[PROGRAM_START + idx] = val;
        }
    }

    pub fn execute_opcode(&mut self, keypad: u16) {
        //let opcode: u16 = ((self.memory[self.program_counter as usize + 1] as u16) << 8) | program[self.program_counter as usize] as u16;
        
        //self.program_counte
        //let X = (opcode >> 8) & 0xF; 

        let op2 = self.memory[self.program_counter as usize + 1];
        let op1 = self.memory[self.program_counter as usize];
        
        let X = op1 & 0xF;
        let Y = op2 >> 4;
        
        self.program_counter += 2;
        match (op1 & 0xF0) >> 4 {
            0x0 => match op2 {
                0xE0 => {
                    self.framebuffer = [0; WIDTH * HEIGHT];
                    self.vram_dirty = true;
                }
                0xEE => self.ret_subroutine(),
                _ => println!("Machine code routine calls are not supported"),
            },
            0x1 => {
                self.program_counter = nnn(op1, op2);

            }
            0x2 => self.call_subroutine(op1, op2),
            0x3 => {
                if self.registers[(op1 & 0xF) as usize] == op2 {
                    self.program_counter += 2;
                }
            },
            0x4 => {
                if self.registers[X as usize] != op2 {
                    self.program_counter += 2;
                }
            },
            0x5 => if (op2 & 0xF) == 0 {
                if self.registers[X as usize] == self.registers[Y as usize] {
                    self.program_counter += 2;
                }
            },
            0x6 => self.registers[X as usize] = op2,
            0x7 => self.registers[X as usize] = self.registers[X as usize].wrapping_add(op2),
            0x8 => match op2 & 0xF {
                0x0 => self.registers[X as usize] = self.registers[Y as usize],
                0x1 => self.registers[X as usize] |= self.registers[Y as usize],
                0x2 => self.registers[X as usize] &= self.registers[Y as usize],
                0x3 => self.registers[X as usize] ^= self.registers[Y as usize],
                0x4 => {
                    let carry;
                    (self.registers[X as usize], carry) = self.registers[X as usize].overflowing_add(self.registers[Y as usize]);
                    
                    self.registers[0xF] = carry as u8;
                },
                0x5 => {
                    let borrow;

                    (self.registers[X as usize], borrow) = self.registers[X as usize].overflowing_sub(self.registers[Y as usize]);
                    self.registers[0xF] = !borrow as u8;
                },
                0x6 => {
                    self.registers[0xF] = self.registers[X as usize] & 0x1;

                    self.registers[X as usize] >>= 1;
                },
                0x7 => {
                    let borrow;

                    (self.registers[X as usize], borrow) = self.registers[Y as usize].overflowing_sub(self.registers[X as usize]);
                    self.registers[0xF] = borrow as u8;

                },
                0xE => {
                    self.registers[0xF] = (self.registers[X as usize] & 0x80) >> 7;
                    self.registers[X as usize] <<= 1;
                },
                _ => println!("Unimplemented opcode: 0x{:x}{:x}", op1, op2)
            },
            0x9 => if op2 & 0xF == 0 {
                if self.registers[X as usize] != self.registers[Y as usize] {
                    self.program_counter += 2;
                }
            },
            0xA => self.reg_i = nnn(op1, op2),
            0xB => self.program_counter = nnn(op1, op2) + (self.registers[0] as u16),
            0xC => self.registers[X as usize] = rand::random::<u8>() & op2,
            0xD => {
                self.registers[0xF] = 0;

                let vx = self.registers[X as usize] as usize;
                let vy = self.registers[Y as usize] as usize;
                let height = op2 & 0xF;
                for line_num in 0..height {
                    let line = self.memory[(self.reg_i + line_num as u16) as usize ];
                    for col in 0..8 {
                        if line & (0x80 >> col) != 0 {
                            let index = (vx as usize + col as usize) + (vy as usize + line_num as usize) * WIDTH as usize;
                            if index >= self.framebuffer.len() {
                                break;
                            }

                            if self.framebuffer[index] != 0 {
                                self.registers[0xF] = 1;
                            }

                            self.framebuffer[(vx as usize + col as usize) + (vy as usize + line_num as usize) * WIDTH as usize] ^= 1;
                        }
                    }
                }

                self.vram_dirty = true;
            }
            0xE => {
                let keydown = keypad & (1 << self.registers[X as usize]) != 0;
                match op2 {
                    0x9E => if keydown {
                        self.program_counter += 2;
                    }

                    0xA1 => if !keydown {
                        self.program_counter += 2;
                    }

                    _ => ()
                }
            }

            0xF => match op2 {
                0x07 => self.registers[X as usize] = self.delay_timer,
                0x0A => {
                    let newly_pressed = (keypad ^ self.prev_keypad) & keypad;
                    if newly_pressed == 0 {
                        self.program_counter -= 2;
                    } else {

                        for i in 0..0x10 {
                            if ((1 << i) & newly_pressed) != 0 {
                                self.registers[X as usize] = i;
                                break;
                            }
                        }
                    }
                    
                }

                0x15 => self.delay_timer = self.registers[X as usize],
                0x18 => self.sound_timer = self.registers[X as usize],

                0x1E => self.reg_i += self.registers[X as usize] as u16,
                0x29 => self.reg_i = ((self.registers[X as usize] & 0xF) as u16) * 5,
                0x33 => {
                    let mut temp = self.registers[X as usize];
                    for i in 0..=2 {
                        self.memory[(self.reg_i + (2 - i)) as usize] = temp % 10;
                        temp /= 10;
                    }
                }

                0x55 => {
                    for i in 0..=X {
                        self.memory[(self.reg_i + i as u16) as usize] = self.registers[i as usize];
                    }
                }

                0x65 => {
                    for i in 0..=X {
                        self.registers[i as usize] = self.memory[(self.reg_i + i as u16) as usize];
                    }
                }

                _ => println!("Unimplemented opcode: 0x{:x}{:x}", op1, op2)

            }
            _ => println!("Unimplemented opcode: 0x{:x}{:x}", op1, op2)
        }

        self.prev_keypad = keypad;

    }
    
    pub fn interpret(&mut self) {
        self.program_counter = 0;

        loop {
            self.execute_opcode(0);
        }
    }
}

#[inline(always)]
fn nnn(operand1: u8, operand2: u8) -> u16 {
    return (((operand1 & 0xF) as u16) << 8) | operand2 as u16;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jmp() {
        let mut cpu = CPU::new();
        cpu.load_program(vec![0xDA, 0x1E]);
        
        assert_eq!(cpu.program_counter, 0, "Init Error: Program Counter not zero at initialization");
        
        cpu.execute_opcode(0);
        
        assert_eq!(cpu.program_counter, 0xEDA, "Instruction Error: The Jump Instruction is not working properly");
    }

    #[test]
    fn test_subroutine() {
        let mut cpu = CPU::new();

        cpu.load_program(vec![0x06, 0x20, 0x00, 0x00, 0x00, 0x00, 0xEE, 0x00]);
        
        assert_eq!(cpu.program_counter, 0, "Init Error: Program Counter not zero at initialization");
        
        cpu.execute_opcode(0);
        assert_eq!(cpu.program_counter, 0x006, "Instruction Error: Subroutine not called");
        cpu.execute_opcode(0);
        assert_eq!(cpu.program_counter, 0x002, "Instruction Error: Program did not return from subroutine");
    }
}