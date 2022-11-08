// walk0
const WALK0_WIDTH: u32 = 8;
const WALK0_HEIGHT: u32 = 16;
const WALK0_FLAGS: u32 = 1; // BLIT_2BPP
const WALK0: [u8; 32] = [ 0x00,0x00,0x00,0x00,0x01,0x45,0x05,0x15,0x05,0x14,0x04,0x10,0x05,0x50,0x15,0x54,0x17,0x74,0x16,0x64,0x15,0x54,0x3f,0xfc,0x37,0xfc,0x0f,0xf4,0x05,0x50,0x10,0x14 ];


use crate::image::Image;
pub const WALK0_IMAGE: Image = Image {
    width: WALK0_WIDTH,
    height: WALK0_HEIGHT,
    flags: WALK0_FLAGS,
    data: &WALK0,
};