use crate::level::*;
use crate::kmath::*;
use crate::renderer::*;
use crate::rect::*;
use crate::application::*;
use crate::game::*;
use crate::colour_picker::*;

use std::collections::HashMap;
// use glow::event::Event;

pub struct Editor {
    pub cursor_state: CursorState,
    pub level: Level,
}

impl Scene for Editor {
    fn handle_event(&mut self, event: &glutin::event::Event<()>) -> SceneOutcome {
        let mut key_cmd_schema = HashMap::new();
        key_cmd_schema.insert(glutin::event::VirtualKeyCode::Space, EditorCommand::PlayLevel);
        key_cmd_schema.insert(glutin::event::VirtualKeyCode::C, EditorCommand::OpenColourPicker);

        let command = match event {
            glutin::event::Event::WindowEvent {event: glutin::event::WindowEvent::KeyboardInput {
                input: glutin::event::KeyboardInput { virtual_keycode, ..}, ..}, ..} => 
                {
                    if let Some(cmd) = key_cmd_schema.get(&virtual_keycode.unwrap()) {
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
        match signal {
            SceneSignal::Colour(c) => {
                self.handle_command(EditorCommand::SetColour(c))},
            _ => {SceneOutcome::None},
        }
    }
    fn draw(&self, gl: &glow::Context, r: &mut Renderer, egui: &mut egui_glow::EguiGlow, window: &winit::window::Window) {
        self.draw(r, Rect::new(0.0, 0.0, 1.0, 1.0));
    }
}

impl Editor {
    pub fn new() -> Editor {
        Editor {
            cursor_state: CursorState::ColourPlacer(Vec3::new(1.0, 0.0, 0.0)),
            level: Level::new(6,6),
        }
    }

    // scenes also need to interpret popped values
    // why im not using dynamic dispatch again? or static dispatch with generics
    // I want my vecs to work

    // pub fn translate_event(&self, event: Event) -> Option<EditorCommand> {
    //     // schema usings
    // }

    pub fn handle_command(&mut self, command: EditorCommand) -> SceneOutcome {
        match command {
            EditorCommand::OpenColourPicker => {return SceneOutcome::Push(Box::new(ColourPicker{}))},
            EditorCommand::SetCursor(state) => {self.cursor_state = state},
            EditorCommand::Curse(x, y) => {
                match self.cursor_state {
                    CursorState::ColourPicker => {self.cursor_state = CursorState::ColourPlacer(self.level.get_tile(x, y))},
                    CursorState::ColourPlacer(colour) => {self.level.set_tile(x, y, colour)},
                    CursorState::PlacePlayer => {},
                    CursorState::PlacePowerup(n) => {},
                    CursorState::PlaceGoal => {},
                    CursorState::ClearEntity => {},
                }
            },
            EditorCommand::Resize(w, h) => {self.level.resize(w,h)},
            EditorCommand::SetColour(colour) => {self.cursor_state = CursorState::ColourPlacer(colour)},
            EditorCommand::PlayLevel => {return SceneOutcome::Push(Box::new(Game {level: self.level.clone()}))},
            EditorCommand::SaveLevel => {},
            EditorCommand::LoadLevel => {},
            EditorCommand::ClearTape => {},
            EditorCommand::SetTape(pos, colour) => {},
        }
        return SceneOutcome::None;
    }

    pub fn draw(&self, renderer: &mut Renderer, screen_rect: Rect) {
        let level_rect = screen_rect.fit_center_square();
        renderer.draw_rect(screen_rect, Vec3::new(0.0, 1.0, 0.0), 1.0); // y not red lmao
        self.level.draw(renderer, level_rect);
    }
}

#[derive(Clone, Copy)]
pub enum CursorState {
    ColourPicker,
    ColourPlacer(Vec3),
    PlacePlayer,
    PlacePowerup(i32),
    PlaceGoal,
    ClearEntity,
}

#[derive(Clone, Copy)]
pub enum EditorCommand {
    SetCursor(CursorState),
    Curse(i32, i32),

    Resize(i32, i32),

    OpenColourPicker,

    SetColour(Vec3),
    
    PlayLevel,
    SaveLevel,
    LoadLevel,

    ClearTape,
    SetTape(i32, Vec3),
}

