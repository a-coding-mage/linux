// SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause)

/*
 * BTF-to-C dumper test for multi-dimensional array output.
 *
 * Copyright (c) 2019 Facebook
 */
/* ----- START-EXPECTED-OUTPUT ----- */
pub type arr_t = [::std::os::raw::c_int; 2];

pub type multiarr_t = [[[::std::os::raw::c_int; 5]; 4]; 3];

pub type ptr_arr_t = [*mut ::std::os::raw::c_int; 6];

pub type ptr_multiarr_t = [[[[
    *mut ::std::os::raw::c_int;
    10
]; 9]; 8]; 7];

pub type fn_ptr_arr_t = [Option<unsafe extern "C" fn() -> *mut ::std::os::raw::c_int>; 11];

pub type fn_ptr_multiarr_t =
    [[Option<unsafe extern "C" fn() -> *mut ::std::os::raw::c_int>; 13]; 12];

#[repr(C)]
pub struct root_struct {
    pub _1: arr_t,
    pub _2: multiarr_t,
    pub _3: ptr_arr_t,
    pub _4: ptr_multiarr_t,
    pub _5: fn_ptr_arr_t,
    pub _6: fn_ptr_multiarr_t,
}

/* ------ END-EXPECTED-OUTPUT ------ */

#[no_mangle]
pub unsafe extern "C" fn f(_s: *mut root_struct) -> ::std::os::raw::c_int {
    return 0;
}
