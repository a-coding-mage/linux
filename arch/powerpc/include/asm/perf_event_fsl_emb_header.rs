/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Performance event support - Freescale embedded specific definitions.
 *
 * Copyright 2008-2009 Paul Mackerras, IBM Corporation.
 * Copyright 2010 Freescale Semiconductor, Inc.
 */

// Dependencies supplied by the surrounding kernel translation.

pub const MAX_HWEVENTS: ::core::ffi::c_int = 6;

/* event flags */
pub const FSL_EMB_EVENT_VALID: ::core::ffi::c_int = 1;
pub const FSL_EMB_EVENT_RESTRICTED: ::core::ffi::c_int = 2;

/* upper half of event flags is PMLCb */
pub const FSL_EMB_EVENT_THRESHMUL: u64 = 0x0000_0700_0000_0000;
pub const FSL_EMB_EVENT_THRESH: u64 = 0x0000_003f_0000_0000;

#[repr(C)]
pub struct fsl_emb_pmu {
    pub name: *const ::core::ffi::c_char,
    pub n_counter: ::core::ffi::c_int, /* total number of counters */

    /*
     * The number of contiguous counters starting at zero that
     * can hold restricted events, or zero if there are no
     * restricted events.
     *
     * This isn't a very flexible method of expressing constraints,
     * but it's very simple and is adequate for existing chips.
     */
    pub n_restricted: ::core::ffi::c_int,

    /* Returns event flags and PMLCb (FSL_EMB_EVENT_*) */
    pub xlate_event: Option<unsafe extern "C" fn(event_id: u64) -> u64>,

    pub n_generic: ::core::ffi::c_int,
    pub generic_events: *mut ::core::ffi::c_int,
    pub cache_events: *mut [[[::core::ffi::c_int; PERF_COUNT_HW_CACHE_RESULT_MAX]; PERF_COUNT_HW_CACHE_OP_MAX]; PERF_COUNT_HW_CACHE_MAX],
}

unsafe extern "C" {
    pub fn register_fsl_emb_pmu(pmu: *mut fsl_emb_pmu) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
