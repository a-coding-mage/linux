/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2016-2018, The Linux Foundation. All rights reserved.
 */

// C dependencies: <soc/qcom/tcs.h>, <linux/platform_device.h>
// The CONFIG_QCOM_RPMH build condition is preserved below with Rust cfgs.

#[cfg(feature = "CONFIG_QCOM_RPMH")]
extern "C" {
    pub fn rpmh_read(dev: *const device, cmd: *mut tcs_cmd) -> i32;

    pub fn rpmh_write(
        dev: *const device,
        state: rpmh_state,
        cmd: *const tcs_cmd,
        n: u32,
    ) -> i32;

    pub fn rpmh_write_async(
        dev: *const device,
        state: rpmh_state,
        cmd: *const tcs_cmd,
        n: u32,
    ) -> i32;

    pub fn rpmh_write_batch(
        dev: *const device,
        state: rpmh_state,
        cmd: *const tcs_cmd,
        n: *mut u32,
    ) -> i32;

    pub fn rpmh_invalidate(dev: *const device);
}

#[cfg(not(feature = "CONFIG_QCOM_RPMH"))]
#[inline]
pub unsafe fn rpmh_read(_dev: *const device, _cmd: *mut tcs_cmd) -> i32 {
    -19 // -ENODEV
}

#[cfg(not(feature = "CONFIG_QCOM_RPMH"))]
#[inline]
pub unsafe fn rpmh_write(
    _dev: *const device,
    _state: rpmh_state,
    _cmd: *const tcs_cmd,
    _n: u32,
) -> i32 {
    -19 // -ENODEV
}

#[cfg(not(feature = "CONFIG_QCOM_RPMH"))]
#[inline]
pub unsafe fn rpmh_write_async(
    _dev: *const device,
    _state: rpmh_state,
    _cmd: *const tcs_cmd,
    _n: u32,
) -> i32 {
    -19 // -ENODEV
}

#[cfg(not(feature = "CONFIG_QCOM_RPMH"))]
#[inline]
pub unsafe fn rpmh_write_batch(
    _dev: *const device,
    _state: rpmh_state,
    _cmd: *const tcs_cmd,
    _n: *mut u32,
) -> i32 {
    -19 // -ENODEV
}

#[cfg(not(feature = "CONFIG_QCOM_RPMH"))]
#[inline]
pub unsafe fn rpmh_invalidate(_dev: *const device) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
