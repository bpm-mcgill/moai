/**
OBJs are a lot faster to load in and will work for simple models.

GLTFs are for loading in more complicated models with animations and complex materials.
*/

use crate::gl::VertexAttrib;
use crate::gl::VBO;
pub mod obj;
pub mod gltf;

/**
    Write big description for this since it will be directly interfaced by the user
*/
pub struct Model {
    //model_mat: glam::Mat4
    meshes: Vec<Mesh>
}

/**
### Internal Sub-model mesh
Batch-loaded models don't have materials since the material data will</br>
be contained within the VBO.
*/
pub struct Mesh{
    vbo: VBO,
    batched: bool,
    material: Option<Material>,
}

// Will implement uniform buffers in the future
pub struct Material {
    name: String,
    shader: i32, // The id for the shader the material uses
    //data: PBRMaterialData,
}

// Get the amount of texture slots available
/* 
let max_tex = unsafe {
    let mut units = 0;
    gl::GetIntegerv(gl::MAX_TEXTURE_IMAGE_UNITS, &mut units);
    units
};
println!("{}", max_tex);
*/