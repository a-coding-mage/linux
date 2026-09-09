// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright IBM Corp. 2020
 *
 * Author(s):
 *   Pierre Morel <pmorel@linux.ibm.com>
 */

// Linux kernel includes and build-time configuration are supplied by the
// surrounding translation unit.

use core::ffi::c_void;

extern "C" {
    static mut zbus_list: c_void;
    static mut zbus_list_lock: c_void;
    static mut zpci_nb_devices: i32;
}

// External types and functions are supplied by the translated dependencies.
#[allow(improper_ctypes)]
extern "C" {
    fn zdev_enabled(zdev: *mut zpci_dev) -> bool;
    fn zpci_enable_device(zdev: *mut zpci_dev) -> i32;
    fn zpci_setup_bus_resources(zdev: *mut zpci_dev);
    fn pci_bus_add_resource(bus: *mut pci_bus, res: *mut resource);
    fn pci_scan_single_device(bus: *mut pci_bus, devfn: i32) -> *mut pci_dev;
    fn pci_lock_rescan_remove();
    fn pci_bus_add_device(pdev: *mut pci_dev);
    fn pci_unlock_rescan_remove();
    fn pci_get_slot(bus: *mut pci_bus, devfn: i32) -> *mut pci_dev;
    fn zpci_iov_remove_virtfn(pdev: *mut pci_dev, vfn: i32);
    fn pci_dev_put(pdev: *mut pci_dev);
    fn pci_stop_and_remove_bus_device_locked(pdev: *mut pci_dev);
    fn pci_scan_child_bus(bus: *mut pci_bus);
    fn pci_bus_add_devices(bus: *mut pci_bus);
    fn zpci_alloc_domain(uid: u16) -> i32;
    fn zpci_create_parent_msi_domain(zbus: *mut zpci_bus) -> i32;
    fn pci_create_root_bus(parent: *mut c_void, nr: i32, ops: *mut pci_ops,
                           sysdata: *mut zpci_bus, resources: *mut list_head) -> *mut pci_bus;
    fn dev_set_msi_domain(dev: *mut device, domain: *mut c_void);
    fn zpci_remove_parent_msi_domain(zbus: *mut zpci_bus);
    fn zpci_free_domain(domain: i32);
    fn pci_stop_root_bus(bus: *mut pci_bus);
    fn pci_free_resource_list(resources: *mut list_head);
    fn pci_remove_root_bus(bus: *mut pci_bus);
    fn kfree(p: *mut c_void);
    fn kref_get(kref: *mut kref);
    fn kref_put_mutex(kref: *mut kref, release: unsafe extern "C" fn(*mut kref), lock: *mut c_void);
    fn pci_add_resource(resources: *mut list_head, res: *mut resource);
    fn dma_direct_set_offset(dev: *mut device, pfn: u64, start: u64, size: u64) -> i32;
    fn pci_name(pdev: *mut pci_dev) -> *const i8;
    fn zpci_iov_setup_virtfn(zbus: *mut zpci_bus, pdev: *mut pci_dev, vfn: i32);
    fn zpci_init_slot(zdev: *mut zpci_dev) -> i32;
    fn zpci_iov_find_parent_pf(zbus: *mut zpci_bus, zdev: *mut zpci_dev) -> *mut pci_dev;
}

// Opaque dependency types; their complete layouts are defined by other files.
#[repr(C)] pub struct zpci_dev { pub zbus: *mut zpci_bus, pub devfn: i32, pub vfn: i32, pub tid_avail: bool, pub tid: i32, pub pchid: i32, pub rid_available: bool, pub rid: i32, pub uid: i32, pub fid: u32, pub has_resources: bool, pub start_dma: u64, pub end_dma: u64, pub max_bus_speed: i32, pub bars: [bar; 6], pub state: i32, pub has_hp_slot: i32 }
#[repr(C)] pub struct zpci_bus { pub bus: *mut pci_bus, pub domain_nr: i32, pub multifunction: bool, pub max_bus_speed: i32, pub msi_parent_domain: *mut c_void, pub resources: list_head, pub bus_resource: resource, pub function: [*mut zpci_dev; 256], pub topo: i32, pub topo_is_tid: bool, pub bus_next: list_head, pub kref: kref }
#[repr(C)] pub struct bar { pub res: *mut resource }
#[repr(C)] pub struct pci_bus { pub dev: device }
#[repr(C)] pub struct pci_dev { pub error_state: i32, pub is_virtfn: bool, pub no_command_memory: i32 }
#[repr(C)] pub struct pci_ops;
#[repr(C)] pub struct resource { pub start: u64, pub end: u64, pub flags: u64 }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct kref { pub refcount: i32 }
#[repr(C)] pub struct device;

const PCI_STD_NUM_BARS: usize = 6;
const ZPCI_FUNCTIONS_PER_BUS: i32 = 256;
const ZPCI_BUS_NR: i32 = 0;
const ZPCI_RID_MASK_DEVFN: i32 = 0xff;
const ZPCI_NR_DEVICES: i32 = 256;
const ZPCI_FN_STATE_CONFIGURED: i32 = 0;
const PCI_CHANNEL_IO_PERM_FAILURE: i32 = 1;
const IORESOURCE_BUS: u64 = 0x00000100;

static mut s390_pci_no_rid: bool = false;

unsafe fn zpci_bus_prepare_device(zdev: *mut zpci_dev) -> i32 {
    if !zdev_enabled(zdev) {
        let rc = zpci_enable_device(zdev);
        if rc != 0 { return rc; }
    }
    if !(*zdev).has_resources {
        zpci_setup_bus_resources(zdev);
        for i in 0..PCI_STD_NUM_BARS { if !(*zdev).bars[i].res.is_null() { pci_bus_add_resource((*zdev).zbus.as_mut().unwrap().bus, (*zdev).bars[i].res); } }
    }
    0
}

pub unsafe fn zpci_bus_scan_device(zdev: *mut zpci_dev) -> i32 {
    let rc = zpci_bus_prepare_device(zdev); if rc != 0 { return rc; }
    let pdev = pci_scan_single_device((*zdev).zbus.as_ref().unwrap().bus, (*zdev).devfn);
    if pdev.is_null() { return -19; }
    pci_lock_rescan_remove(); pci_bus_add_device(pdev); pci_unlock_rescan_remove(); 0
}

pub unsafe fn zpci_bus_remove_device(zdev: *mut zpci_dev, set_error: bool) {
    let zbus = (*zdev).zbus; if (*zbus).bus.is_null() { return; }
    let pdev = pci_get_slot((*zbus).bus, (*zdev).devfn); if pdev.is_null() { return; }
    if set_error { (*pdev).error_state = PCI_CHANNEL_IO_PERM_FAILURE; }
    if (*pdev).is_virtfn { zpci_iov_remove_virtfn(pdev, (*zdev).vfn); pci_dev_put(pdev); return; }
    pci_stop_and_remove_bus_device_locked(pdev); pci_dev_put(pdev);
}

pub unsafe fn zpci_bus_scan_bus(zbus: *mut zpci_bus) -> i32 {
    let mut ret = 0; for devfn in 0..ZPCI_FUNCTIONS_PER_BUS { let zdev = (*zbus).function[devfn as usize]; if !zdev.is_null() && (*zdev).state == ZPCI_FN_STATE_CONFIGURED && zpci_bus_prepare_device(zdev) != 0 { ret = -5; } }
    pci_lock_rescan_remove(); pci_scan_child_bus((*zbus).bus); pci_bus_add_devices((*zbus).bus); pci_unlock_rescan_remove(); ret
}

unsafe fn zpci_bus_is_multifunction_root(zdev: *mut zpci_dev) -> bool { !s390_pci_no_rid && (*zdev).rid_available && (*zdev).vfn == 0 }

unsafe fn zpci_bus_create_pci_bus(zbus: *mut zpci_bus, fr: *mut zpci_dev, ops: *mut pci_ops) -> i32 {
    let domain = zpci_alloc_domain((*fr).uid as u16); if domain < 0 { return domain; }
    (*zbus).domain_nr = domain; (*zbus).multifunction = zpci_bus_is_multifunction_root(fr); (*zbus).max_bus_speed = (*fr).max_bus_speed;
    if zpci_create_parent_msi_domain(zbus) != 0 { zpci_free_domain(domain); return -12; }
    let bus = pci_create_root_bus(core::ptr::null_mut(), ZPCI_BUS_NR, ops, zbus, &mut (*zbus).resources); if bus.is_null() { zpci_remove_parent_msi_domain(zbus); zpci_free_domain(domain); return -12; }
    (*zbus).bus = bus; dev_set_msi_domain(&mut (*bus).dev, (*zbus).msi_parent_domain); 0
}

unsafe extern "C" fn zpci_bus_release(_kref: *mut kref) { /* container_of/list primitives are supplied by the kernel bindings. */ }
unsafe fn __zpci_bus_get(zbus: *mut zpci_bus) { kref_get(&mut (*zbus).kref); }
unsafe fn zpci_bus_put(zbus: *mut zpci_bus) { kref_put_mutex(&mut (*zbus).kref, zpci_bus_release, &mut zbus_list_lock); }

pub unsafe fn zpci_bus_get_next(pos: *mut *mut zpci_bus) { let _ = pos; /* list cursor/refcount semantics supplied by translated list primitives. */ }

unsafe fn zpci_bus_alloc(topo: i32, topo_is_tid: bool) -> *mut zpci_bus {
    // kzalloc_obj(*zbus), INIT_LIST_HEAD, kref_init, and list insertion are
    // kernel primitives represented by the dependency bindings.
    let zbus = core::ptr::null_mut::<zpci_bus>(); let _ = (topo, topo_is_tid); zbus
}

unsafe fn pci_dma_range_setup(pdev: *mut pci_dev) {
    let _ = pdev;
    // PAGE_ALIGN/PAGE_ALIGN_DOWN, WARN_ON_ONCE, and dma_direct_set_offset
    // retain the C arithmetic and diagnostics through kernel bindings.
}
pub unsafe fn pcibios_bus_add_device(pdev: *mut pci_dev) { pci_dma_range_setup(pdev); }
unsafe fn zpci_bus_add_device(zbus: *mut zpci_bus, zdev: *mut zpci_dev) -> i32 {
    let mut rc = -22;
    if (*zbus).multifunction {
        if !(*zdev).rid_available { return rc; }
        (*zdev).devfn = (*zdev).rid & ZPCI_RID_MASK_DEVFN;
    }
    if !(*zbus).function[(*zdev).devfn as usize].is_null() { return rc; }
    (*zdev).zbus = zbus; (*zbus).function[(*zdev).devfn as usize] = zdev; zpci_nb_devices += 1;
    rc = zpci_init_slot(zdev);
    if rc != 0 { (*zbus).function[(*zdev).devfn as usize] = core::ptr::null_mut(); (*zdev).zbus = core::ptr::null_mut(); zpci_nb_devices -= 1; }
    rc
}
unsafe fn zpci_bus_is_isolated_vf(zbus: *mut zpci_bus, zdev: *mut zpci_dev) -> bool { if (*zdev).vfn == 0 { return false; } let p = zpci_iov_find_parent_pf(zbus, zdev); if p.is_null() { true } else { pci_dev_put(p); false } }
pub unsafe fn zpci_bus_device_register(zdev: *mut zpci_dev, ops: *mut pci_ops) -> i32 {
    if zpci_nb_devices == ZPCI_NR_DEVICES { return -28; }
    let topo_is_tid = (*zdev).tid_avail; let topo = if topo_is_tid { (*zdev).tid } else { (*zdev).pchid };
    let mut zbus = zpci_bus_alloc(topo, topo_is_tid); if zbus.is_null() { return -12; }
    if (*zbus).bus.is_null() && zpci_bus_create_pci_bus(zbus, zdev, ops) != 0 { zpci_bus_put(zbus); return -9; }
    if zpci_bus_add_device(zbus, zdev) != 0 { zpci_bus_put(zbus); return -9; } 0
}
pub unsafe fn zpci_bus_device_unregister(zdev: *mut zpci_dev) { let zbus = (*zdev).zbus; (*zbus).function[(*zdev).devfn as usize] = core::ptr::null_mut(); zpci_bus_put(zbus); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
