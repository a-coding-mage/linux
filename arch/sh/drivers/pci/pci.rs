// SPDX-License-Identifier: GPL-2.0
/*
 * New-style PCI core.
 *
 * Copyright (c) 2004 - 2009  Paul Mundt
 * Copyright (c) 2002  M. R. Brown
 *
 * Modelled after arch/mips/pci/pci.c:
 *  Copyright (C) 2003, 04 Ralf Baechle (ralf@linux-mips.org)
 */

// Linux kernel dependencies supplied by the surrounding translation unit.
use core::ffi::c_void;

pub type ResourceSizeT = u64;

#[repr(C)]
pub struct Resource {
    pub start: ResourceSizeT,
    pub end: ResourceSizeT,
    pub flags: u64,
}

#[repr(C)]
pub struct PciChannel {
    pub next: *mut PciChannel,
    pub nr_resources: i32,
    pub resources: *mut Resource,
    pub io_offset: ResourceSizeT,
    pub mem_offset: ResourceSizeT,
    pub index: i32,
    pub io_map_base: usize,
    pub bus: *mut PciBus,
    pub pci_ops: *mut c_void,
    pub need_domain_info: i32,
}

#[repr(C)]
pub struct PciBus {
    pub busn_res: Resource,
    pub devices: *mut PciDev,
}

#[repr(C)]
pub struct PciDev {
    pub sysdata: *mut c_void,
    pub bus: *mut PciBus,
    pub devfn: u32,
    pub subordinate: *mut PciBus,
    pub bus_list_next: *mut PciDev,
}

#[repr(C)]
pub struct PciHostBridge {
    pub dev: c_void,
    pub sysdata: *mut c_void,
    pub busnr: i32,
    pub ops: *mut c_void,
    pub swizzle_irq: Option<unsafe extern "C" fn()>,
    pub map_irq: Option<unsafe extern "C" fn()>,
    pub windows: c_void,
    pub bus: *mut PciBus,
}

extern "C" {
    static mut ioport_resource: Resource;
    static mut iomem_resource: Resource;
    static mut sh_io_port_base: usize;
    static mut pci_domains_supported: bool;

    fn pci_alloc_host_bridge(extra: usize) -> *mut PciHostBridge;
    fn pci_free_host_bridge(bridge: *mut PciHostBridge);
    fn pci_add_resource_offset(resources: *mut c_void, res: *mut Resource, offset: ResourceSizeT);
    fn pci_scan_root_bus_bridge(bridge: *mut PciHostBridge) -> i32;
    fn pci_bus_size_bridges(bus: *mut PciBus);
    fn pci_bus_assign_resources(bus: *mut PciBus);
    fn pci_bus_add_devices(bus: *mut PciBus);
    fn pci_common_swizzle();
    fn pcibios_map_platform_irq();
    fn request_resource(parent: *mut Resource, res: *mut Resource) -> i32;
    fn release_resource(res: *mut Resource);
    fn pcibios_enable_timers(hose: *mut PciChannel);
    fn mutex_lock(lock: *mut c_void);
    fn mutex_unlock(lock: *mut c_void);
    fn early_read_config_word(hose: *mut PciChannel, top_bus: i32, current_bus: i32,
                              devfn: u32, where_: u32, value: *mut u16) -> i32;
    fn early_write_config_word(hose: *mut PciChannel, top_bus: i32, current_bus: i32,
                               devfn: u32, where_: u32, value: u16);
    fn pci_read_config_word(dev: *mut PciDev, where_: u32, value: *mut u16);
    fn pci_write_config_word(dev: *mut PciDev, where_: u32, value: u16);
    fn pci_name(dev: *mut PciDev) -> *const u8;
    fn pci_align_resource(dev: *mut PciDev, res: *const Resource,
                          empty_res: *const Resource, size: ResourceSizeT,
                          align: ResourceSizeT) -> ResourceSizeT;
    fn iounmap(addr: *mut c_void);
    fn panic(message: *const u8) -> !;
}

pub static mut PCIBIOS_MIN_IO: u64 = 0x0000;
pub static mut PCIBIOS_MIN_MEM: u64 = 0;

static mut hose_head: *mut PciChannel = core::ptr::null_mut();
static mut hose_tail: *mut *mut PciChannel = &raw mut hose_head;
static mut pci_initialized: i32 = 0;
static mut pci_config_lock: c_void = c_void {};
static mut pci_scan_mutex: c_void = c_void {};

const IORESOURCE_DISABLED: u64 = 0x0000_0200;
const IORESOURCE_IO: u64 = 0x0000_0100;
const IORESOURCE_MEM: u64 = 0x0000_0200;
const PCIBIOS_SUCCESSFUL: i32 = 0;
const PCI_STATUS: u32 = 0x06;

unsafe fn pcibios_scanbus(hose: *mut PciChannel) {
    static mut next_busno: i32 = 0;
    static mut need_domain_info: i32 = 0;
    let resources: *mut c_void = core::ptr::null_mut();
    let bridge = pci_alloc_host_bridge(0);
    if bridge.is_null() { return; }

    for i in 0..(*hose).nr_resources {
        let res = (*hose).resources.add(i as usize);
        let mut offset: ResourceSizeT = 0;
        if (*res).flags & IORESOURCE_DISABLED != 0 { continue; }
        if (*res).flags & IORESOURCE_IO != 0 { offset = (*hose).io_offset; }
        else if (*res).flags & IORESOURCE_MEM != 0 { offset = (*hose).mem_offset; }
        pci_add_resource_offset(resources, res, offset);
    }
    (*bridge).windows = c_void {};
    (*bridge).sysdata = hose as *mut c_void;
    (*bridge).busnr = next_busno;
    (*bridge).ops = (*hose).pci_ops;
    (*bridge).swizzle_irq = Some(pci_common_swizzle);
    (*bridge).map_irq = Some(pcibios_map_platform_irq);
    if pci_scan_root_bus_bridge(bridge) != 0 { pci_free_host_bridge(bridge); return; }
    (*hose).bus = (*bridge).bus;
    need_domain_info |= (*hose).index;
    (*hose).need_domain_info = need_domain_info;
    next_busno = (*hose).busn_res.end as i32 + 1;
    if next_busno > 224 { next_busno = 0; need_domain_info = 1; }
    pci_bus_size_bridges((*hose).bus);
    pci_bus_assign_resources((*hose).bus);
    pci_bus_add_devices((*hose).bus);
}

pub unsafe fn register_pci_controller(hose: *mut PciChannel) -> i32 {
    let mut i = 0;
    while i < (*hose).nr_resources {
        let res = (*hose).resources.add(i as usize);
        if (*res).flags & IORESOURCE_DISABLED == 0 {
            let parent = if (*res).flags & IORESOURCE_IO != 0 { &raw mut ioport_resource } else { &raw mut iomem_resource };
            if request_resource(parent, res) < 0 { break; }
        }
        i += 1;
    }
    if i != (*hose).nr_resources {
        while i > 0 { i -= 1; release_resource((*hose).resources.add(i as usize)); }
        return -1;
    }
    *hose_tail = hose;
    hose_tail = &mut (*hose).next;
    pcibios_enable_timers(hose);
    if pci_initialized != 0 { mutex_lock(&raw mut pci_scan_mutex); pcibios_scanbus(hose); mutex_unlock(&raw mut pci_scan_mutex); }
    0
}

unsafe fn pcibios_init() -> i32 {
    let mut hose = hose_head;
    while !hose.is_null() { pcibios_scanbus(hose); hose = (*hose).next; }
    pci_initialized = 1;
    0
}

pub unsafe fn pcibios_align_resource(data: *mut c_void, res: *const Resource,
                                     empty_res: *const Resource, size: ResourceSizeT,
                                     align: ResourceSizeT) -> ResourceSizeT {
    let dev = data as *mut PciDev;
    let hose = (*dev).sysdata as *mut PciChannel;
    let mut start = (*res).start;
    if (*res).flags & IORESOURCE_IO != 0 {
        if start < PCIBIOS_MIN_IO + (*(*hose).resources).start { start = PCIBIOS_MIN_IO + (*(*hose).resources).start; }
        if start & 0x300 != 0 { start = (start + 0x3ff) & !0x3ff; }
    } else if (*res).flags & IORESOURCE_MEM != 0 {
        start = pci_align_resource(dev, res, empty_res, size, align);
    }
    start
}

unsafe fn pcibios_bus_report_status_early(hose: *mut PciChannel, top_bus: i32,
                                          current_bus: i32, status_mask: u32, warn: i32) {
    let mut pci_devfn: u32 = 0;
    while pci_devfn < 0xff {
        if pci_devfn & 7 != 0 { pci_devfn += 1; continue; }
        let mut status: u16 = 0;
        let ret = early_read_config_word(hose, top_bus, current_bus, pci_devfn,
                                         PCI_STATUS, &mut status);
        if ret == PCIBIOS_SUCCESSFUL && status != 0xffff {
            early_write_config_word(hose, top_bus, current_bus, pci_devfn,
                                    PCI_STATUS, status & status_mask as u16);
        }
        pci_devfn += 1;
    }
}

unsafe fn pcibios_bus_report_status(bus: *mut PciBus, status_mask: u32, _warn: i32) {
    let mut dev = (*bus).devices;
    while !dev.is_null() {
        if !((*dev).bus == bus && (*(*dev).bus).busn_res.start == 0 && (*dev).devfn == 0) {
            let mut status: u16 = 0;
            pci_read_config_word(dev, PCI_STATUS, &mut status);
            if status != 0xffff && (status as u32 & status_mask) != 0 {
                pci_write_config_word(dev, PCI_STATUS, status & status_mask as u16);
            }
        }
        dev = (*dev).bus_list_next;
    }
    dev = (*bus).devices;
    while !dev.is_null() {
        if !(*dev).subordinate.is_null() {
            pcibios_bus_report_status((*dev).subordinate, status_mask, _warn);
        }
        dev = (*dev).bus_list_next;
    }
}

pub unsafe fn pcibios_report_status(status_mask: u32, warn: i32) {
    let mut hose = hose_head;
    while !hose.is_null() {
        if (*hose).bus.is_null() {
            pcibios_bus_report_status_early(hose, (*hose_head).index,
                                            (*hose).index, status_mask, warn);
        } else {
            pcibios_bus_report_status((*hose).bus, status_mask, warn);
        }
        hose = (*hose).next;
    }
}

#[cfg(not(CONFIG_GENERIC_IOMAP))]
pub unsafe fn __pci_ioport_map(dev: *mut PciDev, port: usize, _nr: u32) -> *mut c_void {
    let chan = (*dev).sysdata as *mut PciChannel;
    if (*chan).io_map_base == 0 {
        (*chan).io_map_base = sh_io_port_base;
        if pci_domains_supported { panic(b"To avoid data corruption io_map_base MUST be set with multiple PCI domains.\0".as_ptr()); }
    }
    ((*chan).io_map_base + port) as *mut c_void
}

#[cfg(not(CONFIG_GENERIC_IOMAP))]
pub unsafe fn pci_iounmap(_dev: *mut PciDev, addr: *mut c_void) { iounmap(addr); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
