use crate::body::Body;
use crate::graphics::Graphics;
use crate::image::player::PLAYER_IMAGE;
use crate::input::Inputs;
use crate::palette::set_draw_color;
use crate::save::{load, save, GameData, GAME_DATA_VERSION};
use crate::vector2::Vector2;
use crate::wasm4;
use crate::world::{World, CELL_SIZE};
use crate::world_map::WORLD_HEIGHT;
use fastrand::Rng;
use std::str;

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
    debug: bool,
    score: f32,
}

impl Game {
    pub fn new() -> Self {
        let rng = Rng::with_seed(235);

        let world = World::new();

        let player_x = world.start.x;
        let player_y = world.start.y;

        let player = Body::new(
            "player",
            // Vector2::new(CELL_SIZE as f32 * 13.0, CELL_SIZE as f32 * 235.0),
            Vector2::new(player_x, player_y),
            &PLAYER_IMAGE,
            6.0,
            12.0,
        );

        let fruits = vec![
        //     Body::new(
        //     "fruit",
        //     Vector2::new(
        //         rng.i32(0..wasm4::SCREEN_SIZE as i32) as f32,
        //         rng.i32(0..wasm4::SCREEN_SIZE as i32) as f32,
        //     ),
        //     FRUIT_IMAGE,
        //     CELL_SIZE as f32,
        //     CELL_SIZE as f32,
        // )
        ];

        Self {
            frame_count: 0,
            player,
            player_lookup: MIN_PLAYER_LOOKUP,
            prev_gamepad: 0,
            fruits,
            rng,
            world,
            debug: false,
            score: 0.0,
        }
    }

    pub fn update(&mut self) {
        // updates

        let gamepad = unsafe { *wasm4::GAMEPAD1 };

        self.frame_count += 1;

        let inputs = Inputs::new(gamepad, self.prev_gamepad);

        self.player.input(&inputs, &self.world);

        self.prev_gamepad = unsafe { *wasm4::GAMEPAD1 };

        self.player
            .physical_update(inputs.horizontal_acceralation() as i32, &self.world);

        if self.player.is_grounded(&self.world) && inputs.is_button_pressed(wasm4::BUTTON_UP) {
            self.player_lookup = i32::min(MAX_PLAYER_LOOKUP, self.player_lookup + 2);
        } else {
            self.player_lookup = i32::max(MIN_PLAYER_LOOKUP, self.player_lookup - 4);
        }

        for fruit in self.fruits.iter_mut() {
            fruit.physical_update(0, &self.world);
        }

        self.score = f32::max(
            self.score,
            (WORLD_HEIGHT as f32 * CELL_SIZE as f32 - (self.player.position.y)) as f32,
        );

        if inputs.is_button_just_pressed(wasm4::BUTTON_2) {
            // self.debug = !self.debug;
            let game_data: GameData = GameData {
                version: GAME_DATA_VERSION,
                x: self.player.position.x,
                y: self.player.position.y,
            };
            save(&game_data);
            let loaded: GameData = load();
            // wasm4::trace(format!("{} {}", loaded.x, loaded.y))
        }

        // renders

        let player_center = self.player.center();
        let dx = wasm4::SCREEN_SIZE as i32 / 2 - player_center.x.floor() as i32;
        let dy = wasm4::SCREEN_SIZE as i32 / 2 - player_center.y.floor() as i32
            + i32::max(0, self.player_lookup);
        let graphics = Graphics {
            frame_count: self.frame_count,
            debug: self.debug,
            dx,
            dy,
        };

        set_draw_color(0x22);
        for x in 0..(wasm4::SCREEN_SIZE / 4) {
            wasm4::hline(
                x as i32 * 4,
                (dy as f32 + (WORLD_HEIGHT as f32 * CELL_SIZE as f32 - self.score)) as i32,
                2,
            );
        }

        set_draw_color(0x3210);
        self.world.draw(graphics);

        self.player.draw(graphics, &self.world, &inputs);

        for fruit in self.fruits.iter() {
            fruit.draw(graphics, &self.world, &inputs);
        }

        set_draw_color(0x04);
        wasm4::text(
            int_to_string(self.score.floor() as u32),
            0,
            (dy as f32 + (WORLD_HEIGHT as f32 * CELL_SIZE as f32 - self.score)) as i32 + 2,
        );
        set_draw_color(0x04);
        wasm4::text(
            int_to_string(self.score.floor() as u32),
            2,
            (dy as f32 + (WORLD_HEIGHT as f32 * CELL_SIZE as f32 - self.score)) as i32 + 2,
        );
        set_draw_color(0x02);
        // snip-rust-fmt-code を指定しているので to_string や format! が動かないことに注意
        wasm4::text(
            int_to_string(self.score.floor() as u32),
            1,
            (dy as f32 + (WORLD_HEIGHT as f32 * CELL_SIZE as f32 - self.score)) as i32 + 2,
        );

        if self.debug {
            set_draw_color(0x41);
            wasm4::text(
                format!(
                    "{0: >04}, {1: >04}",
                    self.player.position.x.floor(),
                    self.player.position.y.floor()
                ),
                0,
                0,
            );
        }
    }
}

fn int_to_char(digit: u32) -> u8 {
    match digit {
        0 => b'0',
        1 => b'1',
        2 => b'2',
        3 => b'3',
        4 => b'4',
        5 => b'5',
        6 => b'6',
        7 => b'7',
        8 => b'8',
        9 => b'9',
        _ => b'0',
    }
}

fn int_to_string(v: u32) -> String {
    let buf: &[u8; 4] = &[
        int_to_char(v / 1000),
        int_to_char(v % 1000 / 100),
        int_to_char(v % 1000 % 100 / 10),
        int_to_char(v % 10),
    ];
    let s = match str::from_utf8(buf) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    s.to_string()
}