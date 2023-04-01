use glfw::{Action, Context, Key};
use image::GenericImageView;
use moai::MoaiWindow;
use moai::gl::{VBO, VertexAttrib};
use moai::shader::Shader;

// TODO: Style code consistently

fn main() {
    // TODO: In the future, make an Application struct that sets up the moai project
    //       and allows for configuration. Initialize env_logger then
    env_logger::init();
    let mut window = MoaiWindow::new(String::from("Moai Square"), (3,3), [900,600]);
    unsafe { gl::Viewport(0, 0, 900, 600) };
    unsafe { gl::ClearColor(0.03, 0.01, 0.08, 1.0) };

    let vertices: [f32; 32] = [
        // positions       // colors        // texture coords
         0.5,  0.5, 0.0,   1.0, 0.0, 0.0,   1.0, 1.0, // top right
         0.5, -0.5, 0.0,   0.0, 1.0, 0.0,   1.0, 0.0, // bottom right
        -0.5, -0.5, 0.0,   0.0, 0.0, 1.0,   0.0, 0.0, // bottom left
        -0.5,  0.5, 0.0,   1.0, 1.0, 0.0,   0.0, 1.0  // top left
    ];
    let indices = [
        0, 1, 3,  // first Triangle
        1, 2, 3   // second Triangle
    ];

    // Program will terminate if there's an error (boo hoo too bad)
    let shader = Shader::new().unwrap();
    let vb = VBO::new(&vertices, &indices);
    vb.set_layout(8,&[
        VertexAttrib {size: 3, vtype: gl::FLOAT, normal: gl::FALSE}, // position
        VertexAttrib {size: 3, vtype: gl::FLOAT, normal: gl::FALSE}, // color
        VertexAttrib {size: 2, vtype: gl::FLOAT, normal: gl::FALSE}, // texture coords
    ]);

    let img = image::open("examples/moai.png").expect("Couldn't open image (bruh)").flipv();

    // This will be abstracted into a material class in the future
    let tid = unsafe {
        let tid = moai::gl::texture::gen_texture();
        moai::gl::texture::set_texture_data(
            tid,
            img.as_bytes(),
            img.dimensions(),
            true
        );
        tid
    };

    // Rust png crate is considerably faster, but it doesn't matter since this
    // is frontend code. The user decides how to parse their own images
    shader.bind();
    let bruh: glam::Vec4  = glam::Vec4::new(1.0,0.6,0.3, 1.0);
    shader.set_vec4("col", bruh);

    // Loop until the user closes the window
    while !window.window.should_close() {
        // Swap front and back buffers
        unsafe { gl::Clear(gl::COLOR_BUFFER_BIT) };
        unsafe {
            shader.bind();
            vb.bind();
            gl::BindTexture(gl::TEXTURE_2D, tid);
            gl::DrawElements(gl::TRIANGLES, vb.indices_size, gl::UNSIGNED_INT, std::ptr::null());
        }

        window.window.swap_buffers();

        // Poll for and process events
        window.glfw.poll_events();
        for (_, event) in glfw::flush_messages(&window.events) {
            println!("{:?}", event);
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.window.set_should_close(true)
                },
                _ => {},
            }
        }
    }
}