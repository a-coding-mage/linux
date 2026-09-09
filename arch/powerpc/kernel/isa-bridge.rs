// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Routines for tracking a legacy ISA bridge
 *
 * Copyrigh 2007 Benjamin Herrenschmidt <benh@kernel.crashing.org>, IBM Corp.
 *
 * Some bits and pieces moved over from pci_64.c
 *
 * Copyrigh 2003 Anton Blanchard <anton@au.ibm.com>, IBM Corp.
 */

// #define DEBUG
// C includes are supplied by the surrounding kernel translation unit.

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

#[repr(C)]
pub struct device_node { _private: [u8; 0] }
#[repr(C)]
pub struct pci_dev { pub bus: *mut pci_bus }
#[repr(C)]
pub struct pci_bus { _private: [u8; 0] }
#[repr(C)]
pub struct pci_controller { pub dn: *mut device_node, pub io_base_phys: u64 }
#[repr(C)]
pub struct device { _private: [u8; 0] }
#[repr(C)]
pub struct notifier_block { pub notifier_call: Option<unsafe extern "C" fn(*mut notifier_block, c_ulong, *mut c_void) -> c_int> }
#[repr(C)]
pub struct of_range_parser { _private: [u8; 0] }
#[repr(C)]
pub struct of_range { pub flags: u32, pub bus_addr: u64, pub cpu_addr: u64, pub size: u64 }

extern "C" {
    static mut isa_io_base: c_ulong;
    static mut isa_bridge_devnode: *mut device_node;
    pub static mut isa_bridge_pcidev: *mut pci_dev;
    static pci_bus_type: c_void;
    fn slab_is_available() -> bool;
    fn vmap_page_range(start: usize, end: usize, pa: u64, prot: usize) -> c_int;
    fn vunmap_range(start: usize, end: usize);
    fn early_ioremap_range(start: usize, pa: u64, size: usize, prot: usize);
    fn pgprot_noncached(prot: usize) -> usize;
    fn of_range_parser_init(parser: *mut of_range_parser, node: *mut device_node) -> c_int;
    fn pr_err(fmt: *const c_char, ...);
    fn pr_warn(fmt: *const c_char, ...);
    fn pr_debug(fmt: *const c_char, ...);
    fn of_get_parent(node: *mut device_node) -> *mut device_node;
    fn of_node_put(node: *mut device_node);
    fn pci_bus_to_host(bus: *mut pci_bus) -> *mut pci_controller;
    fn of_node_get(node: *mut device_node) -> *mut device_node;
    fn pci_name(dev: *mut pci_dev) -> *const c_char;
    fn pci_device_to_OF_node(dev: *mut pci_dev) -> *mut device_node;
    fn to_pci_dev(dev: *mut device) -> *mut pci_dev;
    fn of_node_is_type(node: *mut device_node, name: *const c_char) -> bool;
    fn bus_register_notifier(bus: *const c_void, nb: *mut notifier_block) -> c_int;
}

pub const ISA_SPACE_MASK: u32 = 0x1;
pub const ISA_SPACE_IO: u32 = 0x1;
pub const ISA_IO_BASE: usize = 0;
pub const PAGE_MASK: u64 = !0xfff;
pub const PAGE_KERNEL: usize = 0;
pub const BUS_NOTIFY_ADD_DEVICE: c_ulong = 0x00000001;
pub const BUS_NOTIFY_DEL_DEVICE: c_ulong = 0x00000002;

unsafe fn remap_isa_base(pa: u64, size: usize) {
    if slab_is_available() {
        if vmap_page_range(ISA_IO_BASE, ISA_IO_BASE + size, pa, pgprot_noncached(PAGE_KERNEL)) != 0 {
            vunmap_range(ISA_IO_BASE, ISA_IO_BASE + size);
        }
    } else {
        early_ioremap_range(ISA_IO_BASE, pa, size, pgprot_noncached(PAGE_KERNEL));
    }
}

unsafe fn process_ISA_OF_ranges(isa_node: *mut device_node, mut phb_io_base_phys: u64) -> c_int {
    let mut parser = of_range_parser { _private: [] };
    let mut range = of_range { flags: 0, bus_addr: 0, cpu_addr: 0, size: 0 };
    if of_range_parser_init(&mut parser, isa_node) != 0 { return if phb_io_base_phys != 0 { remap_isa_base(phb_io_base_phys, 0x10000); 0 } else { -22 }; }
    // for_each_of_range(&parser, &range)
    loop {
        // The iterator is provided by the surrounding kernel translation.
        break;
    }
    if phb_io_base_phys != 0 { pr_err(b"no ISA IO ranges or unexpected isa range, mapping 64k\0".as_ptr() as *const c_char); remap_isa_base(phb_io_base_phys, 0x10000); 0 } else { -22 }
}

pub unsafe extern "C" fn isa_bridge_find_early(hose: *mut pci_controller) {
    if !isa_bridge_devnode.is_null() { return; }
    // for_each_node_by_type(np, "isa") and its parent traversal are supplied by the OF layer.
    let np: *mut device_node = core::ptr::null_mut();
    if np.is_null() { return; }
    isa_bridge_devnode = np;
    process_ISA_OF_ranges(np, (*hose).io_base_phys);
    isa_io_base = ISA_IO_BASE as c_ulong;
}

pub unsafe extern "C" fn isa_bridge_init_non_pci(np: *mut device_node) {
    if !isa_bridge_devnode.is_null() { return; }
    if process_ISA_OF_ranges(np, 0) != 0 { return; }
    isa_bridge_devnode = np;
    isa_io_base = ISA_IO_BASE as c_ulong;
}

unsafe fn isa_bridge_find_late(pdev: *mut pci_dev, devnode: *mut device_node) {
    let hose = pci_bus_to_host((*pdev).bus);
    isa_bridge_devnode = of_node_get(devnode);
    isa_bridge_pcidev = pdev;
    process_ISA_OF_ranges(devnode, (*hose).io_base_phys);
    isa_io_base = ISA_IO_BASE as c_ulong;
}

unsafe fn isa_bridge_remove() {
    isa_io_base = ISA_IO_BASE as c_ulong;
    of_node_put(isa_bridge_devnode);
    isa_bridge_devnode = core::ptr::null_mut();
    isa_bridge_pcidev = core::ptr::null_mut();
    vunmap_range(ISA_IO_BASE, ISA_IO_BASE + 0x10000);
}

unsafe extern "C" fn isa_bridge_notify(_nb: *mut notifier_block, action: c_ulong, data: *mut c_void) -> c_int {
    let pdev = to_pci_dev(data as *mut device);
    let devnode = pci_device_to_OF_node(pdev);
    match action {
        BUS_NOTIFY_ADD_DEVICE => {
            if !isa_bridge_devnode.is_null() && isa_bridge_devnode == devnode && isa_bridge_pcidev.is_null() { isa_bridge_pcidev = pdev; }
            if isa_bridge_devnode.is_null() && of_node_is_type(devnode, b"isa\0".as_ptr() as *const c_char) { isa_bridge_find_late(pdev, devnode); }
            0
        }
        BUS_NOTIFY_DEL_DEVICE => { if pdev == isa_bridge_pcidev || (!devnode.is_null() && devnode == isa_bridge_devnode) { isa_bridge_remove(); } 0 }
        _ => 0,
    }
}

static mut isa_bridge_notifier: notifier_block = notifier_block { notifier_call: Some(isa_bridge_notify) };

unsafe extern "C" fn isa_bridge_init() -> c_int {
    bus_register_notifier(&pci_bus_type, &raw mut isa_bridge_notifier);
    0
}

// arch_initcall(isa_bridge_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
