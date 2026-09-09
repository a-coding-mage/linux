/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by linux/sh_clk.h in the original header.

/* Should be defined by processor-specific code */
extern "C" {
    #[deprecated]
    pub fn arch_init_clk_ops(ops: *mut *mut sh_clk_ops, type_: i32);

    pub fn arch_clk_init() -> i32;
}

/* arch/sh/kernel/cpu/clock-cpg.c */
extern "C" {
    #[deprecated]
    pub fn cpg_clk_init() -> i32;
}

/* arch/sh/kernel/cpu/clock.c */
extern "C" {
    pub fn clk_init() -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
