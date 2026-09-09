// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2021 Heiko Stuebner <heiko@sntech.de>
 */

const CSR_TH_SXSTATUS: u32 = 0x5c0;
const SXSTATUS_MAEE: usize = 0x200000;

const RISCV_ALTERNATIVES_EARLY_BOOT: u32 = 0;
const RISCV_ALTERNATIVES_BOOT: u32 = 1;
const RISCV_ALTERNATIVES_MODULE: u32 = 2;

const ERRATA_THEAD_MAE: u32 = 0;
const ERRATA_THEAD_PMU: u32 = 1;
const ERRATA_THEAD_NUMBER: u32 = 2;
const RISCV_VENDOR_EXT_ALTERNATIVES_BASE: u32 = 0x8000;
const THEAD_VENDOR_ID: u64 = 0x5b7;

#[repr(C)]
pub struct AltEntry {
    pub vendor_id: u64,
    pub patch_id: u64,
    pub alt_len: usize,
}

#[repr(C)]
pub struct RiscvNonstdCacheOps {
    pub wback: Option<unsafe extern "C" fn(usize, usize)>,
    pub inv: Option<unsafe extern "C" fn(usize, usize)>,
    pub wback_inv: Option<unsafe extern "C" fn(usize, usize)>,
}

extern "C" {
    static mut riscv_cbom_block_size: usize;
    static mut text_mutex: core::ffi::c_void;

    fn csr_read(csr: u32) -> usize;
    fn riscv_noncoherent_supported();
    fn riscv_noncoherent_register_cache_ops(ops: *const RiscvNonstdCacheOps);
    fn ghostwrite_set_vulnerable();
    fn patch_text_nosync(oldptr: *mut core::ffi::c_void,
                         altptr: *mut core::ffi::c_void,
                         len: usize);
    fn mutex_lock(lock: *mut core::ffi::c_void);
    fn mutex_unlock(lock: *mut core::ffi::c_void);
    fn local_flush_icache_all();
    fn memcpy(dst: *mut core::ffi::c_void,
              src: *const core::ffi::c_void,
              len: usize) -> *mut core::ffi::c_void;
}

#[inline]
fn config_errata_thead_mae() -> bool { cfg!(feature = "CONFIG_ERRATA_THEAD_MAE") }
#[inline]
fn config_errata_thead_cmo() -> bool { cfg!(feature = "CONFIG_ERRATA_THEAD_CMO") }
#[inline]
fn config_errata_thead_pmu() -> bool { cfg!(feature = "CONFIG_ERRATA_THEAD_PMU") }
#[inline]
fn config_errata_thead_ghostwrite() -> bool { cfg!(feature = "CONFIG_ERRATA_THEAD_GHOSTWRITE") }

unsafe fn errata_probe_mae(stage: u32, arch_id: usize, impid: usize) -> bool {
    if !config_errata_thead_mae() { return false; }
    if arch_id != 0 || impid != 0 { return false; }
    if stage != RISCV_ALTERNATIVES_EARLY_BOOT && stage != RISCV_ALTERNATIVES_MODULE { return false; }
    if csr_read(CSR_TH_SXSTATUS) & SXSTATUS_MAEE == 0 { return false; }
    true
}

unsafe fn thead_cmo_op(op: u32, start: usize, size: usize, cachesize: usize) {
    let mut a0 = start & !(cachesize.wrapping_sub(1));
    let end = start.wrapping_add(size);
    core::arch::asm!(
        "mv a0, {start}",
        "j 2f",
        "3:",
        ".4byte {op}",
        "add a0, a0, {cache}",
        "2:",
        "bltu a0, {end}, 3b",
        ".4byte 0x0190000b",
        start = in(reg) a0,
        cache = in(reg) cachesize,
        end = in(reg) end,
        op = const op,
        out("a0") a0,
        options(nostack)
    );
}

unsafe extern "C" fn thead_errata_cache_inv(paddr: usize, size: usize) {
    thead_cmo_op(0x02a5000b, paddr, size, riscv_cbom_block_size);
}

unsafe extern "C" fn thead_errata_cache_wback(paddr: usize, size: usize) {
    thead_cmo_op(0x0295000b, paddr, size, riscv_cbom_block_size);
}

unsafe extern "C" fn thead_errata_cache_wback_inv(paddr: usize, size: usize) {
    thead_cmo_op(0x02b5000b, paddr, size, riscv_cbom_block_size);
}

static THEAD_ERRATA_CMO_OPS: RiscvNonstdCacheOps = RiscvNonstdCacheOps {
    wback: Some(thead_errata_cache_wback),
    inv: Some(thead_errata_cache_inv),
    wback_inv: Some(thead_errata_cache_wback_inv),
};

unsafe fn errata_probe_cmo(stage: u32, arch_id: usize, impid: usize) -> bool {
    if !config_errata_thead_cmo() { return false; }
    if arch_id != 0 || impid != 0 { return false; }
    if stage == RISCV_ALTERNATIVES_EARLY_BOOT { return false; }
    if stage == RISCV_ALTERNATIVES_BOOT {
        riscv_cbom_block_size = 64;
        riscv_noncoherent_supported();
        riscv_noncoherent_register_cache_ops(&THEAD_ERRATA_CMO_OPS);
    }
    true
}

unsafe fn errata_probe_pmu(stage: u32, arch_id: usize, impid: usize) -> bool {
    if !config_errata_thead_pmu() { return false; }
    if arch_id != 0 || impid != 0 { return false; }
    if stage == RISCV_ALTERNATIVES_EARLY_BOOT { return false; }
    true
}

unsafe fn errata_probe_ghostwrite(stage: u32, arch_id: usize, impid: usize) -> bool {
    if !config_errata_thead_ghostwrite() { return false; }
    if arch_id != 0 || impid != 0 { return false; }
    if stage != RISCV_ALTERNATIVES_EARLY_BOOT { return false; }
    ghostwrite_set_vulnerable();
    true
}

unsafe fn thead_errata_probe(stage: u32, archid: usize, impid: usize) -> u32 {
    let mut cpu_req_errata = 0;
    if errata_probe_mae(stage, archid, impid) { cpu_req_errata |= 1 << ERRATA_THEAD_MAE; }
    errata_probe_cmo(stage, archid, impid);
    if errata_probe_pmu(stage, archid, impid) { cpu_req_errata |= 1 << ERRATA_THEAD_PMU; }
    errata_probe_ghostwrite(stage, archid, impid);
    cpu_req_errata
}

pub unsafe extern "C" fn thead_errata_patch_func(
    begin: *mut AltEntry, end: *mut AltEntry, archid: usize, impid: usize, stage: u32,
) {
    let cpu_req_errata = thead_errata_probe(stage, archid, impid);
    let mut alt = begin;
    while alt < end {
        if (*alt).vendor_id != THEAD_VENDOR_ID || (*alt).patch_id >= ERRATA_THEAD_NUMBER {
            alt = alt.add(1);
            continue;
        }
        let tmp = 1u32 << (*alt).patch_id;
        if cpu_req_errata & tmp != 0 {
            let oldptr = (alt as *mut u8).add((*alt).alt_len) as *mut core::ffi::c_void;
            let altptr = oldptr;
            if stage == RISCV_ALTERNATIVES_EARLY_BOOT {
                memcpy(oldptr, altptr, (*alt).alt_len);
            } else {
                mutex_lock(&mut text_mutex);
                patch_text_nosync(oldptr, altptr, (*alt).alt_len);
                mutex_unlock(&mut text_mutex);
            }
        }
        alt = alt.add(1);
    }
    if stage == RISCV_ALTERNATIVES_EARLY_BOOT { local_flush_icache_all(); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
