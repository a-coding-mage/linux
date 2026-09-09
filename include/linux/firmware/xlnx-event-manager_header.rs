/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Xilinx Event Management Driver
 *
 * Copyright (C) 2024, Advanced Micro Devices, Inc.
 */

// Dependency supplied by linux/firmware/xlnx-zynqmp.h is referenced here.

pub const CB_MAX_PAYLOAD_SIZE: u32 = 4u32; /* In payload maximum 32bytes */

pub const EVENT_SUBSYSTEM_RESTART: u32 = 4u32;

pub const PM_DEV_ACPU_0_0: u32 = 0x1810c0afu32;
pub const PM_DEV_ACPU_0: u32 = 0x1810c003u32;

/* ************************ Exported Function **************************** */

pub type event_cb_func_t = unsafe extern "C" fn(payload: *const u32, data: *mut core::ffi::c_void);

/* IS_REACHABLE(CONFIG_XLNX_EVENT_MANAGER) */
#[cfg(feature = "xlnx_event_manager")]
extern "C" {
    pub fn xlnx_register_event(
        cb_type: crate::pm_api_cb_id,
        node_id: u32,
        event: u32,
        wake: bool,
        cb_fun: event_cb_func_t,
        data: *mut core::ffi::c_void,
    ) -> core::ffi::c_int;

    pub fn xlnx_unregister_event(
        cb_type: crate::pm_api_cb_id,
        node_id: u32,
        event: u32,
        cb_fun: event_cb_func_t,
        data: *mut core::ffi::c_void,
    ) -> core::ffi::c_int;
}

#[cfg(not(feature = "xlnx_event_manager"))]
pub unsafe fn xlnx_register_event(
    _cb_type: crate::pm_api_cb_id,
    _node_id: u32,
    _event: u32,
    _wake: bool,
    _cb_fun: event_cb_func_t,
    _data: *mut core::ffi::c_void,
) -> core::ffi::c_int {
    -19
}

#[cfg(not(feature = "xlnx_event_manager"))]
pub unsafe fn xlnx_unregister_event(
    _cb_type: crate::pm_api_cb_id,
    _node_id: u32,
    _event: u32,
    _cb_fun: event_cb_func_t,
    _data: *mut core::ffi::c_void,
) -> core::ffi::c_int {
    -19
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
