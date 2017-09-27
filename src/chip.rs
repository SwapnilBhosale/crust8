use std::fmt;
use std::fmt::Debug;
use cpu::Cpu;
use rand::*;
use rand::Rng;

const MEMORY_SIZE : usize = 4 * 1024;
const GRAPHICS_SIZE : usize = 64*32;
const DEF_PC_LOC : usize = 0x200;



pub struct Chip{
    pub memory : [u8;MEMORY_SIZE] ,
    pub cpu : Cpu,
    pub gfx : [u8;GRAPHICS_SIZE] 
}

impl Default for Chip {
    fn default() -> Chip{
        Chip{
            memory : [0;MEMORY_SIZE],
            cpu : Cpu::default(),
            gfx : [0;GRAPHICS_SIZE]
        }
    }
}

impl Chip{
    pub fn start_cpu(&mut self,program : &Vec<u8>){
        let char_set = [
            0xF0,0x90,0x90,0x90,0xF0,
            0x20,0x60,0x20,0x20,0x70,
            0xF0,0x10,0xF0,0x80,0xF0,
            0xF0,0x10,0xF0,0x10,0xF0,
            0x90,0x90,0xF0,0x10,0x10,
            0xF0,0x80,0xF0,0x10,0xF0,
            0xF0,0x80,0xF0,0x90,0xF0,
            0xF0,0x10,0x20,0x40,0x40,
            0xF0,0x90,0xF0,0x90,0xF0,
            0xF0,0x90,0xF0,0x10,0xF0,
            0xF0,0x90,0xF0,0x90,0x90,
            0xE0,0x90,0xE0,0x90,0xE0,
            0xF0,0x80,0x80,0x80,0xF0,
            0xE0,0x90,0x90,0x90,0xE0,
            0xF0,0x80,0xF0,0x80,0xF0,
            0xF0,0x80,0xF0,0x80,0x80, 
            ];

        for i in (0..80){
            self.memory[i] = char_set[i];
        }

        self.cpu.reg_pc = DEF_PC_LOC ;
        let mut i = 512;
        for b in program.iter(){
            self.memory[i] = *b;
            i += 1;
        }
    }

    pub fn run(&mut self){
        let mut is_draw_needed = false;
        // Change this to OpenGL::V2_1 if not working.
        
        // Create an Glutin window.
        
    }

    pub fn execute(&mut self) -> bool {
        let mut is_draw_needed = false;
        let mut opcode = 0;
        //println!(" {:x} {:x} {:x}",self.cpu.reg_pc,self.memory[self.cpu.reg_pc],self.memory[self.cpu.reg_pc + 1]);
        opcode = ((self.memory[self.cpu.reg_pc] as u16) << 8) | self.memory[self.cpu.reg_pc + 1] as u16;
        //println!("Opcode : {:x}",opcode);
        match opcode & 0xF000 {
            0x0000 => {
                match opcode & 0x000F {
                    0x0000 => {
                        for i in(0..GRAPHICS_SIZE){
                            self.gfx[i]=0;
                        }
                        self.cpu.reg_pc += 2;
                        is_draw_needed = true;
                    }
                    0x000E => {
                        self.cpu.reg_sp -= 1;
                        self.cpu.reg_pc = self.cpu.stack[self.cpu.reg_sp as usize] as usize; 
                        self.cpu.reg_pc += 2;
                    },
                     _ => {panic!("invalid instructiion in 0x0000- {}", opcode);}
                }
            }
            0x1000 => {
                //println!("jumping to address {:x}", opcode & 0x0FFF);
                self.cpu.reg_pc = (opcode & 0x0FFF) as usize;
            }, 
            0x2000 => {
                self.cpu.stack[self.cpu.reg_sp as usize] = self.cpu.reg_pc as u16;
                self.cpu.reg_sp += 1;
                self.cpu.reg_pc = (opcode & 0x0FFF) as usize;
            },
            0x3000 => {
                let reg_val = self.cpu.reg_gpr[((opcode & 0x0F00) >> 8) as usize];
                let nn_val = (opcode & 0x00FF) as u8;
                if reg_val == nn_val {
                    self.cpu.reg_pc += 4;
                }else{
                    self.cpu.reg_pc += 2;
                }
            },
            0x4000 => {
                if self.cpu.reg_gpr[((opcode & 0x0F00) >> 8) as usize] as u8 != (opcode & 0x00FF) as u8{
                    self.cpu.reg_pc += 4;
                }else{
                    self.cpu.reg_pc += 2;
                }
            }
            0x5000 => {
                if self.cpu.reg_gpr[((opcode & 0x0F00) >> 8) as usize] == self.cpu.reg_gpr[((opcode & 0x00F0) >> 4) as usize] {
                    self.cpu.reg_pc += 4;
                }else {
                   self.cpu.reg_pc += 2; 
                }
            },
            0x6000 => {
                self.cpu.reg_gpr[((opcode & 0x0F00) >> 8) as usize] = (opcode & 0x00FF) as u8; 
                self.cpu.reg_pc += 2;
            },
            0x7000 => {
                self.cpu.reg_gpr[((opcode & 0x0F00) >> 8) as usize] = self.cpu.reg_gpr[((opcode & 0x0F00) >> 8) as usize].wrapping_add((opcode & 0x00FF) as u8);
                self.cpu.reg_pc += 2;
            },
            0x8000 => {
                match opcode & 0x000F {
                    0x0000 => {
                        self.cpu.reg_gpr[((opcode & 0x0F00) >> 8 ) as usize] = self.cpu.reg_gpr[((opcode & 0x00F0)>>4) as usize];
                        self.cpu.reg_pc += 2;
                    },
                    0x0001 => {
                        self.cpu.reg_gpr[((opcode & 0x0F00) >> 8) as usize] |= self.cpu.reg_gpr[((opcode & 0x00F0) >> 4) as usize];
                        self.cpu.reg_pc += 2;
                    },
                    0x0002 => {
                        self.cpu.reg_gpr[((opcode & 0x0F00) >> 8) as usize] &= self.cpu.reg_gpr[((opcode & 0x00F0) >> 4) as usize];
                        self.cpu.reg_pc += 2;
                    },
                    0x0003 => {
                        self.cpu.reg_gpr[((opcode & 0x0F00) >> 8) as usize] ^= self.cpu.reg_gpr[((opcode & 0x00F0) >> 4) as usize];
                        self.cpu.reg_pc += 2;
                    },
                    0x0004 => {
                        let r = self.cpu.reg_gpr[((opcode & 0x00F0) >> 4) as usize] as u16 + self.cpu.reg_gpr[((opcode & 0x0F00) >> 8) as usize] as u16;
                        if r > 255 { 
                            self.cpu.reg_gpr[15] = 1;
                        }else{
                            self.cpu.reg_gpr[15] = 0;
                        }
                        self.cpu.reg_gpr[((opcode & 0x0F00) >> 8) as usize] = r as u8;
                        self.cpu.reg_pc += 2;
                    },
                    0x0005 => {
                        self.cpu.reg_gpr[15] = if self.cpu.reg_gpr[((opcode & 0x00F0) >> 4) as usize] > self.cpu.reg_gpr[((opcode & 0x0F00) >> 8) as usize]{
                            0
                        }
                        else{
                            1
                        };
                        self.cpu.reg_gpr[((opcode & 0x0F00) >> 8) as usize] = self.cpu.reg_gpr[((opcode & 0x0F00) >> 8) as usize].wrapping_sub(self.cpu.reg_gpr[((opcode & 0x00F0) >> 4) as usize]);
                        self.cpu.reg_pc += 2;
                    },
                    0x0006 => {
                        self.cpu.reg_gpr[15] = self.cpu.reg_gpr[((opcode & 0x0F00) >> 8) as usize] & 0x1;
                        self.cpu.reg_gpr[((opcode & 0x0F00) >> 8) as usize] = self.cpu.reg_gpr[ ((opcode & 0x00F0) >> 4) as usize] << 1;
                        self.cpu.reg_pc += 2;
                    },
                    0x0007 => {
                        self.cpu.reg_gpr[15] = if self.cpu.reg_gpr[((opcode & 0x0F00) >> 8) as usize] > self.cpu.reg_gpr[((opcode & 0x00F0) >> 4) as usize]{
                            0
                        }
                        else{
                            1
                        };
                        self.cpu.reg_gpr[((opcode & 0x0F00) >> 8) as usize] = self.cpu.reg_gpr[((opcode & 0x00F0) >> 4) as usize].wrapping_sub(self.cpu.reg_gpr[((opcode & 0x0F00) >> 8) as usize]);
                        self.cpu.reg_pc += 2;
                    },
                    0x000E => {
                        self.cpu.reg_gpr[15] =  self.cpu.reg_gpr[((opcode & 0x00F0) >> 4) as usize] >> 7;
                        self.cpu.reg_gpr[((opcode & 0x0F00) >> 8) as usize] = self.cpu.reg_gpr[ ((opcode & 0x00F0) >> 4) as usize ] >> 1 ;
                        self.cpu.reg_pc += 2;
                    },
                     _ => {panic!("invalid instructiion in 0x8000- {}", opcode);} 
                }
            },
            0x9000 => {
                if self.cpu.reg_gpr[((opcode & 0x0F00) >> 8) as usize] != self.cpu.reg_gpr[((opcode & 0x00F0) >> 4) as usize]{
                    self.cpu.reg_pc += 4;
                }
                else{
                    self.cpu.reg_pc += 2;
                } 
            },
            0xA000 => {
                self.cpu.reg_I = opcode & 0x0FFF;
                self.cpu.reg_pc += 2;
            },
            0xB000 => {
                self.cpu.reg_pc = self.cpu.reg_gpr[0] as usize + (opcode & 0x0FFF) as usize;
            },
            0xC000 => {
                let r = random::<u8>() & (opcode & 0x00FF) as u8;
                //println!("generation rand - {}", r);

                self.cpu.reg_gpr[((opcode & 0x0F00) >> 8) as usize] = r;
                self.cpu.reg_pc += 2;
            }
            0xD000 => {
                let x = self.cpu.reg_gpr[((opcode & 0x0F00) >> 8) as usize] as u32;
                let y = self.cpu.reg_gpr[((opcode & 0x00F0) >> 4) as usize] as u32;

                //println!("{:x}{:x}",x,y);
                let n = (opcode & 0x000F) as u32;
                self.cpu.reg_gpr[0x0F] = 0;
                let mut pixel = 0;
                for yline in (0..n as u32){
                    pixel = self.memory[(self.cpu.reg_I+yline as u16) as usize];
                    for xline in (0..8 as u32){
                        if (pixel & (0x80 >> xline)) != 0{
                            //println!("x - {}, xline - {}, y = {}, yline - {}", x, xline,  y, yline);
                            
                            if self.gfx[(x + xline + ((y+yline)*64)) as usize] == 1{
                                self.cpu.reg_gpr[0x0F] = 1;
                            }
                            self.gfx[(x + xline + ((y+yline)*64)) as usize] ^= 1;
                        }
                    }
                }
                
                is_draw_needed = true;
                self.cpu.reg_pc += 2;

            },
            0xE000 => {
                match opcode & 0x00FF {
                    0x009E => {
                        if self.cpu.keys[self.cpu.reg_gpr[((opcode & 0x0F00) >> 8) as usize] as usize] != 0{
                            self.cpu.reg_pc += 4;
                        }
                        else{
                            self.cpu.reg_pc += 2;
                        }
                    },
                    0x00A1 => {
                       if self.cpu.keys[self.cpu.reg_gpr[((opcode & 0x0F00) >> 8) as usize] as usize] == 0 {
                           self.cpu.reg_pc += 4;
                       }else{
                           self.cpu.reg_pc += 2;
                       }
                    },
                    _ => {panic!("invalid instructiion in 0xE000 - {}", opcode);} 
                }
            },
            0xF000 => {
                match opcode & 0x00FF {
                    0x0007 => {
                        self.cpu.reg_gpr[((opcode & 0x0F00) >> 8) as usize] = self.cpu.reg_dt; 
                        self.cpu.reg_pc += 2; 
                    },
                    0x000A => {
                        let mut key_pressed = false;
                        for i in (0..16 as usize){
                            if self.cpu.keys[i] != 0{
                                self.cpu.reg_gpr[((opcode & 0x0F00) >> 8) as usize] = i as u8;
                                key_pressed = true;
                            }
                        }
                        if key_pressed{
                            self.cpu.reg_pc += 2;
                        }
                    },
                    0x0015 => {
                        self.cpu.reg_dt = self.cpu.reg_gpr[((opcode & 0x0F00) >> 8) as usize];
                        self.cpu.reg_pc += 2;
                    },
                    0x0018 => {
                        self.cpu.reg_st = self.cpu.reg_gpr[((opcode & 0x0F00) >> 8) as usize];
                        self.cpu.reg_pc += 2;
                    },
                    0x001E => {
                        self.cpu.reg_I += self.cpu.reg_gpr[((opcode & 0x0F00) >> 8) as usize] as u16;
                        self.cpu.reg_pc += 2;
                    },
                    0x0029 => {
                        self.cpu.reg_I = self.cpu.reg_gpr[((opcode & 0x0F00) >> 8) as usize] as u16 * 5;
                        self.cpu.reg_pc += 2;
                    },
                    0x0033 => {
                        let mut v = self.cpu.reg_gpr[((opcode & 0x0F00) >> 8) as usize];
                        self.memory[self.cpu.reg_I as usize] = v % 10;
                        v = v / 10;
                        self.memory[(self.cpu.reg_I + 1) as usize] = v % 10;
                        v = v / 10;
                        self.memory[(self.cpu.reg_I + 2) as usize] = v % 10;
                        self.cpu.reg_pc += 2;
                    },
                    0x0055 => {
                        for i in (0..((opcode & 0x0F00) >> 8) + 1){
                            self.memory[(self.cpu.reg_I + i) as usize] = self.cpu.reg_gpr[i as usize];
                        }
                        self.cpu.reg_I += ((opcode & 0x0F00) >> 8) + 1;
                        self.cpu.reg_pc += 2;
                    },
                    0x0065 => {
                        for i in (0..((opcode & 0x0F00) >> 8) +1 ){
                            self.cpu.reg_gpr[i as usize] = self.memory[(self.cpu.reg_I + i) as usize];
                        }
                        self.cpu.reg_I += ((opcode & 0x0F00) >> 8) + 1; 
                        self.cpu.reg_pc += 2;
                    },
                     _ => {panic!("invalid instructiion in 0xF000 - {}", opcode);} 
                }
            },
            _ => {panic!("invalid instructiion - {:x}", opcode);}
        }
        if self.cpu.reg_dt > 0{
            self.cpu.reg_dt -= 1;
        }
        is_draw_needed
    }
}



impl Debug for Chip {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.memory[..].fmt(formatter)
    }
}
