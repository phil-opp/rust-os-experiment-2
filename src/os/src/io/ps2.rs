
pub unsafe fn init() {
    use {in_byte, out_byte};

    const COMMAND_PORT: u16 = 0x64;
    const STATUS_PORT: u16 = 0x64;
    const DATA_PORT: u16 = 0x60;

    fn wait_for_free_input_buffer() {
        loop {
            let status = unsafe{in_byte(STATUS_PORT)};
            if status & 0b10 == 0 {
                break; // input buffer is free
            }
        }
    }

    fn wait_for_filled_output_buffer() {
        loop {
            let status = unsafe{in_byte(STATUS_PORT)};
            if status & 0b01 == 1 {
                break; // output buffer is filled
            }
        }
    }

    unsafe fn send_command(command: u8) {
        wait_for_free_input_buffer();
        out_byte(COMMAND_PORT, command)
    }

    unsafe fn send_data(data: u8) {
        wait_for_free_input_buffer();
        out_byte(DATA_PORT, data)
    }

    unsafe fn read_data() -> u8 {
        wait_for_filled_output_buffer();
        in_byte(DATA_PORT)
    }

    // disable devices
    send_command(0xAD);
    send_command(0xA7);

    // flush output buffer
    in_byte(DATA_PORT);

    // get the configuration byte
    send_command(0x20);
    let mut configuration = read_data();
    //let dual_channel = (configuration & 0b100000) != 0;
    let dual_channel = false; // TODO

    // disable IRQs and translation
    configuration &= 0b10111100;
    send_command(0x60);
    send_data(configuration);

    // perform self tests
    send_command(0xAA);
    if read_data() != 0x55 {
        panic!("keyboard self test failed");
    }

    // perform interface tests
    send_command(0xAB);
    assert!(read_data() == 0);
    if dual_channel {
        send_command(0xA9);
        assert!(read_data() == 0);
    }

    // send reset
    send_data(0xFF);
    assert!(read_data() == 0xFA);
    assert!(read_data() == 0xAA);

    // enable interrupts
    configuration |= 0b01;
    if dual_channel {
        configuration |= 0b10;
    }
    send_command(0x60);
    send_data(configuration);

    // enable devices
    send_command(0xAE);
    if dual_channel {
        send_command(0xA8);
    }
}
