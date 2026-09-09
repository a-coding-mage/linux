/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * C header guard: _ASM_UM_FPU_API_H
 *
 * The C header includes <linux/types.h>; required types are supplied by the
 * surrounding translation unit.
 *
 * Copyright (c) 2020 Cambridge Greys Ltd
 * Copyright (c) 2020 Red Hat Inc.
 * A set of "dummy" defines to allow the direct inclusion
 * of x86 optimized copy, xor, etc routines into the
 * UML code tree.
 */

#[inline]
pub fn kernel_fpu_begin() {}

#[inline]
pub fn kernel_fpu_end() {}

#[inline]
pub fn irq_fpu_usable() -> bool {
    true
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
