use crate::gl::VertexAttrib;
use super::*;
//use crate::gl::VBO;

struct PBRMaterialData {
    ambient: [f64; 3],
    diffuse: [f64; 3],
    specular: [f64; 3],
    shininess: f64,
    optical_density: f64,
    ambient_texture: u32, // The opengl id for the texture
    diffuse_texture: u32,
    normal_texture: u32,
    shininess_texture: u32,
    alpha_texture: u32,
    illumination: Option<IlluminationModel>,
}

/**
### Wrapper for .mtl illumination models

Used in materials to decide which shader to render with. Some of the options aren't implemented</br>
and some will never be implemented. Objects loaded using an illumination model not implemented</br>
will get a default shader.

---
</br>

|Variant Name          |Illumination model                                                  |
|----------------------|--------------------------------------------------------------------|
|`COL`                 |Color on, Ambient off                                               |
|`COL_AMB`             |Color on, Ambient on                                                |
|`HIGHLIGHT`           |Highlight on                                                        |
|`REFL_RAY`            |Reflection on, Raytrace on                                          |
|`T_GLASS_REFL_RAY`    |Transparency: Glass on, Reflection: Raytrace on                     |
|`REFL_FRES_RAY`       |Reflection: Fresnel on, Raytrace on                                 |
|`T_REFR_RAY`          |Transparency: Refraction on, Reflection: Fresnel off, Raytrace on   |
|`T_REFR_REFL_FRES_RAY`|Transparency: Refraction on, Reflection: Fresnel on, Raytrace on    |
|`REFL`                |Reflection: Ray off                                                 |
|`T_GLASS_REFL`        |Transparency: Glass on, Reflection: Ray off                         |
|`SHADOW_CAST_INVIS`   |Cast shadows on invisable surfaces                                  |
http://paulbourke.net/dataformats/mtl/
*/
#[allow(non_camel_case_types)]
pub enum IlluminationModel {
    COL = 0,
    COL_AMB = 1,
    HIGHLIGHT = 2,
    REFL_RAY = 3,
    T_GLASS_REFL_RAY = 4,
    REFL_FRES_RAY = 5,
    T_REFR_RAY = 6,
    T_REFR_REFL_FRES_RAY = 7,
    REFL = 8,
    T_GLASS_REFL = 9,
    SHADOW_CAST_INVIS = 10,
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

// Load model into the least amount of meshes possible
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

    return Ok(Model {
        meshes: vec![]
    });
}