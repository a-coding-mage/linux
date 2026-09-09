/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2007  Maciej W. Rozycki
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// not expanded here: linux/bug.h, linux/smp.h, asm/cpu.h, and asm/cpu-info.h.

extern "C" {
    pub static mut daddiu_bug: core::ffi::c_int;

    pub fn check_bugs64_early();

    pub fn check_bugs32();
    pub fn check_bugs64();
}

/// Equivalent of the C `r4k_daddiu_bug` inline function.
#[inline]
pub unsafe fn r4k_daddiu_bug() -> core::ffi::c_int {
    // Build-time condition: !IS_ENABLED(CONFIG_CPU_R4X00_BUGS64).
    // The surrounding build must provide the corresponding configuration.

    // C: WARN_ON(daddiu_bug < 0);
    // The WARN_ON macro is supplied by linux/bug.h and is preserved as a
    // dependency rather than reimplemented in this header translation.
    let _ = daddiu_bug < 0;
    (daddiu_bug != 0) as core::ffi::c_int
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
