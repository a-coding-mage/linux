/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * cec-notifier.h - notify CEC drivers of physical address changes
 *
 * Copyright 2016 Russell King.
 * Copyright 2016-2017 Cisco Systems, Inc. and/or its affiliates. All rights reserved.
 */

// The C header includes <linux/err.h> and <media/cec.h>; their symbols are
// expected to be supplied by the surrounding translation unit.

#[repr(C)]
pub struct device;
#[repr(C)]
pub struct edid;
#[repr(C)]
pub struct cec_adapter;
#[repr(C)]
pub struct cec_notifier;
#[repr(C)]
pub struct cec_connector_info;

// Equivalent to: IS_REACHABLE(CONFIG_CEC_CORE) && IS_ENABLED(CONFIG_CEC_NOTIFIER)
// The enabled branch is preserved as declarations; select the configuration in
// the consuming build.

extern "C" {
    pub fn cec_notifier_conn_register(
        hdmi_dev: *mut device,
        port_name: *const core::ffi::c_char,
        conn_info: *const cec_connector_info,
    ) -> *mut cec_notifier;

    pub fn cec_notifier_conn_unregister(n: *mut cec_notifier);

    pub fn cec_notifier_cec_adap_register(
        hdmi_dev: *mut device,
        port_name: *const core::ffi::c_char,
        adap: *mut cec_adapter,
    ) -> *mut cec_notifier;

    pub fn cec_notifier_cec_adap_unregister(
        n: *mut cec_notifier,
        adap: *mut cec_adapter,
    );

    pub fn cec_notifier_set_phys_addr(n: *mut cec_notifier, pa: u16);

    pub fn cec_notifier_set_phys_addr_from_edid(
        n: *mut cec_notifier,
        edid: *const edid,
    );

    pub fn cec_notifier_parse_hdmi_phandle(dev: *mut device) -> *mut device;
}

// Disabled configuration branch from the C header. These inline definitions
// are retained under a Rust configuration predicate selected by the build.
#[cfg(not(all(feature = "cec_core_reachable", feature = "cec_notifier")))]
pub unsafe fn cec_notifier_conn_register_disabled(
    _hdmi_dev: *mut device,
    _port_name: *const core::ffi::c_char,
    _conn_info: *const cec_connector_info,
) -> *mut cec_notifier {
    // A non-NULL pointer is expected on success.
    0xdeadfeedusize as *mut cec_notifier
}

#[cfg(not(all(feature = "cec_core_reachable", feature = "cec_notifier")))]
pub unsafe fn cec_notifier_conn_unregister_disabled(_n: *mut cec_notifier) {}

#[cfg(not(all(feature = "cec_core_reachable", feature = "cec_notifier")))]
pub unsafe fn cec_notifier_cec_adap_register_disabled(
    _hdmi_dev: *mut device,
    _port_name: *const core::ffi::c_char,
    _adap: *mut cec_adapter,
) -> *mut cec_notifier {
    // A non-NULL pointer is expected on success.
    0xdeadfeedusize as *mut cec_notifier
}

#[cfg(not(all(feature = "cec_core_reachable", feature = "cec_notifier")))]
pub unsafe fn cec_notifier_cec_adap_unregister_disabled(
    _n: *mut cec_notifier,
    _adap: *mut cec_adapter,
) {
}

#[cfg(not(all(feature = "cec_core_reachable", feature = "cec_notifier")))]
pub unsafe fn cec_notifier_set_phys_addr_disabled(_n: *mut cec_notifier, _pa: u16) {}

#[cfg(not(all(feature = "cec_core_reachable", feature = "cec_notifier")))]
pub unsafe fn cec_notifier_set_phys_addr_from_edid_disabled(
    _n: *mut cec_notifier,
    _edid: *const edid,
) {
}

#[cfg(not(all(feature = "cec_core_reachable", feature = "cec_notifier")))]
pub unsafe fn cec_notifier_parse_hdmi_phandle_disabled(_dev: *mut device) -> *mut device {
    // ERR_PTR(-ENODEV), with -ENODEV supplied by linux/err.h in C.
    (-19isize) as *mut device
}

#[inline]
pub unsafe fn cec_notifier_phys_addr_invalidate(n: *mut cec_notifier) {
    cec_notifier_set_phys_addr(n, CEC_PHYS_ADDR_INVALID);
}

// Supplied by <media/cec.h>.
extern "C" {
    pub static CEC_PHYS_ADDR_INVALID: u16;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
