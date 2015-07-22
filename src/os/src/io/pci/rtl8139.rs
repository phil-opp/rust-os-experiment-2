use io::arch::Port;

pub fn init(ioaddr: u16) {
    let mut nic = Rtl8139 {
        config_1: Port::new(ioaddr + 0x52),
        command: Port::new(ioaddr + 0x37),
        rbstart: Port::new(ioaddr + 0x30),
        interrupt_mask: Port::new(ioaddr + 0x3C),
        interrupt_service: Port::new(ioaddr + 0x3E),
        rcr: Port::new(ioaddr + 0x44),
    };

    unsafe {
        // turn it on
        nic.config_1.out8(0x00);
        // software reset
        nic.command.out8(0x10);
        while nic.command.in8() & 0x10 != 0 {}
        // init receive buffer (physical address)
        nic.rbstart.out32(0x0); // TODO size 8192 + 16 + 1500
        // accept TOK (transmit ok) and ROK (receive ok)
        nic.interrupt_mask.out16(0x0005);
        // accept broadcast, multicast, physical match, and "all" packets,
        // and don't wrap the buffer
        nic.rcr.out32(0xf | (1 << 7));
        // enable receive and transmitter
        nic.command.out8(/*0x0C*/ 0x04); // TODO enable receiver after setting receive buffer
    }
}

pub struct Rtl8139 {
    config_1: Port,
    command: Port,
    rbstart: Port,
    interrupt_mask: Port,
    interrupt_service: Port,
    rcr: Port,
}
