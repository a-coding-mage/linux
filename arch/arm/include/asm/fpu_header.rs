/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2023 SiFive
 */

// Dependency intent: symbols supplied by asm/neon.h are referenced below.

macro_rules! kernel_fpu_available {
    () => {
        cpu_has_neon()
    };
}

macro_rules! kernel_fpu_begin {
    () => {
        kernel_neon_begin()
    };
}

macro_rules! kernel_fpu_end {
    () => {
        kernel_neon_end()
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
