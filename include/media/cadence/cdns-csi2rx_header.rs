/* SPDX-License-Identifier: GPL-2.0+ */

// Dependency: <media/v4l2-subdev.h>

#[repr(C)]
pub struct v4l2_subdev {
    _private: [u8; 0],
}

/**
 * cdns_csi2rx_negotiate_ppc - Negotiate pixel-per-clock on output interface
 *
 * @subdev: point to &struct v4l2_subdev
 * @pad: pad number of the source pad
 * @ppc: pointer to requested pixel-per-clock value
 *
 * Returns 0 on success, negative error code otherwise.
 */
unsafe extern "C" {
    pub fn cdns_csi2rx_negotiate_ppc(
        subdev: *mut v4l2_subdev,
        pad: ::core::ffi::c_uint,
        ppc: *mut u8,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
