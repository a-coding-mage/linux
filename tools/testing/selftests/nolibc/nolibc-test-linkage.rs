/* SPDX-License-Identifier: GPL-2.0 */

// Translated from "nolibc-test-linkage.h" dependency and <errno.h>.
// The C source obtains `&errno`; keep `errno` as an external dependency.

use core::ffi::{c_int, c_void};

extern "C" {
    static mut errno: c_int;
}

#[no_mangle]
pub unsafe extern "C" fn linkage_test_errno_addr() -> *mut c_void {
    core::ptr::addr_of_mut!(errno).cast::<c_void>()
}

#[no_mangle]
pub static mut linkage_test_constructor_test_value: c_int = 0;

unsafe extern "C" fn constructor1() {
    linkage_test_constructor_test_value |= 1 << 0;
}

unsafe extern "C" fn constructor2() {
    linkage_test_constructor_test_value |= 1 << 1;
}

#[used]
#[cfg_attr(
    any(target_os = "linux", target_os = "android"),
    link_section = ".init_array"
)]
static CONSTRUCTOR1: unsafe extern "C" fn() = constructor1;

#[used]
#[cfg_attr(
    any(target_os = "linux", target_os = "android"),
    link_section = ".init_array"
)]
static CONSTRUCTOR2: unsafe extern "C" fn() = constructor2;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
