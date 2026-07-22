use glow::HasContext;

pub unsafe fn compile_shader(
    gl: &glow::Context,
    shader_type: u32,
    source: &str,
) -> Result<glow::Shader, String> {
    let shader = gl.create_shader(shader_type)?;
    gl.shader_source(shader, source);
    gl.compile_shader(shader);

    if !gl.get_shader_compile_status(shader) {
        return Err(gl.get_shader_info_log(shader));
    }

    Ok(shader)
}

pub unsafe fn link_program(
    gl: &glow::Context,
    vertex_source: &str,
    fragment_source: &str,
) -> Result<glow::Program, String> {
    let vertex_shader = compile_shader(gl, glow::VERTEX_SHADER, vertex_source)?;
    let fragment_shader = compile_shader(gl, glow::FRAGMENT_SHADER, fragment_source)?;

    let program = gl.create_program()?;
    gl.attach_shader(program, vertex_shader);
    gl.attach_shader(program, fragment_shader);
    gl.link_program(program);

    gl.delete_shader(vertex_shader);
    gl.delete_shader(fragment_shader);

    if !gl.get_program_link_status(program) {
        return Err(gl.get_program_info_log(program));
    }

    Ok(program)
}
