/* SPDX-License-Identifier: GPL-2.0 */

#[repr(C)]
pub struct completion {
    pub done: ::std::os::raw::c_uint,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
