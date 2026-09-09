/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright IBM Corp. 2024
 *
 * Author(s):
 *   Niklas Schnelle <schnelle@linux.ibm.com>
 *
 */

#[repr(C)]
pub struct zpci_dev {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn zpci_report_status(
        zdev: *mut zpci_dev,
        operation: *const core::ffi::c_char,
        status: *const core::ffi::c_char,
    ) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
