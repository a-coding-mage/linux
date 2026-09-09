/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2019 Western Digital Corporation or its affiliates.
 * Copyright (c) 2025 Ventana Micro Systems Inc.
 */

/* Translated from the C header; declarations supplied by included headers remain external. */

#[repr(C)]
pub struct kvm_gstage {
    pub kvm: *mut kvm,
    pub flags: ::core::ffi::c_ulong,
    pub vmid: ::core::ffi::c_ulong,
    pub pgd: *mut pgd_t,
    pub pgd_levels: ::core::ffi::c_ulong,
}

pub const KVM_GSTAGE_FLAGS_LOCAL: ::core::ffi::c_ulong = 1 << 0;

#[repr(C)]
pub struct kvm_gstage_mapping {
    pub addr: gpa_t,
    pub pte: pte_t,
    pub level: u32,
}

#[cfg(target_pointer_width = "64")]
pub const kvm_riscv_gstage_index_bits: ::core::ffi::c_ulong = 9;
#[cfg(not(target_pointer_width = "64"))]
pub const kvm_riscv_gstage_index_bits: ::core::ffi::c_ulong = 10;

extern "C" {
    pub static mut kvm_riscv_gstage_max_pgd_levels: ::core::ffi::c_ulong;
}

pub const kvm_riscv_gstage_pgd_xbits: ::core::ffi::c_ulong = 2;
pub const kvm_riscv_gstage_pgd_size: ::core::ffi::c_ulong =
    1 << (HGATP_PAGE_SHIFT + kvm_riscv_gstage_pgd_xbits);

#[inline]
pub const unsafe fn kvm_riscv_gstage_gpa_bits(pgd_levels: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong {
    HGATP_PAGE_SHIFT + pgd_levels * kvm_riscv_gstage_index_bits + kvm_riscv_gstage_pgd_xbits
}

#[inline]
pub const unsafe fn kvm_riscv_gstage_gpa_size(pgd_levels: ::core::ffi::c_ulong) -> u64 {
    1u64 << kvm_riscv_gstage_gpa_bits(pgd_levels)
}

extern "C" {
    pub fn kvm_riscv_gstage_get_leaf(
        gstage: *mut kvm_gstage, addr: gpa_t, ptepp: *mut *mut pte_t, ptep_level: *mut u32,
    ) -> bool;
    pub fn kvm_riscv_gstage_set_pte(
        gstage: *mut kvm_gstage,
        pcache: *mut kvm_mmu_memory_cache,
        map: *const kvm_gstage_mapping,
    ) -> i32;
    pub fn kvm_riscv_gstage_try_update_pte(
        gstage: *mut kvm_gstage, level: u32, addr: gpa_t, ptep: *mut pte_t,
        old_pte: pte_t, new_pte: pte_t,
    ) -> bool;
    pub fn kvm_riscv_gstage_map_page(
        gstage: *mut kvm_gstage, pcache: *mut kvm_mmu_memory_cache, gpa: gpa_t,
        hpa: phys_addr_t, page_size: ::core::ffi::c_ulong, page_rdonly: bool,
        page_exec: bool, out_map: *mut kvm_gstage_mapping,
    ) -> i32;
    pub fn kvm_riscv_gstage_split_huge(
        gstage: *mut kvm_gstage, pcache: *mut kvm_mmu_memory_cache, addr: gpa_t,
        target_level: u32, flush: bool,
    ) -> bool;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum kvm_riscv_gstage_op {
    GSTAGE_OP_NOP = 0,
    GSTAGE_OP_CLEAR,
    GSTAGE_OP_WP,
}

extern "C" {
    pub fn kvm_riscv_gstage_op_pte(
        gstage: *mut kvm_gstage, addr: gpa_t, ptep: *mut pte_t, ptep_level: u32,
        op: kvm_riscv_gstage_op,
    ) -> bool;
    pub fn kvm_riscv_gstage_unmap_range(
        gstage: *mut kvm_gstage, start: gpa_t, size: gpa_t, may_block: bool,
    ) -> bool;
    pub fn kvm_riscv_gstage_wp_range(gstage: *mut kvm_gstage, start: gpa_t, end: gpa_t) -> bool;
    pub fn kvm_riscv_gstage_wp_pt_masked(
        gstage: *mut kvm_gstage, base_gfn: gfn_t, mask: ::core::ffi::c_ulong,
    ) -> bool;
    pub fn kvm_riscv_gstage_mode_detect();
}

#[inline]
pub unsafe fn kvm_riscv_gstage_mode(pgd_levels: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong {
    match pgd_levels {
        2 => HGATP_MODE_SV32X4,
        3 => HGATP_MODE_SV39X4,
        4 => HGATP_MODE_SV48X4,
        5 => HGATP_MODE_SV57X4,
        _ => {
            WARN_ON_ONCE(1);
            HGATP_MODE_OFF
        }
    }
}

#[inline]
pub unsafe fn kvm_riscv_gstage_init(gstage: *mut kvm_gstage, kvm: *mut kvm) {
    (*gstage).kvm = kvm;
    (*gstage).flags = 0;
    (*gstage).vmid = (*kvm).arch.vmid.vmid;
    (*gstage).pgd = (*kvm).arch.pgd;
    (*gstage).pgd_levels = (*kvm).arch.pgd_levels;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
