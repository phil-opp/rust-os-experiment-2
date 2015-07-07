pub struct Port(u16);

impl Port {
    pub const fn new(port_address: u16) -> Port {
        Port(port_address)
    }

    pub unsafe fn out_byte(&mut self, data: u8) {
        asm!("outb %al, %dx" :: "{dx}"(self.0), "{al}"(data) :: "volatile");
    }

    pub unsafe fn out_word(&mut self, data: u16) {
        asm!("outw %ax, %dx" :: "{dx}"(self.0), "{ax}"(data) :: "volatile");
    }

    pub unsafe fn out_double(&mut self, data: u32) {
        asm!("outl %eax, %dx" :: "{dx}"(self.0), "{eax}"(data) :: "volatile");
    }

    pub unsafe fn in_byte(&self) -> u8 {
        let ret: u8;
        asm!("inb %dx, %al" : "={al}"(ret) : "{dx}"(self.0) :: "volatile");
        ret
    }

    pub unsafe fn in_word(&self) -> u16 {
        let ret: u16;
        asm!("inw %dx, %ax" : "={ax}"(ret) : "{dx}"(self.0) :: "volatile");
        ret
    }

    pub unsafe fn in_double(&self) -> u32 {
        let ret: u32;
        asm!("inl %dx, %eax" : "={eax}"(ret) : "{dx}"(self.0) :: "volatile");
        ret
    }
}
