// SPDX-License-Identifier: GPL-2.0-only

// C dependencies:
// #include <test_progs.h>
// #include "mem_rdonly_untrusted.skel.h"

extern "C" {
    fn RUN_TESTS(test_name: *const ::std::os::raw::c_char);
}

#[no_mangle]
pub unsafe extern "C" fn test_mem_rdonly_untrusted() {
    RUN_TESTS(b"mem_rdonly_untrusted\0".as_ptr() as *const ::std::os::raw::c_char);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
