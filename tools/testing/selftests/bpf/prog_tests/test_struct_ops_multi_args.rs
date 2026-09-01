// SPDX-License-Identifier: GPL-2.0

// C dependencies:
// #include <test_progs.h>
// #include "struct_ops_multi_args.skel.h"

extern "C" {
    fn RUN_TESTS(name: *const ::core::ffi::c_char);
}

#[no_mangle]
pub unsafe extern "C" fn test_struct_ops_multi_args() {
    RUN_TESTS(b"struct_ops_multi_args\0".as_ptr() as *const ::core::ffi::c_char);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
