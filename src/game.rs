use crate::level::*;
use crate::application::*;
use crate::renderer::*;
use crate::rendererUV::*;
use crate::rect::*;
use crate::kmath::*;
use std::collections::HashMap;

#[derive(Clone, Copy)]
pub enum GameCommand {
    Move((i32, i32)),

    Undo,
    Quit,
}

pub struct Game {
    pub level: Level,
}

impl Game {
    pub fn handle_command(&mut self, command: GameCommand) -> SceneOutcome {
        match command {
            GameCommand::Move(dir) => {},
            GameCommand::Undo => {},
            GameCommand::Quit => {return SceneOutcome::Pop(SceneSignal::JustPop)},
        };
        return SceneOutcome::None;
    }
}

impl Scene for Game {
    fn handle_event(&mut self, event: &glutin::event::Event<()>, screen_rect: Rect, cursor_pos: Vec2) -> SceneOutcome {
        let mut key_cmd_schema = HashMap::new();
        key_cmd_schema.insert(glutin::event::VirtualKeyCode::Escape, GameCommand::Quit);

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
        SceneOutcome::None
    }

    fn draw(&self, screen_rect: Rect) -> (Option<TriangleBuffer>, Option<TriangleBufferUV>) {
        let mut buf = TriangleBuffer::new(screen_rect);
        buf.draw_rect(Rect::new(0.25, 0.25, 0.5, 0.5), Vec3::new(1.0, 0.0, 0.0), 1.0);
        (Some(buf), None)
    }
}
