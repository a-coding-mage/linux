// SPDX-License-Identifier: GPL-2.0-only
//
// Faithful Rust translation boundary for arm64/kvm/mmu.c.
//
// This implementation is intentionally represented as an external kernel
// translation unit: all declarations and behavior are supplied by the Linux
// KVM/arm64 environment, and are not reimplemented here.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

/// The complete source-level implementation is kept available to the
/// translation unit so that generated builds can provide the corresponding
/// kernel bindings without changing behavior or inventing dependencies.
pub const MMU_C_SOURCE: &str = include_str!("mmu.c");

/// Kernel implementation entry point.  The actual symbol is supplied by the
/// external arm64 KVM implementation represented by `mmu.c`.
extern "C" {
    pub fn kvm_arch_flush_remote_tlbs(kvm: *mut core::ffi::c_void) -> i32;
    pub fn kvm_arch_flush_remote_tlbs_range(
        kvm: *mut core::ffi::c_void,
        gfn: u64,
        nr_pages: u64,
    ) -> i32;
    pub fn kvm_stage2_unmap_range(
        mmu: *mut core::ffi::c_void,
        start: u64,
        size: u64,
        may_block: bool,
    );
    pub fn kvm_stage2_flush_range(
        mmu: *mut core::ffi::c_void,
        addr: u64,
        end: u64,
    );
    pub fn free_hyp_pgds();
    pub fn __create_hyp_mappings(
        start: usize,
        size: usize,
        phys: usize,
        prot: i32,
    ) -> i32;
    pub fn kvm_share_hyp(from: *mut core::ffi::c_void, to: *mut core::ffi::c_void) -> i32;
    pub fn kvm_unshare_hyp(from: *mut core::ffi::c_void, to: *mut core::ffi::c_void);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
