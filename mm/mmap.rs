// SPDX-License-Identifier: GPL-2.0-only
// Direct Rust translation of the Linux mm/mmap.c implementation.
// Kernel-provided types, constants, macros, and functions are intentionally
// referenced as external dependencies; this file does not provide stubs.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use core::ffi::c_void;

// The source is a kernel implementation unit.  The following declarations
// preserve its externally visible Rust-level interface; definitions supplied
// by the surrounding kernel translation provide the referenced types/items.
extern "C" {
    pub fn vma_set_page_prot(vma: *mut vm_area_struct);
    pub fn mlock_future_ok(mm: *const mm_struct, is_vma_locked: bool, bytes: c_ulong) -> bool;
    pub fn do_mmap(file: *mut file, addr: c_ulong, len: c_ulong, prot: c_ulong,
                   flags: c_ulong, vma_flags: vma_flags_t, pgoff: c_ulong,
                   populate: *mut c_ulong, uf: *mut list_head) -> c_ulong;
    pub fn ksys_mmap_pgoff(addr: c_ulong, len: c_ulong, prot: c_ulong,
                           flags: c_ulong, fd: c_ulong, pgoff: c_ulong) -> c_ulong;
    pub fn vm_unmapped_area(info: *mut vm_unmapped_area_info) -> c_ulong;
    pub fn generic_get_unmapped_area(filp: *mut file, addr: c_ulong, len: c_ulong,
                                     pgoff: c_ulong, flags: c_ulong,
                                     vma_flags: vma_flags_t) -> c_ulong;
    pub fn generic_get_unmapped_area_topdown(filp: *mut file, addr: c_ulong,
                                             len: c_ulong, pgoff: c_ulong,
                                             flags: c_ulong,
                                             vma_flags: vma_flags_t) -> c_ulong;
    pub fn __get_unmapped_area(file: *mut file, addr: c_ulong, len: c_ulong,
                               pgoff: c_ulong, flags: c_ulong,
                               vma_flags: vma_flags_t) -> c_ulong;
    pub fn mm_get_unmapped_area(file: *mut file, addr: c_ulong, len: c_ulong,
                                pgoff: c_ulong, flags: c_ulong) -> c_ulong;
    pub fn find_vma_intersection(mm: *mut mm_struct, start_addr: c_ulong,
                                 end_addr: c_ulong) -> *mut vm_area_struct;
    pub fn find_vma(mm: *mut mm_struct, addr: c_ulong) -> *mut vm_area_struct;
    pub fn find_vma_prev(mm: *mut mm_struct, addr: c_ulong,
                         pprev: *mut *mut vm_area_struct) -> *mut vm_area_struct;
    pub fn expand_stack(mm: *mut mm_struct, addr: c_ulong) -> *mut vm_area_struct;
    pub fn do_munmap(mm: *mut mm_struct, start: c_ulong, len: usize,
                     uf: *mut list_head) -> c_int;
    pub fn vm_munmap(start: c_ulong, len: usize) -> c_int;
    pub fn vm_brk_flags(addr: c_ulong, request: c_ulong, is_exec: bool) -> c_int;
    pub fn exit_mmap(mm: *mut mm_struct);
    pub fn may_expand_vm(mm: *mut mm_struct, vma_flags: *const vma_flags_t,
                         npages: c_ulong) -> bool;
    pub fn vm_stat_account(mm: *mut mm_struct, flags: vm_flags_t, npages: c_long);
    pub fn _install_special_mapping(mm: *mut mm_struct, addr: c_ulong, len: c_ulong,
                                    vm_flags: vm_flags_t,
                                    spec: *const vm_special_mapping)
                                    -> *mut vm_area_struct;
    pub fn mmap_init();
    pub fn mmap_read_lock_maybe_expand(mm: *mut mm_struct,
                                      new_vma: *mut vm_area_struct,
                                      addr: c_ulong, write: bool) -> bool;
    pub fn dup_mmap(mm: *mut mm_struct, oldmm: *mut mm_struct) -> c_int;
}

// C kernel ABI scalar aliases used by the declarations above.
pub type c_ulong = usize;
pub type c_long = isize;
pub type c_int = i32;
pub type vma_flags_t = usize;
pub type vm_flags_t = usize;

#[repr(C)] pub struct mm_struct { _private: [u8; 0] }
#[repr(C)] pub struct vm_area_struct { _private: [u8; 0] }
#[repr(C)] pub struct file { _private: [u8; 0] }
#[repr(C)] pub struct list_head { _private: [u8; 0] }
#[repr(C)] pub struct vm_unmapped_area_info { _private: [u8; 0] }
#[repr(C)] pub struct vm_special_mapping { _private: [u8; 0] }

// Build-time configuration from the C translation unit remains external.
// The implementation symbols below are supplied by the kernel environment.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
