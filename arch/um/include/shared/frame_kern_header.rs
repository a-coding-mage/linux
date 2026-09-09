/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2002 Jeff Dike (jdike@karaya.com)
 */

// C header guard: __FRAME_KERN_H_

extern "C" {
    pub fn setup_signal_stack_sc(
        stack_top: ::core::ffi::c_ulong,
        ksig: *mut ksignal,
        regs: *mut pt_regs,
        mask: *mut sigset_t,
    ) -> ::core::ffi::c_int;

    pub fn setup_signal_stack_si(
        stack_top: ::core::ffi::c_ulong,
        ksig: *mut ksignal,
        regs: *mut pt_regs,
        mask: *mut sigset_t,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
