/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright 2012 Freescale Semiconductor, Inc.
 * Copyright 2012 Linaro Ltd.
 */

/* C build-time condition: CONFIG_CPU_IDLE */
#[cfg(feature = "CONFIG_CPU_IDLE")]
extern "C" {
    pub fn imx5_cpuidle_init() -> ::core::ffi::c_int;
    pub fn imx6q_cpuidle_init() -> ::core::ffi::c_int;
    pub fn imx6sl_cpuidle_init() -> ::core::ffi::c_int;
    pub fn imx6sx_cpuidle_init() -> ::core::ffi::c_int;
    pub fn imx7ulp_cpuidle_init() -> ::core::ffi::c_int;
}

#[cfg(not(feature = "CONFIG_CPU_IDLE"))]
#[inline]
pub fn imx5_cpuidle_init() -> ::core::ffi::c_int {
    0
}

#[cfg(not(feature = "CONFIG_CPU_IDLE"))]
#[inline]
pub fn imx6q_cpuidle_init() -> ::core::ffi::c_int {
    0
}

#[cfg(not(feature = "CONFIG_CPU_IDLE"))]
#[inline]
pub fn imx6sl_cpuidle_init() -> ::core::ffi::c_int {
    0
}

#[cfg(not(feature = "CONFIG_CPU_IDLE"))]
#[inline]
pub fn imx6sx_cpuidle_init() -> ::core::ffi::c_int {
    0
}

#[cfg(not(feature = "CONFIG_CPU_IDLE"))]
#[inline]
pub fn imx7ulp_cpuidle_init() -> ::core::ffi::c_int {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
