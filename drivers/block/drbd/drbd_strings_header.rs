/* SPDX-License-Identifier: GPL-2.0-only */

// The enum types are supplied by the corresponding dependency.
extern "C" {
    pub fn drbd_conn_str(arg: crate::drbd_conns) -> *const core::ffi::c_char;
    pub fn drbd_role_str(arg: crate::drbd_role) -> *const core::ffi::c_char;
    pub fn drbd_disk_str(arg: crate::drbd_disk_state) -> *const core::ffi::c_char;
    pub fn drbd_set_st_err_str(arg: crate::drbd_state_rv) -> *const core::ffi::c_char;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
