// SPDX-License-Identifier: GPL-2.0
/*
 * PCI operations for the Sega Dreamcast
 *
 * Copyright (C) 2001, 2002  M. R. Brown
 * Copyright (C) 2002, 2003  Paul Mundt
 */

// Kernel and machine-header dependencies from the original C source are
// supplied by the surrounding translation unit.

#[repr(C)]
pub struct pci_bus {
    pub number: u8,
}

unsafe extern "C" {
    fn inb(address: usize) -> u8;
    fn inw(address: usize) -> u16;
    fn inl(address: usize) -> u32;
    fn outb(value: u8, address: usize);
    fn outw(value: u16, address: usize);
    fn outl(value: u32, address: usize);

    static GAPSPCI_BBA_CONFIG: usize;
}

const PCIBIOS_DEVICE_NOT_FOUND: i32 = -1;
const PCIBIOS_SUCCESSFUL: i32 = 0;

/*
 * The !gapspci_config_access case really shouldn't happen, ever, unless
 * someone implicitly messes around with the last devfn value.. otherwise we
 * only support a single device anyways, and if we didn't have a BBA, we
 * wouldn't make it terribly far through the PCI setup anyways.
 *
 * Also, we could very easily support both Type 0 and Type 1 configurations
 * here, but since it doesn't seem that there is any such implementation in
 * existence, we don't bother.
 *
 * I suppose if someone actually gets around to ripping the chip out of
 * the BBA and hanging some more devices off of it, then this might be
 * something to take into consideration. However, due to the cost of the BBA,
 * and the general lack of activity by DC hardware hackers, this doesn't seem
 * likely to happen anytime soon.
 */
unsafe fn gapspci_config_access(bus: u8, devfn: u32) -> i32 {
    ((bus == 0) && (devfn == 0)) as i32
}

/*
 * We can also actually read and write in b/w/l sizes! Thankfully this part
 * was at least done right, and we don't have to do the stupid masking and
 * shifting that we do on the 7751! Small wonders never cease to amaze.
 */
pub unsafe fn gapspci_read(
    bus: *mut pci_bus,
    devfn: u32,
    where_: i32,
    size: i32,
    val: *mut u32,
) -> i32 {
    *val = 0xffff_ffff;

    if gapspci_config_access((*bus).number, devfn) == 0 {
        return PCIBIOS_DEVICE_NOT_FOUND;
    }

    let address = GAPSPCI_BBA_CONFIG.wrapping_add(where_ as usize);
    match size {
        1 => *val = inb(address) as u32,
        2 => *val = inw(address) as u32,
        4 => *val = inl(address),
        _ => {}
    }

    PCIBIOS_SUCCESSFUL
}

pub unsafe fn gapspci_write(
    bus: *mut pci_bus,
    devfn: u32,
    where_: i32,
    size: i32,
    val: u32,
) -> i32 {
    if gapspci_config_access((*bus).number, devfn) == 0 {
        return PCIBIOS_DEVICE_NOT_FOUND;
    }

    let address = GAPSPCI_BBA_CONFIG.wrapping_add(where_ as usize);
    match size {
        1 => outb(val as u8, address),
        2 => outw(val as u16, address),
        4 => outl(val as u32, address),
        _ => {}
    }

    PCIBIOS_SUCCESSFUL
}

#[repr(C)]
pub struct pci_ops {
    pub read: unsafe fn(*mut pci_bus, u32, i32, i32, *mut u32) -> i32,
    pub write: unsafe fn(*mut pci_bus, u32, i32, i32, u32) -> i32,
}

#[no_mangle]
pub static gapspci_pci_ops: pci_ops = pci_ops {
    read: gapspci_read,
    write: gapspci_write,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
