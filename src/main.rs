mod renderer;

use glow::HasContext;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::GLProfile;

use renderer::*;

fn main() -> Result<(), String> {
    let sdl = sdl2::init()?;
    let video = sdl.video()?;

    let vertex_shader_source: &str = include_str!("../assets/shaders/triangle.vert");
    let fragment_shader_source: &str = include_str!("../assets/shaders/triangle.frag");

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

    let _gl_context = window.gl_create_context()?;
    window.gl_make_current(&_gl_context)?;

    let gl = unsafe {
        glow::Context::from_loader_function(|s| video.gl_get_proc_address(s) as *const _)
    };

    let shader = Shader::new(&gl, vertex_shader_source, fragment_shader_source)?;

    let mut x = 0.0f32;
    let mut y = 0.0f32;

    let vertices: [f32; 24] = [
        0.5, 0.5, 0.0, 1.0, 0.0, 0.0, -0.5, 0.5, 0.0, 0.0, 1.0, 0.0, -0.5, -0.5, 0.0, 0.0, 0.0,
        1.0, 0.5, -0.5, 0.0, 1.0, 1.0, 0.0,
    ];

    let indices: [u32; 6] = [0, 1, 2, 0, 3, 2];

    let mesh = Mesh::new(&gl, &vertices, &indices)?;

    let mut event_pump = sdl.event_pump()?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,

                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => y += 0.05,

                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => y -= 0.05,

                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => x -= 0.05,

                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => x += 0.05,

                Event::KeyDown {
                    keycode: Some(Keycode::ESCAPE),
                    ..
                } => break 'running,

                _ => {}
            }
        }

        unsafe {
            gl.clear_color(0.1, 0.2, 0.3, 1.0);
            gl.clear(glow::COLOR_BUFFER_BIT);
        }

        shader.bind(&gl);
        shader.set_uniform_2f(&gl, "offset", x, y);

        mesh.draw(&gl);

        window.gl_swap_window();
    }

    Ok(())
}
