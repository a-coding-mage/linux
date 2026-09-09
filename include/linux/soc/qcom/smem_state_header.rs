/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::c_char;

// Forward declarations supplied by other dependencies.
#[repr(C)]
pub struct device;
#[repr(C)]
pub struct device_node;
#[repr(C)]
pub struct qcom_smem_state;

#[repr(C)]
pub struct qcom_smem_state_ops {
    pub update_bits: Option<unsafe extern "C" fn(*mut core::ffi::c_void, u32, u32) -> i32>,
}

// CONFIG_QCOM_SMEM_STATE is a build-time condition from the original header.
#[cfg(feature = "CONFIG_QCOM_SMEM_STATE")]
extern "C" {
    pub fn qcom_smem_state_get(
        dev: *mut device,
        con_id: *const c_char,
        bit: *mut u32,
    ) -> *mut qcom_smem_state;
    pub fn devm_qcom_smem_state_get(
        dev: *mut device,
        con_id: *const c_char,
        bit: *mut u32,
    ) -> *mut qcom_smem_state;
    pub fn qcom_smem_state_put(state: *mut qcom_smem_state);
    pub fn qcom_smem_state_update_bits(
        state: *mut qcom_smem_state,
        mask: u32,
        value: u32,
    ) -> i32;
    pub fn qcom_smem_state_register(
        of_node: *mut device_node,
        ops: *const qcom_smem_state_ops,
        data: *mut core::ffi::c_void,
    ) -> *mut qcom_smem_state;
    pub fn qcom_smem_state_unregister(state: *mut qcom_smem_state);
}

#[cfg(not(feature = "CONFIG_QCOM_SMEM_STATE"))]
#[inline]
pub unsafe fn qcom_smem_state_get(
    _dev: *mut device,
    _con_id: *const c_char,
    _bit: *mut u32,
) -> *mut qcom_smem_state {
    core::mem::transmute::<isize, *mut qcom_smem_state>(-22)
}

#[cfg(not(feature = "CONFIG_QCOM_SMEM_STATE"))]
#[inline]
pub unsafe fn devm_qcom_smem_state_get(
    _dev: *mut device,
    _con_id: *const c_char,
    _bit: *mut u32,
) -> *mut qcom_smem_state {
    core::mem::transmute::<isize, *mut qcom_smem_state>(-22)
}

#[cfg(not(feature = "CONFIG_QCOM_SMEM_STATE"))]
#[inline]
pub unsafe fn qcom_smem_state_put(_state: *mut qcom_smem_state) {}

#[cfg(not(feature = "CONFIG_QCOM_SMEM_STATE"))]
#[inline]
pub unsafe fn qcom_smem_state_update_bits(
    _state: *mut qcom_smem_state,
    _mask: u32,
    _value: u32,
) -> i32 {
    -22
}

#[cfg(not(feature = "CONFIG_QCOM_SMEM_STATE"))]
#[inline]
pub unsafe fn qcom_smem_state_register(
    _of_node: *mut device_node,
    _ops: *const qcom_smem_state_ops,
    _data: *mut core::ffi::c_void,
) -> *mut qcom_smem_state {
    core::mem::transmute::<isize, *mut qcom_smem_state>(-22)
}

#[cfg(not(feature = "CONFIG_QCOM_SMEM_STATE"))]
#[inline]
pub unsafe fn qcom_smem_state_unregister(_state: *mut qcom_smem_state) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
