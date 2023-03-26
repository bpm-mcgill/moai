use gl::types::*;
use log::{debug, error};
struct SingleShader {
    stype: GLenum,
    content: String
}

pub struct Shader{
    sid: u32,
}

impl Shader{

    // TODO: more testing on the security of this function
    fn check_status(sid: u32, shader_type: GLenum) -> Result<i32, String> {
        unsafe{
            let mut success = 0;
            gl::GetShaderiv(sid, gl::COMPILE_STATUS, &mut success);
            if success == 1 {return Ok(success)}

            let mut info: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0_i32;
            gl::GetShaderInfoLog(sid, 1024, &mut log_len, info.as_mut_ptr().cast());
            info.set_len(log_len.try_into().unwrap_or(0));
            let err_msg = format!("{} SHADER COMPILATION FAILED\n -> {}", shader_type, String::from_utf8_lossy(&info));
            return Err(err_msg);
        };
    }

    fn load_shader(shader_content: &str) -> Vec<SingleShader> {
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

    pub fn new() -> Result<Self, String>{
        // TODO: Handle this properly
        let shader_content = include_str!("../../res/basic.shader");
        let shaders = Shader::load_shader(shader_content);
        let sid = unsafe {
            let shade_program = gl::CreateProgram();
            for shade in shaders{
                let shade_id = gl::CreateShader(shade.stype);
                let c_str_vert = std::ffi::CString::new(shade.content.as_bytes()).unwrap();
                gl::ShaderSource(shade_id, 1, &c_str_vert.as_ptr(), std::ptr::null());
                gl::CompileShader(shade_id);

                if let Err(err_msg) = Shader::check_status(shade_id, shade.stype) {
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
        debug!("Successfully built shader basic.shader!");
        Ok(Shader {sid})
    }

    pub fn bind(&self){
        unsafe {gl::UseProgram(self.sid)}
    }

    pub fn unbind(){
        unsafe {gl::UseProgram(0)}
    }
}