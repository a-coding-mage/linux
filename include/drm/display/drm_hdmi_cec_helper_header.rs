/* SPDX-License-Identifier: MIT */

use core::ffi::c_char;

pub enum drm_connector {}
pub enum cec_msg {}
pub enum device {}

#[repr(C)]
pub struct drm_connector_hdmi_cec_funcs {
    /**
     * @init: perform hardware-specific initialization before registering the CEC adapter
     */
    pub init: Option<unsafe extern "C" fn(connector: *mut drm_connector) -> core::ffi::c_int>,

    /**
     * @uninit: perform hardware-specific teardown for the CEC adapter
     */
    pub uninit: Option<unsafe extern "C" fn(connector: *mut drm_connector)>,

    /**
     * @enable: enable or disable CEC adapter
     */
    pub enable: Option<unsafe extern "C" fn(
        connector: *mut drm_connector,
        enable: bool,
    ) -> core::ffi::c_int>,

    /**
     * @log_addr: set adapter's logical address, can be called multiple
     * times if adapter supports several LAs
     */
    pub log_addr: Option<unsafe extern "C" fn(
        connector: *mut drm_connector,
        logical_addr: u8,
    ) -> core::ffi::c_int>,

    /**
     * @transmit: start transmission of the specified CEC message
     */
    pub transmit: Option<unsafe extern "C" fn(
        connector: *mut drm_connector,
        attempts: u8,
        signal_free_time: u32,
        msg: *mut cec_msg,
    ) -> core::ffi::c_int>,
}

extern "C" {
    pub fn drmm_connector_hdmi_cec_register(
        connector: *mut drm_connector,
        funcs: *const drm_connector_hdmi_cec_funcs,
        name: *const c_char,
        available_las: u8,
        dev: *mut device,
    ) -> core::ffi::c_int;

    pub fn drm_connector_hdmi_cec_received_msg(
        connector: *mut drm_connector,
        msg: *mut cec_msg,
    );

    pub fn drm_connector_hdmi_cec_transmit_done(
        connector: *mut drm_connector,
        status: u8,
        arb_lost_cnt: u8,
        nack_cnt: u8,
        low_drive_cnt: u8,
        error_cnt: u8,
    );

    pub fn drm_connector_hdmi_cec_transmit_attempt_done(
        connector: *mut drm_connector,
        status: u8,
    );
}

// When CONFIG_DRM_DISPLAY_HDMI_CEC_NOTIFIER_HELPER is enabled, this declaration
// is provided by the notifier helper implementation.
#[cfg(CONFIG_DRM_DISPLAY_HDMI_CEC_NOTIFIER_HELPER)]
extern "C" {
    pub fn drmm_connector_hdmi_cec_notifier_register(
        connector: *mut drm_connector,
        port_name: *const c_char,
        dev: *mut device,
    ) -> core::ffi::c_int;
}

// Equivalent to the disabled CONFIG_DRM_DISPLAY_HDMI_CEC_NOTIFIER_HELPER branch.
#[cfg(not(CONFIG_DRM_DISPLAY_HDMI_CEC_NOTIFIER_HELPER))]
#[inline]
pub unsafe fn drmm_connector_hdmi_cec_notifier_register(
    _connector: *mut drm_connector,
    _port_name: *const c_char,
    _dev: *mut device,
) -> core::ffi::c_int {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
