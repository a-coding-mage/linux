/* SPDX-License-Identifier: GPL-2.0 */

/* VDSO clocksources */
macro_rules! VDSO_ARCH_CLOCKMODES {
    () => {
        VDSO_CLOCKMODE_TICK,
        VDSO_CLOCKMODE_STICK
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
