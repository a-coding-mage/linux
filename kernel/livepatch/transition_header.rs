/* SPDX-License-Identifier: GPL-2.0 */

// Dependency equivalent of: #include <linux/livepatch.h>

use std::os::raw::c_int;

#[repr(C)]
pub struct klp_patch {
    _private: [u8; 0],
}

extern "C" {
    pub static mut klp_transition_patch: *mut klp_patch;

    pub fn klp_init_transition(patch: *mut klp_patch, state: c_int);
    pub fn klp_cancel_transition();
    pub fn klp_start_transition();
    pub fn klp_try_complete_transition();
    pub fn klp_reverse_transition();
    pub fn klp_force_transition();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
