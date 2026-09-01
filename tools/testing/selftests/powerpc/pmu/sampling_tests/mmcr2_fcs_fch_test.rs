// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2022, Madhavan Srinivasan, IBM Corp.
 */

// C dependencies from:
// #include <signal.h>
// #include <stdio.h>
// #include <stdlib.h>
// #include <sys/types.h>
// #include "../event.h"
// #include "misc.h"
// #include "utils.h"

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct siginfo_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mcontext_t {
    pub gp_regs: [u64; 0],
}

#[repr(C)]
pub struct ucontext_t {
    pub uc_mcontext: mcontext_t,
}

#[repr(C)]
pub struct sigaction {
    pub sa_sigaction: Option<unsafe extern "C" fn(c_int, *mut siginfo_t, *mut c_void)>,
    pub sa_flags: c_int,
}

#[repr(C)]
pub struct perf_event_attr {
    pub sample_regs_intr: u64,
    pub exclude_kernel: u64,
}

#[repr(C)]
pub struct event {
    pub attr: perf_event_attr,
    pub fd: c_int,
    pub mmap_buffer: *mut c_void,
}

extern "C" {
    fn thirty_two_instruction_loop(loops: c_int);

    static platform_extended_mask: u64;

    static PT_MSR: usize;
    static MSR_HV: u64;
    static SA_SIGINFO: c_int;
    static SIGUSR2: c_int;

    fn sigaction(signum: c_int, act: *const sigaction, oldact: *mut sigaction) -> c_int;
    fn kill(pid: c_int, sig: c_int) -> c_int;
    fn getpid() -> c_int;

    fn check_pvr_for_sampling_tests() -> c_int;
    fn event_init_sampling(event: *mut event, config: u64);
    fn event_open(event: *mut event) -> c_int;
    fn event_sample_buf_mmap(fd: c_int, mmap_pages: c_int) -> *mut c_void;
    fn event_enable(event: *mut event) -> c_int;
    fn event_disable(event: *mut event) -> c_int;
    fn collect_samples(mmap_buffer: *mut c_void) -> c_int;
    fn get_intr_regs(event: *mut event, mmap_buffer: *mut c_void) -> *mut u64;
    fn get_reg_value(intr_regs: *mut u64, name: *const c_char) -> u64;
    fn get_mmcr2_fch(value: u64, shift: c_int) -> u64;
    fn get_mmcr2_fcs(value: u64, shift: c_int) -> u64;
    fn event_close(event: *mut event);
    fn test_harness(
        test_function: Option<unsafe extern "C" fn() -> c_int>,
        name: *const c_char,
    ) -> c_int;
}

unsafe fn FAIL_IF(condition: bool) -> c_int {
    if condition {
        return 1;
    }

    0
}

unsafe fn SKIP_IF(condition: bool) -> c_int {
    if condition {
        return 0;
    }

    0
}

static mut is_hv: bool = false;

unsafe extern "C" fn sig_usr2_handler(signum: c_int, info: *mut siginfo_t, data: *mut c_void) {
    let uctx: *mut ucontext_t = data as *mut ucontext_t;

    is_hv = (((*uctx).uc_mcontext.gp_regs[PT_MSR] & MSR_HV) != 0) as bool;
}

/*
 * A perf sampling test for mmcr2
 * fields : fcs, fch.
 */
unsafe extern "C" fn mmcr2_fcs_fch() -> c_int {
    let sigact: sigaction = sigaction {
        sa_sigaction: Some(sig_usr2_handler),
        sa_flags: SA_SIGINFO,
    };
    let mut event: event = core::mem::zeroed();
    let intr_regs: *mut u64;

    if FAIL_IF(sigaction(SIGUSR2, &sigact, core::ptr::null_mut()) != 0) != 0 {
        return 1;
    }
    if FAIL_IF(kill(getpid(), SIGUSR2) != 0) != 0 {
        return 1;
    }

    /* Check for platform support for the test */
    if SKIP_IF(check_pvr_for_sampling_tests() != 0) != 0 {
        return 0;
    }

    /* Init the event for the sampling test */
    event_init_sampling(&mut event, 0x1001e);
    event.attr.sample_regs_intr = platform_extended_mask;
    event.attr.exclude_kernel = 1;
    if FAIL_IF(event_open(&mut event) != 0) != 0 {
        return 1;
    }
    event.mmap_buffer = event_sample_buf_mmap(event.fd, 1);

    if FAIL_IF(event_enable(&mut event) != 0) != 0 {
        return 1;
    }

    /* workload to make the event overflow */
    thirty_two_instruction_loop(10000);

    if FAIL_IF(event_disable(&mut event) != 0) != 0 {
        return 1;
    }

    /* Check for sample count */
    if FAIL_IF(collect_samples(event.mmap_buffer) == 0) != 0 {
        return 1;
    }

    intr_regs = get_intr_regs(&mut event, event.mmap_buffer);

    /* Check for intr_regs */
    if FAIL_IF(intr_regs.is_null()) != 0 {
        return 1;
    }

    /*
     * Verify that fcs and fch field of MMCR2 match
     * with corresponding modifier fields.
     */
    if is_hv {
        if FAIL_IF(
            event.attr.exclude_kernel
                != get_mmcr2_fch(get_reg_value(intr_regs, b"MMCR2\0".as_ptr() as *const c_char), 1),
        ) != 0
        {
            return 1;
        }
    } else if FAIL_IF(
        event.attr.exclude_kernel
            != get_mmcr2_fcs(get_reg_value(intr_regs, b"MMCR2\0".as_ptr() as *const c_char), 1),
    ) != 0
    {
        return 1;
    }

    event_close(&mut event);
    return 0;
}

#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    return test_harness(Some(mmcr2_fcs_fch), b"mmcr2_fcs_fch\0".as_ptr() as *const c_char);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
