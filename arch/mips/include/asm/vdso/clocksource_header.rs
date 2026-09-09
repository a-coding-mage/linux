/* SPDX-License-Identifier: GPL-2.0-or-later */

// Equivalent of the C preprocessor macro VDSO_ARCH_CLOCKMODES.
// The referenced clock mode constants are supplied by other dependencies.
macro_rules! VDSO_ARCH_CLOCKMODES {
    () => {
        VDSO_CLOCKMODE_R4K,
        VDSO_CLOCKMODE_GIC
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
