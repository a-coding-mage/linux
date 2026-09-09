/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Based on arch/arm/include/asm/system_misc.h
 *
 * Copyright (C) 2012 ARM Ltd.
 */

// C header guard: __ASM_SYSTEM_MISC_H
// Declarations are omitted when compiling as an assembler source (__ASSEMBLER__).
// C dependencies: linux/compiler.h, linux/linkage.h, linux/irqflags.h,
// linux/signal.h, linux/ratelimit.h, linux/reboot.h

use core::ffi::{c_char, c_int, c_long, c_ulong};

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

#[repr(C)]
pub struct siginfo {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mm_struct {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn die(msg: *const c_char, regs: *mut pt_regs, err: c_long);

    pub fn arm64_notify_die(
        str_: *const c_char,
        regs: *mut pt_regs,
        signo: c_int,
        sicode: c_int,
        far: c_ulong,
        err: c_ulong,
    );

    pub fn __show_regs(regs: *mut pt_regs);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
