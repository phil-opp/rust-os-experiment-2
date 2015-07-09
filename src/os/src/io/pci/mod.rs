use super::arch::Port;
use std::mem;

const CONFIG_ADDRESS: Port = Port::new(0xCF8);
const CONFIG_DATA: Port = Port::new(0xCFC);

#[derive(Debug)]
struct Header {
    common: CommonHeader,
    specific: HeaderType,
}

#[derive(Debug)]
#[repr(packed)]
struct CommonHeader {
    vendor_id: u16,
    device_id: u16,
    command: u16,
    status: u16,
    revision_id: u8,
    prog_if: u8,
    subclass: u8,
    class_code: u8,
    cache_line_size: u8,
    latency_timer: u8,
    header_type: u8,
    bist: u8,
}

#[derive(Debug)]
enum HeaderType {
    Standard(Header0),
    Pci2PciBridge(Header1),
    Other,
}

#[derive(Debug)]
#[repr(packed)]
struct Header0 {
    base_addresses: [u32; 6],
    cardbus_cis_pointer: u32,
    subsystem_vendor_id: u16,
    subsystem_id: u16,
    expansion_rom_base_address: u32,
    capabilities_pointer: u8,
    _reserved: [u8; 7],
    interrupt_line: u8,
    interrupt_pin: u8,
    min_grant: u8,
    max_latency: u8,
}

#[derive(Debug)]
#[repr(packed)]
struct Header1 {
    base_addresses: [u32; 2],
    primary_bus_number: u8,
    secondary_bus_number: u8,
    subordinate_bus_number: u8,
    secondary_latency_timer: u8,
    io_base: u8,
    io_limit: u8,
    secondary_status: u16,
    memory_base: u16,
    memory_limit: u16,
    prefetchable_memory_base: u16,
    prefetchable_memory_limit: u16,
    prefetchable_base_upper: u32,
    prefetchable_limit_upper: u32,
    io_base_upper: u16,
    io_limit_upper: u16,
    capability_pointer: u8,
    _reserved: [u8; 3],
    expansion_rom_base_address: u32,
    interrupt_line: u8,
    interrupt_pin: u8,
    bridge_control: u16,
}

#[derive(Debug)]
struct Device {
    bus: u8,
    slot: u8,
    function: u8,
    header: Header,
}

fn config_read(bus: u8, slot: u8, func: u8, offset: u8) -> u32 {
    let bus = bus as u32;
    let slot = slot as u32;
    let func = func as u32;
    let offset = offset as u32;

    let address = bus << 16 | slot << 11 | func << 8 | (offset & 0xfc) | 0x80000000;

    unsafe {
        CONFIG_ADDRESS.out32(address);
        CONFIG_DATA.in32()
    }
}

fn exists(bus:u8, slot: u8, func: u8) -> bool {
    let first_line = config_read(bus, slot, func, 0);
    let vendor_id = first_line as u16;
    vendor_id != 0xFFFF
}

fn get_header(bus: u8, slot: u8, func: u8) -> Option<Header> {
    if !exists(bus, slot, func) {
        return None;
    }

    let mut common_buffer = [0u32; 4];
    for (i, line) in (0..).zip(common_buffer.as_mut()) {
        *line = config_read(bus, slot, func, i * 4);
    }
    let common: CommonHeader = unsafe{mem::transmute(common_buffer)};

    let mut specific_buffer = [0u32; 12];
    for (i, line) in (4..).zip(specific_buffer.as_mut()) {
        *line = config_read(bus, slot, func, i * 4);
    }
    let specific = match common.header_type & 0x7F {
        0 => HeaderType::Standard(unsafe{mem::transmute(specific_buffer)}),
        1 => HeaderType::Pci2PciBridge(unsafe{mem::transmute(specific_buffer)}),
        typ => {println!("unknown type {}", typ); HeaderType::Other},
    };

    Some(Header{common: common, specific: specific})
}

fn add_devices_on_bus(devices: &mut Vec<Device>, bus: u8) {
    for (slot, header) in (0..256).filter_map(|slot|
        get_header(bus, slot as u8, 0).map(|h| (slot as u8, h)))
    {
        // check if multi function device
        if header.common.header_type & 0x80 != 0 {
            for device in (1..8).filter_map(|func| get_header(bus, slot, func).map(
                |h| Device{header:h, bus: bus, slot: slot, function: func}))
            {
                devices.push(device)
            }
        }

        // check if it's a pci to pci bridge
        if header.common.class_code == 0x06 && header.common.subclass  == 0x04 {

            let secondary_bus = match header.specific {
                HeaderType::Pci2PciBridge(ref specific) => specific.secondary_bus_number,
                _ => continue, //unreachable!(),
            };
            add_devices_on_bus(devices, secondary_bus);
        }

        devices.push(Device{header: header, bus: bus, slot: slot, function: 0});
    }
}

fn get_devices() -> Vec<Device> {
    let mut devices = Vec::new();
    let root_header = get_header(0, 0, 0).unwrap();
    // if it's a multi function device, we have multiple host controllers
    match root_header.common.header_type {
        typ if (typ & 0x80 == 0) => add_devices_on_bus(&mut devices, 0),
        _ => for func in (0..8).take_while(|func| exists(0, 0, *func)) {
            add_devices_on_bus(&mut devices, func)
        },
    }
    devices
}

pub fn print_devices() {
    for device in get_devices() {
        let h = device.header.common;
        if h.class_code == 2 {
            println!("{}:{}:{} class: {}-{} vendor: {:x} device_id: {:x}", device.bus, device.slot, device.function, h.class_code, h.subclass, h.vendor_id, h.device_id)
        }
    }
}
