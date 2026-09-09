/* SPDX-License-Identifier: GPL-2.0 */

// Dependency: <linux/init.h>

#[cfg(feature = "CONFIG_SMP")]
unsafe extern "C" {
    pub fn mpc85xx_smp_init();
    pub fn mpc85xx_setup_pmc() -> ::core::ffi::c_int;
}

#[cfg(not(feature = "CONFIG_SMP"))]
#[inline]
pub fn mpc85xx_smp_init() {
    /* Nothing to do */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
