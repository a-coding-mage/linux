// SPDX-License-Identifier: GPL-2.0-only
/*
 * Erratas to be applied for Andes CPU cores
 *
 *  Copyright (C) 2023 Renesas Electronics Corporation.
 *
 * Author: Lad Prabhakar <prabhakar.mahadev-lad.rj@bp.renesas.com>
 */

// Dependencies supplied by the surrounding kernel/Rust bindings.

const ANDES_AX45MP_MARCHID: u64 = 0x8000_0000_0000_8a45;
const ANDES_AX45MP_MIMPID: u64 = 0x500;
const ANDES_SBI_EXT_ANDES: u32 = 0x0900_031e;

const ANDES_SBI_EXT_IOCP_SW_WORKAROUND: usize = 1;

#[repr(C)]
struct SbiRet {
    error: isize,
    value: isize,
}

unsafe extern "C" {
    fn sbi_ecall(
        ext: u32,
        fid: usize,
        arg0: usize,
        arg1: usize,
        arg2: usize,
        arg3: usize,
        arg4: usize,
        arg5: usize,
    ) -> SbiRet;

    static mut riscv_cbom_block_size: usize;
    fn riscv_noncoherent_supported();
}

unsafe fn ax45mp_iocp_sw_workaround() -> isize {
    let ret: SbiRet;

    /*
     * ANDES_SBI_EXT_IOCP_SW_WORKAROUND SBI EXT checks if the IOCP is missing and
     * cache is controllable only then CMO will be applied to the platform.
     */
    ret = sbi_ecall(
        ANDES_SBI_EXT_ANDES,
        ANDES_SBI_EXT_IOCP_SW_WORKAROUND,
        0,
        0,
        0,
        0,
        0,
        0,
    );

    if ret.error != 0 { 0 } else { ret.value }
}

unsafe fn errata_probe_iocp(
    _stage: u32,
    arch_id: u64,
    impid: u64,
) {
    static mut DONE: bool = false;

    // Build-time condition preserved from CONFIG_ERRATA_ANDES_CMO.
    if !cfg!(feature = "ERRATA_ANDES_CMO") {
        return;
    }

    if DONE {
        return;
    }

    DONE = true;

    if arch_id != ANDES_AX45MP_MARCHID || impid != ANDES_AX45MP_MIMPID {
        return;
    }

    if ax45mp_iocp_sw_workaround() == 0 {
        return;
    }

    /* Set this just to make core cbo code happy */
    riscv_cbom_block_size = 1;
    riscv_noncoherent_supported();
}

pub unsafe fn andes_errata_patch_func(
    begin: *mut core::ffi::c_void,
    end: *mut core::ffi::c_void,
    archid: u64,
    impid: u64,
    stage: u32,
) {
    let _ = (begin, end);

    // BUILD_BUG_ON(ERRATA_ANDES_NUMBER >= RISCV_VENDOR_EXT_ALTERNATIVES_BASE);

    // RISCV_ALTERNATIVES_BOOT is supplied by the surrounding kernel bindings.
    if stage == RISCV_ALTERNATIVES_BOOT {
        errata_probe_iocp(stage, archid, impid);
    }

    /* we have nothing to patch here ATM so just return back */
}

// External build-time constant supplied by the surrounding kernel bindings.
unsafe extern "C" {
    static RISCV_ALTERNATIVES_BOOT: u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
