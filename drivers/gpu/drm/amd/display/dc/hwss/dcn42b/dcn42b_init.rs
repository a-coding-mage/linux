/*
 * SPDX-License-Identifier: MIT
 *
 * Copyright 2026 Advanced Micro Devices, Inc.
 */

// Dependencies supplied by the corresponding C headers:
// dcn20/dcn20_hwseq.h, dcn42/dcn42_init.h, dcn42b_hwseq.h,
// and dcn42b_init.h.

use core::ffi::c_void;

#[repr(C)]
pub struct dc {
    pub hwseq: *mut dc_hwseq,
}

#[repr(C)]
pub struct dc_hwseq {
    pub funcs: dc_hwseq_funcs,
}

#[repr(C)]
pub struct dc_hwseq_funcs {
    pub init_pipes: Option<unsafe extern "C" fn(*mut c_void)>,
}

unsafe extern "C" {
    pub fn dcn42_hw_sequencer_init_functions(dc: *mut dc);
    pub fn dcn42b_init_pipes(dc: *mut c_void);
}

pub unsafe fn dcn42b_hw_sequencer_init_functions(dc: *mut dc) {
    /* Initialize with dcn42 functions first */
    dcn42_hw_sequencer_init_functions(dc);

    /* Override only init_pipes with dcn42b version */
    (*(*dc).hwseq).funcs.init_pipes = Some(dcn42b_init_pipes);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
