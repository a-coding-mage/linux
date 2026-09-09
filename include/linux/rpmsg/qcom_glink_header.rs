/* SPDX-License-Identifier: GPL-2.0 */

// C dependency: <linux/device.h>

use core::ffi::c_char;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct qcom_glink_smem {
    _private: [u8; 0],
}

// Build-time condition: IS_ENABLED(CONFIG_RPMSG_QCOM_GLINK).
#[cfg(feature = "CONFIG_RPMSG_QCOM_GLINK")]
unsafe extern "C" {
    pub fn qcom_glink_ssr_notify(ssr_name: *const c_char);
}

#[cfg(not(feature = "CONFIG_RPMSG_QCOM_GLINK"))]
#[inline]
pub unsafe fn qcom_glink_ssr_notify(_ssr_name: *const c_char) {}

// Build-time condition: IS_ENABLED(CONFIG_RPMSG_QCOM_GLINK_SMEM).
#[cfg(feature = "CONFIG_RPMSG_QCOM_GLINK_SMEM")]
unsafe extern "C" {
    pub fn qcom_glink_smem_register(
        parent: *mut device,
        node: *mut device_node,
    ) -> *mut qcom_glink_smem;
    pub fn qcom_glink_smem_unregister(glink: *mut qcom_glink_smem);
}

#[cfg(not(feature = "CONFIG_RPMSG_QCOM_GLINK_SMEM"))]
#[inline]
pub unsafe fn qcom_glink_smem_register(
    _parent: *mut device,
    _node: *mut device_node,
) -> *mut qcom_glink_smem {
    core::ptr::null_mut()
}

#[cfg(not(feature = "CONFIG_RPMSG_QCOM_GLINK_SMEM"))]
#[inline]
pub unsafe fn qcom_glink_smem_unregister(_glink: *mut qcom_glink_smem) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
