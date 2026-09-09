// SPDX-License-Identifier: GPL-2.0
/*
 * PCI support for the Sega Dreamcast
 *
 * Copyright (C) 2001, 2002  M. R. Brown
 * Copyright (C) 2002, 2003  Paul Mundt
 *
 * This file originally bore the message (with enclosed-$):
 *	Id: pci.c,v 1.3 2003/05/04 19:29:46 lethal Exp
 *	Dreamcast PCI: Supports SEGA Broadband Adaptor only.
 */

use core::ffi::{c_char, c_int, c_void};

// Supplied by the corresponding Linux architecture headers.
const ENODEV: c_int = 19;
const EINVAL: c_int = 22;
const IORESOURCE_IO: u64 = 0x0000_0100;
const IORESOURCE_MEM: u64 = 0x0000_0200;

extern "C" {
    static gapspci_pci_ops: c_void;
    fn inb(port: usize) -> u8;
    fn inw(port: usize) -> u16;
    fn inl(port: usize) -> u32;
    fn outb(value: u8, port: usize);
    fn outw(value: u16, port: usize);
    fn outl(value: u32, port: usize);
    fn cpu_relax();
    fn register_pci_controller(controller: *mut PciChannel) -> c_int;
}

// These addresses are supplied by <mach/pci.h>.
const GAPSPCI_REGS: usize = crate::GAPSPCI_REGS;
const GAPSPCI_BBA_CONFIG: usize = crate::GAPSPCI_BBA_CONFIG;
const GAPSPCI_BBA_CONFIG_SIZE: usize = crate::GAPSPCI_BBA_CONFIG_SIZE;
const GAPSPCI_DMA_BASE: usize = crate::GAPSPCI_DMA_BASE;
const GAPSPCI_DMA_SIZE: usize = crate::GAPSPCI_DMA_SIZE;

#[repr(C)]
struct Resource {
    name: *const c_char,
    start: usize,
    end: usize,
    flags: u64,
}

#[repr(C)]
struct PciChannel {
    pci_ops: *const c_void,
    resources: *mut Resource,
    nr_resources: usize,
    io_offset: usize,
    mem_offset: usize,
}

static mut GAPSPCI_RESOURCES: [Resource; 2] = [
    Resource {
        name: b"GAPSPCI IO\0".as_ptr() as *const c_char,
        start: GAPSPCI_BBA_CONFIG,
        end: GAPSPCI_BBA_CONFIG + GAPSPCI_BBA_CONFIG_SIZE - 1,
        flags: IORESOURCE_IO,
    },
    Resource {
        name: b"GAPSPCI mem\0".as_ptr() as *const c_char,
        start: GAPSPCI_DMA_BASE,
        end: GAPSPCI_DMA_BASE + GAPSPCI_DMA_SIZE - 1,
        flags: IORESOURCE_MEM,
    },
];

static mut DREAMCAST_PCI_CONTROLLER: PciChannel = PciChannel {
    pci_ops: unsafe { &gapspci_pci_ops },
    resources: unsafe { GAPSPCI_RESOURCES.as_mut_ptr() },
    nr_resources: GAPSPCI_RESOURCES.len(),
    io_offset: 0x0000_0000,
    mem_offset: 0x0000_0000,
};

/*
 * gapspci init
 */
#[allow(non_snake_case)]
unsafe fn gapspci_init() -> c_int {
    let mut idbuf = [0u8; 16];

    /*
     * FIXME: All of this wants documenting to some degree,
     * even some basic register definitions would be nice.
     *
     * I haven't seen anything this ugly since.. maple.
     */
    for i in 0..16 {
        idbuf[i] = inb(GAPSPCI_REGS + i);
    }

    if idbuf != *b"GAPSPCI_BRIDGE_2" {
        return -ENODEV;
    }

    outl(0x5a14_a501, GAPSPCI_REGS + 0x18);

    for _ in 0..1_000_000 {
        cpu_relax();
    }

    if inl(GAPSPCI_REGS + 0x18) != 1 {
        return -EINVAL;
    }

    outl(0x0100_0000, GAPSPCI_REGS + 0x20);
    outl(0x0100_0000, GAPSPCI_REGS + 0x24);

    outl(GAPSPCI_DMA_BASE as u32, GAPSPCI_REGS + 0x28);
    outl((GAPSPCI_DMA_BASE + GAPSPCI_DMA_SIZE) as u32, GAPSPCI_REGS + 0x2c);

    outl(1, GAPSPCI_REGS + 0x14);
    outl(1, GAPSPCI_REGS + 0x34);

    /* Setting Broadband Adapter */
    outw(0xf900, GAPSPCI_BBA_CONFIG + 0x06);
    outl(0x0000_0000, GAPSPCI_BBA_CONFIG + 0x30);
    outb(0x00, GAPSPCI_BBA_CONFIG + 0x3c);
    outb(0xf0, GAPSPCI_BBA_CONFIG + 0x0d);
    outw(0x0006, GAPSPCI_BBA_CONFIG + 0x04);
    outl(0x0000_2001, GAPSPCI_BBA_CONFIG + 0x10);
    outl(0x0100_0000, GAPSPCI_BBA_CONFIG + 0x14);

    register_pci_controller(&mut DREAMCAST_PCI_CONTROLLER)
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
