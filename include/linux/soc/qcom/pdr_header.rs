/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent: declarations from <linux/soc/qcom/qmi.h> are supplied by
// other translated files.

pub const SERVREG_NAME_LENGTH: usize = 64;
pub const SERVREG_PFR_LENGTH: usize = 256;

#[repr(C)]
pub struct pdr_service {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pdr_handle {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum servreg_service_state {
    SERVREG_LOCATOR_ERR = 0x1,
    SERVREG_SERVICE_STATE_DOWN = 0x0FFFFFFF,
    SERVREG_SERVICE_STATE_UP = 0x1FFFFFFF,
    SERVREG_SERVICE_STATE_EARLY_DOWN = 0x2FFFFFFF,
    SERVREG_SERVICE_STATE_UNINIT = 0x7FFFFFFF,
}

unsafe extern "C" {
    pub fn pdr_handle_alloc(
        status: Option<unsafe extern "C" fn(state: i32, service_path: *mut core::ffi::c_char, priv_: *mut core::ffi::c_void)>,
        priv_: *mut core::ffi::c_void,
    ) -> *mut pdr_handle;

    pub fn pdr_add_lookup(
        pdr: *mut pdr_handle,
        service_name: *const core::ffi::c_char,
        service_path: *const core::ffi::c_char,
    ) -> *mut pdr_service;

    pub fn pdr_restart_pd(pdr: *mut pdr_handle, pds: *mut pdr_service) -> i32;

    pub fn pdr_handle_release(pdr: *mut pdr_handle);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
