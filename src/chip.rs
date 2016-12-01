const MEMORY_SIZE : usize = 4 * 1024;
const DEF_PC_LOC : usize = 0x200;
use std::fmt;
use std::fmt::Debug;
use cpu::Cpu;

pub struct Chip{
    pub memory : [u8;MEMORY_SIZE] ,
    pub cpu : Cpu
}

impl Default for Chip {
    fn default() -> Chip{
        Chip{
            memory : [0;MEMORY_SIZE],
            cpu : Cpu::default()
        }
    }
}

impl Chip{
    pub fn start_cpu(&mut self,program : Vec<u8>){
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

        self.cpu.reg_pc = DEF_PC_LOC as u16;
        let mut i = 512;
        for b in &program{
            self.memory[i] = *b;
            i += 1;
        }
    }

    pub fn run(&mut self){
        let mut is_draw_needed = false;
        let window: PistonWindow = WindowSettings::new("crust8", [640, 320]) .exit_on_esc(true).into();
        let black = [0.0, 0.0, 0.0, 1.0];
        let red = [1.0, 0.0, 0.0, 1.0];
        for e in window {
            if let Some(r) = e.render_args() {
                e.draw_2d(|c, g| {
                    loop{
                        is_draw_needed = self.execute();
                        self.cpu.reg_pc += 2; 
                    }
                });
            }
        }
    }

    pub fn execute(&mut self) -> bool {
        let mut is_draw_needed = false;
        let mut opcode = 0;
        opcode = ((self.memory[self.cpu.reg_pc] as u16) << 8) | self.memory[self.cpu.reg_pc + 1] as u16;
        println!("Opcode : {}",opcode);
        match opcode{
            0x00E0 => {
                is_draw_needed = true;
                for i in(0..2048){
                    self.cpu.gfx[i]=0;
                }
            }
        }
    }
}



impl Debug for Chip {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.memory[..].fmt(formatter)
    }
}
