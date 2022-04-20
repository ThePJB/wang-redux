use crate::level::*;
use crate::application::*;
use crate::renderer::*;
use crate::kmath::*;
use crate::manifest::*;
use crate::kgui::*;
use crate::rendererUV::TriangleBufferUV;

use std::collections::HashMap;

use glutin::event::ElementState;
use glutin::event::VirtualKeyCode;


pub struct Game {
    pub level: Level,
    pub place_tile: Tile,
    pub place_idx: i32,
}

impl Game {

}

impl Scene for Game {
    fn frame(&mut self, inputs: FrameInputState) -> (SceneOutcome, TriangleBuffer, Option<TriangleBufferUV>) {
        let mut buf = TriangleBuffer::new(inputs.screen_rect);
        let mut buf_uv = TriangleBufferUV::new(inputs.screen_rect, ATLAS_W, ATLAS_H);
        
        let click = inputs.events.iter().any(|e| match e {KEvent::MouseLeft(true) => true, _ => false});
        let clickr = inputs.events.iter().any(|e| match e {KEvent::MouseRight(true) => true, _ => false});

        let (maybe_rollover_palette, maybe_rollover_grid) = self.level.frame(&mut buf, &mut buf_uv, inputs.screen_rect, &inputs, Some(self.place_idx));
        if let Some(rollover_palette) = maybe_rollover_palette {
            if click || inputs.held_lmb {
                self.place_tile = self.level.tile_palette[rollover_palette as usize];
                self.place_idx = rollover_palette;
            }
        }
        if let Some((x, y)) = maybe_rollover_grid {
            if click || inputs.held_lmb {
                if self.level.can_place(x, y, self.place_tile) && !self.level.get_locked(x, y) {
                    self.level.set_tile(x, y, self.place_tile);
                }
            } else if (clickr || inputs.held_rmb) && !self.level.get_locked(x, y) {
                self.level.clear_tile(x, y);
            }
        }

        for event in inputs.events {
            match event {
                KEvent::Keyboard(VirtualKeyCode::Q, true) => self.place_tile = [self.place_tile[1], self.place_tile[2], self.place_tile[3], self.place_tile[0]],
                KEvent::Keyboard(VirtualKeyCode::E, true) => self.place_tile = [self.place_tile[3], self.place_tile[0], self.place_tile[1], self.place_tile[2]],
                KEvent::Keyboard(VirtualKeyCode::Escape, true) => {return (SceneOutcome::Pop(SceneSignal::JustPop), buf, None)},
                _ => {},
            }
        }

        (SceneOutcome::None, buf, Some(buf_uv))
    }
    
    fn handle_signal(&mut self, signal: SceneSignal) -> SceneOutcome {
        SceneOutcome::None
    }
}
