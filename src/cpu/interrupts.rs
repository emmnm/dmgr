
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

    pub fn is_active(mask:u8) {

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


}
