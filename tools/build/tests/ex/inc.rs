// SPDX-License-Identifier: GPL-2.0
// C source conditionally included "krava.h" when INCLUDE was defined.

#[no_mangle]
pub extern "C" fn inc() -> ::std::os::raw::c_int {
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
