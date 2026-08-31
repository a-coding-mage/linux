/* SPDX-License-Identifier: GPL-2.0 */

#[repr(C)]
pub struct completion {
    pub done: ::std::os::raw::c_uint,
}
