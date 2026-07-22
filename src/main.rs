mod renderer;
mod shader;

use renderer::Renderer;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::GLProfile;

fn main() -> Result<(), String> {
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

    let renderer = Renderer::new(&gl)?;

    let mut x = 0.0f32;
    let mut y = 0.0f32;

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
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        renderer.draw(&gl, x, y);
        window.gl_swap_window();
    }

    Ok(())
}
