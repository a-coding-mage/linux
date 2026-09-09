/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * linux/arch/arm64/include/asm/neon.h
 *
 * Copyright (C) 2013 Linaro Ltd <ard.biesheuvel@linaro.org>
 */

// C dependency: <linux/types.h>
// C dependency: <asm/fpsimd.h>

#[macro_export]
macro_rules! cpu_has_neon {
    () => {
        system_supports_fpsimd()
    };
}

extern "C" {
    pub fn kernel_neon_begin(state: *mut user_fpsimd_state);
    pub fn kernel_neon_end(state: *mut user_fpsimd_state);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
