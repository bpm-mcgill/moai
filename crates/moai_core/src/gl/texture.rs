use std::ffi::c_void;

// TODO: Make a way to set the configuration for the texture
pub unsafe fn gen_texture() -> u32{
    // Generate texture and set id
    let mut texture_id = 0;
    gl::GenTextures(1, &mut texture_id);
    gl::BindTexture(gl::TEXTURE_2D, texture_id);

    // Set the configuration for the texture
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

    return texture_id;
}

pub unsafe fn set_texture_data(texture_id: u32, data: &[u8], dimensions: (u32, u32), mipmap: bool) {
    gl::BindTexture(gl::TEXTURE_2D, texture_id);
    gl::TexImage2D(gl::TEXTURE_2D,
        0,
        gl::RGB as i32,
        dimensions.0 as i32,
        dimensions.1 as i32,
        0,
        gl::RGB,
        gl::UNSIGNED_BYTE,
        &data[0] as *const u8 as *const c_void);
    if mipmap {gl::GenerateMipmap(gl::TEXTURE_2D)};
}