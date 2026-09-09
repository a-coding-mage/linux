/* SPDX-License-Identifier: GPL-2.0 */

// Translated from nitrox_common.h.
// The declarations below are supplied by the corresponding device/request
// headers and kernel interfaces in the complete translation.

use core::ffi::{c_int, c_ulong, c_void};

#[repr(C)]
pub struct nitrox_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct se_crypto_request {
    _private: [u8; 0],
}

#[repr(C)]
pub struct work_struct {
    _private: [u8; 0],
}

// completion_t is defined by the kernel/request dependencies.
pub type completion_t = unsafe extern "C" fn();

extern "C" {
    pub fn nitrox_crypto_register() -> c_int;
    pub fn nitrox_crypto_unregister();
    pub fn nitrox_register_aeads() -> c_int;
    pub fn nitrox_unregister_aeads();
    pub fn nitrox_register_skciphers() -> c_int;
    pub fn nitrox_unregister_skciphers();
    pub fn crypto_alloc_context(ndev: *mut nitrox_device) -> *mut c_void;
    pub fn crypto_free_context(ctx: *mut c_void);
    pub fn nitrox_get_first_device() -> *mut nitrox_device;
    pub fn nitrox_put_device(ndev: *mut nitrox_device);

    pub fn nitrox_common_sw_init(ndev: *mut nitrox_device) -> c_int;
    pub fn nitrox_common_sw_cleanup(ndev: *mut nitrox_device);

    pub fn pkt_slc_resp_tasklet(data: c_ulong);
    pub fn nitrox_process_se_request(
        ndev: *mut nitrox_device,
        req: *mut se_crypto_request,
        cb: completion_t,
        cb_arg: *mut c_void,
    ) -> c_int;
    pub fn backlog_qflush_work(work: *mut work_struct);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
