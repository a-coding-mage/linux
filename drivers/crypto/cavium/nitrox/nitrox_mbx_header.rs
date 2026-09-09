/* SPDX-License-Identifier: GPL-2.0 */

// Declaration of the externally defined device type used by this interface.
pub enum nitrox_device {}

extern "C" {
    pub fn nitrox_mbox_init(ndev: *mut nitrox_device) -> ::core::ffi::c_int;
    pub fn nitrox_mbox_cleanup(ndev: *mut nitrox_device);
    pub fn nitrox_pf2vf_mbox_handler(ndev: *mut nitrox_device);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
