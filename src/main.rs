mod interconnect;
mod cpu;


use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use cpu::Cpu;

fn main() {
    let rom_file_name = env::args().nth(1).unwrap();
    println!("Rom file Name : {}",rom_file_name);
    let rom = read_bin(rom_file_name);
    let mut cpu = Cpu::new();
    println!("{:#?}",&cpu);
}

fn read_bin<P:AsRef<Path>>(path:P)-> Vec<u8> {
    let mut file = File::open(path).unwrap();
    let mut file_buf = Vec::new();
    file.read_to_end(&mut file_buf).unwrap();
    file_buf
}

