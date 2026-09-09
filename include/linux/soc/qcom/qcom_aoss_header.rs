/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2021, The Linux Foundation. All rights reserved.
 */

// Translated from qcom_aoss.h.
// The Linux headers <linux/err.h> and <linux/device.h> provide the original
// error-pointer helpers and device type.

#[repr(C)]
pub struct qmp {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

// IS_ENABLED(CONFIG_QCOM_AOSS_QMP)
#[cfg(CONFIG_QCOM_AOSS_QMP)]
unsafe extern "C" {
    pub fn qmp_send(qmp: *mut qmp, fmt: *const core::ffi::c_char, ...) -> i32;
    pub fn qmp_get(dev: *mut device) -> *mut qmp;
    pub fn qmp_put(qmp: *mut qmp);
}

// The following inline definitions are used when CONFIG_QCOM_AOSS_QMP is not enabled.
#[cfg(not(CONFIG_QCOM_AOSS_QMP))]
#[inline]
pub unsafe fn qmp_send(
    _qmp: *mut qmp,
    _fmt: *const core::ffi::c_char,
    ...
) -> i32 {
    -19 // -ENODEV
}

#[cfg(not(CONFIG_QCOM_AOSS_QMP))]
#[inline]
pub unsafe fn qmp_get(_dev: *mut device) -> *mut qmp {
    (-19isize) as *mut qmp // ERR_PTR(-ENODEV)
}

#[cfg(not(CONFIG_QCOM_AOSS_QMP))]
#[inline]
pub unsafe fn qmp_put(_qmp: *mut qmp) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
