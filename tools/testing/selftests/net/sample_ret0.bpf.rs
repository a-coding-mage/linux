/* SPDX-License-Identifier: (GPL-2.0 OR BSD-2-Clause) */

// #define SEC(name) __attribute__((section(name), used))

/* Sample program which should always load for testing control paths. */
#[no_mangle]
#[link_section = "xdp"]
#[used]
pub static FUNC: unsafe extern "C" fn() -> ::core::ffi::c_int = func;

unsafe extern "C" fn func() -> ::core::ffi::c_int {
    return 0;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
