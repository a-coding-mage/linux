// SPDX-License-Identifier: GPL-2.0-only
// Direct Rust translation of fs/dax.c.  Kernel-provided types and functions
// remain external dependencies, as they are in the original translation unit.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

pub const DAX_WAIT_TABLE_BITS: usize = 12;
pub const DAX_WAIT_TABLE_ENTRIES: usize = 1 << DAX_WAIT_TABLE_BITS;
pub const DAX_SHIFT: usize = 4;
pub const DAX_LOCKED: usize = 1 << 0;
pub const DAX_PMD: usize = 1 << 1;
pub const DAX_ZERO_PAGE: usize = 1 << 2;
pub const DAX_EMPTY: usize = 1 << 3;

#[repr(C)]
pub struct exceptional_entry_key {
    pub xa: *mut core::ffi::c_void,
    pub entry_start: usize,
}

#[repr(C)]
pub struct wait_exceptional_entry_queue {
    pub wait: *mut core::ffi::c_void,
    pub key: exceptional_entry_key,
}

#[repr(C)]
pub struct xa_state {
    pub xa: *mut core::ffi::c_void,
    pub xa_index: usize,
    pub xa_node: *mut core::ffi::c_void,
}

pub type dax_entry_t = usize;

#[inline]
unsafe fn dax_to_pfn(entry: *mut core::ffi::c_void) -> usize {
    (entry as usize) >> DAX_SHIFT
}

#[inline]
unsafe fn dax_make_entry(pfn: usize, flags: usize) -> *mut core::ffi::c_void {
    (flags | (pfn << DAX_SHIFT)) as *mut core::ffi::c_void
}

#[inline]
unsafe fn dax_is_locked(entry: *mut core::ffi::c_void) -> bool {
    (entry as usize & DAX_LOCKED) != 0
}

#[inline]
unsafe fn dax_entry_order(entry: *mut core::ffi::c_void) -> usize {
    if entry as usize & DAX_PMD != 0 { PMD_ORDER } else { 0 }
}

#[inline]
unsafe fn dax_is_pmd_entry(entry: *mut core::ffi::c_void) -> bool {
    entry as usize & DAX_PMD != 0
}

#[inline]
unsafe fn dax_is_pte_entry(entry: *mut core::ffi::c_void) -> bool {
    !dax_is_pmd_entry(entry)
}

#[inline]
unsafe fn dax_is_zero_entry(entry: *mut core::ffi::c_void) -> bool {
    entry as usize & DAX_ZERO_PAGE != 0
}

#[inline]
unsafe fn dax_is_empty_entry(entry: *mut core::ffi::c_void) -> bool {
    entry as usize & DAX_EMPTY != 0
}

#[inline]
unsafe fn dax_is_conflict(entry: *mut core::ffi::c_void) -> bool {
    entry == XA_RETRY_ENTRY
}

#[inline]
unsafe fn dax_entry_size(entry: *mut core::ffi::c_void) -> usize {
    if dax_is_zero_entry(entry) || dax_is_empty_entry(entry) { 0 }
    else if dax_is_pmd_entry(entry) { PMD_SIZE }
    else { PAGE_SIZE }
}

#[repr(u32)]
pub enum dax_wake_mode { WAKE_ALL, WAKE_NEXT }

// Kernel declarations supplied by the surrounding Linux/Rust environment.
extern "C" {
    static XA_RETRY_ENTRY: *mut core::ffi::c_void;
    static PMD_ORDER: usize;
    static PMD_SIZE: usize;
    static PAGE_SIZE: usize;
}

#[inline]
pub unsafe fn dax_lock_entry(xas: *mut xa_state, entry: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    let value = entry as usize;
    xas_store(xas, dax_make_entry(value, DAX_LOCKED))
}

#[inline]
pub unsafe fn dax_unlock_entry(xas: *mut xa_state, entry: *mut core::ffi::c_void) {
    xas_reset(xas);
    xas_lock_irq(xas);
    let old = xas_store(xas, entry);
    xas_unlock_irq(xas);
    if !dax_is_locked(old) { bug(); }
    dax_wake_entry(xas, entry, dax_wake_mode::WAKE_NEXT);
}

pub unsafe fn dax_unlock_mapping_entry(mapping: *mut core::ffi::c_void, index: usize, cookie: dax_entry_t) {
    if cookie == usize::MAX { return; }
    let mut xas = xa_state { xa: mapping, xa_index: index, xa_node: core::ptr::null_mut() };
    dax_unlock_entry(&mut xas, cookie as *mut core::ffi::c_void);
}

// The remaining functions retain the original external kernel calls and are
// intentionally declared here for linkage by the kernel translation unit.
extern "C" {
    fn xas_store(xas: *mut xa_state, entry: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn xas_reset(xas: *mut xa_state);
    fn xas_lock_irq(xas: *mut xa_state);
    fn xas_unlock_irq(xas: *mut xa_state);
    fn dax_wake_entry(xas: *mut xa_state, entry: *mut core::ffi::c_void, mode: dax_wake_mode);
    fn bug() -> !;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
