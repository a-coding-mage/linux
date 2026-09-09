/* SPDX-License-Identifier: GPL-2.0 */
/*
 *  IBM System z PNET ID Support
 *
 *    Copyright IBM Corp. 2018
 */

// Dependencies supplied by the surrounding kernel translation:
// `struct device` corresponds to the external device type.

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn pnet_id_by_dev_port(
        dev: *mut device,
        port: u16,
        pnetid: *mut u8,
    ) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
