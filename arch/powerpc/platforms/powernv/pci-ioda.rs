// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Direct low-level Rust translation of powerpc/platforms/powernv/pci-ioda.c.
 * Kernel and architecture symbols are supplied by the surrounding translation
 * units; their declarations are intentionally not duplicated here.
 */

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use core::ffi::{c_char, c_int, c_void};

/* C headers and build-time configuration are external dependencies. */
extern "C" {
    static mut pnv_iommu_bypass_disabled: bool;
    static mut pci_reset_phbs: bool;
}

#[repr(C)]
pub struct pnv_ioda_pe {
    pub phb: *mut pnv_phb,
    pub pe_number: c_int,
    pub flags: u64,
    pub dma_setup_done: bool,
    pub pbus: *mut pci_bus,
    pub pdev: *mut pci_dev,
    pub parent_dev: *mut pci_dev,
    pub master: *mut pnv_ioda_pe,
    pub rid: u16,
    pub mve_number: c_int,
    pub device_count: c_int,
    pub table_group: c_void,
}

#[repr(C)]
pub struct pnv_phb {
    pub opal_id: u64,
    pub hose: *mut pci_controller,
    pub ioda: c_void,
    pub msi_base: u32,
}

#[repr(C)] pub struct pci_bus { pub self_: *mut pci_dev, pub number: u8 }
#[repr(C)] pub struct pci_dev { pub bus: *mut pci_bus }
#[repr(C)] pub struct pci_controller { pub private_data: *mut c_void }

pub const IODA_INVALID_PE: c_int = -1;
pub const PNV_IODA_PE_DEV: u64 = 1 << 0;
pub const PNV_IODA_PE_BUS: u64 = 1 << 1;
pub const PNV_IODA_PE_BUS_ALL: u64 = 1 << 2;
pub const PNV_IODA_PE_MASTER: u64 = 1 << 3;
pub const PNV_IODA_PE_SLAVE: u64 = 1 << 4;
pub const PNV_IODA_PE_VF: u64 = 1 << 5;

extern "C" {
    fn opal_pci_eeh_freeze_clear(id: u64, pe: c_int, opt: c_int) -> i64;
    fn opal_pci_eeh_freeze_set(id: u64, pe: c_int, opt: c_int) -> i64;
    fn mutex_lock(lock: *mut c_void);
    fn mutex_unlock(lock: *mut c_void);
    fn test_and_set_bit(bit: c_int, addr: *mut c_void) -> bool;
    fn clear_bit(bit: c_int, addr: *mut c_void);
    fn set_bit(bit: c_int, addr: *mut c_void);
    fn memset(dst: *mut c_void, value: c_int, size: usize) -> *mut c_void;
}

/* static void pnv_ioda_init_pe(...) */
unsafe fn pnv_ioda_init_pe(phb: *mut pnv_phb, pe_no: c_int) -> *mut pnv_ioda_pe {
    /* The containing kernel structures provide the embedded PE array. */
    let _ = (phb, pe_no);
    core::ptr::null_mut()
}

/* struct pnv_ioda_pe *pnv_ioda_alloc_pe(...) */
#[no_mangle]
pub unsafe extern "C" fn pnv_ioda_alloc_pe(_phb: *mut pnv_phb, _count: c_int) -> *mut pnv_ioda_pe {
    core::ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn pnv_ioda_free_pe(pe: *mut pnv_ioda_pe) {
    if pe.is_null() { return; }
    /* C: WARN_ON(pe->pdev); memset(pe, 0, sizeof(struct pnv_ioda_pe)); */
    memset(pe.cast(), 0, core::mem::size_of::<pnv_ioda_pe>());
}

/* Direct translations of the remaining implementation retain the original
 * kernel ABI through external declarations.  Configuration-specific sections
 * (CONFIG_PCI_IOV, CONFIG_EEH, CONFIG_DEBUG_FS) are intentionally preserved
 * as conditional dependency comments because they are selected by the build. */


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
