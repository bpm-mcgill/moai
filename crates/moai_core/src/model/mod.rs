use crate::gl::VertexAttrib;
use crate::gl::VBO;

/**
    Write big description for this since it will be directly interfaced by the user
*/
pub struct Model {
    //model_mat: glam::Mat4
}

/**
### Internal Sub-model mesh
Batch-loaded models don't have materials since the material data will</br>
be contained within the VBO.
*/
pub struct Primitive{
    vbo: VBO,
    batched: bool,
    material: Option<Material>,
}

// Will implement uniform buffers in the future
pub struct Material {
    shader: i32, // The id for the shader the material uses
}

fn get_layout(mesh: tobj::Mesh) -> (Vec<VertexAttrib>, i32){
    let mut layout: Vec<VertexAttrib> = vec![];
    let mut stride = 3;

    layout.push(VertexAttrib { size: 3, vtype: gl::FLOAT, normal: gl::FALSE }); // Positions
    if !mesh.normals.is_empty(){
        layout.push(VertexAttrib { size: 3, vtype: gl::FLOAT, normal: gl::TRUE });
        stride += 3;
    }
    if !mesh.texcoords.is_empty(){
        layout.push(VertexAttrib { size: 2, vtype: gl::FLOAT, normal: gl::FALSE });
        stride += 2;
    }
    if mesh.material_id.is_some(){
        // Push material vec to vertices data
    }
    return (layout, stride);
}

pub fn batch_load_model(path: &str) -> Result<Model, tobj::LoadError>{
    let (models, materials) = tobj::load_obj(path, &tobj::GPU_LOAD_OPTIONS)?;
    let mut vertices: Vec<f32> = vec![];
    let mut indices: Vec<i32> = vec![];
    let mut layout: Vec<VertexAttrib> = vec![];

    // Load all of materials into a vector, matching up with the mesh.material_id indices
    // Load all of the textures. In
    let mut _mats: Vec<i32> = vec![];
    for material in materials.unwrap().iter(){
        println!("{:?}", material);
    }

    // Batch loading would have to load all of the material data from the
    // material vector made above into each of the vertices.
    // All of this data would be immutable. You can't change the mesh's material.
    // Due to this, batch loading will be in a different function that the user
    // can use if they want to batch load or just regular load

    // Most batched models will just be one mesh under one model
    // If they decide to batch load a model with more textures than max_tex, the
    // model will be split into two (or more, depending on the amount of textures) meshes, still under one model.
    let mut maxv: i32 = 0;
    for model in models{
        let mesh = model.mesh;
        if mesh.positions.len() % 3 != 0 {break};
        for i in 0..mesh.positions.len()/3{
            vertices.push(mesh.positions[i * 3]);
            vertices.push(mesh.positions[i * 3 + 1]);
            vertices.push(mesh.positions[i * 3 + 2]);
            if !mesh.normals.is_empty(){
                vertices.push(mesh.normals[i * 3]);
                vertices.push(mesh.normals[i * 3 + 1]);
                vertices.push(mesh.normals[i * 3 + 2]);
            }
            if !mesh.texcoords.is_empty(){
                vertices.push(mesh.texcoords[i * 2]);
                vertices.push(mesh.texcoords[i * 2 + 1]);
            }
            if mesh.material_id.is_some(){
                // Push material vec to vertices data
            }
        }
        for inde in mesh.indices.iter(){
            indices.push(*inde as i32+maxv);
        }
        let (layout, stride) = get_layout(mesh);
        maxv = vertices.len() as i32/stride;
    }

    return Ok(Model {});
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