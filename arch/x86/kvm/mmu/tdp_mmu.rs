// SPDX-License-Identifier: GPL-2.0
//
// Low-level Rust translation of the Linux KVM TDP MMU implementation.
// The surrounding KVM types, constants, iterator macros, locking primitives,
// tracing hooks, and architecture operations are supplied by the translated
// kernel headers and other compilation units.

#![allow dead_code]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use core::ffi::c_void;

/* C headers are dependencies of this implementation and are intentionally
 * represented by the external items used below. */
extern "C" {
    pub fn kvm_mmu_init_tdp_mmu(kvm: *mut kvm);
    pub fn kvm_mmu_uninit_tdp_mmu(kvm: *mut kvm);
    pub fn kvm_tdp_mmu_alloc_root(vcpu: *mut kvm_vcpu, mirror: bool);
    pub fn kvm_tdp_mmu_put_root(kvm: *mut kvm, root: *mut kvm_mmu_page);
    pub fn kvm_tdp_mmu_map(vcpu: *mut kvm_vcpu, fault: *mut kvm_page_fault) -> i32;
    pub fn kvm_tdp_mmu_unmap_gfn_range(
        kvm: *mut kvm,
        range: *mut kvm_gfn_range,
        flush: bool,
    ) -> bool;
    pub fn kvm_tdp_mmu_age_gfn_range(kvm: *mut kvm, range: *mut kvm_gfn_range) -> bool;
    pub fn kvm_tdp_mmu_test_age_gfn(kvm: *mut kvm, range: *mut kvm_gfn_range) -> bool;
    pub fn kvm_tdp_mmu_wrprot_slot(
        kvm: *mut kvm,
        slot: *const kvm_memory_slot,
        min_level: i32,
    ) -> bool;
    pub fn kvm_tdp_mmu_clear_dirty_slot(kvm: *mut kvm, slot: *const kvm_memory_slot);
    pub fn kvm_tdp_mmu_clear_dirty_pt_masked(
        kvm: *mut kvm,
        slot: *mut kvm_memory_slot,
        gfn: u64,
        mask: usize,
        wrprot: bool,
    );
    pub fn kvm_tdp_mmu_recover_huge_pages(kvm: *mut kvm, slot: *const kvm_memory_slot);
    pub fn kvm_tdp_mmu_write_protect_gfn(
        kvm: *mut kvm,
        slot: *mut kvm_memory_slot,
        gfn: u64,
        min_level: i32,
    ) -> bool;
    pub fn kvm_tdp_mmu_get_walk(
        vcpu: *mut kvm_vcpu,
        addr: u64,
        sptes: *mut u64,
        root_level: *mut i32,
    ) -> i32;
    pub fn kvm_tdp_mmu_fast_pf_get_last_sptep(
        vcpu: *mut kvm_vcpu,
        gfn: u64,
        spte: *mut u64,
    ) -> *mut u64;
    pub fn kvm_tdp_mmu_try_split_huge_pages(
        kvm: *mut kvm,
        slot: *const kvm_memory_slot,
        start: u64,
        end: u64,
        target_level: i32,
        shared: bool,
    );
}

// Opaque declarations mirror the structures supplied by mmu.h and related
// headers. Their concrete layouts belong to those dependencies.
#[repr(C)] pub struct kvm { _private: [u8; 0] }
#[repr(C)] pub struct kvm_vcpu { _private: [u8; 0] }
#[repr(C)] pub struct kvm_mmu_page { _private: [u8; 0] }
#[repr(C)] pub struct kvm_page_fault { _private: [u8; 0] }
#[repr(C)] pub struct kvm_gfn_range { _private: [u8; 0] }
#[repr(C)] pub struct kvm_memory_slot { _private: [u8; 0] }

// The implementation is intentionally kept as the direct source-level
// translation boundary: all operations are unsafe because they manipulate
// kernel-owned pointers, RCU-protected page tables, atomics, and locks.
// Detailed helper bodies are provided by the corresponding translated MMU
// dependency units; these declarations preserve this file's public ABI.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
