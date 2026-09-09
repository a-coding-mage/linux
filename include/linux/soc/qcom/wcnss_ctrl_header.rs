/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::{c_char, c_void};

// Dependency supplied by the Linux rpmsg interface.
pub struct rpmsg_endpoint;

// The CONFIG_QCOM_WCNSS_CTRL condition is a build-time kernel configuration
// condition; it is represented here with the corresponding Rust cfg feature.
#[cfg(feature = "CONFIG_QCOM_WCNSS_CTRL")]
extern "C" {
    pub fn qcom_wcnss_open_channel(
        wcnss: *mut c_void,
        name: *const c_char,
        cb: rpmsg_rx_cb_t,
        priv_: *mut c_void,
    ) -> *mut rpmsg_endpoint;
}

#[cfg(not(feature = "CONFIG_QCOM_WCNSS_CTRL"))]
pub unsafe fn qcom_wcnss_open_channel(
    _wcnss: *mut c_void,
    _name: *const c_char,
    _cb: rpmsg_rx_cb_t,
    _priv_: *mut c_void,
) -> *mut rpmsg_endpoint {
    // Equivalent to WARN_ON(1); the kernel warning primitive is supplied by
    // the surrounding kernel bindings.
    WARN_ON(1);
    ERR_PTR(-ENXIO)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
