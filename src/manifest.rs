use crate::kmath::*;

pub const PLAYER: i32 = 0;
pub const DEAD_PLAYER: i32 = 3;
pub const POWERUP: i32 = 1;
pub const GOAL: i32 = 2;
pub const PLAY: i32 = 20;
pub const OPEN: i32 = 21;
pub const SAVE: i32 = 22;
pub const PLUS_H: i32 = 23;
pub const MINUS_H: i32 = 24;
pub const PLUS_W: i32 = 25;
pub const MINUS_W: i32 = 26;
pub const PLUS_TAPE: i32 = 27;
pub const MINUS_TAPE: i32 = 28;

pub const COLOURS: [Vec3; 7] = [
    Vec3::new(0.8, 0.1, 0.1),
    Vec3::new(1.0, 1.0, 0.0),
    Vec3::new(0.0, 1.0, 0.0),
    Vec3::new(0.0, 1.0, 1.0),
    Vec3::new(0.0, 0.0, 1.0),
    Vec3::new(0.1, 0.1, 0.1),
    Vec3::new(1.0, 1.0, 1.0),
];