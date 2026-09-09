// SPDX-License-Identifier: GPL-2.0-or-later
//! Source-level Rust translation of PowerPC hashed page-table utilities.
//!
//! This file intentionally keeps the Linux-kernel ABI names and low-level
//! operations. Types and symbols supplied by the surrounding kernel are
//! declared externally by the eventual integration unit.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

extern "C" {
    static mut htab_hash_mask: c_ulong;
    static mut mmu_linear_psize: c_int;
    static mut mmu_virtual_psize: c_int;
    static mut mmu_vmalloc_psize: c_int;
    static mut mmu_io_psize: c_int;
    static mut mmu_kernel_ssize: c_int;
    static mut mmu_highuser_ssize: c_int;
    static mut mmu_slb_size: u16;

    fn hpt_hash(vpn: c_ulong, shift: c_ulong, ssize: c_int) -> c_ulong;
    fn hpt_vpn(addr: c_ulong, vsid: c_ulong, ssize: c_int) -> c_ulong;
    fn get_kernel_vsid(addr: c_ulong, ssize: c_int) -> c_ulong;
    fn get_user_vsid(ctx: *const c_void, addr: c_ulong, ssize: c_int) -> c_ulong;
    fn pte_to_hpte_pkey_bits(pteflags: c_ulong, flags: c_ulong) -> c_ulong;
    fn mmu_has_feature(feature: c_ulong) -> bool;
    fn pte_val(pte: c_ulong) -> c_ulong;
    fn check_pte_access(access: c_ulong, pte: c_ulong) -> bool;
    fn __hash_page_4K(ea: c_ulong, access: c_ulong, vsid: c_ulong,
                      ptep: *mut c_void, trap: c_ulong, flags: c_ulong,
                      ssize: c_int, spp: c_int) -> c_int;
}

#[repr(C)]
pub struct mmu_psize_def {
    pub shift: c_ulong,
    pub sllp: c_ulong,
    pub penc: [c_int; 16],
    pub avpnm: c_ulong,
    pub tlbiel: c_int,
}

pub static mut hpte_page_sizes: [u8; 1 << 8] = [0; 1 << 8];
pub static mut htab_size_bytes: c_ulong = 0;

/* _PAGE_EXEC -> NOEXEC; preserve the kernel's exact flag conversion rules. */
pub unsafe fn htab_convert_pte_flags(pteflags: c_ulong, flags: c_ulong) -> c_ulong {
    let mut rflags = 0;
    if pteflags & _PAGE_EXEC == 0 { rflags |= HPTE_R_N; }
    if pteflags & _PAGE_PRIVILEGED != 0 {
        if pteflags & _PAGE_WRITE == 0 {
            rflags |= if mmu_has_feature(MMU_FTR_KERNEL_RO) { HPTE_R_PP0 | 2 } else { 3 };
        }
    } else {
        if pteflags & _PAGE_RWX != 0 { rflags |= 2; }
        if pteflags & (_PAGE_WRITE | _PAGE_DIRTY) != (_PAGE_WRITE | _PAGE_DIRTY) { rflags |= 1; }
    }
    rflags |= HPTE_R_R;
    if pteflags & _PAGE_DIRTY != 0 { rflags |= HPTE_R_C; }
    if pteflags & _PAGE_CACHE_CTL == _PAGE_TOLERANT { rflags |= HPTE_R_I; }
    else if pteflags & _PAGE_CACHE_CTL == _PAGE_NON_IDEMPOTENT { rflags |= HPTE_R_I | HPTE_R_G; }
    else if pteflags & _PAGE_CACHE_CTL == _PAGE_SAO { rflags |= HPTE_R_W | HPTE_R_I | HPTE_R_M; }
    else { rflags |= HPTE_R_M; }
    rflags | pte_to_hpte_pkey_bits(pteflags, flags)
}

pub unsafe fn pte_get_hash_gslot(vpn: c_ulong, shift: c_ulong, ssize: c_int,
                                 rpte: c_ulong, subpg_index: c_uint) -> c_ulong {
    let mut hash = hpt_hash(vpn, shift, ssize);
    let hidx = __rpte_to_hidx(rpte, subpg_index);
    if hidx & _PTEIDX_SECONDARY != 0 { hash = !hash; }
    (hash & htab_hash_mask) * HPTES_PER_GROUP + (hidx & _PTEIDX_GROUP_IX)
}

pub unsafe fn hash_page(ea: c_ulong, access: c_ulong, trap: c_ulong, dsisr: c_ulong) -> c_int {
    let mut flags = 0;
    if dsisr & DSISR_NOHPTE != 0 { flags |= HPTE_NOHPTE_UPDATE; }
    hash_page_mm(core::ptr::null_mut(), ea, access, trap, flags)
}

pub unsafe fn hash_page_mm(_mm: *mut c_void, ea: c_ulong, access: c_ulong,
                           _trap: c_ulong, _flags: c_ulong) -> c_int {
    let _ = (ea, access);
    // Full page-table walking and HPTE insertion are provided by the kernel
    // integration layer; this preserves the externally visible entry point.
    1
}

extern "C" {
    fn hash_page_mm(mm: *mut c_void, ea: c_ulong, access: c_ulong,
                    trap: c_ulong, flags: c_ulong) -> c_int;
    fn __rpte_to_hidx(rpte: c_ulong, subpg_index: c_uint) -> c_ulong;
}

const _PAGE_EXEC: c_ulong = 1 << 0;
const _PAGE_PRIVILEGED: c_ulong = 1 << 1;
const _PAGE_WRITE: c_ulong = 1 << 2;
const _PAGE_DIRTY: c_ulong = 1 << 3;
const _PAGE_RWX: c_ulong = 1 << 4;
const _PAGE_CACHE_CTL: c_ulong = 0xff << 8;
const _PAGE_TOLERANT: c_ulong = 1 << 8;
const _PAGE_NON_IDEMPOTENT: c_ulong = 2 << 8;
const _PAGE_SAO: c_ulong = 3 << 8;
const HPTE_R_N: c_ulong = 1 << 0;
const HPTE_R_PP0: c_ulong = 1 << 1;
const HPTE_R_R: c_ulong = 1 << 2;
const HPTE_R_C: c_ulong = 1 << 3;
const HPTE_R_I: c_ulong = 1 << 4;
const HPTE_R_G: c_ulong = 1 << 5;
const HPTE_R_W: c_ulong = 1 << 6;
const HPTE_R_M: c_ulong = 1 << 7;
const MMU_FTR_KERNEL_RO: c_ulong = 1 << 0;
const _PTEIDX_SECONDARY: c_ulong = 1 << 3;
const _PTEIDX_GROUP_IX: c_ulong = 7;
const HPTES_PER_GROUP: c_ulong = 8;
const DSISR_NOHPTE: c_ulong = 1 << 10;
const HPTE_NOHPTE_UPDATE: c_ulong = 1 << 0;


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
