use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use chip::Chip;
use cpu::Cpu;
use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::*;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    chip : Chip,
    rom : Vec<u8>,
    window: Window
}

impl App {

    fn render(&mut self, args: RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let mut is_draw_needed = self.chip.execute();
        let mut tempData = [[0;32];64];
        for y in (0..32){
            ////println!("y - {}", y);
            for x in (0..64){
                ////println!("x - {}", x);
                tempData[x][y] = if self.chip.gfx[((y*64) + x) as usize] == 0{
                    0 
                }
                else{
                    1 
                };
            }
        }


        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            //use is_draw_required here
            //
            //

            if is_draw_needed {
                clear([0.0; 4], gl);
                for y in (0..32){
                    ////println!("y - {}", y);
                    for x in (0..64){
                        ////println!("x - {}", x);
                        if tempData[x][y] == 0 {

         rectangle(GREEN,
                  [x as f64 * 10.0 + 0.0 as f64, y as f64 * 10.0 + 0.0 as f64, 
                  x as f64 * 10.0 + 10.0 as f64, y as f64 * 10.0 + 10.0 as f64],
                  c.transform, gl);
                        }else{
rectangle(RED,
                  [x as f64 * 10.0 + 0.0 as f64, y as f64 * 10.0 + 0.0 as f64, 
                  x as f64 * 10.0 + 10.0 as f64, y as f64 * 10.0 + 10.0 as f64],
                  c.transform, gl);

                        }
        //break;
                    }
                }
                                is_draw_needed = false;
            }            //clear(GREEN, gl);
        });
    }

    pub fn new(rom:String) -> Self {
        let opengl = OpenGL::V3_2;
        let mut window: Window = WindowSettings::new(
            "crust8",
            [640, 320]
            )
            .opengl(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap();

        App {
            window : window,
            gl: GlGraphics::new(OpenGL::V3_2),
            chip : Chip::default() ,
            rom : Self::read_bin(rom),
        }
    }

    fn read_bin<P:AsRef<Path>>(path:P)-> Vec<u8> {
        let mut file = File::open(path).unwrap();
        let mut file_buf = Vec::new();
        file.read_to_end(&mut file_buf).unwrap();
        file_buf
    }

    pub fn run(&mut self) {

        self.chip.start_cpu(&self.rom);



        let mut events = self.window.events();
        while let Some(e) = events.next(&mut self.window) {
            if let Some(r) = e.render_args() {
                self.render(r);
            }
            if let Some(b) = e.press_args(){
                match &b {
                    &Button::Keyboard(k) => match &k {
                        &Key::Escape => process::exit(0),
                        &Key::D1 => self.chip.cpu.keys[0x1] = 1 as u8,
                        &Key::D2 => self.chip.cpu.keys[0x2] = 1 as u8,
                        &Key::D3 => self.chip.cpu.keys[0x3] = 1 as u8,
                        &Key::D4 => self.chip.cpu.keys[0xC] = 1 as u8,
                        &Key::Q => self.chip.cpu.keys[0x4] = 1 as u8,
                        &Key::W => self.chip.cpu.keys[0x5] = 1 as u8,
                        &Key::E => self.chip.cpu.keys[0x6] = 1 as u8,
                        &Key::R => self.chip.cpu.keys[0xD] = 1 as u8,
                        &Key::A => self.chip.cpu.keys[0x7] = 1 as u8,
                        &Key::S => self.chip.cpu.keys[0x8] = 1 as u8,
                        &Key::D => self.chip.cpu.keys[0x9] = 1 as u8,
                        &Key::F => self.chip.cpu.keys[0xE] = 1 as u8,
                        &Key::Z => self.chip.cpu.keys[0xA] = 1 as u8,
                        &Key::X => self.chip.cpu.keys[0x0] = 1 as u8,
                        &Key::C => self.chip.cpu.keys[0xB] = 1 as u8,
                        &Key::V => self.chip.cpu.keys[0xF] = 1 as u8,

                        _ => {}
                    },
                    _ => {}
                }
            }

            if let Some(b) = e.release_args(){
                match &b {
                    &Button::Keyboard(k) => match &k {
                        &Key::Escape => process::exit(0),
                        &Key::D1 => self.chip.cpu.keys[0x1] = 0 as u8,
                        &Key::D2 => self.chip.cpu.keys[0x2] = 0 as u8,
                        &Key::D3 => self.chip.cpu.keys[0x3] = 0 as u8,
                        &Key::D4 => self.chip.cpu.keys[0xC] = 0 as u8,
                        &Key::Q => self.chip.cpu.keys[0x4] = 0 as u8,
                        &Key::W => self.chip.cpu.keys[0x5] = 0 as u8,
                        &Key::E => self.chip.cpu.keys[0x6] = 0 as u8,
                        &Key::R => self.chip.cpu.keys[0xD] = 0 as u8,
                        &Key::A => self.chip.cpu.keys[0x7] = 0 as u8,
                        &Key::S => self.chip.cpu.keys[0x8] = 0 as u8,
                        &Key::D => self.chip.cpu.keys[0x9] = 0 as u8,
                        &Key::F => self.chip.cpu.keys[0xE] = 0 as u8,
                        &Key::Z => self.chip.cpu.keys[0xA] = 0 as u8,
                        &Key::X => self.chip.cpu.keys[0x0] = 0 as u8,
                        &Key::C => self.chip.cpu.keys[0xB] = 0 as u8,
                        &Key::V => self.chip.cpu.keys[0xF] = 0 as u8,

                        _ => {}
                    },
                    _ => {}
                }
            }

        }
    }    


}
