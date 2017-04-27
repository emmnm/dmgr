use cart::ByteIO;
use context::Context;

pub enum GpuMode {
    HBLANK, //0
    VBLANK, //1
    OAM,    //2
    VRAM,   //3
}

pub struct Gpu {
    vram: Vec<u8>,
    oam: Vec<u8>,
    mode: GpuMode,
    ticks: usize,
    scanline: u8,
    compare: u8,
    lcd_status: u8,
    lcd_control: u8,
    scroll_y: u8,
    scroll_x: u8,
    window_y: u8,
    window_x: u8,
    bg_palette: u8,
    fg_palette_0: u8,
    fg_palette_1: u8,
}

impl Gpu {
    pub fn new() -> Gpu {
        Gpu {
            vram: vec![0;0x2000],
            oam: vec![0;0x100],
            mode: GpuMode::HBLANK,
            ticks: 0,
            scanline: 0,
            compare: 0,
            lcd_status: 0,
            lcd_control: 0,
            scroll_y: 0,
            scroll_x: 0,
            window_y: 0,
            window_x: 0,
            bg_palette: 0x00,
            fg_palette_0: 0x00,
            fg_palette_1: 0x00,
        }
    }

    pub fn get_lcd_control(&self) -> u8 { self.lcd_control }
    pub fn get_scroll_y(&self) -> u8 { self.scroll_y }
    pub fn get_scroll_x(&self) -> u8 { self.scroll_x }
    pub fn get_scanline(&self) -> u8 { self.scanline }

    pub fn write_dma(&mut self, data:&[u8]) {
        for idx in 0x00..0xA0 {
            self.oam[idx] = data[idx];
        }
    }

}

impl ByteIO for Gpu {
    fn read_byte(&self, addr:u16) -> u8 {
        let idx = addr as usize;
        match addr {
            0x8000...0x9FFF => self.vram[idx-0x8000],
            0xFE00...0xFE9F => self.oam[idx-0xFE00],
            0xFF40 => self.lcd_control,
            0xFF41 => {
                0x78 & self.lcd_status |
                (if self.compare == self.scanline {0x04} else {0x00}) |
                match self.mode {
                    GpuMode::HBLANK => {0x00},
                    GpuMode::VBLANK => {0x01},
                    GpuMode::OAM => {0x02},
                    GpuMode::VRAM => {0x03},
                }
            },
            0xFF42 => self.scroll_y,
            0xFF43 => self.scroll_x,
            0xFF44 => self.scanline,
            _ => panic!("invalid gpu read 0x{:04X}",addr),
        }
    }
    fn write_byte(&mut self,addr:u16, val:u8) {
        let idx = addr as usize;
        match addr {
            0x8000...0x9FFF => self.vram[idx-0x8000] = val,
            0xFE00...0xFE9F => self.oam[idx-0xFE00] = val,
            0xFF40 => self.lcd_control = val,
            0xFF41 => self.lcd_status |= 0x78 & val,
            0xFF42 => self.scroll_y = val,
            0xFF43 => self.scroll_x = val,
            //0xFF46 => self.bg_palette = val,
            0xFF47 => self.bg_palette = val,
            0xFF48 => self.fg_palette_0 = val,
            0xFF49 => self.fg_palette_1 = val,
            0xFF4A => self.window_y = val,
            0xFF4B => self.window_x = val,
            _ => panic!("invalid gpu write 0x{:04X}",addr),
        }
    }
}

impl Gpu {

    pub fn step(ctx:&mut Context, cycles: usize) {
        let (mut do_flush, mut do_render_scanline) = (false,false);
        let (mut req_vblank, mut req_lcdstat) = (false,false);
        {
            let gpu = ctx.gpu();
            gpu.ticks += cycles;
            match gpu.mode {
                GpuMode::HBLANK => {
                    if gpu.ticks >= 204 {
                        gpu.scanline += 1;
                        gpu.ticks = 0;
                        if gpu.scanline < 144 {
                            gpu.mode = GpuMode::OAM;
                        } else {
                            gpu.mode = GpuMode::VBLANK;
                            req_vblank = true;
                        }
                    }
                    //                  if self.lcdstat & 0x20 != 0x00 {
                    //                      do_lcdstat_interrupt = true;
                    //                  }
                    //              } else {
                    //                  self.mode` = VBLANK;
                    //                  if self.lcdstat & 0x10 != 0x00 {
                    //                      do_lcdstat_interrupt = true;
                    //                  }
                    //                  do_vblank_interrupt = true;
                    //              }
                    //          }
                    //     },
                },
                GpuMode::VBLANK => {
                    if gpu.ticks >= 456 {
                        gpu.scanline += 1;
                        gpu.ticks = 0;
                        if gpu.scanline > 153 {
                            gpu.scanline = 0;
                            gpu.mode = GpuMode::OAM;
                            do_flush = true;
                        }
                    }
                    //                 if self.lcdstat & 0x20 != 0x00 {
                    //                     do_lcdstat_interrupt = true;
                    //                 }
                },
                GpuMode::OAM => {
                    if gpu.ticks >= 80 {
                        gpu.mode = GpuMode::VRAM;
                        gpu.ticks = 0;
                    }
                },
                GpuMode::VRAM => {
                    if gpu.ticks >= 172 {
                        gpu.mode = GpuMode::HBLANK;
                        gpu.ticks = 0;
                        do_render_scanline = true;
                    }
                    //             if(self.lcdstat & 0x08 != 0x00) { // switch into hblank.
                    //                 do_lcdstat_interrupt = true;
                    //             }
                    //             self.render_scanline(ctx);
                    //         }
                    //     },
                },
            }
        }
        ctx.lcd().set_flush(do_flush);
        ctx.lcd().set_render_scanline(do_render_scanline);
        if req_vblank {
            ctx.ints().request(0x01);
        }
        if req_lcdstat {
            //ctx.ints().request(0x02);
        }
                    // if (0x80 & self.control) == 0x00 {
                    //      self.scanline = 0;
                    //      self.ticks = 0;
                    //      self.mode = VBLANK;
                    //      return (do_vblank_interrupt,do_lcdstat_interrupt);
                    // }
                    //
                    // // the lcd interrupts are
                    // // coincidence interrupt, into oam, into vblank, into hblank.
                    // match self.mode {
                    //     GpuMode::HBLANK => { //mode 0, the gpu is at rest.
                    //          if self.ticks >= 204 {
                    //              self.scanline += 1;
                    //              self.ticks = 0;
                    //              if self.scanline < 144 {
                    //                  self.mode = OAM;
                    //                  if self.lcdstat & 0x20 != 0x00 {
                    //                      do_lcdstat_interrupt = true;
                    //                  }
                    //              } else {
                    //                  self.mode = VBLANK;
                    //                  if self.lcdstat & 0x10 != 0x00 {
                    //                      do_lcdstat_interrupt = true;
                    //                  }
                    //                  do_vblank_interrupt = true;
                    //              }
                    //          }
                    //     },

    }


}
