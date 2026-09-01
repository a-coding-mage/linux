// SPDX-License-Identifier: GPL-2.0
// C dependencies: <numa.h>, <numaif.h>

use std::os::raw::c_int;

extern "C" {
    fn numa_available() -> c_int;
}

fn main() -> c_int {
    unsafe {
        numa_available();
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
