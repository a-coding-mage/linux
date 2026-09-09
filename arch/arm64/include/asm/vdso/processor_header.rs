/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2020 ARM Ltd.
 */

// C header guard: __ASM_VDSO_PROCESSOR_H

// The C declaration is excluded when compiling as assembler.
#[inline]
pub unsafe fn cpu_relax() {
    core::arch::asm!("yield", options(nostack, preserves_flags));
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
