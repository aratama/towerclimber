// walk2
const WALK2_WIDTH: u32 = 8;
const WALK2_HEIGHT: u32 = 16;
const WALK2_FLAGS: u32 = 1; // BLIT_2BPP
const WALK2: [u8; 32] = [ 0x00,0x00,0x00,0x00,0x01,0x45,0x05,0x15,0x05,0x14,0x04,0x10,0x05,0x50,0x15,0x54,0x17,0x74,0x16,0x64,0x15,0x54,0x3f,0xfc,0xff,0xf4,0x4f,0xf0,0x05,0x50,0x01,0x40 ];


use crate::image::Image;
pub const WALK2_IMAGE: Image = Image {
    width: WALK2_WIDTH,
    height: WALK2_HEIGHT,
    flags: WALK2_FLAGS,
    frames: &[&WALK2],
};