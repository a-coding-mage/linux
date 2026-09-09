/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * Copyright (C) 2016 Freescale Semiconductor, Inc.
 * Copyright 2017~2018 NXP
 *
 * Header file containing the public API for the System Controller (SC)
 * Miscellaneous (MISC) function.
 *
 * MISC_SVC (SVC) Miscellaneous Service
 *
 * Module for the Miscellaneous (MISC) service.
 */

// Dependency supplied by linux/firmware/imx/sci.h.

/*
 * This type is used to indicate RPC MISC function calls.
 */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum imx_misc_func {
    IMX_SC_MISC_FUNC_UNKNOWN = 0,
    IMX_SC_MISC_FUNC_SET_CONTROL = 1,
    IMX_SC_MISC_FUNC_GET_CONTROL = 2,
    IMX_SC_MISC_FUNC_SET_MAX_DMA_GROUP = 4,
    IMX_SC_MISC_FUNC_SET_DMA_GROUP = 5,
    IMX_SC_MISC_FUNC_SECO_IMAGE_LOAD = 8,
    IMX_SC_MISC_FUNC_SECO_AUTHENTICATE = 9,
    IMX_SC_MISC_FUNC_DEBUG_OUT = 10,
    IMX_SC_MISC_FUNC_WAVEFORM_CAPTURE = 6,
    IMX_SC_MISC_FUNC_BUILD_INFO = 15,
    IMX_SC_MISC_FUNC_UNIQUE_ID = 19,
    IMX_SC_MISC_FUNC_SET_ARI = 3,
    IMX_SC_MISC_FUNC_BOOT_STATUS = 7,
    IMX_SC_MISC_FUNC_BOOT_DONE = 14,
    IMX_SC_MISC_FUNC_OTP_FUSE_READ = 11,
    IMX_SC_MISC_FUNC_OTP_FUSE_WRITE = 17,
    IMX_SC_MISC_FUNC_SET_TEMP = 12,
    IMX_SC_MISC_FUNC_GET_TEMP = 13,
    IMX_SC_MISC_FUNC_GET_BOOT_DEV = 16,
    IMX_SC_MISC_FUNC_GET_BUTTON_STATUS = 18,
}

/* Control Functions */

// CONFIG_IMX_SCU is a build-time C configuration condition.
#[cfg(feature = "CONFIG_IMX_SCU")]
extern "C" {
    pub fn imx_sc_misc_set_control(
        ipc: *mut imx_sc_ipc,
        resource: u32,
        ctrl: u8,
        val: u32,
    ) -> i32;

    pub fn imx_sc_misc_get_control(
        ipc: *mut imx_sc_ipc,
        resource: u32,
        ctrl: u8,
        val: *mut u32,
    ) -> i32;

    pub fn imx_sc_pm_cpu_start(
        ipc: *mut imx_sc_ipc,
        resource: u32,
        enable: bool,
        phys_addr: u64,
    ) -> i32;
}

// When CONFIG_IMX_SCU is disabled, the C header provides ENOTSUPP stubs.
#[cfg(not(feature = "CONFIG_IMX_SCU"))]
#[inline]
pub unsafe fn imx_sc_misc_set_control(
    _ipc: *mut imx_sc_ipc,
    _resource: u32,
    _ctrl: u8,
    _val: u32,
) -> i32 {
    -ENOTSUPP
}

#[cfg(not(feature = "CONFIG_IMX_SCU"))]
#[inline]
pub unsafe fn imx_sc_misc_get_control(
    _ipc: *mut imx_sc_ipc,
    _resource: u32,
    _ctrl: u8,
    _val: *mut u32,
) -> i32 {
    -ENOTSUPP
}

#[cfg(not(feature = "CONFIG_IMX_SCU"))]
#[inline]
pub unsafe fn imx_sc_pm_cpu_start(
    _ipc: *mut imx_sc_ipc,
    _resource: u32,
    _enable: bool,
    _phys_addr: u64,
) -> i32 {
    -ENOTSUPP
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
