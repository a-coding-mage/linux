// SPDX-License-Identifier: GPL-2.0

// C dependencies:
// #include <test_progs.h>
// #include "uninit_stack.skel.h"
// RUN_TESTS! and uninit_stack are supplied by translated external dependencies.

#[no_mangle]
pub unsafe extern "C" fn test_uninit_stack() {
    RUN_TESTS!(uninit_stack);
}
