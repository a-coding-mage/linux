/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * c 2001 PPC 64 Team, IBM Corp
 */

/* Translated from the C header. The original declarations are kernel- and
 * configuration-dependent; referenced types and symbols are supplied by
 * other headers/modules. */

use core::ffi::{c_int, c_ulong, c_void};

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pci_dn {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pci_dev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pci_controller {
    _private: [u8; 0],
}

#[repr(C)]
pub struct eeh_dev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct eeh_pe {
    _private: [u8; 0],
}

#[repr(C)]
pub struct list_head {
    _private: [u8; 0],
}

/* Bus Unit ID macros; get low and hi 32-bits of the 64-bit BUID */
#[inline]
pub const fn buid_hi(buid: u64) -> u32 {
    (buid >> 32) as u32
}

#[inline]
pub const fn buid_lo(buid: u64) -> u32 {
    buid as u32
}

/* PCI device_node operations */
pub type PciTraverseDeviceNodesFn =
    unsafe extern "C" fn(*mut device_node, *mut c_void) -> *mut c_void;

extern "C" {
    pub static mut isa_io_base: c_ulong;
    pub static mut hose_list: list_head;
    pub static mut isa_bridge_pcidev: *mut pci_dev; /* may be NULL if no ISA bus */

    pub fn pci_traverse_device_nodes(
        start: *mut device_node,
        function: Option<PciTraverseDeviceNodesFn>,
        data: *mut c_void,
    ) -> *mut c_void;
    pub fn pci_devs_phb_init_dynamic(phb: *mut pci_controller);

    /* From rtas_pci.h */
    pub fn init_pci_config_tokens();
    pub fn get_phb_buid(node: *mut device_node) -> c_ulong;
    pub fn rtas_setup_phb(phb: *mut pci_controller) -> c_int;

    pub fn rtas_pci_dn_read_config(
        pdn: *mut pci_dn,
        where_: c_int,
        size: c_int,
        val: *mut u32,
    ) -> c_int;
    pub fn rtas_pci_dn_write_config(
        pdn: *mut pci_dn,
        where_: c_int,
        size: c_int,
        val: u32,
    ) -> c_int;

    pub fn eeh_addr_cache_insert_dev(dev: *mut pci_dev);
    pub fn eeh_addr_cache_rmv_dev(dev: *mut pci_dev);
    pub fn eeh_addr_cache_get_dev(addr: c_ulong) -> *mut eeh_dev;
    pub fn eeh_slot_error_detail(pe: *mut eeh_pe, severity: c_int);
    pub fn eeh_pci_enable(pe: *mut eeh_pe, function: c_int) -> c_int;
    pub fn eeh_pe_reset_full(pe: *mut eeh_pe, include_passed: bool) -> c_int;
    pub fn eeh_save_bars(edev: *mut eeh_dev);
    pub fn eeh_pe_state_mark(pe: *mut eeh_pe, state: c_int);
    pub fn eeh_pe_mark_isolated(pe: *mut eeh_pe);
    pub fn eeh_pe_state_clear(pe: *mut eeh_pe, state: c_int, include_passed: bool);
    pub fn eeh_pe_state_mark_with_cfg(pe: *mut eeh_pe, state: c_int);
    pub fn eeh_pe_dev_mode_mark(pe: *mut eeh_pe, mode: c_int);
    pub fn eeh_sysfs_add_device(pdev: *mut pci_dev);
    pub fn eeh_sysfs_remove_device(pdev: *mut pci_dev);

    pub fn uli_init();
}

#[inline]
pub const fn pci_busno(bdfn: u32) -> u32 {
    (bdfn >> 8) & 0xff
}

/* CONFIG_IOMMU_API and platform configuration determine whether these are
 * external functions or empty inline functions in the original header. */
#[inline]
pub unsafe fn ppc_iommu_register_device(_phb: *mut pci_controller) {}

#[inline]
pub unsafe fn ppc_iommu_unregister_device(_phb: *mut pci_controller) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
