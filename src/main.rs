mod chip;
mod cpu;


use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use chip::Chip;
use cpu::Cpu;

fn main() {
    let rom_file_name = env::args().nth(1).unwrap();
    println!("Rom file Name : {}",rom_file_name);
    let rom = read_bin(rom_file_name);
    let mut chip = Chip::default();
    println!("{:#?}",&chip);
    chip.start_cpu(rom);
    println!(" pc value : {}",&chip.cpu.reg_pc);
    print_ram_location(chip);
    
}

fn read_bin<P:AsRef<Path>>(path:P)-> Vec<u8> {
    let mut file = File::open(path).unwrap();
    let mut file_buf = Vec::new();
    file.read_to_end(&mut file_buf).unwrap();
    file_buf
}

fn print_ram_location(chip : chip::Chip){
    for i in (512..600){
        println!("{}",chip.memory[i]);
    }
}

