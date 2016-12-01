use interconnect::*;


const NUM_GPR : usize = 16;
const NUM_STACK : usize = 16;
const MEMORY_SIZE : usize = 4 * 1024;

#[derive(Debug)]
pub struct Cpu {
    reg_gpr : [u8;NUM_GPR],
    reg_I : u16,
    reg_pc : u16,
    reg_sp : u8,
    stack : [u16;NUM_STACK],
    reg_st : u8,
    reg_dt : u8,
    _interconnect : InterConnect
     
}



impl Cpu {
    pub fn new() -> Cpu{
       Cpu{
           reg_gpr : [0;NUM_GPR],
           reg_I : 0,
           reg_pc : 0,
           reg_sp : 0,
           stack : [0;NUM_STACK],
           reg_st : 0,
           reg_dt : 0,
           _interconnect : InterConnect::default()
       }
    }
}


