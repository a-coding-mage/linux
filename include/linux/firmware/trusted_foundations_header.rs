/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (c) 2013, NVIDIA Corporation.
 */

/*
 * Support for the Trusted Foundations secure monitor.
 *
 * Trusted Foundation comes active on some ARM consumer devices (most
 * Tegra-based devices sold on the market are concerned). Such devices can only
 * perform some basic operations, like setting the CPU reset vector, through
 * SMC calls to the secure monitor. The calls are completely specific to
 * Trusted Foundations, and do *not* follow the SMC calling convention or the
 * PSCI standard.
 */

// C header dependencies are supplied by other translated units.

pub const TF_PM_MODE_LP0: ::core::ffi::c_int = 0;
pub const TF_PM_MODE_LP1: ::core::ffi::c_int = 1;
pub const TF_PM_MODE_LP1_NO_MC_CLK: ::core::ffi::c_int = 2;
pub const TF_PM_MODE_LP2: ::core::ffi::c_int = 3;
pub const TF_PM_MODE_LP2_NOFLUSH_L2: ::core::ffi::c_int = 4;
pub const TF_PM_MODE_NONE: ::core::ffi::c_int = 5;

#[repr(C)]
pub struct trusted_foundations_platform_data {
    pub version_major: ::core::ffi::c_uint,
    pub version_minor: ::core::ffi::c_uint,
}

// CONFIG_TRUSTED_FOUNDATIONS is a build-time condition from the original
// header. The enabled declarations are preserved here; the fallback inline
// implementations are provided below for the disabled configuration.
#[cfg(feature = "CONFIG_TRUSTED_FOUNDATIONS")]
unsafe extern "C" {
    pub fn register_trusted_foundations(
        pd: *mut trusted_foundations_platform_data,
    );
    pub fn of_register_trusted_foundations();
    pub fn trusted_foundations_registered() -> bool;
}

#[cfg(not(feature = "CONFIG_TRUSTED_FOUNDATIONS"))]
pub unsafe fn tf_dummy_write_sec(
    _val: ::core::ffi::c_ulong,
    _reg: ::core::ffi::c_uint,
) {
}

#[cfg(not(feature = "CONFIG_TRUSTED_FOUNDATIONS"))]
pub unsafe fn register_trusted_foundations(
    _pd: *mut trusted_foundations_platform_data,
) {
    /*
     * If the system requires TF and we cannot provide it, continue booting
     * but disable features that cannot be provided.
     */
    pr_err("No support for Trusted Foundations, continuing in degraded mode.\n");
    pr_err("Secondary processors as well as CPU PM will be disabled.\n");

    // IS_ENABLED(CONFIG_CACHE_L2X0) from the C header.
    #[cfg(feature = "CONFIG_CACHE_L2X0")]
    {
        pr_err("L2X0 cache will be kept disabled.\n");
        outer_cache.write_sec = Some(tf_dummy_write_sec);
    }

    // IS_ENABLED(CONFIG_SMP) from the C header.
    #[cfg(feature = "CONFIG_SMP")]
    {
        setup_max_cpus = 0;
    }
    cpu_idle_poll_ctrl(true);
}

#[cfg(not(feature = "CONFIG_TRUSTED_FOUNDATIONS"))]
pub unsafe fn of_register_trusted_foundations() {
    let np = of_find_compatible_node(
        ::core::ptr::null_mut(),
        ::core::ptr::null_mut(),
        "tlm,trusted-foundations",
    );

    if np.is_null() {
        return;
    }
    of_node_put(np);
    /*
     * If we find the target should enable TF but does not support it,
     * fail as the system won't be able to do much anyway
     */
    register_trusted_foundations(::core::ptr::null_mut());
}

#[cfg(not(feature = "CONFIG_TRUSTED_FOUNDATIONS"))]
pub unsafe fn trusted_foundations_registered() -> bool {
    false
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
