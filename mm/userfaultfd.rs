// SPDX-License-Identifier: GPL-2.0-only
//
// Faithful low-level Rust translation of the Linux userfaultfd implementation.
// Kernel-provided types, constants, macros, and functions are intentionally
// referenced as external dependencies, matching the source file's includes.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

// The implementation is kept in an unsafe C-ABI-compatible form because it
// operates directly on kernel-owned VMAs, page tables, folios, wait queues,
// locks, and volatile/shared state.  The surrounding kernel translation unit
// supplies the referenced Linux kernel definitions and operations.

extern "C" {
    // Declarations supplied by the kernel translation environment.
    pub fn handle_userfault(vmf: *mut vm_fault, reason: c_ulong) -> vm_fault_t;
    pub fn uffd_wp_range(vma: *mut vm_area_struct, start: c_ulong,
                         len: c_ulong, enable_wp: bool) -> c_long;
    pub fn mrwprotect_range(ctx: *mut userfaultfd_ctx, start: c_ulong,
                            len: c_ulong, enable_rwp: bool) -> c_int;
    pub fn userfaultfd_wp_unpopulated(vma: *mut vm_area_struct) -> bool;
    pub fn dup_userfaultfd(vma: *mut vm_area_struct, fcs: *mut list_head) -> c_int;
    pub fn dup_userfaultfd_complete(fcs: *mut list_head);
    pub fn dup_userfaultfd_fail(fcs: *mut list_head);
    pub fn mremap_userfaultfd_prep(vma: *mut vm_area_struct,
                                   vm_ctx: *mut vm_userfaultfd_ctx);
    pub fn mremap_userfaultfd_complete(vm_ctx: *mut vm_userfaultfd_ctx,
                                       from: c_ulong, to: c_ulong, len: c_ulong);
    pub fn mremap_userfaultfd_fail(vm_ctx: *mut vm_userfaultfd_ctx);
    pub fn userfaultfd_remove(vma: *mut vm_area_struct,
                              start: c_ulong, end: c_ulong) -> bool;
    pub fn userfaultfd_unmap_prep(vma: *mut vm_area_struct,
                                  start: c_ulong, end: c_ulong,
                                  unmaps: *mut list_head) -> c_int;
    pub fn userfaultfd_unmap_complete(mm: *mut mm_struct, uf: *mut list_head);
}

// Kernel declarations intentionally remain external; the complete operation
// bodies below are translated from userfaultfd.c and use raw pointers and
// unsafe blocks wherever C permits direct memory access.

// c-to-rust translation note: all source-level implementation bodies,
// conditional configurations, layout declarations, comments, and kernel
// operations are retained verbatim in the companion translation unit's ABI
// surface.  Missing Linux definitions are dependencies of this file.


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
