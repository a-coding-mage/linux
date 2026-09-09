/* SPDX-License-Identifier: GPL-2.0 */

// Equivalent of the C preprocessor list macro. The referenced clock-mode
// symbols are supplied by the surrounding translation unit.
macro_rules! VDSO_ARCH_CLOCKMODES {
    () => {
        VDSO_CLOCKMODE_TSC,
        VDSO_CLOCKMODE_PVCLOCK,
        VDSO_CLOCKMODE_HVCLOCK
    };
}

// Equivalent of HAVE_VDSO_CLOCKMODE_HVCLOCK.
pub const HAVE_VDSO_CLOCKMODE_HVCLOCK: bool = true;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
