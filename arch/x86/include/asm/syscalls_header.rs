/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * syscalls.h - Linux syscall interfaces (arch-specific)
 *
 * Copyright (c) 2008 Jaswinder Singh Rajput
 */

/* Common in X86_32 and X86_64 */
/* kernel/ioport.c */
// C: long ksys_ioperm(unsigned long from, unsigned long num, int turn_on);
extern "C" {
    pub fn ksys_ioperm(
        from: core::ffi::c_ulong,
        num: core::ffi::c_ulong,
        turn_on: core::ffi::c_int,
    ) -> core::ffi::c_long;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
