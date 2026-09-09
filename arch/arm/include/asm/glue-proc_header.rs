/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Rust translation of arch/arm/include/asm/glue-proc.h.
 *
 * The CPU configuration symbols below are build-time C preprocessor
 * conditions; they are retained as cfg-intent comments because their
 * definitions are supplied by the surrounding build.
 */

// Dependency: <asm/glue.h> supplies the __glue token-concatenation facility.

/*
 * Work out if we need multiple CPU support.  The C header selects the first
 * configured CPU as CPU_NAME and sets MULTI_CPU when more than one CPU is
 * configured (CONFIG_CPU_V7 always forces MULTI_CPU).
 *
 * CONFIG_CPU_ARM7TDMI  -> cpu_arm7tdmi
 * CONFIG_CPU_ARM720T   -> cpu_arm720
 * CONFIG_CPU_ARM740T   -> cpu_arm740
 * CONFIG_CPU_ARM9TDMI  -> cpu_arm9tdmi
 * CONFIG_CPU_ARM920T   -> cpu_arm920
 * CONFIG_CPU_ARM922T   -> cpu_arm922
 * CONFIG_CPU_FA526     -> cpu_fa526
 * CONFIG_CPU_ARM925T   -> cpu_arm925
 * CONFIG_CPU_ARM926T   -> cpu_arm926
 * CONFIG_CPU_ARM940T   -> cpu_arm940
 * CONFIG_CPU_ARM946E   -> cpu_arm946
 * CONFIG_CPU_SA110     -> cpu_sa110
 * CONFIG_CPU_SA1100    -> cpu_sa1100
 * CONFIG_CPU_ARM1020   -> cpu_arm1020
 * CONFIG_CPU_ARM1020E  -> cpu_arm1020e
 * CONFIG_CPU_ARM1022   -> cpu_arm1022
 * CONFIG_CPU_ARM1026   -> cpu_arm1026
 * CONFIG_CPU_XSCALE    -> cpu_xscale
 * CONFIG_CPU_XSC3      -> cpu_xsc3
 * CONFIG_CPU_MOHAWK    -> cpu_mohawk
 * CONFIG_CPU_FEROCEON  -> cpu_feroceon
 * CONFIG_CPU_V6/V6K    -> cpu_v6
 * CONFIG_CPU_V7M       -> cpu_v7m
 * CONFIG_CPU_PJ4B      -> cpu_pj4b
 * CONFIG_CPU_V7        -> MULTI_CPU
 */

/*
 * When exactly one CPU is selected, the C preprocessor defines the following
 * aliases using __glue(CPU_NAME, suffix).  Keep the aliases as declarative
 * Rust macros so the external glue implementation remains supplied by the
 * including build.
 */
#[cfg(not(feature = "multi_cpu"))]
macro_rules! cpu_proc_init {
    () => { __glue!(CPU_NAME, _proc_init) };
}
#[cfg(not(feature = "multi_cpu"))]
macro_rules! cpu_proc_fin {
    () => { __glue!(CPU_NAME, _proc_fin) };
}
#[cfg(not(feature = "multi_cpu"))]
macro_rules! cpu_reset {
    () => { __glue!(CPU_NAME, _reset) };
}
#[cfg(not(feature = "multi_cpu"))]
macro_rules! cpu_do_idle {
    () => { __glue!(CPU_NAME, _do_idle) };
}
#[cfg(not(feature = "multi_cpu"))]
macro_rules! cpu_dcache_clean_area {
    () => { __glue!(CPU_NAME, _dcache_clean_area) };
}
#[cfg(not(feature = "multi_cpu"))]
macro_rules! cpu_do_switch_mm {
    () => { __glue!(CPU_NAME, _switch_mm) };
}
#[cfg(not(feature = "multi_cpu"))]
macro_rules! cpu_set_pte_ext {
    () => { __glue!(CPU_NAME, _set_pte_ext) };
}
#[cfg(not(feature = "multi_cpu"))]
macro_rules! cpu_suspend_size {
    () => { __glue!(CPU_NAME, _suspend_size) };
}
#[cfg(not(feature = "multi_cpu"))]
macro_rules! cpu_do_suspend {
    () => { __glue!(CPU_NAME, _do_suspend) };
}
#[cfg(not(feature = "multi_cpu"))]
macro_rules! cpu_do_resume {
    () => { __glue!(CPU_NAME, _do_resume) };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
