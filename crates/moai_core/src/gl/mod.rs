use gl::types::*;
use std::os::raw::c_void;
use std::mem::size_of;
pub mod texture;

/**
### Encapsulates VBOS + EBOS and VAOs

The struct has a `new` method used to initalize a new VBO
with the data provided
*/
pub struct VBO {
    vbo_id: u32,
    vao_id: u32,

    // Used for rendering
    pub indices_size: i32

    //ebo_id: u32,
}

/// Contains specifications for a single vertex attribute
pub struct VertexAttrib {
    // The number of components of this attribute per vertex
    pub size: i32,
    pub vtype: GLenum,
    pub normal: u8
}

impl VBO {
    /**
    ### Constructs a new VBO, EBO, and VAO.
    Method will generate the buffers, populate them with data,</br>
    then return a VBO struct containing the data

    **Arguments**:
    * **vertices**: Float array of the object's vertices
    * **indices**: The type of shader being checked so it can be logged if there's an error

    **Returns**:
    * A result enum of either a i32 (success) or a string (error message) 
    */
    pub fn new(vertices: &[f32], indices: &[i32]) -> Self{
        let (vbo_id, vao_id, _ebo_id) = unsafe {
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

            // Unbind buffers so they aren't overwritten
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);

            // return buffer ids
            (vbo, vao, ebo)
        };
        return VBO {
            vbo_id,
            vao_id,
            indices_size: indices.len() as i32
            //ebo_id
        };
    }

    /**
    ### Configure the layout for the VBO data
    Method will configure the layout via vertex attributes so OpenGL</br>
    knows how to interpret and group the data provided to it.

    **Arguments**:
    * **elements_per_vertex**: The amount of numbers in a single vertex
    * **layout**: An array of `VertexAttrib`s that will be in a single vertex
    */
    pub fn set_layout(&self, elements_per_vertex: i32, layout: &[VertexAttrib]) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo_id);
            gl::BindVertexArray(self.vao_id);
        };
        let stride = elements_per_vertex * size_of::<GLfloat>() as GLsizei;

        let mut offset = 0;
        let mut attrib_index = 0;
        for attrib in layout {
            unsafe {
                gl::VertexAttribPointer(
                    attrib_index,
                    attrib.size,
                    attrib.vtype,
                    attrib.normal,
                    stride,
                    (offset * size_of::<GLfloat>() as i32) as *const c_void
                );
                gl::EnableVertexAttribArray(attrib_index);
            }
            attrib_index += 1;
            offset += attrib.size;
        }
        drop(layout);
    }

    pub fn bind(&self) {
        unsafe {gl::BindVertexArray(self.vao_id)};
    }

    pub fn unbind(&self) {
        unsafe {gl::BindVertexArray(0)};
    }
}