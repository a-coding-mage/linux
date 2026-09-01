// SPDX-License-Identifier: GPL-2.0

// C dependencies:
// #include "compute_live_registers.skel.h"
// #include "test_progs.h"

#[no_mangle]
pub extern "C" fn test_compute_live_registers() {
    RUN_TESTS!(compute_live_registers);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
