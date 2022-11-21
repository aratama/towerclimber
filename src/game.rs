use crate::input::Inputs;
use crate::music::level::LEVEL_BGM_SCORE;
use crate::scene::ending_scene::EndingScene;
use crate::scene::game_scene::GameScene;
use crate::scene::title_scene::TitleScene;
use crate::scene::Scene;
use crate::sound::{set_bgm, update_bgm};
use crate::wasm4;

pub struct Game {
    prev_gamepad: u8,
    scene: Scene,
    title_scene: TitleScene,
    game_scene: GameScene,
    ending_scene: EndingScene,
}

impl Game {
    pub fn new() -> Self {
        set_bgm(Some(&LEVEL_BGM_SCORE));

        Game {
            prev_gamepad: 0,
            scene: Scene::TitleScene(TitleScene::new()),
            title_scene: TitleScene::new(),
            game_scene: GameScene::new(),
            ending_scene: EndingScene::new(),
        }
    }

    pub fn update(&mut self) {
        let gamepad = unsafe { *wasm4::GAMEPAD1 };
        let inputs = Inputs::new(gamepad, self.prev_gamepad);
        let result = match { &mut self.scene } {
            Scene::TitleScene(t) => t.update(&inputs),
            Scene::GameScene(g) => g.update(&inputs),
            Scene::EndingScene(e) => e.update(&inputs),
        };
        match result {
            Option::None => {}
            Option::Some(next) => self.scene = next,
        }
        self.prev_gamepad = unsafe { *wasm4::GAMEPAD1 };

        update_bgm();
    }
}
