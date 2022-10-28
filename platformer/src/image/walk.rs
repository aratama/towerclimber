// walk0
const WALK0_WIDTH: u32 = 8;
const WALK0_HEIGHT: u32 = 16;
const WALK0_FLAGS: u32 = 1; // BLIT_2BPP
const WALK0: [u8; 32] = [
    0x00, 0x00, 0x00, 0x00, 0x01, 0x45, 0x05, 0x15, 0x05, 0x14, 0x04, 0x10, 0x05, 0x50, 0x15, 0x54,
    0x17, 0x74, 0x16, 0x64, 0x15, 0x54, 0x3f, 0xfc, 0x37, 0xfc, 0x0f, 0xf4, 0x05, 0x50, 0x10, 0x14,
];

// walk1
const WALK1_WIDTH: u32 = 8;
const WALK1_HEIGHT: u32 = 16;
const WALK1_FLAGS: u32 = 1; // BLIT_2BPP
const WALK1: [u8; 32] = [
    0x00, 0x00, 0x00, 0x00, 0x01, 0x45, 0x05, 0x15, 0x05, 0x14, 0x04, 0x10, 0x05, 0x50, 0x15, 0x54,
    0x17, 0x74, 0x16, 0x64, 0x15, 0x54, 0x3f, 0xfc, 0x3f, 0xfc, 0x1f, 0xf4, 0x05, 0x50, 0x04, 0x10,
];

// walk2
const WALK2_WIDTH: u32 = 8;
const WALK2_HEIGHT: u32 = 16;
const WALK2_FLAGS: u32 = 1; // BLIT_2BPP
const WALK2: [u8; 32] = [
    0x00, 0x00, 0x00, 0x00, 0x01, 0x45, 0x05, 0x15, 0x05, 0x14, 0x04, 0x10, 0x05, 0x50, 0x15, 0x54,
    0x17, 0x74, 0x16, 0x64, 0x15, 0x54, 0x3f, 0xfc, 0xff, 0xf4, 0x4f, 0xf0, 0x05, 0x50, 0x01, 0x40,
];

// walk3
const WALK3_WIDTH: u32 = 8;
const WALK3_HEIGHT: u32 = 16;
const WALK3_FLAGS: u32 = 1; // BLIT_2BPP
const WALK3: [u8; 32] = [
    0x00, 0x00, 0x00, 0x00, 0x01, 0x45, 0x05, 0x15, 0x05, 0x14, 0x04, 0x10, 0x05, 0x50, 0x15, 0x54,
    0x17, 0x74, 0x16, 0x64, 0x15, 0x54, 0x3f, 0xfc, 0x3f, 0xfc, 0x1f, 0xf4, 0x05, 0x50, 0x04, 0x10,
];

use crate::image::{Animation, Image};
pub const WALK_ANIMATION_WIDTH: u32 = 8;
pub const WALK_ANIMATION_HEIGHT: u32 = 16;
pub const WALK_ANIMATION: Animation = Animation {
    frames: &[
        Image {
            width: WALK0_WIDTH,
            height: WALK0_HEIGHT,
            data: &WALK0,
        },
        Image {
            width: WALK1_WIDTH,
            height: WALK1_HEIGHT,
            data: &WALK1,
        },
        Image {
            width: WALK2_WIDTH,
            height: WALK2_HEIGHT,
            data: &WALK2,
        },
        Image {
            width: WALK3_WIDTH,
            height: WALK3_HEIGHT,
            data: &WALK3,
        },
    ],
};