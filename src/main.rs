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

    let _gl_context = window.gl_create_context()?;
    window.gl_make_current(&_gl_context)?;

    let mut event_pump = sdl.event_pump()?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::ESCAPE),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        window.gl_swap_window();
    }

    Ok(())
}
