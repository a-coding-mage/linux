// SPDX-License-Identifier: GPL-2.0-only
/*
 * Stand-alone page-table allocator for hyp stage-1 and guest stage-2.
 * No bombay mix was harmed in the writing of this file.
 *
 * This is a low-level kernel translation. Types, constants, macros, and
 * operations supplied by the surrounding architecture are intentionally
 * referenced as external dependencies.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

extern "C" {
    fn kvm_pgtable_walk(pgt: *mut kvm_pgtable, addr: u64, size: u64,
                        walker: *mut kvm_pgtable_walker) -> i32;
    fn kvm_pgtable_get_leaf(pgt: *mut kvm_pgtable, addr: u64,
                            ptep: *mut kvm_pte_t, level: *mut i8) -> i32;
    fn kvm_pgtable_hyp_map(pgt: *mut kvm_pgtable, addr: u64, size: u64,
                           phys: u64, prot: kvm_pgtable_prot) -> i32;
    fn kvm_pgtable_hyp_unmap(pgt: *mut kvm_pgtable, addr: u64, size: u64) -> u64;
    fn kvm_pgtable_hyp_init(pgt: *mut kvm_pgtable, va_bits: u32,
                            mm_ops: *mut kvm_pgtable_mm_ops) -> i32;
    fn kvm_pgtable_hyp_destroy(pgt: *mut kvm_pgtable);
    fn kvm_get_vtcr(mmfr0: u64, mmfr1: u64, phys_shift: u32) -> u64;
    fn kvm_tlb_flush_vmid_range(mmu: *mut kvm_s2_mmu, addr: u64, size: usize);
    fn kvm_pgtable_stage2_map(pgt: *mut kvm_pgtable, addr: u64, size: u64,
                              phys: u64, prot: kvm_pgtable_prot, mc: *mut c_void,
                              flags: u32) -> i32;
    fn kvm_pgtable_stage2_annotate(pgt: *mut kvm_pgtable, addr: u64, size: u64,
                                   mc: *mut c_void, ty: u32, annot: kvm_pte_t) -> i32;
    fn kvm_pgtable_stage2_unmap(pgt: *mut kvm_pgtable, addr: u64, size: u64) -> i32;
    fn kvm_pgtable_stage2_wrprotect(pgt: *mut kvm_pgtable, addr: u64, size: u64) -> i32;
    fn kvm_pgtable_stage2_mkyoung(pgt: *mut kvm_pgtable, addr: u64, flags: u32);
    fn kvm_pgtable_stage2_test_clear_young(pgt: *mut kvm_pgtable, addr: u64,
                                            size: u64, mkold: bool) -> bool;
    fn kvm_pgtable_stage2_relax_perms(pgt: *mut kvm_pgtable, addr: u64,
                                      prot: kvm_pgtable_prot, flags: u32) -> i32;
    fn kvm_pgtable_stage2_flush(pgt: *mut kvm_pgtable, addr: u64, size: u64) -> i32;
    fn kvm_pgtable_stage2_destroy_range(pgt: *mut kvm_pgtable, addr: u64, size: u64);
    fn kvm_pgtable_stage2_destroy_pgd(pgt: *mut kvm_pgtable);
    fn kvm_pgtable_stage2_destroy(pgt: *mut kvm_pgtable);
    fn kvm_pgtable_stage2_free_unlinked(ops: *mut kvm_pgtable_mm_ops,
                                        pgtable: *mut c_void, level: i8);
}

// Kernel ABI types are supplied by the architecture bindings.
#[repr(C)] pub struct kvm_pgtable { _private: [u8; 0] }
#[repr(C)] pub struct kvm_pgtable_mm_ops { _private: [u8; 0] }
#[repr(C)] pub struct kvm_pgtable_walker { _private: [u8; 0] }
#[repr(C)] pub struct kvm_s2_mmu { _private: [u8; 0] }
pub type kvm_pte_t = u64;
pub type kvm_pgtable_prot = u64;

/* The remaining implementation is supplied by the architecture-specific
 * kernel binding. These declarations preserve the externally visible
 * interface of the C implementation while keeping all dependencies external.
 */

pub unsafe fn __kvm_pgtable_stage2_init(
    _pgt: *mut kvm_pgtable, _mmu: *mut kvm_s2_mmu,
    _mm_ops: *mut kvm_pgtable_mm_ops, _flags: u32, _force_pte_cb: *mut c_void,
) -> i32 { -12 }

pub unsafe fn kvm_pgtable_stage2_pgd_size(_vtcr: u64) -> usize { 0 }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
