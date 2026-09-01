// SPDX-License-Identifier: GPL-2.0

// C dependencies:
// #include <test_progs.h>
// #include "uninit_stack.skel.h"
// RUN_TESTS! and uninit_stack are supplied by translated external dependencies.

#[no_mangle]
pub unsafe extern "C" fn test_uninit_stack() {
    RUN_TESTS!(uninit_stack);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
