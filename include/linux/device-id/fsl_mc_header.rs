/* SPDX-License-Identifier: GPL-2.0 */

/**
 * struct fsl_mc_device_id - MC object device identifier
 * @vendor: vendor ID
 * @obj_type: MC object type
 *
 * Type of entries in the "device Id" table for MC object devices supported by
 * a MC object device driver. The last entry of the table has vendor set to 0x0
 */
#[repr(C)]
pub struct fsl_mc_device_id {
    pub vendor: __u16,
    pub obj_type: [::core::ffi::c_char; 16],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
