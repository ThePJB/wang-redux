use crate::level::*;
use crate::kmath::*;
use crate::renderer::*;
use crate::rendererUV::*;
use crate::rect::*;
use crate::application::*;
use crate::game::*;

use std::collections::HashMap;

use glutin::event::ElementState;
use glutin::event::MouseButton;
use glutin::event::VirtualKeyCode;
use glutin::event::Event;
use glutin::event::WindowEvent::KeyboardInput;
use glutin::event::WindowEvent::MouseInput;
use glutin::event::WindowEvent::CursorMoved;


#[derive(Clone, Copy, Debug)]
pub enum CursorState {
    ColourPlacer(Vec3),
    PlacePlayer,
    PlacePowerup,
    PlaceGoal,
    ClearEntity,
}

#[derive(Clone, Copy, Debug)]
pub enum EditorCommand {
    SetCursor(CursorState),
    Curse(i32, i32),

    Resize(i32, i32),

    SetColour(Vec3),
    
    PlayLevel,
    SaveLevel,
    LoadLevel,

    PowerupInc,
    PowerupDec,

    ClearTape,
    SetTape(i32, Vec3),
}


pub struct Button {
    rect: Rect,
    command: EditorCommand,
    hotkey: VirtualKeyCode,
    appearance: ButtonAppearance,
}

pub enum ButtonAppearance {
    Colour(Vec3),
    Texture(i32),
}

pub struct Editor {
    pub cursor_state: CursorState,
    pub level: Level,
    pub powerup_amount: i32,
}

impl Scene for Editor {
    fn handle_event(&mut self, event: &Event<()>, screen_rect: Rect, cursor_pos: Vec2) -> SceneOutcome {
        let buttons = self.buttons(screen_rect);

        let mut key_cmd_schema = HashMap::new();
        key_cmd_schema.insert(VirtualKeyCode::Space, EditorCommand::PlayLevel);

        let command = match event {
            Event::WindowEvent {event, ..} => match event {

                KeyboardInput { input: glutin::event::KeyboardInput { virtual_keycode: Some(virtual_code), state: ElementState::Pressed, ..}, ..} => {
                    if let Some(cmd) = key_cmd_schema.get(virtual_code) {
                        Some(*cmd)
                   } else {
                       None
                   }
                },

                MouseInput { button: glutin::event::MouseButton::Left, state: glutin::event::ElementState::Pressed, ..} => {
                    let gui_button_cmd = buttons.iter().filter(|b| b.rect.contains(cursor_pos)).map(|b| b.command).nth(0);
                    if gui_button_cmd.is_some() {
                        gui_button_cmd
                    } else {
                        let level_rect = screen_rect.fit_center_square();
                        if level_rect.contains(cursor_pos) {
                            println!("{:?}", level_rect.relative_point(cursor_pos));
                            let (x, y) = level_rect.grid_square(level_rect.relative_point(cursor_pos), self.level.w, self.level.h);
                            Some(EditorCommand::Curse(x, y))
                        } else {
                            None
                        }
                    }
                },
                _ => None,
            },
            _ => None,
        };

        if let Some(command) = command {
            self.handle_command(command)
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

    fn draw(&self, screen_rect: Rect) -> (Option<TriangleBuffer>, Option<TriangleBufferUV>) {
        let mut buf = TriangleBuffer::new(screen_rect);
        let mut buf_uv = TriangleBufferUV::new(screen_rect, 20, 20);

        buf.draw_rect(screen_rect.child(0.0, 0.0, 1.0, 1.0), Vec3::new(0.9, 0.1, 0.9), 1.0);
        let level_rect = screen_rect.fit_center_square();
        self.level.draw(&mut buf, &mut buf_uv, level_rect);

        let buttons = self.buttons(screen_rect);

        for button in buttons {
            buf.draw_rect(button.rect, Vec3::new(0.2, 0.2, 0.2), 2.0);
            buf.draw_rect(button.rect.child(0.97, 0.0, 0.03, 1.0), Vec3::new(0.1, 0.1, 0.1), 3.0);
            buf.draw_rect(button.rect.child(0.0, 0.97, 1.0, 0.03), Vec3::new(0.1, 0.1, 0.1), 3.0);
            match button.appearance {
                ButtonAppearance::Colour(colour) => buf.draw_rect(button.rect.dilate(-0.01), colour, 4.0),
                ButtonAppearance::Texture(idx) => {buf_uv.draw_sprite(button.rect.fit_center_square(), idx, 5.0)},
            }
        }

        (Some(buf), Some(buf_uv))
    }
}

impl Editor {
    pub fn new() -> Editor {
        Editor {
            cursor_state: CursorState::ColourPlacer(Vec3::new(1.0, 0.0, 0.0)),
            level: Level::new(6,6),
            powerup_amount: 3,
        }
    }

    pub fn buttons(&self, screen_rect: Rect) -> Vec<Button> {
        let mut buttons = Vec::new();

        let level_rect = screen_rect.fit_center_square();
        let lpane = Rect::new(screen_rect.x, screen_rect.y, level_rect.x, screen_rect.h);
        let rpane = Rect::new(level_rect.x + level_rect.w, screen_rect.y, level_rect.x, screen_rect.h);


        buttons.push(Button {rect: lpane.grid_child(0, 0, 2, 7).dilate(-0.01), command: EditorCommand::PlayLevel, hotkey: VirtualKeyCode::Space, appearance: ButtonAppearance::Colour(Vec3::new(0.0, 0.0, 0.0))});
        buttons.push(Button {rect: lpane.grid_child(1, 0, 2, 7).dilate(-0.01), command: EditorCommand::SaveLevel, hotkey: VirtualKeyCode::S, appearance: ButtonAppearance::Colour(Vec3::new(0.0, 0.0, 0.0))});
        buttons.push(Button {rect: lpane.grid_child(1, 1, 2, 7).dilate(-0.01), command: EditorCommand::LoadLevel, hotkey: VirtualKeyCode::L, appearance: ButtonAppearance::Colour(Vec3::new(0.0, 0.0, 0.0))});

        buttons.push(Button {rect: lpane.grid_child(0, 2, 2, 7).dilate(-0.01), command: EditorCommand::SetCursor(CursorState::PlacePlayer), hotkey: VirtualKeyCode::P, appearance: ButtonAppearance::Texture(0)});
        buttons.push(Button {rect: lpane.grid_child(1, 2, 2, 7).dilate(-0.01), command: EditorCommand::SetCursor(CursorState::PlaceGoal), hotkey: VirtualKeyCode::G, appearance: ButtonAppearance::Texture(2)});
        buttons.push(Button {rect: lpane.grid_child(0, 3, 2, 7).dilate(-0.01), command: EditorCommand::PowerupInc, hotkey: VirtualKeyCode::LBracket, appearance: ButtonAppearance::Colour(Vec3::new(0.0, 0.0, 0.0))});
        buttons.push(Button {rect: lpane.grid_child(1, 3, 2, 7).dilate(-0.01), command: EditorCommand::PowerupDec, hotkey: VirtualKeyCode::RBracket, appearance: ButtonAppearance::Colour(Vec3::new(0.0, 0.0, 0.0))});
        buttons.push(Button {rect: lpane.grid_child(0, 4, 2, 7).dilate(-0.01), command: EditorCommand::SetCursor(CursorState::PlacePowerup), hotkey: VirtualKeyCode::U, appearance: ButtonAppearance::Texture(1)});
        buttons.push(Button {rect: lpane.grid_child(1, 4, 2, 7).dilate(-0.01), command: EditorCommand::SetCursor(CursorState::ClearEntity), hotkey: VirtualKeyCode::D, appearance: ButtonAppearance::Colour(Vec3::new(0.0, 0.0, 0.0))});
        
        buttons.push(Button {rect: lpane.grid_child(0, 5, 2, 7).dilate(-0.01), command: EditorCommand::Resize(1, 0), hotkey: VirtualKeyCode::F24, appearance: ButtonAppearance::Colour(Vec3::new(0.0, 0.0, 0.0))});
        buttons.push(Button {rect: lpane.grid_child(1, 5, 2, 7).dilate(-0.01), command: EditorCommand::Resize(-1, 0), hotkey: VirtualKeyCode::F24, appearance: ButtonAppearance::Colour(Vec3::new(0.0, 0.0, 0.0))});
        buttons.push(Button {rect: lpane.grid_child(0, 6, 2, 7).dilate(-0.01), command: EditorCommand::Resize(0, 1), hotkey: VirtualKeyCode::F24, appearance: ButtonAppearance::Colour(Vec3::new(0.0, 0.0, 0.0))});
        buttons.push(Button {rect: lpane.grid_child(1, 6, 2, 7).dilate(-0.01), command: EditorCommand::Resize(0, -1), hotkey: VirtualKeyCode::F24, appearance: ButtonAppearance::Colour(Vec3::new(0.0, 0.0, 0.0))});

        let colours = vec![
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(1.0, 1.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, 1.0, 1.0),
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 1.0, 1.0),
        ];

        for (i, colour) in colours.iter().enumerate() {
            buttons.push(Button {rect: rpane.grid_child(0, i as i32, 1, 7).dilate(-0.01), appearance: ButtonAppearance::Colour(*colour), command: EditorCommand::SetColour(*colour), hotkey: VirtualKeyCode::F24});
        }

        return buttons;
    }

    // scenes also need to interpret popped values
    // why im not using dynamic dispatch again? or static dispatch with generics
    // I want my vecs to work

    // pub fn translate_event(&self, event: Event) -> Option<EditorCommand> {
    //     // schema usings
    // }

    pub fn handle_command(&mut self, command: EditorCommand) -> SceneOutcome {
        println!("Editor Command: {:?}", command);
        match command {
            EditorCommand::PowerupInc => {self.powerup_amount += 1},
            EditorCommand::PowerupDec => {self.powerup_amount -= 1},
            EditorCommand::SetCursor(state) => {self.cursor_state = state},
            EditorCommand::Curse(x, y) => {
                match self.cursor_state {
                    CursorState::ColourPlacer(colour) => {self.level.set_tile(x, y, colour)},
                    CursorState::PlacePlayer => {self.level.player = (x, y)},
                    CursorState::PlacePowerup => {
                        self.level.powerups.retain(|(px, py, n)| x != *px || y != *py);
                        self.level.powerups.push((x, y, self.powerup_amount));
                    },
                    CursorState::PlaceGoal => {self.level.goal = (x, y)},
                    CursorState::ClearEntity => {self.level.powerups.retain(|(px, py, n)| x != *px || y != *py)},
                }
            },
            EditorCommand::Resize(w, h) => {self.level.resize(self.level.w + w, self.level.h + h)},
            EditorCommand::SetColour(colour) => {self.cursor_state = CursorState::ColourPlacer(colour)},
            EditorCommand::PlayLevel => {return SceneOutcome::Push(Box::new(Game {level: self.level.clone()}))},
            EditorCommand::SaveLevel => {},
            EditorCommand::LoadLevel => {},
            EditorCommand::ClearTape => {},
            EditorCommand::SetTape(pos, colour) => {},
        }
        return SceneOutcome::None;
    }
}
