use gl::types::*;
use std::os::raw::c_void;
use std::mem::size_of;

// Rename this since it encapsulates VBOS + EBOS and VAOs
pub struct VBO {
    //vbo_id: u32,
    vao_id: u32,
    //ebo_id: u32,
}

impl VBO {
    pub fn new(vertices: &[f32], indices: &[i32]) -> Self{
        //let (vbo_id, vao_id, ebo_id) = unsafe {
        let vao_id = unsafe {
            // Placeholder variables, will be populated with buffer ids
            let (mut vbo, mut vao, mut ebo) = (0, 0, 0);
            
            // Generate buffers
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::GenBuffers(1, &mut ebo);

            // Set the buffer data to the input data (vertices/VBO) under generated VAO
            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * size_of::<GLfloat>()) as GLsizeiptr,
                &vertices[0] as *const f32 as *const c_void,
                gl::STATIC_DRAW
            );
            
            // Set buffer data for EBO under same VAO
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                           (indices.len() * size_of::<GLfloat>()) as GLsizeiptr,
                           &indices[0] as *const i32 as *const c_void,
                           gl::STATIC_DRAW);

            // TODO: fix this so layouts can be automatically generated
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                3 * size_of::<GLfloat>() as GLsizei,
                std::ptr::null()
            );
            gl::EnableVertexAttribArray(0);

            // Unbind buffers so they aren't overwritten
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);

            // return buffer ids
            //(vbo, vao, ebo)
            vao
        };
        return VBO {
            //vbo_id,
            vao_id,
            //ebo_id
        };
    }

    pub fn bind(&self) {
        unsafe {gl::BindVertexArray(self.vao_id)};
    }

    pub fn unbind(&self) {
        unsafe {gl::BindVertexArray(0)};
    }
}