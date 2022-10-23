use crate::body::Body;
use crate::fruit::FRUIT_IMAGE;
use crate::player::PLAYER_IMAGE;
use crate::vector2::Vector2;
use crate::wasm4;
use crate::world;
use fastrand::Rng;

pub struct Inputs {
    gamepad: u8,
    prev_gamepad: u8,
}

impl Inputs {
    pub fn new(gamepad: u8, prev_gamepad: u8) -> Self {
        Self {
            gamepad,
            prev_gamepad,
        }
    }

    pub fn is_button_pressed(&self, button: u8) -> bool {
        (self.gamepad) & button != 0
    }

    pub fn is_button_just_pressed(&self, button: u8) -> bool {
        let just = self.gamepad & (self.gamepad ^ self.prev_gamepad);
        (just) & button != 0
    }
}