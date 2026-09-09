/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * drmem.h: Power specific logical memory block representation
 *
 * Copyright 2017 IBM Corporation
 */

// Dependency supplied by the Linux scheduler headers.
unsafe extern "C" {
    fn cond_resched();
}

#[repr(C)]
pub struct drmem_lmb {
    pub base_addr: u64,
    pub drc_index: u32,
    pub aa_index: u32,
    pub flags: u32,
}

#[repr(C)]
pub struct drmem_lmb_info {
    pub lmbs: *mut drmem_lmb,
    pub n_lmbs: i32,
    pub lmb_size: u64,
}

pub enum device_node {}
pub enum property {}

unsafe extern "C" {
    pub static mut drmem_info: *mut drmem_lmb_info;
}

pub unsafe fn drmem_lmb_next(
    mut lmb: *mut drmem_lmb,
    start: *const drmem_lmb,
) -> *mut drmem_lmb {
    /*
     * DLPAR code paths can take several milliseconds per element
     * when interacting with firmware. Ensure that we don't
     * unfairly monopolize the CPU.
     */
    lmb = lmb.add(1);
    if (lmb.offset_from(start) as usize) % 16 == 0 {
        cond_resched();
    }

    lmb
}

/*
 * C macro equivalents:
 * for_each_drmem_lmb_in_range(lmb, start, end):
 *   for ((lmb) = (start); (lmb) < (end); lmb = drmem_lmb_next(lmb, start))
 * for_each_drmem_lmb(lmb): iterates from drmem_info->lmbs[0] through
 *   drmem_info->lmbs[drmem_info->n_lmbs].
 */

/*
 * The of_drconf_cell_v1 struct defines the layout of the LMB data
 * specified in the ibm,dynamic-memory device tree property.
 * The property itself is a 32-bit value specifying the number of
 * LMBs followed by an array of of_drconf_cell_v1 entries, one
 * per LMB.
 */
#[repr(C)]
pub struct of_drconf_cell_v1 {
    pub base_addr: u64, // __be64
    pub drc_index: u32, // __be32
    pub reserved: u32,  // __be32
    pub aa_index: u32,  // __be32
    pub flags: u32,     // __be32
}

/*
 * Version 2 of the ibm,dynamic-memory property is defined as a
 * 32-bit value specifying the number of LMB sets followed by an
 * array of of_drconf_cell_v2 entries, one per LMB set.
 */
#[repr(C, packed)]
pub struct of_drconf_cell_v2 {
    pub seq_lmbs: u32,
    pub base_addr: u64,
    pub drc_index: u32,
    pub aa_index: u32,
    pub flags: u32,
}

pub const DRCONF_MEM_ASSIGNED: u32 = 0x00000008;
pub const DRCONF_MEM_AI_INVALID: u32 = 0x00000040;
pub const DRCONF_MEM_RESERVED: u32 = 0x00000080;
pub const DRCONF_MEM_HOTREMOVABLE: u32 = 0x00000100;

pub unsafe fn drmem_lmb_size() -> u64 {
    (*drmem_info).lmb_size
}

pub const DRMEM_LMB_RESERVED: u32 = 0x80000000;

pub unsafe fn drmem_mark_lmb_reserved(lmb: *mut drmem_lmb) {
    (*lmb).flags |= DRMEM_LMB_RESERVED;
}

pub unsafe fn drmem_remove_lmb_reservation(lmb: *mut drmem_lmb) {
    (*lmb).flags &= !DRMEM_LMB_RESERVED;
}

pub unsafe fn drmem_lmb_reserved(lmb: *mut drmem_lmb) -> bool {
    ((*lmb).flags & DRMEM_LMB_RESERVED) != 0
}

unsafe extern "C" {
    pub fn drmem_lmb_memory_max() -> u64;
    pub fn walk_drmem_lmbs(
        dn: *mut device_node,
        data: *mut core::ffi::c_void,
        func: Option<unsafe extern "C" fn(
            *mut drmem_lmb,
            *const *const u32,
            *mut core::ffi::c_void,
        ) -> i32>,
    ) -> i32;
    pub fn drmem_update_dt() -> i32;

    #[cfg(CONFIG_PPC_PSERIES)]
    pub fn walk_drmem_lmbs_early(
        node: core::ffi::c_ulong,
        data: *mut core::ffi::c_void,
        func: Option<unsafe extern "C" fn(
            *mut drmem_lmb,
            *const *const u32,
            *mut core::ffi::c_void,
        ) -> i32>,
    ) -> i32;

    #[cfg(CONFIG_PPC_PSERIES)]
    pub fn drmem_update_lmbs(prop: *mut property);
}

pub unsafe fn invalidate_lmb_associativity_index(lmb: *mut drmem_lmb) {
    (*lmb).aa_index = 0xffffffff;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
