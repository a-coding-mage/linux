// SPDX-License-Identifier: GPL-2.0
//
// Dependencies supplied by the Linux kernel headers are referenced here as
// declarations; their definitions are provided by the surrounding build.

use core::ffi::{c_char, c_int};

#[repr(C)]
pub struct pci_bus {
    pub number: u8,
}

#[repr(C)]
pub struct pci_dev {
    pub bus: *const pci_bus,
}

unsafe extern "C" {
    fn evt2irq(event: u16) -> c_int;
    fn printk(fmt: *const c_char, ...) -> c_int;
}

pub unsafe fn pcibios_map_platform_irq(
    dev: *const pci_dev,
    slot: u8,
    pin: u8,
) -> c_int {
    let irq: c_int;

    if (*(*dev).bus).number == 0 {
        match slot {
            4 => return evt2irq(0x2a0), // eth0
            8 => return evt2irq(0x2a0), // eth1
            6 => return evt2irq(0x240), // PCI bridge
            _ => {
                let format = b"<3>PCI: Bad IRQ mapping request for slot %d\n\0";
                printk(format.as_ptr() as *const c_char, slot as c_int);
                return evt2irq(0x240);
            }
        }
    } else {
        irq = match pin {
            0 => evt2irq(0x240),
            1 => evt2irq(0x240),
            2 => evt2irq(0x240),
            3 => evt2irq(0x240),
            4 => evt2irq(0x240),
            _ => -1,
        };
    }
    irq
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
