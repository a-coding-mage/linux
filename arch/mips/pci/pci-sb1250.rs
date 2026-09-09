// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2001,2002,2003 Broadcom Corporation
 * Copyright (C) 2004 by Ralf Baechle (ralf@linux-mips.org)
 */

/* BCM1250-specific PCI support. */

// Dependencies supplied by the surrounding kernel translation.

const PCI_BUS_ENABLED: i32 = 1;
const LDT_BUS_ENABLED: i32 = 2;
const PCI_DEVICE_MODE: i32 = 4;

const PCI_BRIDGE_DEVICE: i32 = 0;
const LDT_BRIDGE_DEVICE: i32 = 1;

static mut cfg_space: *mut core::ffi::c_void = core::ptr::null_mut();
static mut sb1250_bus_status: i32 = 0;

#[cfg(CONFIG_SIBYTE_HAS_LDT)]
static mut ldt_eoi_space: usize = 0;

// External types and symbols are provided by the kernel environment.
extern "C" {
    fn ioremap(phys_addr: usize, size: usize) -> *mut core::ffi::c_void;
    fn iounmap(addr: *mut core::ffi::c_void);
    fn __raw_readq(addr: usize) -> u64;
    fn set_io_port_base(base: usize);
    fn register_pci_controller(controller: *mut pci_controller);
    fn pci_set_flags(flags: u32);
    fn printk(fmt: *const u8, ...);
}

#[repr(C)]
pub struct pci_bus {
    pub number: u32,
}

#[repr(C)]
pub struct pci_dev {
    pub irq: i32,
}

#[repr(C)]
pub struct pci_ops {
    pub read: Option<unsafe extern "C" fn(*mut pci_bus, u32, i32, i32, *mut u32) -> i32>,
    pub write: Option<unsafe extern "C" fn(*mut pci_bus, u32, i32, i32, u32) -> i32>,
}

#[repr(C)]
pub struct resource {
    pub name: *const u8,
    pub start: usize,
    pub end: usize,
    pub flags: usize,
}

#[repr(C)]
pub struct pci_controller {
    pub pci_ops: *mut pci_ops,
    pub mem_resource: *mut resource,
    pub io_resource: *mut resource,
    pub io_map_base: usize,
}

extern "C" {
    static mut PCIBIOS_MIN_IO: usize;
    static mut PCIBIOS_MIN_MEM: usize;
    static mut ioport_resource: resource;
    static mut iomem_resource: resource;
}

const PCIBIOS_BAD_REGISTER_NUMBER: i32 = 1;
const PCIBIOS_SUCCESSFUL: i32 = 0;
const PCI_PROBE_ONLY: u32 = 0x0001;
const IORESOURCE_MEM: usize = 0x00000200;
const IORESOURCE_IO: usize = 0x00000100;
const PCI_COMMAND: u32 = 0x04;
const PCI_COMMAND_MASTER: u32 = 0x0004;
const M_SYS_PCI_HOST: u64 = 0;
const A_SCD_SYSTEM_CFG: usize = 0;
const A_PHYS_LDTPCI_CFG_MATCH_BITS: usize = 0;
const A_PHYS_LDTPCI_IO_MATCH_BYTES: usize = 0;
const A_PHYS_LDT_SPECIAL_MATCH_BYTES: usize = 0;

#[inline]
unsafe fn cfgoffset(bus: u32, devfn: u32, where_: u32) -> u32 {
    (bus << 16).wrapping_add(devfn << 8).wrapping_add(where_)
}

#[inline]
unsafe fn cfgaddr(bus: *mut pci_bus, devfn: u32, where_: u32) -> u32 {
    cfgoffset((*bus).number, devfn, where_)
}

#[inline]
unsafe fn readcfg32(addr: u32) -> u32 {
    *((cfg_space as *mut u8).add((addr & !3) as usize) as *mut u32)
}

#[inline]
unsafe fn writecfg32(addr: u32, data: u32) {
    *((cfg_space as *mut u8).add((addr & !3) as usize) as *mut u32) = data;
}

#[no_mangle]
pub unsafe extern "C" fn pcibios_map_irq(dev: *const pci_dev, _slot: u8, _pin: u8) -> i32 {
    (*dev).irq
}

#[no_mangle]
pub unsafe extern "C" fn pcibios_plat_dev_init(_dev: *mut pci_dev) -> i32 {
    0
}

#[inline]
unsafe fn pci_slot(devfn: u32) -> u32 { devfn >> 3 }

unsafe fn sb1250_pci_can_access(bus: *mut pci_bus, devfn: i32) -> i32 {
    let mut devno: u32;
    if (sb1250_bus_status & (PCI_BUS_ENABLED | PCI_DEVICE_MODE)) == 0 { return 0; }
    if (*bus).number == 0 {
        devno = pci_slot(devfn as u32);
        if devno == LDT_BRIDGE_DEVICE as u32 { return if sb1250_bus_status & LDT_BUS_ENABLED != 0 { 1 } else { 0 }; }
        else if sb1250_bus_status & PCI_DEVICE_MODE != 0 { return 0; }
        else { return 1; }
    } else { 1 }
}

unsafe extern "C" fn sb1250_pcibios_read(bus: *mut pci_bus, devfn: u32, where_: i32, size: i32, val: *mut u32) -> i32 {
    let mut data: u32 = 0;
    if (size == 2 && where_ & 1 != 0) || (size == 4 && where_ & 3 != 0) { return PCIBIOS_BAD_REGISTER_NUMBER; }
    if sb1250_pci_can_access(bus, devfn as i32) != 0 { data = readcfg32(cfgaddr(bus, devfn, where_ as u32)); } else { data = 0xffff_ffff; }
    if size == 1 { *val = (data >> (((where_ & 3) << 3) as u32)) & 0xff; }
    else if size == 2 { *val = (data >> (((where_ & 3) << 3) as u32)) & 0xffff; } else { *val = data; }
    PCIBIOS_SUCCESSFUL
}

unsafe extern "C" fn sb1250_pcibios_write(bus: *mut pci_bus, devfn: u32, where_: i32, size: i32, val: u32) -> i32 {
    let cfgaddr_ = cfgaddr(bus, devfn, where_ as u32);
    if (size == 2 && where_ & 1 != 0) || (size == 4 && where_ & 3 != 0) { return PCIBIOS_BAD_REGISTER_NUMBER; }
    if sb1250_pci_can_access(bus, devfn as i32) == 0 { return PCIBIOS_BAD_REGISTER_NUMBER; }
    let shift = ((where_ & 3) << 3) as u32;
    let mut data = readcfg32(cfgaddr_);
    if size == 1 { data = (data & !(0xff << shift)) | (val << shift); }
    else if size == 2 { data = (data & !(0xffff << shift)) | (val << shift); } else { data = val; }
    writecfg32(cfgaddr_, data);
    PCIBIOS_SUCCESSFUL
}

#[no_mangle]
pub static mut sb1250_pci_ops: pci_ops = pci_ops { read: Some(sb1250_pcibios_read), write: Some(sb1250_pcibios_write) };

static mut sb1250_mem_resource: resource = resource { name: b"SB1250 PCI MEM\0".as_ptr(), start: 0x4000_0000, end: 0x5fff_ffff, flags: IORESOURCE_MEM };
static mut sb1250_io_resource: resource = resource { name: b"SB1250 PCI I/O\0".as_ptr(), start: 0, end: 0x01ff_ffff, flags: IORESOURCE_IO };

#[no_mangle]
pub static mut sb1250_controller: pci_controller = pci_controller { pci_ops: &raw mut sb1250_pci_ops, mem_resource: &raw mut sb1250_mem_resource, io_resource: &raw mut sb1250_io_resource, io_map_base: 0 };

#[no_mangle]
pub unsafe extern "C" fn sb1250_pcibios_init() -> i32 {
    pci_set_flags(PCI_PROBE_ONLY);
    PCIBIOS_MIN_IO = 0x8000;
    PCIBIOS_MIN_MEM = 0x0100_0000;
    ioport_resource.end = 0x01ff_ffff;
    iomem_resource.end = 0xffff_ffff;
    cfg_space = ioremap(A_PHYS_LDTPCI_CFG_MATCH_BITS, 16 * 1024 * 1024);
    let reg = __raw_readq(A_SCD_SYSTEM_CFG);
    if reg & M_SYS_PCI_HOST == 0 { sb1250_bus_status |= PCI_DEVICE_MODE; }
    else {
        let cmdreg = readcfg32(cfgoffset(0, PCI_BRIDGE_DEVICE as u32 * 8, PCI_COMMAND));
        if cmdreg & PCI_COMMAND_MASTER == 0 { iounmap(cfg_space); return 0; }
        sb1250_bus_status |= PCI_BUS_ENABLED;
    }
    let io_map_base = ioremap(A_PHYS_LDTPCI_IO_MATCH_BYTES, 1024 * 1024);
    sb1250_controller.io_map_base = io_map_base as usize;
    set_io_port_base(io_map_base as usize);
    #[cfg(CONFIG_SIBYTE_HAS_LDT)] {
        let cmdreg = readcfg32(cfgoffset(0, LDT_BRIDGE_DEVICE as u32 * 8, PCI_COMMAND));
        if cmdreg & PCI_COMMAND_MASTER != 0 {
            sb1250_bus_status |= LDT_BUS_ENABLED;
            ldt_eoi_space = ioremap(A_PHYS_LDT_SPECIAL_MATCH_BYTES, 4 * 1024 * 1024) as usize;
        }
    }
    register_pci_controller(&raw mut sb1250_controller);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
