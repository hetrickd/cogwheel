// Scan PCI bus and print devices

use x86_64::instructions::port::{Port, PortGeneric, ReadWriteAccess};
use crate::println;

fn pci_config_read_word(bus: u8, slot: u8, func: u8, offset: u8) -> u16 {
    let mut address: u32 = 0x80000000;
    address |= ((bus as u32) << 16) | ((slot as u32) << 11) | ((func as u32) << 8) | ((offset as u32) & 0xfc);
    let mut port: PortGeneric<u32, ReadWriteAccess> = Port::new(0xcf8);
    unsafe {
        port.write(address);
        let mut port: PortGeneric<u16, ReadWriteAccess> = Port::new(0xcfc);
        let mut temp: u16;

        // Unchecked port.read() >> ((offset & 2) * 8)
        temp = port.read();
        temp = temp.unchecked_shr(((offset as u32 & 2) * 8) & 0xffff);

        temp
    }
}

fn pci_check_device_vendor(bus: u8, slot: u8) -> u32 {
    let vendor: u16;
    let mut device: u16 = 0xffff;

    vendor = pci_config_read_word(bus, slot, 0, 0);
    if vendor != 0xffff {
        device = pci_config_read_word(bus, slot, 0, 2);
    }

    ((device as u32) << 16) | (vendor as u32)
}

fn check_function(bus: u8, device: u8, func: u8) {
    let base_class: u8;
    let sub_class: u8;
    let secondary_bus: u8;

    base_class = get_base_class(bus, device, func);
    sub_class = get_sub_class(bus, device, func);
    if (base_class == 0x06) && (sub_class == 0x04) {
        secondary_bus = get_secondary_bus(bus, device, func);
        check_bus(secondary_bus);
    }

    println!("Found function {} at bus {} device {} - base class {} sub class {}", func, bus, device, base_class, sub_class);
}

fn get_base_class(bus: u8, device: u8, func: u8) -> u8 {
    let class: u8;

    class = pci_config_read_word(bus, device, func, 0x0b) as u8;
    class
}

fn get_sub_class(bus: u8, device: u8, func: u8) -> u8 {
    pci_config_read_word(bus, device, func, 0x0a) as u8
}

fn get_secondary_bus(bus: u8, device: u8, func: u8) -> u8 {
    let secondary_bus: u8;

    secondary_bus = pci_config_read_word(bus, device, func, 0x19) as u8;
    secondary_bus
}

fn check_device(bus: u8, device: u8) {
    let device_vendor_id = pci_check_device_vendor(bus, device);
    let vendor_id: u16 = device_vendor_id as u16;
    let device_id: u16 = (device_vendor_id >> 16) as u16;
    if vendor_id == 0xffff {
        return;
    }

    check_function(bus, device, 0);
    let header_type = pci_config_read_word(bus, device, 0, 0x0e) as u8;
    if (header_type & 0x80) != 0 {
        for func in 1..8 {
            let device_vendor_id = pci_check_device_vendor(bus, device);
            let vendor_id: u16 = device_vendor_id as u16;
            if vendor_id != 0xffff {
                check_function(bus, device, func);
            }
        }
    }

    println!("Found device {} at bus {} - vendor id {}, device id {}", device, bus, vendor_id, device_id);
}

fn check_all_buses() {
    let mut bus: u8 = 0;

    let header_type = pci_config_read_word(0, 0, 0, 0x0e) as u8;
    if header_type & 0x80 == 0 {
        check_bus(0);
    } else {
        for _i in 0..8 {
            check_bus(bus);
            bus += 1;
        }
    }
}

fn check_bus(bus: u8) {
    for device in 0..32 {
        check_device(bus, device);
    }
}

pub fn scan_pci() {
    check_all_buses();
}