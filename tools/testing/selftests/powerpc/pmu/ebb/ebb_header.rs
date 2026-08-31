/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright 2014, Michael Ellerman, IBM Corp.
 */

// C header dependencies:
// #include "../event.h"
// #include "../lib.h"
// #include "trace.h"
// #include "reg.h"

use core::ffi::{c_int, c_uint, c_ulong};

pub const fn PMC_INDEX(pmc: c_int) -> c_int {
    pmc - 1
}

pub const NUM_PMC_VALUES: usize = 128;

#[repr(C)]
pub struct ebb_state_stats {
    pub pmc_count: [u64; 6],
    pub ebb_count: c_int, /* volatile in C */
    pub spurious: c_int,
    pub negative: c_int,
    pub no_overflow: c_int,
}

#[repr(C)]
pub struct ebb_state {
    pub stats: ebb_state_stats,
    pub pmc_enable: [bool; 6],
    pub trace: *mut trace_buffer,
}

unsafe extern "C" {
    pub static mut ebb_state: ebb_state;
}

pub const COUNTER_OVERFLOW: u64 = 0x80000000u64;

#[inline]
pub fn pmc_sample_period(value: u32) -> u32 {
    (COUNTER_OVERFLOW as u32).wrapping_sub(value)
}

#[inline]
pub unsafe fn ebb_enable_pmc_counting(pmc: c_int) {
    unsafe {
        ebb_state.pmc_enable[PMC_INDEX(pmc) as usize] = true;
    }
}

unsafe extern "C" {
    pub fn ebb_check_count(pmc: c_int, sample_period: u64, fudge: c_int) -> bool;
    pub fn event_leader_ebb_init(e: *mut event);
    pub fn event_ebb_init(e: *mut event);
    pub fn event_bhrb_init(e: *mut event, ifm: c_uint);
    pub fn setup_ebb_handler(callee: Option<unsafe extern "C" fn()>);
    pub fn standard_ebb_callee();
    pub fn ebb_event_enable(e: *mut event) -> c_int;
    pub fn ebb_global_enable();
    pub fn ebb_global_disable();
    pub fn ebb_is_supported() -> bool;
    pub fn ebb_freeze_pmcs();
    pub fn ebb_unfreeze_pmcs();
    pub fn count_pmc(pmc: c_int, sample_period: u32) -> c_int;
    pub fn dump_ebb_state();
    pub fn dump_summary_ebb_state();
    pub fn dump_ebb_hw_state();
    pub fn clear_ebb_stats();
    pub fn write_pmc(pmc: c_int, value: u64);
    pub fn read_pmc(pmc: c_int) -> u64;
    pub fn reset_ebb_with_clear_mask(mmcr0_clear_mask: c_ulong);
    pub fn reset_ebb();
    pub fn ebb_check_mmcr0() -> c_int;

    pub static mut sample_period: u64;

    pub fn core_busy_loop() -> c_int;
    pub fn ebb_child(read_pipe: pipe, write_pipe: pipe) -> c_int;
    pub fn catch_sigill(func: Option<unsafe extern "C" fn()>) -> c_int;
    pub fn write_pmc1();
}
