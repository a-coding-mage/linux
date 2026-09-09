/* SPDX-License-Identifier: GPL-2.0 */
/*
 * R-Car Gen3 Clock Pulse Generator Library
 *
 * Copyright (C) 2015-2018 Glider bvba
 * Copyright (C) 2019 Renesas Electronics Corp.
 *
 * Based on clk-rcar-gen3.c
 *
 * Copyright (C) 2015 Renesas Electronics Corp.
 */

// C header dependencies and build-time annotations are supplied by the
// surrounding translation unit.

extern "C" {
    pub static mut cpg_lock: spinlock_t;
}

#[repr(C)]
pub struct cpg_simple_notifier {
    pub nb: notifier_block,
    pub reg: *mut core::ffi::c_void,
    pub saved: u32,
}

extern "C" {
    pub fn cpg_simple_notifier_register(
        notifiers: *mut raw_notifier_head,
        csn: *mut cpg_simple_notifier,
    );

    pub fn cpg_reg_modify(reg: *mut core::ffi::c_void, clear: u32, set: u32);

    // __init
    pub fn cpg_sdh_clk_register(
        name: *const core::ffi::c_char,
        sdnckcr: *mut core::ffi::c_void,
        parent_name: *const core::ffi::c_char,
        notifiers: *mut raw_notifier_head,
    ) -> *mut clk;

    // __init
    pub fn cpg_sd_clk_register(
        name: *const core::ffi::c_char,
        sdnckcr: *mut core::ffi::c_void,
        parent_name: *const core::ffi::c_char,
    ) -> *mut clk;

    // __init
    pub fn cpg_rpc_clk_register(
        name: *const core::ffi::c_char,
        rpcckcr: *mut core::ffi::c_void,
        parent_name: *const core::ffi::c_char,
        notifiers: *mut raw_notifier_head,
    ) -> *mut clk;

    // __init
    pub fn cpg_rpcd2_clk_register(
        name: *const core::ffi::c_char,
        rpcckcr: *mut core::ffi::c_void,
        parent_name: *const core::ffi::c_char,
    ) -> *mut clk;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
