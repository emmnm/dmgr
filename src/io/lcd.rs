use sdl2::render::Renderer;
use sdl2::render::Texture;

use constants::{WHITE,LIGHT_GRAY,DARK_GRAY,BLACK,COLORS};
use cart::ByteIO;
use context::Context;

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
            tiles: vec![vec![vec![0;8];8];384],
            //sprites: vec![Sprite::new_blank();40],
            background_palette: [0,1,2,3],
            foreground_palette: [[0,1,2,3],[0,1,2,3]],

            do_render_scanline:false,
            do_flush:false,
        }
    }

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
            Lcd::render_to_texture(ctx,renderer,texture);
            Lcd::flush(ctx,renderer,texture);
        } else if ctx.lcd().do_render_scanline {
            //Lcd::render_scanline(ctx,renderer,texture)
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
        let scanline = ctx.gpu().get_scanline() as usize;
        let scroll_y = ctx.gpu().get_scroll_y() as usize;
        let scroll_x = ctx.gpu().get_scroll_x() as usize;

        if lcd_control & 0x80 == 0x00 {
            return;
        }

        let tile_pattern_data = if 0x10 & lcd_control == 0x00 {0x8800} else {0x8000};
        let background_map_addr = if 0x08 & lcd_control == 0x00 { 0x9800 } else {0x9C00};

        texture.with_lock(None, |buffer: &mut [u8], pitch:usize| {

            // turn on disabling background.
            for sdl_y in 0..143 {
                for sdl_x in 0..160 {
                    let world_y = sdl_y + scroll_y;
                    let world_x = scroll_x + sdl_x;

                    let tile_idx_y = world_y >> 3;
                    let tile_idx_x = world_x >> 3;

                    let tile_mem_addr = background_map_addr + (tile_idx_y << 5) + tile_idx_x;
                    let mut tile = ctx.gpu().read_byte(tile_mem_addr as u16) as usize;

                    // if tile_pattern_table == 0x8800 {
                    //     // 0 lies around 9000, which is our tile[0x100]
                    //     tile = (((tile as i8) as i32) + 0x100i32) as usize;
                    // }

                    let tile_pixel_y = world_y & 0x07;
                    let tile_pixel_x = 0x07 - (world_x & 0x07);

                    let pixel_mem_addr = (tile_pattern_data + (tile << 4) + (tile_pixel_y << 1));
                    let high = ctx.gpu().read_byte(pixel_mem_addr as u16) >> tile_pixel_x;
                    let low = ctx.gpu().read_byte(pixel_mem_addr as u16 +1) >> tile_pixel_x;
                    let col = ((0x01 & high) << 1) | (0x01 & low);

                    let rgb = COLORS[col as usize].rgb();
                    buffer[pitch * sdl_y + 3 * sdl_x + 0] = rgb.0;
                    buffer[pitch * sdl_y + 3 * sdl_x + 1] = rgb.1;
                    buffer[pitch * sdl_y + 3 * sdl_x + 2] = rgb.2;
                }
            }


            if 0x02 & lcd_control > 0x00 {
                for idx in 0..40 { //40 sprites in OAM.
                    let sprite_addr = (0xFE00 + (idx << 2)) as u16; //4B wide.
                    let sprite_y = (ctx.gpu().read_byte(sprite_addr) as i32 - 16) as usize;
                    let sprite_x = (ctx.gpu().read_byte(sprite_addr+1) as i32 - 8) as usize;

                    for sdl_y in sprite_y..sprite_y+8 {
                        for sdl_x in sprite_x..sprite_x+8 {
                            if sdl_y < 144 && sdl_x < 160 {
                                let offset = pitch * sdl_y + 3 * sdl_x;
                                buffer[offset + 0] = 0xCC;
                                buffer[offset + 1] = 0xCC;
                                buffer[offset + 2] = 0xCC;
                            }
                        }
                    }
                }
            }

//             if self.control & 0x02 > 0x00 {
//     ctx.texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
//         for idx in 0..40 {
//
//             if sy <= self.scanline as i32 && (sy+8) > self.scanline as i32 {
//
//                 let ref palette = if sprite.palette_flag {self.foreground_palette[1]} else {self.foreground_palette[0]};
//                 let tilerow = if sprite.flip_y {7 - (self.scanline as i32 - sy)} else {self.scanline as i32 - sy};
//                 for x in 0..8 {
//                     if sx + x >= 0 && sx+x < 160 {
//
//                         let offset = ((self.scanline as i32) * (pitch as i32) + ((sx + x) as i32 * 3)) as usize;
//                         let lookup_x = if sprite.flip_x { 7 - x} else {x } as usize;
//                         let lookup_y = tilerow as usize; //if sprite.flip_y { 7 - tilerow} else {tilerow} as usize;
//                         let col = palette[self.tiles[sprite.tile as usize][lookup_y][lookup_x] as usize];
//                         // sprite data 00 is transparent.
//                         if col != 0 {
//                             let rgb = COLORS[col as usize].rgb();
//                             buffer[offset + 0] = rgb.0;
//                             buffer[offset + 1] = rgb.1;
//                             buffer[offset + 2] = rgb.2;
//                         }
//                     }
//                 }
//             }
//         }
//     }).unwrap();
// }


        });
    }


    pub fn render_scanline(ctx:&mut Context, renderer:&mut Renderer, texture:&mut Texture) {
        let lcd_control = ctx.gpu().get_lcd_control() as usize;
        let scanline = ctx.gpu().get_scanline() as usize;
        let scroll_y = ctx.gpu().get_scroll_y() as usize;
        let scroll_x = ctx.gpu().get_scroll_x() as usize;

        if lcd_control & 0x80 == 0x00 {
            return;
        }

        // Data of tiles themselves.
        //bit 4 (aka 0x10) & control - bg tile data select where 0 = 8800-97ff, 1 = 8000-8fff
        // 8000 is used for sprites and background. 0 -255
        // 8800 can be used for background and window and from -128 to 127
        let tile_pattern_data = if 0x10 & lcd_control == 0x00 {0x8800} else {0x8000};

        // WHICH TILE DO I USE?
        //bit 3 (aka 0x08) & control - bg tile map display select where 0=9800-9Bff, 1 = 9C00-9FFF
        let background_map_addr = if 0x08 & lcd_control == 0x00 { 0x9800 } else {0x9C00};

        texture.with_lock(None, |buffer: &mut [u8], pitch:usize| {
            if 0x01 & lcd_control > 0x00 {
                for sdl_x in 0..160 {

                    let world_y = scanline + scroll_y;
                    let world_x = scroll_x + sdl_x;

                    let tile_idx_y = world_y >> 3;
                    let tile_idx_x = world_x >> 3;

                    let tile_mem_addr = background_map_addr + (tile_idx_y << 5) + tile_idx_x; //+ (tile_idx_y << 5) + tile_idx_x;
                    let mut tile = ctx.gpu().read_byte(tile_mem_addr as u16) as usize;

                    // if tile_pattern_table == 0x8800 {
                    //     // 0 lies around 9000, which is our tile[0x100]
                    //     tile = (((tile as i8) as i32) + 0x100i32) as usize;
                    // }

                    let tile_pixel_y = world_y & 0x07;
                    let tile_pixel_x = 0x07 - (world_x & 0x07);

                    let pixel_mem_addr = (tile_pattern_data + (tile << 4) + (tile_pixel_y << 1));
                    // println!("tpd{:08X}",tile_pattern_data);
                    // println!("tile {:08X}",tile << 4);
                    // println!("y {:08X}",tile_pixel_y << 4);
                    // println!("{:08X}",pixel_mem_addr);
                    let high = ctx.gpu().read_byte(pixel_mem_addr as u16) >> tile_pixel_x;
                    let low = ctx.gpu().read_byte(pixel_mem_addr as u16 +1) >> tile_pixel_x;
                    let col = ((0x01 & high) << 1) | (0x01 & low);

                    let rgb = COLORS[col as usize].rgb();
                    buffer[pitch * scanline + 3 * sdl_x + 0] = rgb.0;
                    buffer[pitch * scanline + 3 * sdl_x + 1] = rgb.1;
                    buffer[pitch * scanline + 3 * sdl_x + 2] = rgb.2;
                }
            }
        });

                //             let col = self.background_palette[self.tiles[tile][y][x]];
                //             x += 1;
                //             if x >= 8 {
                //                 x = 0;
                //                 line_offset = (line_offset + 1) & 31; //only 32 tiles.
                //                 tile = self.vram[(map_offset + row_offset + line_offset) as usize] as usize;
                //                 if tile_pattern_table == 0x0800 { //
                //                     //0 lies around 9000 - which is our tile[0x100]
                //                     tile = (((tile as i8) as i32) + 0x100i32) as usize;
                //                 }
                //             }
                //         }
                //     }).unwrap();
                // }
    }

}
