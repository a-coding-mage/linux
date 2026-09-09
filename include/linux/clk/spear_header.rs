/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2020 STMicroelectronics - All Rights Reserved
 *
 * Author: Lee Jones <lee.jones@linaro.org>
 */

/* C header guard: __LINUX_CLK_SPEAR_H */

/* CONFIG_ARCH_SPEAR3XX */
#[cfg(CONFIG_ARCH_SPEAR3XX)]
unsafe extern "C" {
    pub fn spear3xx_clk_init(misc_base: *mut core::ffi::c_void,
                              soc_config_base: *mut core::ffi::c_void);
}

#[cfg(not(CONFIG_ARCH_SPEAR3XX))]
#[inline]
pub unsafe fn spear3xx_clk_init(_misc_base: *mut core::ffi::c_void,
                                _soc_config_base: *mut core::ffi::c_void) {}

/* CONFIG_ARCH_SPEAR6XX */
#[cfg(CONFIG_ARCH_SPEAR6XX)]
unsafe extern "C" {
    pub fn spear6xx_clk_init(misc_base: *mut core::ffi::c_void);
}

#[cfg(not(CONFIG_ARCH_SPEAR6XX))]
#[inline]
pub unsafe fn spear6xx_clk_init(_misc_base: *mut core::ffi::c_void) {}

/* CONFIG_MACH_SPEAR1310 */
#[cfg(CONFIG_MACH_SPEAR1310)]
unsafe extern "C" {
    pub fn spear1310_clk_init(misc_base: *mut core::ffi::c_void,
                               ras_base: *mut core::ffi::c_void);
}

#[cfg(not(CONFIG_MACH_SPEAR1310))]
#[inline]
pub unsafe fn spear1310_clk_init(_misc_base: *mut core::ffi::c_void,
                                 _ras_base: *mut core::ffi::c_void) {}

/* CONFIG_MACH_SPEAR1340 */
#[cfg(CONFIG_MACH_SPEAR1340)]
unsafe extern "C" {
    pub fn spear1340_clk_init(misc_base: *mut core::ffi::c_void);
}

#[cfg(not(CONFIG_MACH_SPEAR1340))]
#[inline]
pub unsafe fn spear1340_clk_init(_misc_base: *mut core::ffi::c_void) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
