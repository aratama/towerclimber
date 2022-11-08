// use buddy_alloc::{BuddyAllocParam, FastAllocParam, NonThreadsafeAlloc};

// const FAST_HEAP_SIZE: usize = 1024 * 1024; // 32 KB
// const HEAP_SIZE: usize = 5333824; // 1M
// const LEAF_SIZE: usize = 1024;

// pub static mut FAST_HEAP: [u8; FAST_HEAP_SIZE] = [0u8; FAST_HEAP_SIZE];
// pub static mut HEAP: [u8; HEAP_SIZE] = [0u8; HEAP_SIZE];

// // This allocator can't work in tests since it's non-threadsafe.
// #[cfg_attr(not(test), global_allocator)]
// static ALLOC: NonThreadsafeAlloc = unsafe {
//     let fast_param = FastAllocParam::new(FAST_HEAP.as_ptr(), FAST_HEAP_SIZE);
//     let buddy_param = BuddyAllocParam::new(HEAP.as_ptr(), HEAP_SIZE, LEAF_SIZE);
//     NonThreadsafeAlloc::new(fast_param, buddy_param)
// };

mod aabb;
// #[cfg(feature = "buddy-alloc")]
mod alloc;
mod animation;
mod body;
mod direction;
mod game;
mod graphics;
mod image;
mod input;
mod palette;
mod point;
mod save;
mod sound;
mod vector2;
mod wasm4;
mod world;
mod world_map;
use game::Game;
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref GAME: Mutex<Game> = Mutex::new(Game::new());
}

#[no_mangle]
fn start() {
    // palette::set_palette([0xfff6d3, 0xf9a875, 0xeb6b6f, 0x7c3f58]);
}

#[no_mangle]
fn update() {
    GAME.lock().expect("game_state").update();
}