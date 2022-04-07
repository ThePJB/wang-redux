use crate::level::*;
use crate::application::*;
use crate::renderer::*;
use crate::rendererUV::*;
use crate::rect::*;
use crate::kmath::*;
use crate::manifest::*;

use std::collections::HashMap;
use std::fs::*;
use std::io::Read;

use glutin::event::ElementState;


#[derive(Clone, Copy)]
pub enum MenuCommand {
    Move((i32, i32)),
    Select,
    Quit,
}

pub struct LevelMenu {
    selection: i32,
    levels: Vec<LevelMetadata>,
    width: i32,
}

impl LevelMenu {
    pub fn new() -> LevelMenu {
        let mut levels: Vec<LevelMetadata> = Vec::new();
        let entries = read_dir("levels/").unwrap();
        for entry in entries {
            let path = entry.unwrap().path();
            if !path.is_dir() {
                let mut f = File::open(path).unwrap();
                let mut contents = String::new();
                f.read_to_string(&mut contents).unwrap();

                let level_metadata = serde_json::from_str(&contents);
                levels.push(level_metadata.unwrap());
            }
        }

        println!("levels len: {}", levels.len());

        LevelMenu { selection: 0, levels, width: 4 }
    }

    pub fn handle_command(&mut self, command: MenuCommand) -> SceneOutcome {
        match command {
            MenuCommand::Move(dir) => {
                let new_selection = self.selection + dir.0 + self.width * dir.1;
                if new_selection >= 0 && new_selection < self.levels.len() as i32 {
                    self.selection = new_selection;
                }
            },
            MenuCommand::Select => {return SceneOutcome::Pop(SceneSignal::LevelChoice(self.levels[self.selection as usize].level.clone()))},
            MenuCommand::Quit => {return SceneOutcome::Pop(SceneSignal::JustPop)},
        };
        return SceneOutcome::None;
    }
}

impl Scene for LevelMenu {
    fn handle_event(&mut self, event: &glutin::event::Event<()>, screen_rect: Rect, cursor_pos: Vec2) -> SceneOutcome {
        let mut key_cmd_schema = HashMap::new();
        key_cmd_schema.insert(glutin::event::VirtualKeyCode::Escape, MenuCommand::Quit);
        key_cmd_schema.insert(glutin::event::VirtualKeyCode::Space, MenuCommand::Select);
        key_cmd_schema.insert(glutin::event::VirtualKeyCode::Return, MenuCommand::Select);
        key_cmd_schema.insert(glutin::event::VirtualKeyCode::W, MenuCommand::Move((0, -1)));
        key_cmd_schema.insert(glutin::event::VirtualKeyCode::S, MenuCommand::Move((0, 1)));
        key_cmd_schema.insert(glutin::event::VirtualKeyCode::A, MenuCommand::Move((-1, 0)));
        key_cmd_schema.insert(glutin::event::VirtualKeyCode::D, MenuCommand::Move((1, 0)));

        let command = match event {
            glutin::event::Event::WindowEvent {event: glutin::event::WindowEvent::KeyboardInput {
                input: glutin::event::KeyboardInput { virtual_keycode: Some(virtual_code), state: ElementState::Pressed, ..}, ..}, ..} => 
                {
                    if let Some(cmd) = key_cmd_schema.get(virtual_code) {
                         Some(cmd)
                    } else {
                        None
                    }
                },
            _ => {None},
        };

        if let Some(command) = command {
            self.handle_command(*command)
        } else {
            SceneOutcome::None
        }
    }

    fn handle_signal(&mut self, signal: SceneSignal) -> SceneOutcome {
        SceneOutcome::None
    }

    fn draw(&self, screen_rect: Rect) -> (Option<TriangleBuffer>, Option<TriangleBufferUV>) {
        let mut buf = TriangleBuffer::new(screen_rect);
        let mut bufUV = TriangleBufferUV::new(screen_rect, 20, 20);

        let w = self.width;
        let h = 4;

        for j in 0..h {
            for i in 0..w {
                let level_idx = i + j*w;
                let level_rect = screen_rect.fit_center_square().grid_child(i, j, w, h);
                if level_idx == self.selection {
                    buf.draw_rect(level_rect, Vec3::new(1.0, 1.0, 1.0), 1.0);
                }
                if let Some(level) = self.levels.get(level_idx as usize) {
                    level.level.draw(&mut buf, &mut bufUV, level_rect.dilate(-0.01));
                }
            }
        }

        (Some(buf), Some(bufUV))
    }
}
