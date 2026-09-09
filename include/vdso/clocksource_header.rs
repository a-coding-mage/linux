/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent from <vdso/limits.h> and <asm/vdso/clocksource.h> is
// preserved here; their declarations are supplied by other translated files.

#[repr(i32)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum VdsoClockMode {
    VdsoClockmodeNone = 0,

    // Corresponds to the build-time VDSO_ARCH_CLOCKMODES expansion.
    #[cfg(feature = "VDSO_ARCH_CLOCKMODES")]
    VdsoArchClockmodes,

    VdsoClockmodeMax,

    /* Indicator for time namespace VDSO */
    VdsoClockmodeTimens = i32::MAX,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
