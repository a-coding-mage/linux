/* SPDX-License-Identifier: GPL-2.0 */
// Header guard: LINUX_DEVICE_ID_VCHIQ_H

use std::os::raw::c_char;

#[repr(C)]
pub struct vchiq_device_id {
    pub name: [c_char; 32],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
