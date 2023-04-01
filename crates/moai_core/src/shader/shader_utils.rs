use std::{collections::HashMap, ptr::null_mut};
use gl::types::*;

pub struct SingleShader {
    pub stype: GLenum,
    pub content: String
}

pub fn check_status(sid: u32, shader_type: GLenum) -> Result<i32, String> {
    unsafe{
        let mut success = 0;
        gl::GetShaderiv(sid, gl::COMPILE_STATUS, &mut success);
        if success == 1 {return Ok(success)}

        let mut info: Vec<u8> = Vec::with_capacity(1024);
        let mut log_len = 0;

        gl::GetShaderInfoLog(sid, 1024, &mut log_len, info.as_mut_ptr().cast());
        info.set_len(log_len.try_into().unwrap_or(0));

        let err_msg = format!("{} SHADER COMPILATION FAILED\n -> {}", shader_type, String::from_utf8_lossy(&info));
        return Err(err_msg);
    };
}

pub fn load_shader(shader_content: &str) -> Vec<SingleShader> {
    let mut shaders: Vec<SingleShader> = vec![];
    let splitted: Vec<&str> = shader_content.split('»').collect();
    for splat in splitted{
        let splattered = splat.split_once("\n");
        // The first element is "", this will skip it
        if splattered.is_none() { continue }
        // Unwrap is okay because of the check above
        let splattered = splattered.unwrap();

        // Could implement strum EnumVariants
        //  - https://docs.rs/strum_macros/0.24.3/strum_macros/derive.EnumString.html
        if splattered.0 == "Vertex"{
            shaders.push(SingleShader{stype: gl::VERTEX_SHADER, content: splattered.1.to_string()});
        }
        else if splattered.0 == "Fragment"{
            shaders.push(SingleShader{stype: gl::FRAGMENT_SHADER, content: splattered.1.to_string()});
        }
    }
    shaders
}

// Get all of the uniforms (uniform blocks in the future) and populate the uniforms hashmap
pub fn get_uniforms(program_id: u32) -> HashMap<String, i32> {
    let mut uniforms: HashMap<String, i32> = HashMap::new(); // Will store the fetched uniforms
    // Uniform names can't be longer than 128 bytes
    let max_uniform_size = 128;
    unsafe{
        let mut uniform_count: i32 = 0;
        // Only active unis because inactive unis wouldn't have an affect
        gl::GetProgramiv(program_id, gl::ACTIVE_UNIFORMS, &mut uniform_count);
        for i in 0..uniform_count as u32 {
            let mut uniform_name_buffer: Vec<u8> = Vec::with_capacity(max_uniform_size);
            let mut name_len = 0;
            gl::GetActiveUniform(
                program_id,
                i,
                max_uniform_size as i32,
                &mut name_len,
                null_mut(),
                null_mut(),
                uniform_name_buffer.as_mut_ptr().cast()
            );
            uniform_name_buffer.set_len(name_len.try_into().unwrap_or(0));
            let uniform_location = gl::GetUniformLocation(program_id, uniform_name_buffer.as_ptr().cast());

            let uniform_name = String::from_utf8_lossy(&uniform_name_buffer).to_string();
            uniforms.insert(uniform_name, uniform_location);

        }
    }
    uniforms
}
