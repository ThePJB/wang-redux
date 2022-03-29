use crate::level::*;
use crate::kmath::*;

pub struct Editor {
    pub cursor_state: CursorState,
    pub level: Level,
}

impl Editor {
    pub fn new() -> Editor {
        Editor {
            cursor_state: CursorState::ColourPlacer(Vec3::new(1.0, 0.0, 0.0)),
            level: Level::new(10,10),
        }
    }

    pub fn handle_input(&mut self, command: EditorCommand) {
        match command {
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
            EditorCommand::PlayLevel => {},
            EditorCommand::SaveLevel => {},
            EditorCommand::LoadLevel => {},
            EditorCommand::ClearTape => {},
            EditorCommand::SetTape(pos, colour) => {},
        }
    }

    pub fn draw() {

    }
}

pub enum CursorState {
    ColourPicker,
    ColourPlacer(Vec3),
    PlacePlayer,
    PlacePowerup(i32),
    PlaceGoal,
    ClearEntity,
}

pub enum EditorCommand {
    SetCursor(CursorState),
    Curse(i32, i32),

    Resize(i32, i32),

    SetColour(Vec3),
    
    PlayLevel,
    SaveLevel,
    LoadLevel,

    ClearTape,
    SetTape(i32, Vec3),
}

