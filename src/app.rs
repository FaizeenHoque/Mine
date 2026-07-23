use glow::HasContext;
use sdl2::{
    event::Event,
    keyboard::Keycode,
    video::{GLContext, GLProfile, Window},
};

use crate::renderer::{Mesh, Shader};

pub struct App {
    sdl: sdl2::Sdl,
    window: Window,
    gl_context: GLContext,
    gl: glow::Context,

    shader: Shader,
    mesh: Mesh,

    event_pump: sdl2::EventPump,

    x: f32,
    y: f32,
}

impl App {
    pub fn new() -> Result<Self, String> {
        let sdl = sdl2::init()?;
        let video = sdl.video()?;

        {
            let gl_attr = video.gl_attr();
            gl_attr.set_context_profile(GLProfile::Core);
            gl_attr.set_context_version(3, 3);
        }

        let window = video
            .window("Mine", 1200, 800)
            .opengl()
            .build()
            .map_err(|e| e.to_string())?;

        let gl_context = window.gl_create_context()?;
        window.gl_make_current(&gl_context)?;

        let gl = unsafe {
            glow::Context::from_loader_function(|s| video.gl_get_proc_address(s) as *const _)
        };

        let vertex_shader_source = include_str!("../assets/shaders/triangle.vert");
        let fragment_shader_source = include_str!("../assets/shaders/triangle.frag");

        let vertices: [f32; 24] = [
            0.5, 0.5, 0.0, 1.0, 0.0, 0.0, -0.5, 0.5, 0.0, 0.0, 1.0, 0.0, -0.5, -0.5, 0.0, 0.0, 0.0,
            1.0, 0.5, -0.5, 0.0, 1.0, 1.0, 0.0,
        ];

        let indices: [u32; 6] = [0, 1, 2, 0, 3, 2];

        let shader = Shader::new(&gl, vertex_shader_source, fragment_shader_source)?;

        let mesh = Mesh::new(&gl, &vertices, &indices)?;

        let event_pump = sdl.event_pump()?;

        Ok(Self {
            sdl,
            window,
            gl_context,
            gl,
            shader,
            mesh,
            event_pump,
            x: 0.0,
            y: 0.0,
        })
    }

    pub fn run(&mut self) {
        'running: loop {
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => break 'running,

                    Event::KeyDown {
                        keycode: Some(Keycode::W),
                        ..
                    } => self.y += 0.05,

                    Event::KeyDown {
                        keycode: Some(Keycode::S),
                        ..
                    } => self.y -= 0.05,

                    Event::KeyDown {
                        keycode: Some(Keycode::A),
                        ..
                    } => self.x -= 0.05,

                    Event::KeyDown {
                        keycode: Some(Keycode::D),
                        ..
                    } => self.x += 0.05,

                    Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,

                    _ => {}
                }
            }

            unsafe {
                self.gl.clear_color(0.1, 0.2, 0.3, 1.0);
                self.gl.clear(glow::COLOR_BUFFER_BIT);
            }

            self.shader.bind(&self.gl);
            self.shader
                .set_uniform_2f(&self.gl, "offset", self.x, self.y);

            self.mesh.draw(&self.gl);

            self.window.gl_swap_window();
        }
    }
}
