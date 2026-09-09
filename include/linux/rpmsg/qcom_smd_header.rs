/* SPDX-License-Identifier: GPL-2.0 */

// Translated from linux/rpmsg/qcom_smd.h.
// The C header includes <linux/device.h>; these opaque declarations represent
// the externally supplied types used by this header.
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct qcom_smd_edge {
    _private: [u8; 0],
}

// Equivalent of IS_ENABLED(CONFIG_RPMSG_QCOM_SMD). The build configuration
// selects either the external declarations or the disabled inline stubs.
#[cfg(feature = "CONFIG_RPMSG_QCOM_SMD")]
extern "C" {
    pub fn qcom_smd_register_edge(
        parent: *mut device,
        node: *mut device_node,
    ) -> *mut qcom_smd_edge;

    pub fn qcom_smd_unregister_edge(edge: *mut qcom_smd_edge);
}

#[cfg(not(feature = "CONFIG_RPMSG_QCOM_SMD"))]
#[inline]
pub unsafe fn qcom_smd_register_edge(
    _parent: *mut device,
    _node: *mut device_node,
) -> *mut qcom_smd_edge {
    core::ptr::null_mut()
}

#[cfg(not(feature = "CONFIG_RPMSG_QCOM_SMD"))]
#[inline]
pub unsafe fn qcom_smd_unregister_edge(_edge: *mut qcom_smd_edge) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
