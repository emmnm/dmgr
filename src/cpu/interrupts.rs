pub struct Interrupts {
    master_enable: bool,
    interrupt_enable: u8,
    interrupt_flag: u8,
}

impl Interrupts {
    pub fn new() -> Interrupts {
        Interrupts {
            master_enable: false,
            interrupt_enable: 0x00,
            interrupt_flag: 0x00,
        }
    }
    pub fn reset(&mut self) {
        self.master_enable = false;
        self.interrupt_enable = 0x00;
        self.interrupt_flag = 0x00;
    }

    pub fn enable(&mut self) {
        self.master_enable = true;
    }

    pub fn disable(&mut self) {
        self.master_enable = false;
    }

    pub fn is_master_enabled(&self) -> bool {
        return self.master_enable
    }

    pub fn request(&mut self,mask: u8) {
        self.interrupt_flag |= mask;
    }
    pub fn clear(&mut self, mask:u8) {
        self.interrupt_flag &= !mask;
    }

    pub fn read_filtered(&self) -> u8 {
        self.interrupt_enable & self.interrupt_flag
    }

    pub fn read_enable(&self) -> u8 {
        self.interrupt_enable
    }

    pub fn write_enable(&mut self, val:u8) {
        self.interrupt_enable = val;
    }

    pub fn write_flag(&mut self, val: u8) {
        self.interrupt_flag = val;
    }
    pub fn read_flag(&mut self) -> u8 {
        self.interrupt_flag
    }


}
