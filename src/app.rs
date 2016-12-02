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

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    chip : Chip,
    rom : Vec<u8>,
    window: Window
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let is_draw_required = self.chip.execute();
        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            //use is_draw_required here
            clear(GREEN, gl);
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
                self.render(&r);
            }
        }

    }


}
