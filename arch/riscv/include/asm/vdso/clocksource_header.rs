/* SPDX-License-Identifier: GPL-2.0 */

// VDSO_ARCH_CLOCKMODES expands to the architecture timer clock mode.
// VDSO_CLOCKMODE_ARCHTIMER is supplied by the surrounding dependencies.
macro_rules! VDSO_ARCH_CLOCKMODES {
    () => {
        VDSO_CLOCKMODE_ARCHTIMER
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
