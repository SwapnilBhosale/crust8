const NUM_GPR : usize = 16;
const NUM_STACK : usize = 16;
const MEMORY_SIZE : usize = 4 * 1024;
const GRAPHICS_SIZE : usize = 64*32;

#[derive(Debug,Default)]
pub struct Cpu {
    gfx : [u8;GRAPHICS_SIZE];
    reg_gpr : [u8;NUM_GPR],
    reg_I : u16,
    pub reg_pc : u16,
    pub reg_sp : u8,
    stack : [u16;NUM_STACK],
    reg_st : u8,
    reg_dt : u8,
}
