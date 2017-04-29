use sdl2::render::Renderer;
use sdl2::render::Texture;

use constants::{WHITE,LIGHT_GRAY,DARK_GRAY,BLACK,COLORS};
use cart::ByteIO;
use context::Context;

const MAX_FRAME_SKIP: usize = 2;
static mut FRAME_SKIP:usize = 0;

pub struct Lcd {
    tiles: Vec<Vec<Vec<usize>>>,
    //sprites: Vec<Sprite>,
    /* Pallettes */
    background_palette: [usize;4],
    foreground_palette: [[usize;4];2],

    do_render_scanline:bool,
    do_flush:bool,
}

impl Lcd {

    pub fn new() -> Lcd {
        Lcd {
            tiles: vec![vec![vec![0;8];8];192],
            //sprites: vec![Sprite::new_blank();40],
            background_palette: [0,1,2,3],
            foreground_palette: [[0,1,2,3],[0,1,2,3]],

            do_render_scanline:false,
            do_flush:false,
        }
    }

    // fn update_tile(&mut self, offset:usize, val:u8) {
    // //offset is the index into the vram.
    // let low_addr = offset & !(0x01);
    // let tile = offset >> 4; //each tile is 16bytes.
    // let y = (offset >> 1) & 0x7; //8 heights, with top at offset 0.
    // for x in 0..8 {
    //     let index = 1 << (7 - x);
    //     self.tiles[tile][y][x]
    //         (if self.vram[low_addr] & index > 0 {1} else {0}) +
    //         (if self.vram[low_addr+1] & index > 0 {2} else {0});
    // }
//}

    pub fn set_flush(&mut self, cond:bool) {
        self.do_flush = cond;
    }
    pub fn set_render_scanline(&mut self, cond:bool) {
        self.do_render_scanline = cond;
    }
}

impl ByteIO for Lcd {
    fn read_byte(&self, addr:u16) -> u8 {
        0
    }
    fn write_byte(&mut self, addr:u16, val:u8) {
        match addr {
            //0x
            _ => panic!("invalid lcd write 0x{:04X}", addr),
        }
    }
}

impl Lcd {

    pub fn step(ctx:&mut Context, renderer:&mut Renderer, texture:&mut Texture, cycles: usize) {
        if ctx.lcd().do_flush {
            unsafe {
                if FRAME_SKIP == 0 {
                    Lcd::render_to_texture(ctx,renderer,texture);
                    Lcd::flush(ctx,renderer,texture);
                }
                FRAME_SKIP = (FRAME_SKIP + 1) % MAX_FRAME_SKIP;
            }
        } else if ctx.lcd().do_render_scanline {
            //Lcd::render_scanline(ctx,renderer,texture)
        }
    }

    // pub fn print_tile(ctx:&mut Context, tile:usize) {
    //     println!("Tile 0x{:X}", tile);
    //     for y in 0..8 {
    //         for x in 0..8 {
    //             match self.tiles[tile][y][x] {
    //                 0x00 => {print!("_")}
    //                 0x01 => {print!("▧")}
    //                 0x02 => {print!("▩")}
    //                 0x03 => {print!("■")}
    //                 _ => {print!(" ")},
    //             }
    //         }
    //         println!("");
    //     }
    // }

    pub fn print_tiles(ctx:&mut Context) {
        for t in 0..0x180 { //more than 255 of them.
            //self.print_tile(t);
        }
        println!("Tilemap 9800");
        for y in 0..32 {
            for x in 0..32 {
                let offset = y * 32 + x + 0x9800;
                print!("{:02x} ",ctx.gpu().read_byte(offset));
            }
            println!("");
        }
        println!("Tilemap 9C00");
        for y in 0..32 {
            for x in 0..32 {
                let offset = y * 32 + x + 0x9C00;
                print!("{:02x} ",ctx.gpu().read_byte(offset));
            }
            println!("");
        }
    }

    pub fn flush(ctx:&mut Context, renderer:&mut Renderer, texture:&mut Texture) {
        renderer.set_draw_color(WHITE);
        renderer.clear();
        renderer.copy(texture, None, None).unwrap();
        renderer.present();
    }

    pub fn render_to_texture(ctx:&mut Context, renderer:&mut Renderer, texture:&mut Texture) {
        let lcd_control = ctx.gpu().get_lcd_control() as usize;
        let bg_palette = ctx.gpu().get_bg_palette() as usize;
        let fg_palette_0 = ctx.gpu().get_fg_palette_0() as usize;
        let fg_palette_1 = ctx.gpu().get_fg_palette_1() as usize;
        let scroll_y = ctx.gpu().get_scroll_y() as usize;
        let scroll_x = ctx.gpu().get_scroll_x() as usize;
        let window_y = ctx.gpu().get_window_y() as usize;
        let window_x = ctx.gpu().get_window_x() as usize;

        if lcd_control & 0x80 == 0x00 {
            return;
        }

        let tile_pattern_data = if 0x10 & lcd_control == 0x00 {0x8800} else {0x8000};
        let background_map_addr = if 0x08 & lcd_control == 0x00 { 0x9800 } else {0x9C00};

        texture.with_lock(None, |buffer: &mut [u8], pitch:usize| {

            // BACKGROUND MAP.
            if 0x01 & lcd_control > 0x00 {
                for sdl_y in 0..143 {
                    for sdl_x in 0..160 {
                        let world_y = (sdl_y + scroll_y) % 256;
                        let world_x = (scroll_x + sdl_x) % 256;
                        let tile_idx_y = world_y >> 3;
                        let tile_idx_x = world_x >> 3;
                        let tile_pixel_y = world_y & 0x07;
                        let tile_pixel_x = 0x07 - (world_x & 0x07);

                        let tile_mem_addr = background_map_addr + (tile_idx_y << 5) + tile_idx_x;
                        let mut tile = ctx.gpu().read_byte(tile_mem_addr as u16) as usize;

                        let mut pixel_mem_addr = if tile_pattern_data == 0x8800 {
                            ((0x9000 as i32) +
                                (((tile as i8) as i32) << 4)) as usize
                        } else {
                            ((tile << 4) + tile_pattern_data)
                        };

                        pixel_mem_addr += (tile_pixel_y << 1);
                        let low = ctx.gpu().read_byte(pixel_mem_addr as u16) >> tile_pixel_x;
                        let high = ctx.gpu().read_byte(pixel_mem_addr as u16 +1) >> tile_pixel_x;
                        let col = ((0x01 & high) << 1) | (0x01 & low); //0,1,2,3,4
                        let mask = bg_palette >> (col << 1);
                        let rgb = COLORS[0x03 & mask as usize].rgb();
                        buffer[pitch * sdl_y + 3 * sdl_x + 0] = rgb.0;
                        buffer[pitch * sdl_y + 3 * sdl_x + 1] = rgb.1;
                        buffer[pitch * sdl_y + 3 * sdl_x + 2] = rgb.2;
                    }
                }
            }

            // window enable.
            if 0x20 & lcd_control > 0x00
                && window_x <= 166
                && window_y <= 143 {
                let window_tilemap_select = if (0x40 & lcd_control) == 0x00
                    { 0x9800 } else { 0x9C00 };
                for sdl_y in window_y..143 {
                    for sdl_x in window_x-7..160 {
                        let world_y = sdl_y as usize - (window_y); //(sdl_y + scroll_y) % 256;
                        let world_x = sdl_x as usize - (window_x - 7); //(scroll_x + sdl_x) % 256;
                        let tile_idx_y = world_y >> 3;
                        let tile_idx_x = world_x >> 3;
                        let tile_pixel_y = world_y & 0x07;
                        let tile_pixel_x = 0x07 - (world_x & 0x07);

                        let tile_mem_addr = window_tilemap_select + (tile_idx_y << 5) + tile_idx_x;
                        let mut tile = ctx.gpu().read_byte(tile_mem_addr as u16) as usize;

                        let mut pixel_mem_addr =
                             ((0x9000 as i32) +
                                 (((tile as i8) as i32) << 4)) as usize;
                        pixel_mem_addr += (tile_pixel_y << 1);
                        let low = ctx.gpu().read_byte(pixel_mem_addr as u16) >> tile_pixel_x;
                        let high = ctx.gpu().read_byte(pixel_mem_addr as u16 +1) >> tile_pixel_x;
                        let col = ((0x01 & high) << 1) | (0x01 & low); //0,1,2,3,4
                        let mask = bg_palette >> (col << 1);
                        let rgb = COLORS[0x03 & mask as usize].rgb();
                        buffer[pitch * sdl_y + 3 * sdl_x + 0] = rgb.0;
                        buffer[pitch * sdl_y + 3 * sdl_x + 1] = rgb.1;
                        buffer[pitch * sdl_y + 3 * sdl_x + 2] = rgb.2;
                    }
                }

            }

            // FOREGROUND SPRITES SIMPLE
            if 0x02 & lcd_control > 0x00 {
                for idx in 0..40 {
                    let sprite_addr = (0xFE00 + (idx << 2)) as u16; //4B wide.
                    let sprite_y = (ctx.gpu().read_byte(sprite_addr+0) as i32 - 16) as isize;
                    let sprite_x = (ctx.gpu().read_byte(sprite_addr+1) as i32 - 8) as isize;
                    let sprite_tile = ctx.gpu().read_byte(sprite_addr+2) as usize;
                    let sprite_options = ctx.gpu().read_byte(sprite_addr+3);
                    let sprite_palette = if (0x10 & sprite_options) > 0x00
                        {fg_palette_1} else {fg_palette_0};
                    //CHECK 8x16 MODE.

                    let tile_addr = 0x8000 + (sprite_tile << 4);

                    for sdl_y in sprite_y..sprite_y+8 {
                        for sdl_x in sprite_x..sprite_x+8 {
                            if sdl_y >= 0 && sdl_y < 144 && sdl_x >= 0 && sdl_x < 160 {
                                let offset = pitch * (sdl_y as usize) + 3 * (sdl_x as usize);
                                let mut tile_pixel_x = 0x07 & (sdl_x as usize);
                                let mut tile_pixel_y = 0x07 & (sdl_y as usize);
                                if (0x20 & sprite_options) == 0x00 {
                                    tile_pixel_x = 7 - tile_pixel_x;
                                }
                                if (0x40 & sprite_options) > 0x00 {
                                    tile_pixel_y = 7 - tile_pixel_y;
                                }

                                let pixel_mem_addr = tile_addr + (tile_pixel_y << 1);
                                let low = ctx.gpu().read_byte(pixel_mem_addr as u16) >> tile_pixel_x;
                                let high = ctx.gpu().read_byte(pixel_mem_addr as u16 +1) >> tile_pixel_x;
                                let col = ((0x01 & high) << 1) | (0x01 & low); //0,1,2,3,4
                                if (0x80 & sprite_options) == 0x00 &&   //draw over.
                                    col != 0 {
                                        // or previous values written are 1-3 in bg mode.
                                    let mask = sprite_palette >> (col << 1);
                                    let rgb = COLORS[0x03 & mask as usize].rgb();
                                    buffer[offset + 0] = rgb.0;
                                    buffer[offset + 1] = rgb.1;
                                    buffer[offset + 2] = rgb.2;
                                }
                            }
                        }
                    }
                }
            }
        }).unwrap();
    }

}
