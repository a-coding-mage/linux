/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Based on arch/arm/include/asm/proc-fns.h
 *
 * Copyright (C) 1997-1999 Russell King
 * Copyright (C) 2000 Deep Blue Solutions Ltd
 * Copyright (C) 2012 ARM Ltd.
 */

// C header guard: __ASM_PROCFNS_H

// The declarations below are excluded when building as an assembler source
// file in the original header (the __ASSEMBLER__ condition).

#[repr(C)]
pub struct cpu_suspend_ctx {
    _private: [u8; 0],
}

extern "C" {
    pub fn cpu_do_idle();
    pub fn cpu_do_suspend(ptr: *mut cpu_suspend_ctx);
    pub fn cpu_do_resume(ptr: usize, idmap_ttbr: u64) -> u64;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
