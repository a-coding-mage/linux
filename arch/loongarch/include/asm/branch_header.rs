/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

// Dependency supplied by <asm/ptrace.h>: `pt_regs` with the `csr_era` field.

#[inline]
pub unsafe fn exception_era(regs: *mut pt_regs) -> usize {
    (*regs).csr_era
}

#[inline]
pub unsafe fn compute_return_era(regs: *mut pt_regs) {
    (*regs).csr_era = (*regs).csr_era.wrapping_add(4);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
