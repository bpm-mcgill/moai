use glfw::{Action, Context, Key};
use moai::SCWindow;
use moai::gl::VBO;
use moai::shader::Shader;

fn main() {
    // TODO: In the future, make an Application struct that sets up the moai project
    //       and allows for configuration. Initialize env_logger then
    env_logger::init();
    let mut window = SCWindow::new(String::from("Rust Square"), (3,3), [900,600]);
    unsafe { gl::Viewport(0, 0, 900, 600) };
    unsafe { gl::ClearColor(0.03, 0.01, 0.08, 1.0) };

    let vertices: [f32; 12] = [
         0.5,  0.5, 0.0,  // top right
         0.5, -0.5, 0.0,  // bottom right
        -0.5, -0.5, 0.0,  // bottom left
        -0.5,  0.5, 0.0   // top left
    ];
    let indices: [i32; 6] = [ // note that we start from 0!
        0, 1, 3,  // first Triangle
        1, 2, 3   // second Triangle
    ];

    // Program will terminate if there's an error (boo hoo too bad)
    let shader = Shader::new().unwrap();
    let vb = VBO::new(&vertices, &indices);

    // Loop until the user closes the window
    while !window.window.should_close() {
        // Swap front and back buffers
        unsafe { gl::Clear(gl::COLOR_BUFFER_BIT) };
        unsafe {
            shader.bind();
            vb.bind();
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
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