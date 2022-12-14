// board_up
use crate::wasm4::*;
use crate::image::Image;

const BOARD_UP_WIDTH: u32 = 16;
const BOARD_UP_HEIGHT: u32 = 16;
const BOARD_UP_FLAGS: u32 = BLIT_2BPP;
const BOARD_UP: [u8; 64] = [ 0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x02,0x80,0x00,0xaa,0xaa,0xaa,0xaa,0xaa,0xaa,0xaa,0xaa,0xaa,0xab,0xea,0xaa,0xaa,0xbf,0xfe,0xaa,0xab,0xff,0xff,0xea,0xbf,0xff,0xff,0xfe,0xaa,0xaf,0xfa,0xaa,0xaa,0xaf,0xfa,0xaa,0xaa,0xaf,0xfa,0xaa,0xaa,0xaa,0xaa,0xaa,0xaa,0xaa,0xaa,0xaa,0x00,0x03,0xc0,0x00,0x00,0x02,0x80,0x00 ];

pub const BOARD_UP_IMAGE: Image = Image {
    width: BOARD_UP_WIDTH,
    height: BOARD_UP_HEIGHT,
    flags: BOARD_UP_FLAGS,
    data: &BOARD_UP,
};

