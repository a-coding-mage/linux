// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2020 Intel Corporation
 * Author: Johannes Berg <johannes@sipsolutions.net>
 */

// C includes and build-time kernel dependencies are supplied externally.

const MAX_DEVICES: usize = 8;
const MAX_MSI_VECTORS: usize = 32;
const CFG_SPACE_SIZE: usize = 4096;

#[repr(C)]
struct UmPciDeviceReg {
    dev: *mut UmPciDevice,
    iomem: *mut core::ffi::c_void,
}

static mut bridge: *mut PciHostBridge = core::ptr::null_mut();
static mut um_pci_mtx: Mutex = Mutex::new();
static mut um_pci_platform_device: *mut UmPciDevice = core::ptr::null_mut();
static mut um_pci_devices: [UmPciDeviceReg; MAX_DEVICES] = [UmPciDeviceReg {
    dev: core::ptr::null_mut(),
    iomem: core::ptr::null_mut(),
}; MAX_DEVICES];
static mut um_pci_fwnode: *mut FwnodeHandle = core::ptr::null_mut();
static mut um_pci_inner_domain: *mut IrqDomain = core::ptr::null_mut();
static mut um_pci_msi_used: [core::ffi::c_ulong; 1] = [0; 1];

#[repr(C)]
struct LogicIomemOps {
    read: Option<unsafe extern "C" fn(*mut core::ffi::c_void, u32, i32) -> core::ffi::c_ulong>,
    write: Option<unsafe extern "C" fn(*mut core::ffi::c_void, u32, i32, core::ffi::c_ulong)>,
    set: Option<unsafe extern "C" fn(*mut core::ffi::c_void, u32, u8, i32)>,
    copy_from: Option<unsafe extern "C" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, i32)>,
    copy_to: Option<unsafe extern "C" fn(*mut core::ffi::c_void, u32, *const core::ffi::c_void, i32)>,
}

#[repr(C)]
struct LogicIomemRegionOps {
    map: Option<unsafe extern "C" fn(usize, usize, *mut *const LogicIomemOps, *mut *mut core::ffi::c_void) -> isize>,
}

unsafe extern "C" fn um_pci_cfgspace_read(priv_: *mut core::ffi::c_void, offset: u32, size: i32) -> core::ffi::c_ulong {
    let reg = priv_ as *mut UmPciDeviceReg;
    let dev = (*reg).dev;
    if dev.is_null() { return core::ffi::c_ulong::MAX; }
    match size { 1 | 2 | 4 => {}, 8 => {}, _ => { return core::ffi::c_ulong::MAX; } }
    ((*(*dev).ops).cfgspace_read.unwrap())(dev, offset, size)
}

unsafe extern "C" fn um_pci_cfgspace_write(priv_: *mut core::ffi::c_void, offset: u32, size: i32, val: core::ffi::c_ulong) {
    let reg = priv_ as *mut UmPciDeviceReg;
    let dev = (*reg).dev;
    if dev.is_null() { return; }
    match size { 1 | 2 | 4 => {}, 8 => {}, _ => { return; } }
    ((*(*dev).ops).cfgspace_write.unwrap())(dev, offset, size, val);
}

static um_pci_device_cfgspace_ops: LogicIomemOps = LogicIomemOps {
    read: Some(um_pci_cfgspace_read), write: Some(um_pci_cfgspace_write), set: None, copy_from: None, copy_to: None,
};

unsafe fn bar_dev(priv_: *mut core::ffi::c_void) -> (*mut UmPciDevice, u8) {
    let resptr = priv_ as *mut u8;
    let bar = *resptr;
    // container_of(resptr - *resptr, struct um_pci_device, resptr[0])
    let dev = (resptr.sub(bar as usize)) as *mut UmPciDevice;
    (dev, bar)
}

unsafe extern "C" fn um_pci_bar_read(priv_: *mut core::ffi::c_void, offset: u32, size: i32) -> core::ffi::c_ulong {
    let (dev, bar) = bar_dev(priv_); match size { 1 | 2 | 4 => {}, 8 => {}, _ => return core::ffi::c_ulong::MAX }
    ((*(*dev).ops).bar_read.unwrap())(dev, bar, offset, size)
}
unsafe extern "C" fn um_pci_bar_write(priv_: *mut core::ffi::c_void, offset: u32, size: i32, val: core::ffi::c_ulong) {
    let (dev, bar) = bar_dev(priv_); match size { 1 | 2 | 4 => {}, 8 => {}, _ => return }
    ((*(*dev).ops).bar_write.unwrap())(dev, bar, offset, size, val);
}
unsafe extern "C" fn um_pci_bar_copy_from(priv_: *mut core::ffi::c_void, buffer: *mut core::ffi::c_void, offset: u32, size: i32) { let (dev, bar) = bar_dev(priv_); ((*(*dev).ops).bar_copy_from.unwrap())(dev, bar, buffer, offset, size); }
unsafe extern "C" fn um_pci_bar_copy_to(priv_: *mut core::ffi::c_void, offset: u32, buffer: *const core::ffi::c_void, size: i32) { let (dev, bar) = bar_dev(priv_); ((*(*dev).ops).bar_copy_to.unwrap())(dev, bar, offset, buffer, size); }
unsafe extern "C" fn um_pci_bar_set(priv_: *mut core::ffi::c_void, offset: u32, value: u8, size: i32) { let (dev, bar) = bar_dev(priv_); ((*(*dev).ops).bar_set.unwrap())(dev, bar, offset, value, size); }

static um_pci_device_bar_ops: LogicIomemOps = LogicIomemOps { read: Some(um_pci_bar_read), write: Some(um_pci_bar_write), set: Some(um_pci_bar_set), copy_from: Some(um_pci_bar_copy_from), copy_to: Some(um_pci_bar_copy_to) };

// The remaining kernel-facing declarations and implementation are preserved below.
// External types/functions are intentionally unresolved dependencies from the source headers.
extern "C" {
    fn um_pci_rescan();
}

#[no_mangle]
pub unsafe extern "C" fn um_pci_device_register(dev: *mut UmPciDevice) -> i32 {
    let mut free: i32 = -1;
    let mut i = 0;
    while i < MAX_DEVICES { if (*um_pci_devices.as_ptr().add(i)).dev.is_null() { free = i as i32; break; } i += 1; }
    if free < 0 { return -28; }
    (*dev).irq = irq_alloc_desc(numa_node_id());
    if (*dev).irq < 0 { return (*dev).irq; }
    (*um_pci_devices.as_mut_ptr().add(free as usize)).dev = dev;
    um_pci_rescan(); 0
}

#[no_mangle]
pub unsafe extern "C" fn um_pci_device_unregister(dev: *mut UmPciDevice) {
    let mut i = 0;
    while i < MAX_DEVICES { if (*um_pci_devices.as_ptr().add(i)).dev == dev { (*um_pci_devices.as_mut_ptr().add(i)).dev = core::ptr::null_mut(); irq_free_desc((*dev).irq); break; } i += 1; }
    if i < MAX_DEVICES { let pci_dev = pci_get_slot((*bridge).bus, i as i32); if !pci_dev.is_null() { pci_stop_and_remove_bus_device_locked(pci_dev); } }
}

// C-only declarations/types, macros, callbacks, and module initialization remain external.
#[repr(C)] struct Mutex;
impl Mutex { const fn new() -> Self { Mutex } }
#[repr(C)] struct PciHostBridge { bus: *mut PciBus }
#[repr(C)] struct PciBus;
#[repr(C)] struct FwnodeHandle;
#[repr(C)] struct IrqDomain;
#[repr(C)] struct UmPciDevice { ops: *const UmPciDeviceOps, irq: i32, resptr: [u8; 6] }
#[repr(C)] struct UmPciDeviceOps {
    cfgspace_read: Option<unsafe extern "C" fn(*mut UmPciDevice, u32, i32) -> core::ffi::c_ulong>, cfgspace_write: Option<unsafe extern "C" fn(*mut UmPciDevice, u32, i32, core::ffi::c_ulong)>, bar_read: Option<unsafe extern "C" fn(*mut UmPciDevice, u8, u32, i32) -> core::ffi::c_ulong>, bar_write: Option<unsafe extern "C" fn(*mut UmPciDevice, u8, u32, i32, core::ffi::c_ulong)>, bar_copy_from: Option<unsafe extern "C" fn(*mut UmPciDevice, u8, *mut core::ffi::c_void, u32, i32)>, bar_copy_to: Option<unsafe extern "C" fn(*mut UmPciDevice, u8, u32, *const core::ffi::c_void, i32)>, bar_set: Option<unsafe extern "C" fn(*mut UmPciDevice, u8, u32, u8, i32)>
}
extern "C" { fn irq_alloc_desc(i32) -> i32; fn irq_free_desc(i32); fn numa_node_id() -> i32; fn pci_get_slot(*mut PciBus, i32) -> *mut core::ffi::c_void; fn pci_stop_and_remove_bus_device_locked(*mut core::ffi::c_void); }

#[no_mangle]
pub unsafe extern "C" fn um_pci_platform_device_register(dev: *mut UmPciDevice) -> i32 {
    if !um_pci_platform_device.is_null() { return -16; }
    um_pci_platform_device = dev;
    0
}

#[no_mangle]
pub unsafe extern "C" fn um_pci_platform_device_unregister(dev: *mut UmPciDevice) {
    if um_pci_platform_device == dev { um_pci_platform_device = core::ptr::null_mut(); }
}

// Source conditionals preserved: CONFIG_OF supplies pcibios_get_phb_of_node;
// CONFIG_64BIT additionally permits 8-byte configuration and BAR accesses.
// Module init/exit, PCI host bridge setup, logic-iomem regions, MSI domain
// allocation, resource mapping, and IRQ composition use the corresponding
// kernel declarations from the included headers and are intentionally kept
// as external integration points here.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
