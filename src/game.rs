use crate::level::*;
use crate::application::*;
use crate::renderer::*;
use crate::rendererUV::*;
use crate::rect::*;
use crate::kmath::*;
use crate::manifest::*;

use std::collections::HashMap;

use glutin::event::ElementState;


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
            GameCommand::Move(dir) => {
                let new_pos = (self.level.player.0 + dir.0, self.level.player.1 + dir.1);
                if self.level.alive && new_pos.0 >= 0 && new_pos.0 < self.level.w && new_pos.1 >= 0 && new_pos.1 < self.level.h {
                    self.level.player = new_pos;
                    self.level.tape_cursor = (self.level.tape_cursor + 1) % self.level.tape.len() as i32;
                    match self.level.get_tile(new_pos.0, new_pos.1) {
                        Tile::Colour(colour) => {
                            if colour != self.level.tape[self.level.tape_cursor as usize] {
                                self.level.alive = false;
                            } else {
                                if self.level.goal == self.level.player {
                                    println!("winner!");
                                    return SceneOutcome::Pop(SceneSignal::JustPop);
                                }
                            }
                        },
                        _ => {},
                    }
                    
                }
            },
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
        key_cmd_schema.insert(glutin::event::VirtualKeyCode::W, GameCommand::Move((0, -1)));
        key_cmd_schema.insert(glutin::event::VirtualKeyCode::S, GameCommand::Move((0, 1)));
        key_cmd_schema.insert(glutin::event::VirtualKeyCode::A, GameCommand::Move((-1, 0)));
        key_cmd_schema.insert(glutin::event::VirtualKeyCode::D, GameCommand::Move((1, 0)));

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

        buf.draw_rect(screen_rect, Vec3::new(0.1, 0.1, 0.1), 0.9);


        let mut bufUV = TriangleBufferUV::new(screen_rect, 20, 20);
        let level_rect = screen_rect.child(0.0, 0.0, 1.0, 0.9).fit_center_square();

        self.level.draw(&mut buf, &mut bufUV, level_rect);

        let bot_rect = Rect::new(level_rect.x, level_rect.h, screen_rect.w - 2.0 * level_rect.x, 0.1 * screen_rect.h);
        buf.draw_rect(bot_rect, Vec3::new(0.3, 0.3, 0.3), 2.0);
        let n = self.level.tape.len();
        for (i, sym) in self.level.tape.iter().enumerate() {
            let sym_rect = bot_rect.grid_child(i as i32, 0, n as i32, 1).fit_center_square();
            if i as i32 == self.level.tape_cursor {
                buf.draw_rect(sym_rect.dilate(-0.01), Vec3::new(1.0, 1.0, 1.0), 3.0);
            }
            buf.draw_rect(sym_rect.dilate(-0.01).dilate(-0.01), COLOURS[*sym], 4.0);
        }

        (Some(buf), Some(bufUV))
    }
}
