/* SPDX-License-Identifier: GPL-2.0 */

// VDSO clocksource for both 32 and 64-bit tasks; clocksource for 64-bit tasks
// only. The referenced clock modes are supplied by other translated headers.
macro_rules! VDSO_ARCH_CLOCKMODES {
    () => {
        VDSO_CLOCKMODE_ARCHTIMER,
        VDSO_CLOCKMODE_ARCHTIMER_NOCOMPAT
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
