use enable_interrupts;
use io::stdio;
use std::fmt::Write;
use global;

#[no_mangle]
pub extern "C" fn interrupt_handler(interrupt_number: isize, error_code: isize) {
    match interrupt_number {
        32 => global::spawn(move || print!(".")), // timer
        _ => panic!("interrupt {} with error code 0x{:x}", interrupt_number, error_code),
    }
    unsafe{
        send_eoi(interrupt_number);
        //enable_interrupts();
    };
}

#[no_mangle]
pub extern fn pagefault_handler(address: usize, error_code: isize) {
    panic!("pagefault at {} with error code {}", address, error_code)
}

#[no_mangle]
pub extern fn keyboard_handler(interrupt_number: isize, key_code: usize) {
    unsafe{
        send_eoi(interrupt_number);
        enable_interrupts()
    }
    assert!(interrupt_number == 33);
    global::spawn(move || print!(" [k {}] ", key_code));
}

unsafe fn send_eoi(interrupt_number: isize) {
    use out_byte;
    unsafe fn send_master_eoi() {out_byte(0x20, 0x20)}
    unsafe fn send_slave_eoi() {out_byte(0xA0, 0x20)}

    match interrupt_number {
        i if i >= 40 => {
            send_slave_eoi();
            send_master_eoi();
        },
        32...40 => send_master_eoi(),
        _ => {},
    }
}
