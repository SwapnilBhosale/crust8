mod chip;
mod cpu;
mod app;

extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

use std::env;
use std::fs::File;
use std::io::Read;
use chip::Chip;
use cpu::Cpu;
use app::*;
fn main() {
    let rom_file_name = env::args().nth(1).unwrap();
    
    
    println!("Rom file Name : {}",rom_file_name);
    let mut app = App::new(rom_file_name);
    app.run();
    //let mut chip = Chip::default();
    //println!("{:#?}",&chip);
    //chip.start_cpu(rom);
    //println!(" pc value : {}",&chip.cpu.reg_pc);
    //print_ram_location(chip);
    //chip.run();
    
    
}


fn print_ram_location(chip : chip::Chip){
    for i in (512..600){
        println!("{}",chip.memory[i]);
    }
}

