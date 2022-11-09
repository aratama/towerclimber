// walk3
const WALK3_WIDTH: u32 = 8;
const WALK3_HEIGHT: u32 = 16;
const WALK3_FLAGS: u32 = 1; // BLIT_2BPP
const WALK3: [u8; 32] = [ 0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x51,0x01,0x45,0x01,0x45,0x01,0x04,0x01,0x54,0x05,0x55,0x05,0xdd,0x05,0x99,0x35,0x55,0x3f,0xf0,0x1f,0xd0,0x05,0x40,0x04,0x40 ];


use crate::image::Image;
pub const WALK3_IMAGE: Image = Image {
    width: WALK3_WIDTH,
    height: WALK3_HEIGHT,
    flags: WALK3_FLAGS,
    data: &WALK3,
};
