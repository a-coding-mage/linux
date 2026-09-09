/*
 *  BRIEF MODULE DESCRIPTION
 *     pci_ops for IDT EB434 board
 *
 *  Copyright 2004 IDT Inc. (rischelp@idt.com)
 *  Copyright 2006 Felix Fietkau <nbd@openwrt.org>
 *
 *  This program is free software; you can redistribute it and/or modify it
 *  under the terms of the GNU General Public License as published by the
 *  Free Software Foundation; either version 2 of the License, or
 *  (at your option) any later version.
 */

// Linux kernel and RC32434 declarations are supplied by external dependencies.

const PCI_ACCESS_READ: u8 = 0;
const PCI_ACCESS_WRITE: u8 = 1;

unsafe extern "C" {
    static mut rc32434_pci: *mut rc32434_pci_regs;
    fn rc32434_sync();
    fn msleep(milliseconds: u32);
}

#[repr(C)]
pub struct rc32434_pci_regs {
    pub pcicfga: u32,
    pub pcicfgd: u32,
}

#[repr(C)]
pub struct pci_bus {
    pub number: u8,
}

#[repr(C)]
pub struct pci_ops {
    pub read: Option<unsafe extern "C" fn(*mut pci_bus, u32, i32, i32, *mut u32) -> i32>,
    pub write: Option<unsafe extern "C" fn(*mut pci_bus, u32, i32, i32, u32) -> i32>,
}

const PCIBIOS_SUCCESSFUL: i32 = 0;
const PCI_VENDOR_ID: i32 = 0;

#[inline]
unsafe fn pci_slot(devfn: u32) -> u32 {
    (devfn >> 3) & 0x1f
}

#[inline]
unsafe fn pci_func(devfn: u32) -> u8 {
    (devfn & 7) as u8
}

#[inline]
unsafe fn config_access(
    access_type: u8,
    bus: *mut pci_bus,
    devfn: u32,
    where_: u8,
    data: *mut u32,
) -> i32 {
    let slot = pci_slot(devfn);
    let func = pci_func(devfn);

    (*rc32434_pci).pcicfga = 0x8000_0000
        | ((*bus).number as u32) << 16
        | slot << 11
        | (func as u32) << 8
        | where_ as u32;
    rc32434_sync();

    if access_type == PCI_ACCESS_WRITE {
        (*rc32434_pci).pcicfgd = *data;
    } else {
        *data = (*rc32434_pci).pcicfgd;
    }

    rc32434_sync();
    0
}

unsafe fn read_config_byte(bus: *mut pci_bus, devfn: u32, where_: i32, val: *mut u8) -> i32 {
    let mut data = 0u32;
    let ret = config_access(PCI_ACCESS_READ, bus, devfn, where_ as u8, &mut data);
    *val = (data >> (((where_ as u32) & 3) << 3)) as u8;
    ret
}

unsafe fn read_config_word(bus: *mut pci_bus, devfn: u32, where_: i32, val: *mut u16) -> i32 {
    let mut data = 0u32;
    let ret = config_access(PCI_ACCESS_READ, bus, devfn, where_ as u8, &mut data);
    *val = (data >> (((where_ as u32) & 3) << 3)) as u16;
    ret
}

unsafe fn read_config_dword(bus: *mut pci_bus, devfn: u32, where_: i32, val: *mut u32) -> i32 {
    let mut delay = 1u32;
    if (*bus).number == 0 && pci_slot(devfn) > 21 {
        return 0;
    }

    loop {
        let ret = config_access(PCI_ACCESS_READ, bus, devfn, where_ as u8, val);
        if where_ == PCI_VENDOR_ID
            && (*val == 0xffff_ffff || *val == 0 || *val == 0x0000_ffff || *val == 0xffff_0000)
        {
            if delay > 4 {
                return 0;
            }
            delay *= 2;
            msleep(delay);
            continue;
        }
        return ret;
    }
}

unsafe fn write_config_byte(bus: *mut pci_bus, devfn: u32, where_: i32, val: u8) -> i32 {
    let mut data = 0u32;
    if config_access(PCI_ACCESS_READ, bus, devfn, where_ as u8, &mut data) != 0 { return -1; }
    let shift = ((where_ as u32) & 3) << 3;
    data = (data & !(0xff << shift)) | ((val as u32) << shift);
    if config_access(PCI_ACCESS_WRITE, bus, devfn, where_ as u8, &mut data) != 0 { return -1; }
    PCIBIOS_SUCCESSFUL
}

unsafe fn write_config_word(bus: *mut pci_bus, devfn: u32, where_: i32, val: u16) -> i32 {
    let mut data = 0u32;
    if config_access(PCI_ACCESS_READ, bus, devfn, where_ as u8, &mut data) != 0 { return -1; }
    let shift = ((where_ as u32) & 3) << 3;
    data = (data & !(0xffff << shift)) | ((val as u32) << shift);
    if config_access(PCI_ACCESS_WRITE, bus, devfn, where_ as u8, &mut data) != 0 { return -1; }
    PCIBIOS_SUCCESSFUL
}

unsafe fn write_config_dword(bus: *mut pci_bus, devfn: u32, where_: i32, mut val: u32) -> i32 {
    if config_access(PCI_ACCESS_WRITE, bus, devfn, where_ as u8, &mut val) != 0 { return -1; }
    PCIBIOS_SUCCESSFUL
}

unsafe extern "C" fn pci_config_read(bus: *mut pci_bus, devfn: u32, where_: i32, size: i32, val: *mut u32) -> i32 {
    match size {
        1 => read_config_byte(bus, devfn, where_, val as *mut u8),
        2 => read_config_word(bus, devfn, where_, val as *mut u16),
        _ => read_config_dword(bus, devfn, where_, val),
    }
}

unsafe extern "C" fn pci_config_write(bus: *mut pci_bus, devfn: u32, where_: i32, size: i32, val: u32) -> i32 {
    match size {
        1 => write_config_byte(bus, devfn, where_, val as u8),
        2 => write_config_word(bus, devfn, where_, val as u16),
        _ => write_config_dword(bus, devfn, where_, val),
    }
}

pub static mut rc32434_pci_ops: pci_ops = pci_ops {
    read: Some(pci_config_read),
    write: Some(pci_config_write),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
