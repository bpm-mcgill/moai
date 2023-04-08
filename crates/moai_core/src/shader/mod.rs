use std::{collections::HashMap, ptr::null};
use log::{debug, error};
mod shader_utils;

/**
### Abstracts shader loading, parsing and set up.
Stores the shader program's id in `sid` and a hashmap of the uniforms'
locations in `uniforms`.
*/
pub struct Shader{
    sid: u32,
    uniforms: HashMap<String, i32>,     // name: location
}

// TODO: make shader pipeline
impl Shader{
    /// Construct a new Shader object
    pub fn new() -> Result<Self, String> {
        // TODO: Handle this properly
        let shader_content = include_str!("../../res/basic.shader");
        let shaders = shader_utils::load_shader(shader_content);

        // Create Shader program
        let sid = unsafe {
            let shade_program = gl::CreateProgram();
            for shade in shaders{
                let shade_id = gl::CreateShader(shade.stype);
                let c_str_vert = std::ffi::CString::new(shade.content.as_bytes()).unwrap();
                gl::ShaderSource(shade_id, 1, &c_str_vert.as_ptr(), null());
                gl::CompileShader(shade_id);

                if let Err(err_msg) = shader_utils::check_status(shade_id, shade.stype) {
                    error!("{}", err_msg);
                    gl::DeleteProgram(shade_program);
                    return Err(err_msg);
                }
                
                gl::AttachShader(shade_program, shade_id);
                // After it has been attached, it can be safely deleted
                gl::DeleteShader(shade_id);
            };
            gl::LinkProgram(shade_program);
            shade_program
        };
        let uniforms = shader_utils::get_uniforms(sid);

        debug!("Successfully built shader basic.shader!");
        Ok(Shader {sid, uniforms})
    }
}

// One liners
impl Shader {
    pub fn bind(&self) {
        unsafe {gl::UseProgram(self.sid)}
    }

    pub fn unbind() {
        unsafe {gl::UseProgram(0)}
    }

    fn get_uniform_location(&self, name: &str) -> i32 {
        *self.uniforms.get(name).unwrap_or(&-1)
    }

    pub fn set_int(&self, name: &str, value: i32) {unsafe {gl::Uniform1i(self.get_uniform_location(&name), value) };}
    pub fn set_float(&self, name: &str, value: f32) {unsafe {gl::Uniform1f(self.get_uniform_location(&name), value) };}

    pub fn set_vec4(&self, name: &str, value: glam::Vec4) {unsafe {gl::Uniform4fv(self.get_uniform_location(&name), 1, &value[0]) };}
    pub fn set_vec3(&self, name: &str, value: glam::Vec3) {unsafe {gl::Uniform3fv(self.get_uniform_location(&name), 1, &value[0]) };}
    pub fn set_vec2(&self, name: &str, value: glam::Vec2) {unsafe {gl::Uniform2fv(self.get_uniform_location(&name), 1, &value[0]) };}

    pub fn set_mat4(&self, name: &str, value: glam::Mat4) {unsafe {gl::UniformMatrix4fv(self.get_uniform_location(&name), 1, gl::FALSE, &value.to_cols_array()[0]) };}
    pub fn set_mat3(&self, name: &str, value: glam::Mat3) {unsafe {gl::UniformMatrix3fv(self.get_uniform_location(&name), 1, gl::FALSE, &value.to_cols_array()[0]) };}
    pub fn set_mat2(&self, name: &str, value: glam::Mat2) {unsafe {gl::UniformMatrix2fv(self.get_uniform_location(&name), 1, gl::FALSE, &value.to_cols_array()[0]) };}
}