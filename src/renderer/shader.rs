use glow::HasContext;

pub struct Shader {
    program: glow::Program,
}

impl Shader {
    pub fn new(
        gl: &glow::Context,
        vertex_source: &str,
        fragment_source: &str,
    ) -> Result<Self, String> {
        unsafe {
            let vertex_shader = gl.create_shader(glow::VERTEX_SHADER)?; // create empty vertex shader
            let fragment_shader = gl.create_shader(glow::FRAGMENT_SHADER)?;

            gl.shader_source(vertex_shader, &vertex_source); // give it a source code
            gl.compile_shader(vertex_shader); // compile it

            if !gl.get_shader_compile_status(vertex_shader) {
                // Check if compilation succeeded
                return Err(gl.get_shader_info_log(vertex_shader));
            }

            gl.shader_source(fragment_shader, &fragment_source);
            gl.compile_shader(fragment_shader);

            if !gl.get_shader_compile_status(fragment_shader) {
                return Err(gl.get_shader_info_log(fragment_shader));
            }

            let program = gl.create_program()?;
            gl.attach_shader(program, vertex_shader);
            gl.attach_shader(program, fragment_shader);
            gl.link_program(program);
            gl.delete_shader(vertex_shader);
            gl.delete_shader(fragment_shader);

            if !gl.get_program_link_status(program) {
                panic!("{}", gl.get_program_info_log(program));
            }

            // gl.use_program(Some(program));
            Ok(Self { program })
        }
    }

    pub fn bind(&self, gl: &glow::Context) {
        unsafe {
            gl.use_program(Some(self.program));
        }
    }

    pub fn set_uniform_2f(&self, gl: &glow::Context, name: &str, x: f32, y: f32) {
        unsafe {
            if let Some(location) = gl.get_uniform_location(self.program, name) {
                gl.uniform_2_f32(Some(&location), x, y);
            }
        }
    }
}
