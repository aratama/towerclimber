use crate::body::Body;
use crate::graphics::Graphics;
use crate::image::fruit::FRUIT_IMAGE;
use crate::image::player::PLAYER_IMAGE;
use crate::input::Inputs;
use crate::vector2::Vector2;
use crate::wasm4;
use crate::world::World;
use fastrand::Rng;

const MIN_PLAYER_LOOKUP: i32 = -60;
const MAX_PLAYER_LOOKUP: i32 = 70;

pub struct Game {
    rng: Rng,
    frame_count: u32,
    player: Body,
    player_lookup: i32,
    prev_gamepad: u8,
    fruits: std::vec::Vec<Body>,
    world: World,
}

impl Game {
    pub fn new() -> Self {
        let rng = Rng::with_seed(235);

        let player = Body::new("player", Vector2::new(8.0 * 5.0, 8.0 * 4.0), PLAYER_IMAGE);

        let fruits = vec![Body::new(
            "fruit",
            Vector2::new(
                rng.i32(0..wasm4::SCREEN_SIZE as i32) as f32,
                rng.i32(0..wasm4::SCREEN_SIZE as i32) as f32,
            ),
            FRUIT_IMAGE,
        )];

        let world = World::new();

        Self {
            frame_count: 0,
            player,
            player_lookup: MIN_PLAYER_LOOKUP,
            prev_gamepad: 0,
            fruits,
            rng,
            world,
        }
    }

    pub fn update(&mut self) {
        let gamepad = unsafe { *wasm4::GAMEPAD1 };

        self.frame_count += 1;

        let inputs = Inputs::new(gamepad, self.prev_gamepad);

        self.player.input(inputs, &self.world);

        self.prev_gamepad = unsafe { *wasm4::GAMEPAD1 };

        self.player.update(inputs, &self.world);

        if self.player.is_grounded(&self.world) && inputs.is_button_pressed(wasm4::BUTTON_UP) {
            self.player_lookup = i32::min(MAX_PLAYER_LOOKUP, self.player_lookup + 2);
        } else {
            self.player_lookup = i32::max(MIN_PLAYER_LOOKUP, self.player_lookup - 4);
        }

        for fruit in self.fruits.iter_mut() {
            fruit.update(Inputs::new(0, 0), &self.world);
        }

        let graphics = Graphics {
            frame_count: self.frame_count,
            debug: false,
            dx: wasm4::SCREEN_SIZE as i32 / 2 - self.player.position.x.floor() as i32,
            dy: wasm4::SCREEN_SIZE as i32 / 2 - self.player.position.y.floor() as i32
                + i32::max(0, self.player_lookup),
        };
        self.world.draw(graphics);

        self.player.draw(graphics, &self.world);

        for fruit in self.fruits.iter() {
            fruit.draw(graphics, &self.world);
        }
    }
}
