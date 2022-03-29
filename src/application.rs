use glow::*;
use crate::editor::*;
use crate::game::*;
use crate::renderer::*;
use crate::rect::*;
use crate::kmath::*;

/*
This makes sense
maybe it can receive an "I'm done" signal. Thats an exit code. or Error return

This has gl, etc
this decodes input based on what the thing is

how are we doing the editor/game thing
maybe a scene stack is a good

where does input go.

I've got that interface point of the EditorCommands enum but application probably doesnt hand it to that
Somewhere theres logic to translate mouse coordinates and to look up keys in the schema
mouse logic needs to know about level

is the enum necessary? It constrains things in a pretty nice way making them clean
even though it will be literally make enum and then do enum lol. But its type checked, functional. Separate decoding from dispatching.
*/

pub enum Scene {
    Editor(Editor),
    Game(Game),
}

impl Scene {
    pub fn handle_keypress(&mut self, event: glutin::event::KeyboardInput) {
        match self {
            Scene::Editor(editor) => {

            },
            Scene::Game(game) => {

            },
        }
    }

    pub fn handle_mouse_click(&mut self, button: glutin::event::MouseButton, state: glutin::event::ElementState, x: f32, y: f32) {
        match self {
            Scene::Editor(editor) => {

            },
            Scene::Game(game) => {

            },
        }
    }
}

// boilerplate & scene mgmt
pub struct Application {
    gl: glow::Context,
    window: glutin::WindowedContext<glutin::PossiblyCurrent>,
    egui: egui_glow::EguiGlow,

    renderer: Renderer,

    shader_program: glow::Program,

    pub xres: f32,
    pub yres: f32,
    
    scene_stack: Vec<Scene>,

}

impl Application {
    pub fn new(event_loop: &glutin::event_loop::EventLoop<()>) -> Application {
        let default_xres = 1600.0;
        let default_yres = 900.0;

        let (gl, window) = unsafe { opengl_boilerplate(default_xres, default_yres, event_loop) };
        let egui_glow = egui_glow::EguiGlow::new(&window, &gl);

        let renderer = Renderer::new(&gl, default_xres/default_yres);

        let shader_program = make_shader(&gl, "src/test.vert", "src/test.frag");

        unsafe { gl.use_program(Some(shader_program)); }

        Application {
            gl,
            window,
            egui: egui_glow,
            renderer,

            shader_program,

            xres: default_xres,
            yres: default_yres,

            scene_stack: vec![Scene::Editor(Editor::new())],
        }
    }

    pub fn resize(&mut self, new_xres: f32, new_yres: f32) {
        let ps = winit::dpi::PhysicalSize::new(new_xres as u32, new_yres as u32);
        self.window.resize(ps);
        self.xres = new_xres;
        self.yres = new_yres;
        unsafe { self.gl.viewport(0, 0, new_xres as i32, new_yres as i32) };
        // projection mat aspect ratio too?
        // renderer resize
    }

    pub fn draw(&mut self) {
        unsafe { self.gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT); } 
        
        self.renderer.clear();
        self.renderer.top_left = Vec2::new(0.0, 0.0);
        self.renderer.bot_right = Vec2::new(self.xres/self.yres, 1.0);

        match &self.scene_stack[self.scene_stack.len()-1] {
            Scene::Editor(editor) => {
                let screen_rect = Rect::new(0.0, 0.0, self.xres/self.yres, 1.0);
                let level_rect = screen_rect.fit_center_square();
                self.renderer.draw_rect(screen_rect, Vec3::new(0.0, 1.0, 0.0), 1.0); // y not red lmao
                editor.level.draw(&mut self.renderer, level_rect);
                // self.renderer.draw_rect(level_rect, Vec3::new(1.0, 0.0, 0.0), 2.0); // y not red lmao
            }
            Scene::Game(game) => {
                let level_rect = Rect::new_centered(0.5, 0.5, 1.0, 1.0);
                game.level.draw(&mut self.renderer, level_rect);
            }
        }

        self.renderer.present(&self.gl);
        self.window.swap_buffers().unwrap();
    }

    pub fn destroy(&mut self) {
        self.renderer.destroy(&self.gl);
    }

    pub fn egui_event(&mut self, event: &glutin::event::WindowEvent) {
        self.egui.on_event(event);
    }
}

fn  make_shader(gl: &glow::Context, vert_path: &str, frag_path: &str) -> glow::Program {
    unsafe {
        let program = gl.create_program().expect("Cannot create program");
        let shader_version = "#version 410";
        let shader_sources = [
            (glow::VERTEX_SHADER, std::fs::read_to_string(vert_path).unwrap()),
            (glow::FRAGMENT_SHADER, std::fs::read_to_string(frag_path).unwrap()),
            ];
        let mut shaders = Vec::with_capacity(shader_sources.len());
        for (shader_type, shader_source) in shader_sources.iter() {
            let shader = gl
            .create_shader(*shader_type)
            .expect("Cannot create shader");
            gl.shader_source(shader, &format!("{}\n{}", shader_version, shader_source));
            gl.compile_shader(shader);
            if !gl.get_shader_compile_status(shader) {
                panic!("{}", gl.get_shader_info_log(shader));
            }
            gl.attach_shader(program, shader);
            shaders.push(shader);
        }
        gl.link_program(program);
        if !gl.get_program_link_status(program) {
            panic!("{}", gl.get_program_info_log(program));
        }
        for shader in shaders {
            gl.detach_shader(program, shader);
            gl.delete_shader(shader);
        }
        
        program
    }
}

unsafe fn opengl_boilerplate(xres: f32, yres: f32, event_loop: &glutin::event_loop::EventLoop<()>) -> (glow::Context, glutin::WindowedContext<glutin::PossiblyCurrent>) {
    let window_builder = glutin::window::WindowBuilder::new()
        .with_title("tape")
        .with_inner_size(glutin::dpi::PhysicalSize::new(xres, yres));
    let window = glutin::ContextBuilder::new()
        // .with_depth_buffer(0)
        // .with_srgb(true)
        // .with_stencil_buffer(0)
        .with_vsync(true)
        .build_windowed(window_builder, &event_loop)
        .unwrap()
        .make_current()
        .unwrap();


    let gl = glow::Context::from_loader_function(|s| window.get_proc_address(s) as *const _);
    gl.enable(DEPTH_TEST);
    // gl.enable(CULL_FACE);
    // gl.blend_func(SRC_ALPHA, ONE_MINUS_SRC_ALPHA);
    // gl.enable(BLEND);
    gl.debug_message_callback(|a, b, c, d, msg| {
        println!("{} {} {} {} msg: {}", a, b, c, d, msg);
    });

    (gl, window)
}