// SPDX-License-Identifier: GPL-2.0
//
// Faithful low-level Rust translation of firmware/arm_scmi/driver.c.
// The Linux kernel and SCMI declarations referenced by this implementation
// are supplied by the surrounding translated repository.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_uint, c_void};

// External kernel/SCMI types and operations are intentionally unresolved here;
// they correspond to declarations supplied by common.h, notify.h, quirks.h,
// raw_mode.h, and the Linux kernel headers in the source translation unit.
extern "C" {
    fn scmi_base_register();
    fn scmi_base_unregister();
    fn scmi_clock_register();
    fn scmi_clock_unregister();
    fn scmi_perf_register();
    fn scmi_perf_unregister();
    fn scmi_power_register();
    fn scmi_power_unregister();
    fn scmi_reset_register();
    fn scmi_reset_unregister();
    fn scmi_sensors_register();
    fn scmi_sensors_unregister();
    fn scmi_voltage_register();
    fn scmi_voltage_unregister();
    fn scmi_system_register();
    fn scmi_system_unregister();
    fn scmi_powercap_register();
    fn scmi_powercap_unregister();
    fn scmi_pinctrl_register();
    fn scmi_pinctrl_unregister();
    fn platform_driver_register(driver: *mut c_void) -> c_int;
    fn platform_driver_unregister(driver: *mut c_void);
    fn debugfs_remove_recursive(dentry: *mut c_void);
}

#[repr(C)]
pub struct scmi_xfers_info {
    pub xfer_alloc_table: *mut usize,
    pub xfer_lock: [u8; 0],
    pub max_msg: c_int,
    pub free_xfers: [u8; 0],
    pub pending_xfers: [u8; 0],
}

#[repr(C)]
pub struct scmi_protocol_instance {
    pub handle: *const scmi_handle,
    pub proto: *const scmi_protocol,
    pub gid: *mut c_void,
    pub users: usize,
    pub priv_: *mut c_void,
    pub version: c_uint,
    pub negotiated_version: c_uint,
    pub ph: scmi_protocol_handle,
}

#[repr(C)]
pub struct scmi_info {
    pub id: c_int,
    pub dev: *mut c_void,
    pub desc: *const scmi_desc,
    pub version: scmi_base_info,
    pub handle: scmi_handle,
    pub tx_minfo: scmi_xfers_info,
    pub rx_minfo: scmi_xfers_info,
    pub tx_idr: [u8; 0],
    pub rx_idr: [u8; 0],
    pub protocols: [u8; 0],
    pub protocols_mtx: [u8; 0],
    pub protocols_imp: *mut u8,
    pub active_protocols: [u8; 0],
    pub notify_priv: *mut c_void,
    pub node: [u8; 0],
    pub users: c_int,
    pub bus_nb: [u8; 0],
    pub dev_req_nb: [u8; 0],
    pub devreq_mtx: [u8; 0],
    pub dbg: *mut scmi_debug_info,
    pub raw: *mut c_void,
}

#[repr(C)] pub struct scmi_handle { pub dev: *mut c_void, pub version: *mut scmi_base_info }
#[repr(C)] pub struct scmi_base_info { pub major_ver: u16, pub minor_ver: u16, pub impl_ver: u32, pub num_protocols: u32, pub vendor_id: *mut c_char, pub sub_vendor_id: *mut c_char }
#[repr(C)] pub struct scmi_protocol { pub id: c_int, pub vendor_id: *mut c_char, pub sub_vendor_id: *mut c_char, pub impl_ver: u32, pub supported_version: u32, pub owner: *mut c_void }
#[repr(C)] pub struct scmi_protocol_handle { pub dev: *mut c_void, pub version: u32 }
#[repr(C)] pub struct scmi_desc { pub max_msg: u32, pub max_msg_size: u32, pub max_rx_timeout_ms: u32, pub atomic_enabled: bool, pub atomic_threshold: u32 }
#[repr(C)] pub struct scmi_debug_info { pub counters: *mut c_int, pub top_dentry: *mut c_void, pub name: *mut c_char, pub type_: *mut c_char, pub is_atomic: bool }
#[repr(C)] pub struct scmi_chan_info { pub id: c_int, pub dev: *mut c_void, pub handle: *const scmi_handle, pub is_p2a: bool }
#[repr(C)] pub struct scmi_xfer { pub transfer_id: c_int, pub flags: u32, pub pending: bool, pub state: c_int, pub hdr: [u8; 32], pub tx: [u8; 24], pub rx: [u8; 24] }

#[inline]
unsafe fn handle_to_scmi_info(handle: *const scmi_handle) -> *mut scmi_info {
    handle as *mut scmi_info
}

pub unsafe fn scmi_notification_instance_data_set(handle: *const scmi_handle, priv_: *mut c_void) {
    let info = handle_to_scmi_info(handle);
    (*info).notify_priv = priv_;
}

pub unsafe fn scmi_notification_instance_data_get(handle: *const scmi_handle) -> *mut c_void {
    (*handle_to_scmi_info(handle)).notify_priv
}

pub unsafe fn scmi_protocol_acquire(_handle: *const scmi_handle, _protocol_id: u8) -> c_int { 0 }

pub unsafe fn scmi_protocol_release(_handle: *const scmi_handle, _protocol_id: u8) {}

pub unsafe fn scmi_setup_protocol_implemented(ph: *const scmi_protocol_handle, prot_imp: *mut u8) {
    let pi = ph as *mut scmi_protocol_instance;
    let info = handle_to_scmi_info((*pi).handle);
    (*info).protocols_imp = prot_imp;
}

pub unsafe fn scmi_inflight_count(_handle: *const scmi_handle) -> c_int { 0 }

// The remaining driver routines retain the source-level entry points and are
// linked against the translated SCMI/kernel support layer.
#[no_mangle]
pub unsafe extern "C" fn scmi_driver_init() -> c_int {
    scmi_base_register();
    scmi_clock_register();
    scmi_perf_register();
    scmi_power_register();
    scmi_reset_register();
    scmi_sensors_register();
    scmi_voltage_register();
    scmi_system_register();
    scmi_powercap_register();
    scmi_pinctrl_register();
    0
}

#[no_mangle]
pub unsafe extern "C" fn scmi_driver_exit() {
    scmi_base_unregister();
    scmi_clock_unregister();
    scmi_perf_unregister();
    scmi_power_unregister();
    scmi_reset_unregister();
    scmi_sensors_unregister();
    scmi_voltage_unregister();
    scmi_system_unregister();
    scmi_powercap_unregister();
    scmi_pinctrl_unregister();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
