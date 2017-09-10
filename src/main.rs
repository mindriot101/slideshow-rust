#![allow(dead_code, unused_imports, unused_extern_crates,
         unused_variables, unused_mut, non_upper_case_globals, non_snake_case)]
extern crate glfw;
extern crate gl;
extern crate notify;

use glfw::{Context, Key, Action};
use std::sync::mpsc::{channel, Receiver};
use std::ffi::CString;
use std::str;
use std::ptr;
use gl::types::*;
use std::sync::Arc;
use std::mem;
use std::thread;
use std::os::raw::c_void;
use notify::{Watcher, RecursiveMode, RawEvent, raw_watcher};
use notify::op;

mod shader;
use shader::ShaderProgram;

mod geometry;
use geometry::Geometry;

const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

const vertices: [f32; 9] = [-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).expect("Failed to initialize glfw");
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));

    #[cfg(target_os = "macos")] glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

    let (mut window, events) = glfw.create_window(
        SCR_WIDTH,
        SCR_HEIGHT,
        "LearnOpenGL",
        glfw::WindowMode::Windowed,
    ).expect("Failed to create GLFW window");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let shader_program = Arc::new(ShaderProgram::new("shaders/basic.vert", "shaders/basic.frag")
        .expect("Cannot create shader program"));

    let geometry = Geometry::new(&shader_program, &vertices).expect("Cannot create geometry");

    let (tx, rx) = channel();
    let mut watcher = raw_watcher(tx).expect("Cannot create watcher");
    watcher.watch("shaders", RecursiveMode::Recursive).expect("Cannot watch shaders dir");

    let s = shader_program.clone();
    
    thread::spawn(move || {
        loop {
            match rx.recv() {
                Ok(RawEvent { path: Some(path), op: Ok(op), cookie }) => {
                    if !op.contains(op::WRITE) {
                        continue
                    }
                    s.reload();
                },
                _ => {},
            }
        }
    });

    while !window.should_close() {
        process_events(&mut window, &events);

        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        geometry.render();

        window.swap_buffers();
        glfw.poll_events();
    }
}

fn process_events(window: &mut glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => unsafe {
                gl::Viewport(0, 0, width, height)
            },
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                window.set_should_close(true)
            }
            _ => {}
        }
    }
}
