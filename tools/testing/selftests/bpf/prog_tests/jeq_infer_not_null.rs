// SPDX-License-Identifier: GPL-2.0

// C dependencies:
// #include <test_progs.h>
// #include "jeq_infer_not_null_fail.skel.h"

extern "C" {
    fn RUN_TESTS(test_name: *const ::std::os::raw::c_char);
}

#[no_mangle]
pub unsafe extern "C" fn test_jeq_infer_not_null() {
    RUN_TESTS(b"jeq_infer_not_null_fail\0".as_ptr() as *const ::std::os::raw::c_char);
}
