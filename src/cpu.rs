const NUM_GPR : usize = 16;
const NUM_STACK : usize = 16;
const MEMORY_SIZE : usize = 4 * 1024;
const GRAPHICS_SIZE : usize = 64*32;
const NUM_KEY : usize = 16;

#[derive(Debug,Default)]
pub struct Cpu {
    pub reg_gpr : [u8;NUM_GPR],
    pub reg_I : u16,
    pub reg_pc : usize,
    pub reg_sp : u8,
    pub stack : [u16;NUM_STACK],
    pub reg_st : u8,
    pub reg_dt : u8,
    pub keys : [u8;NUM_KEY]
}
