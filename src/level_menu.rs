use crate::level::*;
use crate::application::*;
use crate::renderer::*;
use crate::rendererUV::*;
use crate::manifest::*;
use crate::kmath::*;
use crate::kgui::*;
use crate::rendererUV::TriangleBufferUV;

use std::collections::HashMap;
use std::fs::*;
use std::io::Read;

use glutin::event::ElementState;
use glutin::event::VirtualKeyCode;


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

        levels.sort_by_key(|lm| lm.level.complexity());
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
    fn frame(&mut self, inputs: FrameInputState) -> (SceneOutcome, TriangleBuffer, Option<TriangleBufferUV>) {
        let mut select = false;

        let outcome = inputs.events.iter().filter_map(|e| match e {
            KEvent::Keyboard(VirtualKeyCode::W, true) => Some(MenuCommand::Move((0, -1))),
            KEvent::Keyboard(VirtualKeyCode::S, true) => Some(MenuCommand::Move((0, 1))),
            KEvent::Keyboard(VirtualKeyCode::A, true) => Some(MenuCommand::Move((-1, 0))),
            KEvent::Keyboard(VirtualKeyCode::D, true) => Some(MenuCommand::Move((1, 0))),
            KEvent::Keyboard(VirtualKeyCode::Return, true) => Some(MenuCommand::Select),
            KEvent::Keyboard(VirtualKeyCode::Escape, true) => Some(MenuCommand::Quit),
            _ => {None},
        }).filter_map(|c| match self.handle_command(c) {
            SceneOutcome::Pop(signal) => Some(SceneOutcome::Pop(signal)),
            _ => None,
        }).nth(0).unwrap_or(SceneOutcome::None);

        let mut buf = TriangleBuffer::new(inputs.screen_rect);
        let mut buf_uv = TriangleBufferUV::new(inputs.screen_rect, ATLAS_W, ATLAS_H);

        let w = self.width;
        let h = 4;

        for j in 0..h {
            for i in 0..w {
                let level_idx = i + j*w;
                let level_rect = inputs.screen_rect.fit_center_square().grid_child(i, j, w, h);
                if level_idx == self.selection {
                    buf.draw_rect(level_rect, Vec3::new(1.0, 1.0, 1.0), 1.0);
                }
                if let Some(level) = self.levels.get(level_idx as usize) {
                    level.level.frame(&mut buf, &mut buf_uv, level_rect.dilate(-0.01), &inputs, None);
                }
            }
        }

        (outcome, buf, Some(buf_uv))
    }
    
    fn handle_signal(&mut self, signal: SceneSignal) -> SceneOutcome {
        SceneOutcome::None
    }
}