use cart::ByteIO;

pub struct Mbc1 {
    bytes: Vec<u8>,
    ram: Vec<u8>,
    rom_bank: usize,
    ram_bank: usize,
    ram_enable: bool,
    rom_select: bool,
}

impl Mbc1 {
    pub fn new(bytes: Vec<u8>) -> Mbc1 {
        Mbc1 {
            bytes:bytes,
            ram: vec![0; 0x8000],
            rom_bank: 0,
            ram_bank: 0,
            ram_enable: false,
            rom_select: true,
        }
    }

    fn fix_rom_bank_index(&mut self) {
        if self.rom_bank == 0x00 ||
            self.rom_bank == 0x20 ||
            self.rom_bank == 0x40 ||
            self.rom_bank == 0x60 {
            self.rom_bank += 1;
        }
    }
}

impl ByteIO for Mbc1 {

    fn read_byte(&self, addr:u16) -> u8 {
        let idx = addr as usize;
        match addr {
            0x0000...0x3FFF => {self.bytes[idx]},
            0x4000...0x7FFF => {self.bytes[(idx - 0x4000) + 0x4000 * self.rom_bank] }, // here!
            0xA000...0xBFFF => { self.ram[(idx-0xA000) + 0x2000 * self.ram_bank] },
            _ => {panic!("Invalid mbc1 read location! 0x{:04X}",addr)},
        }
    }

    fn write_byte(&mut self, addr:u16, val:u8) {
        let idx = addr as usize;
        match addr {
            0x0000...0x1FFF => {
                self.ram_enable = (0x0A & val) > 0x00;
            },
            0x2000...0x3FFF => {
                let low = 0x1F & val as usize;
                let high = (!0x1F) & self.rom_bank;
                self.rom_bank = high | low;
                self.fix_rom_bank_index();
            },
            0xA000...0xBFFF => {
                self.ram[(idx - 0xA000) + 0x2000 * self.ram_bank] = val;
            },
            0x4000...0x5FFF => {
                if self.rom_select {
                    let high = ((val & 0x03) << 5) as usize;
                    self.rom_bank = ((0x1F) & self.rom_bank) | high;
                    self.fix_rom_bank_index();
                } else {
                    panic!("RAM Selection not supported.");
                }
            },
            0x6000...0x7FFF => {
                self.rom_select = val == 0x00;
            },
            _ => {panic!("Invalid mbc1 write location! 0x{:04X}",addr)},
        }
    }

}
