/* SPDX-License-Identifier: GPL-2.0 */

/* VIO */
#[repr(C)]
pub struct vio_device_id {
    pub type_: [i8; 32],
    pub compat: [i8; 32],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
