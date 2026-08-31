/* SPDX-License-Identifier: GPL-2.0 */

unsafe extern "C" {
    pub fn linkage_test_errno_addr() -> *mut core::ffi::c_void;
    pub static mut linkage_test_constructor_test_value: core::ffi::c_int;
}
