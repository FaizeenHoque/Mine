use crate::shader;
use glow::HasContext;

pub struct Renderer {
    pub program: glow::Program,
    pub vao: glow::VertexArray,
    pub vbo: glow::Buffer,
    pub offset_location: Option<glow::UniformLocation>,
}

impl Renderer {
    pub fn new(gl: &glow::Context) -> Result<Self, String> {
        let vertex_shader_source = include_str!("../assets/shaders/triangle.vert");
        let fragment_shader_source = include_str!("../assets/shaders/triangle.frag");

        let program =
            unsafe { shader::link_program(gl, vertex_shader_source, fragment_shader_source)? };

        unsafe {
            gl.use_program(Some(program));
        }

        let offset_location = unsafe { gl.get_uniform_location(program, "offset") };

        let vao = unsafe { gl.create_vertex_array()? };
        let vbo = unsafe { gl.create_buffer()? };

        unsafe {
            gl.bind_vertex_array(Some(vao));
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));

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
        }

        let vertices: [f32; 36] = [
            0.5, 0.5, 0.0, 1.0, 0.0, 0.0, -0.5, 0.5, 0.0, 0.0, 1.0, 0.0, -0.5, -0.5, 0.0, 0.0, 0.0,
            1.0, 0.5, -0.5, 0.0, 1.0, 1.0, 0.0, 0.5, 0.5, 0.0, 1.0, 0.0, 1.0, -0.5, -0.5, 0.0, 0.0,
            1.0, 1.0,
        ];

        unsafe {
            gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                bytemuck::cast_slice(&vertices),
                glow::STATIC_DRAW,
            );
        }

        Ok(Self {
            program,
            vao,
            vbo,
            offset_location,
        })
    }

    pub fn draw(&self, gl: &glow::Context, x: f32, y: f32) {
        unsafe {
            gl.clear_color(0.1, 0.2, 0.3, 1.0);
            gl.clear(glow::COLOR_BUFFER_BIT);

            gl.use_program(Some(self.program));
            gl.uniform_2_f32(self.offset_location.as_ref(), x, y);
            gl.bind_vertex_array(Some(self.vao));
            gl.draw_arrays(glow::TRIANGLES, 0, 6);
        }
    }
}
