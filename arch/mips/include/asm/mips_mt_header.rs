/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Definitions and declarations for MIPS MT support that are common between
 * the VSMP, and AP/SP kernel models.
 */

// Dependency supplied by the original C include: <linux/cpumask.h>.

/*
 * How many VPEs and TCs is Linux allowed to use?  0 means no limit.
 */
unsafe extern "C" {
    pub static mut tclimit: core::ffi::c_int;
    pub static mut vpelimit: core::ffi::c_int;

    pub static mut mt_fpu_cpumask: crate::cpumask_t;
    pub static mut mt_fpemul_threshold: core::ffi::c_ulong;
}

// CONFIG_MIPS_MT selects the external implementation in the original build.
#[cfg(CONFIG_MIPS_MT)]
unsafe extern "C" {
    pub fn mips_mt_set_cpuoptions();
}

#[cfg(not(CONFIG_MIPS_MT))]
#[inline]
pub fn mips_mt_set_cpuoptions() {}

// `struct class` is supplied by another dependency in the translated build.
unsafe extern "C" {
    pub static mt_class: crate::class;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
