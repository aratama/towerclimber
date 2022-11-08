// climb
const CLIMB_WIDTH: u32 = 16;
const CLIMB_HEIGHT: u32 = 16;
const CLIMB_FLAGS: u32 = 1; // BLIT_2BPP
const CLIMB: [u8; 64] = [ 0x00,0x05,0x05,0x40,0x00,0x05,0x14,0x00,0x00,0x04,0x10,0x00,0x00,0x05,0x50,0x00,0x00,0x15,0x54,0x00,0x00,0x17,0x74,0x00,0x00,0x16,0x64,0x00,0x00,0x15,0x57,0x40,0x00,0xff,0xd0,0x00,0x00,0xff,0x00,0x00,0x00,0xff,0x00,0x00,0x00,0x55,0x00,0x00,0x00,0x11,0x00,0x00,0x00,0x01,0x00,0x00,0x00,0x01,0x00,0x00,0x00,0x00,0x00,0x00 ];


use crate::image::Image;
pub const CLIMB_IMAGE: Image = Image {
    width: CLIMB_WIDTH,
    height: CLIMB_HEIGHT,
    flags: CLIMB_FLAGS,
    data: &CLIMB,
};
