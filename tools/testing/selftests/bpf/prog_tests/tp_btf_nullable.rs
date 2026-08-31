// SPDX-License-Identifier: GPL-2.0

// C dependencies:
// #include <test_progs.h>
// #include "test_tp_btf_nullable.skel.h"

extern "C" {
    static env: Env;

    fn test__skip();
}

#[repr(C)]
struct Env {
    has_testmod: bool,
}

// RUN_TESTS(test_tp_btf_nullable)
extern "C" {
    fn RUN_TESTS_test_tp_btf_nullable();
}

#[no_mangle]
pub unsafe extern "C" fn test_tp_btf_nullable() {
    if !env.has_testmod {
        test__skip();
        return;
    }

    RUN_TESTS_test_tp_btf_nullable();
}
