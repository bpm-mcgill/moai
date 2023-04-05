/*
    TODO
    ❗❗THIS IS ENTIRELY TEMPORARY
    GLFW is just here so I didn't have to mess around with
    glutin/winit.
*/

use std::sync::mpsc::Receiver;
use glfw::{Context, Window, WindowEvent, Glfw};

pub struct MoaiWindow {
    // User specified
    pub title: String,
    pub ogl_version: (u32, u32),
    pub dimensions: [u32; 2],

    // GLFW backend
    pub window: Window,
    pub glfw: Glfw,
        // Input
        pub events: Receiver<(f64, WindowEvent)>,

    // GLFW UI
    //pub is_open: bool
}

impl MoaiWindow {
    pub fn new(title: String, ogl_version: (u32, u32), dimensions: [u32; 2]) -> Self{
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        // Causes errors on older systems if placed after create_window
        glfw.window_hint(glfw::WindowHint::ContextVersion(ogl_version.0, ogl_version.1));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

        let (mut window, events) = 
            glfw.create_window(dimensions[0], dimensions[1], title.as_str(), glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

        // Make the window's context current
        window.make_current();
        window.set_key_polling(true);
        window.set_framebuffer_size_polling(true);

        gl::load_with(|s| glfw.get_proc_address_raw(s) as *const _);

        let wind = MoaiWindow {
            //is_open: !window.should_close(),

            title,
            ogl_version,
            dimensions,

            events,
            glfw,
            window,
        };
        return wind;
    }
}