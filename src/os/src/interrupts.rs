use enable_interrupts;
use io::keyboard::ScanCode;
use global;

#[no_mangle]
pub extern "C" fn interrupt_handler(interrupt_number: isize, error_code: isize) {
    match interrupt_number {
        32 => {}, // timer
        _ => panic!("interrupt {} with error code 0x{:x}", interrupt_number, error_code),
    }
    unsafe{
        send_eoi(interrupt_number);
        //enable_interrupts();
    };
}

#[no_mangle]
pub extern fn pagefault_handler(address: usize, error_code: isize) {
    panic!("pagefault at 0x{:x} with error code {}", address, error_code)
}

#[no_mangle]
pub extern fn general_protection_fault_handler(address: usize, error_code: isize) {
    panic!("general protection fault at 0x{:x} with error code {}", address, error_code)
}

#[no_mangle]
pub extern fn keyboard_handler(interrupt_number: isize, key_code: usize) {
    global::data().key_presses.send(ScanCode::new(key_code));
    unsafe{
        send_eoi(interrupt_number);
        enable_interrupts()
    }
    assert!(interrupt_number == 33);
}

unsafe fn send_eoi(interrupt_number: isize) {
    use io::arch::Port;
    let master_port = Port::new(0x20);
    let slave_port = Port::new(0xA0);

    unsafe fn send_eoi(mut port: Port) {port.out8(0x20)}

    match interrupt_number {
        i if i >= 40 => {
            send_eoi(slave_port);
            send_eoi(master_port);
        },
        32...40 => send_eoi(master_port),
        _ => {},
    }
}
