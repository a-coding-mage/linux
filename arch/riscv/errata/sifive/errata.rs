// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (C) 2021 Sifive. */

// Dependencies supplied by the surrounding kernel translation.
#[repr(C)]
pub struct alt_entry {
    pub vendor_id: u64,
    pub patch_id: u64,
    pub alt_len: u32,
}

extern "C" {
    static mut text_mutex: core::ffi::c_void;
    fn mutex_lock(lock: *mut core::ffi::c_void);
    fn mutex_unlock(lock: *mut core::ffi::c_void);
    fn patch_text_nosync(old: *mut core::ffi::c_void, new: *mut core::ffi::c_void, len: u32);
    fn ALT_OLD_PTR(alt: *mut alt_entry) -> *mut core::ffi::c_void;
    fn ALT_ALT_PTR(alt: *mut alt_entry) -> *mut core::ffi::c_void;
    #[cfg(feature = "mmu")]
    static mut tlb_flush_all_threshold: usize;
}

// Header-provided constants.
extern "C" { static ERRATA_SIFIVE_NUMBER: u32; }
const SIFIVE_VENDOR_ID: u64 = 0x489;
const RISCV_VENDOR_EXT_ALTERNATIVES_BASE: u32 = 0x8000;
const RISCV_ALTERNATIVES_EARLY_BOOT: u32 = 0;

#[repr(C)]
struct errata_info_t {
    name: [core::ffi::c_char; 32],
    check_func: Option<unsafe extern "C" fn(usize, usize) -> bool>,
}

unsafe extern "C" fn errata_cip_453_check_func(arch_id: usize, impid: usize) -> bool {
    /*
     * Affected cores:
     * Architecture ID: 0x8000000000000007
     * Implement ID: 0x20181004 <= impid <= 0x20191105
     */
    if arch_id != 0x8000000000000007usize
        || impid < 0x20181004usize || impid > 0x20191105usize { return false; }
    true
}

unsafe extern "C" fn errata_cip_1200_check_func(arch_id: usize, impid: usize) -> bool {
    /*
     * Affected cores:
     * Architecture ID: 0x8000000000000007 or 0x1
     * Implement ID: mimpid[23:0] <= 0x200630 and mimpid != 0x01200626
     */
    if arch_id != 0x8000000000000007usize && arch_id != 0x1usize { return false; }
    if (impid & 0xffffffusize) > 0x200630usize || impid == 0x1200626usize { return false; }
    #[cfg(feature = "mmu")]
    unsafe { tlb_flush_all_threshold = 0; }
    true
}

// C string literals are represented as fixed-size arrays in the C layout.
const CIP_453_NAME: [core::ffi::c_char; 32] = [99,105,112,45,52,53,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
const CIP_1200_NAME: [core::ffi::c_char; 32] = [99,105,112,45,49,50,48,48,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];

static mut errata_list: [errata_info_t; 2] = [
    errata_info_t { name: CIP_453_NAME, check_func: Some(errata_cip_453_check_func) },
    errata_info_t { name: CIP_1200_NAME, check_func: Some(errata_cip_1200_check_func) },
];

unsafe fn sifive_errata_probe(archid: usize, impid: usize) -> u32 {
    let mut cpu_req_errata: u32 = 0;
    let mut idx: u32 = 0;
    while idx < ERRATA_SIFIVE_NUMBER {
        if (errata_list[idx as usize].check_func.unwrap())(archid, impid) {
            cpu_req_errata |= 1u32 << idx;
        }
        idx += 1;
    }
    cpu_req_errata
}

pub unsafe extern "C" fn sifive_errata_patch_func(
    begin: *mut alt_entry, end: *mut alt_entry,
    archid: usize, impid: usize, stage: u32,
) {
    // BUILD_BUG_ON(ERRATA_SIFIVE_NUMBER >= RISCV_VENDOR_EXT_ALTERNATIVES_BASE);
    if stage == RISCV_ALTERNATIVES_EARLY_BOOT { return; }
    let cpu_req_errata = sifive_errata_probe(archid, impid);
    let mut alt = begin;
    while alt < end {
        if (*alt).vendor_id != SIFIVE_VENDOR_ID { alt = alt.add(1); continue; }
        if (*alt).patch_id >= ERRATA_SIFIVE_NUMBER { alt = alt.add(1); continue; }
        let tmp = 1u32 << (*alt).patch_id;
        if cpu_req_errata & tmp != 0 {
            mutex_lock(&mut text_mutex as *mut _);
            patch_text_nosync(ALT_OLD_PTR(alt), ALT_ALT_PTR(alt), (*alt).alt_len);
            mutex_unlock(&mut text_mutex as *mut _);
        }
        alt = alt.add(1);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
