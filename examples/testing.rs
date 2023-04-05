use glfw::{Action, Context, Key};
use image::GenericImageView;
use moai::gl::{VertexAttrib, VBO};
use moai::shader::Shader;
use moai::MoaiWindow;

fn main() {
    // TODO: In the future, make an Application struct that sets up the moai project
    //       and allows for configuration. Initialize env_logger then
    env_logger::init();
    let mut window = MoaiWindow::new(String::from("Moai Demo"), (3, 3), [900, 600]);
    unsafe { gl::Viewport(0, 0, 900, 600) };
    unsafe { gl::ClearColor(0.03, 0.01, 0.08, 1.0) };
    unsafe { gl::Enable(gl::DEPTH_TEST) };
    window.window.set_cursor_pos_polling(true);
    window.window.set_cursor_mode(glfw::CursorMode::Disabled); // Capture mouse (makes view kinda jittery)
    
    let mut vertices = vec![];
    let mut indices: Vec<i32> = vec![];

    let (models, materials) = tobj::load_obj("examples/rx7.obj", &tobj::GPU_LOAD_OPTIONS).unwrap();
    let mut maxv: u32 = 0;
    for (j, model) in models.iter().enumerate(){
        let mesh = &model.mesh;
        if mesh.positions.len() % 3 != 0 {break};

        // Vertex: [PosX, PosY, PosZ, NormX, NormY, NormZ, TexX, TexY, ColX, ColY, ColZ]
        for (i, pos) in mesh.positions.chunks(3).enumerate(){
            if mesh.normals.is_empty() && mesh.texcoords.is_empty(){break};
            vertices.push(pos[0]); // X
            vertices.push(pos[1]); // Y
            vertices.push(pos[2]); // Z
            vertices.push(mesh.normals[i*3]);   // X
            vertices.push(mesh.normals[i*3+1]); // Y
            vertices.push(mesh.normals[i*3+2]); // Z
            vertices.push(mesh.texcoords[i*2]);   // X
            vertices.push(mesh.texcoords[i*2+1]); // Y
        }
        for inde in mesh.indices.iter(){
            indices.push(*inde as i32+maxv as i32);
        }
        maxv = vertices.len() as u32/8 as u32;
    }

    // Program will terminate if there's an error (boo hoo too bad)
    let shader = Shader::new().unwrap();
    let vb = VBO::new(&vertices, &indices);
    vb.set_layout(8, &[
        VertexAttrib { size: 3, vtype: gl::FLOAT, normal: gl::FALSE }, // position
        VertexAttrib { size: 3, vtype: gl::FLOAT, normal: gl::TRUE }, // normals
        VertexAttrib { size: 2, vtype: gl::FLOAT, normal: gl::FALSE }, // texture coords
    ]);

    // Rust png crate is considerably faster, but it doesn't matter since this
    // is frontend code. The user decides how to parse their own images
    
    let img = image::open("examples/moai.png")
        .expect("Couldn't open image (bruh)")
        .flipv();

    // This will be abstracted into a material class in the future
    let tid = unsafe {
        let tid = moai::gl::texture::gen_texture();
        moai::gl::texture::set_texture_data(tid, img.as_bytes(), img.dimensions(), true);
        tid
    };

    let model = glam::Mat4::from_rotation_x(90.0_f32.to_radians());

    shader.bind();
    let bruh: glam::Vec4 = glam::Vec4::new(1.0, 0.6, 0.3, 1.0);
    shader.set_vec4("col", bruh);

    let mut prev_pos = window.window.get_cursor_pos();
    let mut cam = moai::camera::Camera::new(90.0, 900 as f32 / 600 as f32, 0.1, 10000.0);

    // Loop until the user closes the window
    while !window.window.should_close() {
        // Swap front and back buffers

        // Poll for and process events
        window.glfw.poll_events();
        for (_, event) in glfw::flush_messages(&window.events) {
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.window.set_should_close(true)
                },
                glfw::WindowEvent::CursorPos(xpos, ypos) => {
                    cam.yaw += (xpos-prev_pos.0) as f32;
                    cam.pitch += (ypos-prev_pos.1) as f32;
                    prev_pos = (xpos, ypos);
                }
                _ => {}
            }
        }
        if window.window.get_key(Key::W) == Action::Press {
            let dir = cam.get_front();
            cam.translate(dir*0.1);
        }
        if window.window.get_key(Key::A) == Action::Press {
            let dir = cam.get_left();
            cam.translate(dir*0.1);
        }
        if window.window.get_key(Key::S) == Action::Press {
            let dir = cam.get_back();
            cam.translate(dir*0.1);
        }
        if window.window.get_key(Key::D) == Action::Press {
            let dir = cam.get_right();
            cam.translate(dir*0.1);
        }
        cam.update();

        unsafe { gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT) };
        unsafe {
            shader.bind();
            vb.bind();
            shader.set_mat4("view", cam.get_view_projection());
            shader.set_mat4("model", model);
            gl::BindTexture(gl::TEXTURE_2D, tid);
            gl::DrawElements(
                gl::TRIANGLES,
                vb.indices_size,
                gl::UNSIGNED_INT,
                std::ptr::null(),
            );
        }
        window.window.swap_buffers();
    }
}
