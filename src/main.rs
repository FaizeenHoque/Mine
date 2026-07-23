use glow::HasContext;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::GLProfile;

fn main() -> Result<(), String> {
    let sdl = sdl2::init()?;
    let video = sdl.video()?;

    let vertex_shader_source = include_str!("../assets/shaders/triangle.vert");
    let fragment_shader_source = include_str!("../assets/shaders/triangle.frag");

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

    let vertex_shader = unsafe { gl.create_shader(glow::VERTEX_SHADER)? }; // create empty vertex shader
    unsafe {
        gl.shader_source(vertex_shader, &vertex_shader_source); // give it a source code
        gl.compile_shader(vertex_shader); // compile it

        if !gl.get_shader_compile_status(vertex_shader) {
            // Check if compilation succeeded
            panic!("{}", gl.get_shader_info_log(vertex_shader));
        }
    }

    let fragment_shader = unsafe { gl.create_shader(glow::FRAGMENT_SHADER)? };
    unsafe {
        gl.shader_source(fragment_shader, &fragment_shader_source);
        gl.compile_shader(fragment_shader);

        if !gl.get_shader_compile_status(fragment_shader) {
            panic!("{}", gl.get_shader_info_log(fragment_shader));
        }
    }

    let shader_program = unsafe { gl.create_program()? };
    unsafe {
        gl.attach_shader(shader_program, vertex_shader);
        gl.attach_shader(shader_program, fragment_shader);
        gl.link_program(shader_program);
        gl.delete_shader(vertex_shader);
        gl.delete_shader(fragment_shader);

        if !gl.get_program_link_status(shader_program) {
            panic!("{}", gl.get_program_info_log(shader_program));
        }

        gl.use_program(Some(shader_program));
    }

    let offset_location = unsafe { gl.get_uniform_location(shader_program, "offset") };

    let vao = unsafe { gl.create_vertex_array()? };
    unsafe {
        gl.bind_vertex_array(Some(vao));
    }
    let vbo = unsafe { gl.create_buffer()? };
    unsafe {
        gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
    };

    let ebo = unsafe { gl.create_buffer()? };
    unsafe {
        gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ebo));
    };

    unsafe {
        gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
    }
    unsafe {
        gl.enable_vertex_attrib_array(0);
    }

    let mut x = 0.0f32;
    let mut y = 0.0f32;

    unsafe {
        let stride = 6 * std::mem::size_of::<f32>() as i32;

        // position
        gl.enable_vertex_attrib_array(0);
        gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, stride, 0);

        // color
        gl.enable_vertex_attrib_array(1);
        gl.vertex_attrib_pointer_f32(
            1,
            3,
            glow::FLOAT,
            false,
            stride,
            3 * std::mem::size_of::<f32>() as i32,
        );

        gl.uniform_2_f32(offset_location.as_ref(), x, y);
    }

    let vertices: [f32; 24] = [
        0.5, 0.5, 0.0, 1.0, 0.0, 0.0, -0.5, 0.5, 0.0, 0.0, 1.0, 0.0, -0.5, -0.5, 0.0, 0.0, 0.0,
        1.0, 0.5, -0.5, 0.0, 1.0, 1.0, 0.0,
    ];

    let indices: [u32; 6] = [0, 1, 2, 0, 3, 2];

    unsafe {
        gl.buffer_data_u8_slice(
            glow::ARRAY_BUFFER,
            bytemuck::cast_slice(&vertices),
            glow::STATIC_DRAW,
        );

        gl.buffer_data_u8_slice(
            glow::ELEMENT_ARRAY_BUFFER,
            bytemuck::cast_slice(&indices),
            glow::STATIC_DRAW,
        );
    }

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

        unsafe {
            gl.use_program(Some(shader_program));

            gl.uniform_2_f32(offset_location.as_ref(), x, y);

            gl.bind_vertex_array(Some(vao));

            gl.draw_elements(glow::TRIANGLES, 6, glow::UNSIGNED_INT, 0);
        }

        window.gl_swap_window();
    }

    Ok(())
}
