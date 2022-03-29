mod renderer;
mod rect;
mod kmath;
mod editor;
mod level;
mod application;
mod game;

// use glow::*;
// use std::error::Error;
// use kmath::*;
// use renderer::*;
// use rect::*;
// use std::collections::HashSet;
// use std::time::{Duration, SystemTime};

use application::*;
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::ControlFlow;




fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let mut application = Application::new(&event_loop);
    
    let mut frame = 0;
    let mut mouse_x = 0.0;
    let mut mouse_y = 0.0;
    
    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::LoopDestroyed |
            Event::WindowEvent {event: WindowEvent::CloseRequested, ..} |
            Event::WindowEvent {event: WindowEvent::KeyboardInput {
                input: glutin::event::KeyboardInput { virtual_keycode: Some(glutin::event::VirtualKeyCode::Escape), ..}, ..}, ..}
            => {
                *control_flow = ControlFlow::Exit;
            },

            Event::NewEvents(_) => {
                // prev_frame_start = curr_frame_start;
                // curr_frame_start = SystemTime::now();
                // dt = curr_frame_start.duration_since(prev_frame_start).unwrap().as_secs_f64();
                frame += 1;
            },

            Event::MainEventsCleared => {
                // application.update(&held_keys, dt as f32);
                application.draw();
                // game.window.window().set_title(&format!("RustVox | {:.2}ms", dt*1000.0));
            },

            Event::WindowEvent { ref event, .. } => match event {
                // WindowEvent::KeyboardInput { input: glutin::event::KeyboardInput, ..} => {},
                WindowEvent::KeyboardInput { input, .. } => {

                },
                WindowEvent::CursorMoved { position: pos, ..} => {
                    mouse_x = pos.x as f32 / application.xres;
                    mouse_y = pos.y as f32 / application.yres;
                },
                WindowEvent::MouseInput { state, button, .. } => {

                },
                _ => (),
            }
            _ => (),
        }
    });
}