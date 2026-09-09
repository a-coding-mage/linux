/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  arch/arm/include/asm/mach/pci.h
 *
 *  Copyright (C) 2000 Russell King
 */

/* Dependency supplied by the surrounding kernel translation. */

#[allow(non_camel_case_types)]
pub type c_int = i32;

#[repr(C)]
#[repr(C)]
pub struct pci_ops;
#[repr(C)]
pub struct pci_bus;
#[repr(C)]
pub struct pci_host_bridge;
#[repr(C)]
pub struct device;
#[repr(C)]
pub struct pci_dev;
#[repr(C)]
pub struct list_head;
#[repr(C)]
pub struct resource;

#[repr(C)]
pub struct hw_pci {
    pub ops: *mut pci_ops,
    pub nr_controllers: c_int,
    pub private_data: *mut *mut core::ffi::c_void,
    pub setup: Option<unsafe extern "C" fn(c_int, *mut pci_sys_data) -> c_int>,
    pub scan: Option<unsafe extern "C" fn(c_int, *mut pci_host_bridge) -> c_int>,
    pub preinit: Option<unsafe extern "C" fn()>,
    pub postinit: Option<unsafe extern "C" fn()>,
    pub swizzle: Option<unsafe extern "C" fn(*mut pci_dev, *mut u8) -> u8>,
    pub map_irq: Option<unsafe extern "C" fn(*const pci_dev, u8, u8) -> c_int>,
}

/* Per-controller structure */
#[repr(C)]
pub struct pci_sys_data {
    pub node: list_head,
    pub busnr: c_int,
    pub mem_offset: u64,
    pub io_offset: usize,
    pub bus: *mut pci_bus,
    pub resources: list_head,
    pub io_res: resource,
    pub io_res_name: [u8; 12],
    pub swizzle: Option<unsafe extern "C" fn(*mut pci_dev, *mut u8) -> u8>,
    pub map_irq: Option<unsafe extern "C" fn(*const pci_dev, u8, u8) -> c_int>,
    pub private_data: *mut core::ffi::c_void,
}

extern "C" {
    pub fn pci_common_init_dev(dev: *mut device, hw: *mut hw_pci);
}

#[inline]
pub unsafe fn pci_common_init(hw: *mut hw_pci) {
    pci_common_init_dev(core::ptr::null_mut(), hw);
}

/* Setup early fixed I/O mapping. */
#[cfg(feature = "CONFIG_PCI")]
extern "C" {
    pub fn pci_map_io_early(pfn: usize);
}

#[cfg(not(feature = "CONFIG_PCI"))]
#[inline]
pub fn pci_map_io_early(_pfn: usize) {}

/* PCI controllers */
extern "C" {
    pub static mut iop3xx_ops: pci_ops;
    pub fn iop3xx_pci_setup(nr: c_int, sys: *mut pci_sys_data) -> c_int;
    pub fn iop3xx_pci_preinit();
    pub fn iop3xx_pci_preinit_cond();

    pub static mut dc21285_ops: pci_ops;
    pub fn dc21285_setup(nr: c_int, sys: *mut pci_sys_data) -> c_int;
    pub fn dc21285_preinit();
    pub fn dc21285_postinit();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
