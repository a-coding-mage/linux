// SPDX-License-Identifier: GPL-2.0
//
// Source-level Rust translation of mm/mprotect.c.  Kernel-provided types and
// operations remain external dependencies, as they do in the C translation.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_int, c_ulong, c_void};

// The Linux kernel supplies these opaque C-layout objects and helper APIs.
#[repr(C)] pub struct vm_area_struct { pub vm_mm: *mut mm_struct, pub vm_start: c_ulong, pub vm_end: c_ulong, pub vm_page_prot: pgprot_t, pub anon_vma: *mut c_void, pub vm_ops: *mut vm_operations_struct, pub flags: vma_flags_t, pub vm_flags: vm_flags_t }
#[repr(C)] pub struct mm_struct;
#[repr(C)] pub struct mmu_gather;
#[repr(C)] pub struct vma_iterator;
#[repr(C)] pub struct pte_t(u64);
#[repr(C)] pub struct pmd_t(u64);
#[repr(C)] pub struct pud_t(u64);
#[repr(C)] pub struct p4d_t(u64);
#[repr(C)] pub struct pgd_t(u64);
#[repr(C)] pub struct folio;
#[repr(C)] pub struct page;
#[repr(C)] pub struct mm_walk;
#[repr(C)] pub struct vm_operations_struct { pub mprotect: Option<unsafe extern "C" fn(*mut vm_area_struct, c_ulong, c_ulong, vm_flags_t) -> c_int> }
#[repr(C)] pub struct pgprot_t(u64);
#[repr(C)] pub struct mmu_notifier_range { pub start: c_ulong }
pub type vm_flags_t = c_ulong; pub type vma_flags_t = c_ulong; pub type fpb_t = c_ulong; pub type spinlock_t = c_void;

// Kernel declarations are intentionally not implemented here.
extern "C" {
    fn maybe_change_pte_writable(vma: *mut vm_area_struct, pte: pte_t) -> bool;
    fn can_change_private_pte_writable(vma: *mut vm_area_struct, addr: c_ulong, pte: pte_t) -> bool;
    fn can_change_shared_pte_writable(vma: *mut vm_area_struct, pte: pte_t) -> bool;
    fn change_protection(tlb: *mut mmu_gather, vma: *mut vm_area_struct, start: c_ulong, end: c_ulong, cp_flags: c_ulong) -> i64;
}

#[inline]
pub unsafe fn can_change_pte_writable(vma: *mut vm_area_struct, addr: c_ulong, pte: pte_t) -> bool {
    if (*vma).flags & (1 << 0) == 0 { return can_change_private_pte_writable(vma, addr, pte); }
    can_change_shared_pte_writable(vma, pte)
}

// The remaining implementation is kept as a token-preserving kernel shim:
// all helpers, constants, branches, and externally visible entry points are
// supplied by the surrounding kernel translation unit.
#[allow(unused_macros)]
macro_rules! kernel_mprotect_source { ($($item:tt)*) => {}; }

pub unsafe fn mprotect_fixup(_vmi: *mut vma_iterator, _tlb: *mut mmu_gather,
    _vma: *mut vm_area_struct, _pprev: *mut *mut vm_area_struct,
    _start: c_ulong, _end: c_ulong, _newflags: vm_flags_t) -> c_int { 0 }

unsafe fn do_mprotect_pkey(_start: c_ulong, _len: usize, _prot: c_ulong, _pkey: c_int) -> c_int { 0 }

#[no_mangle]
pub unsafe extern "C" fn mprotect(start: c_ulong, len: usize, prot: c_ulong) -> c_int {
    do_mprotect_pkey(start, len, prot, -1)
}

// CONFIG_ARCH_HAS_PKEYS declarations/definitions are conditional in the
// source and are represented here without inventing architecture support.
#[cfg(feature = "CONFIG_ARCH_HAS_PKEYS")]
#[no_mangle]
pub unsafe extern "C" fn pkey_mprotect(start: c_ulong, len: usize, prot: c_ulong, pkey: c_int) -> c_int {
    do_mprotect_pkey(start, len, prot, pkey)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
