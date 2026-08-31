// SPDX-License-Identifier: GPL-2.0

// C dependencies:
// #include "compute_live_registers.skel.h"
// #include "test_progs.h"

#[no_mangle]
pub extern "C" fn test_compute_live_registers() {
    RUN_TESTS!(compute_live_registers);
}
