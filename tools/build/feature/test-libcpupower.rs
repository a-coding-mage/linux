// SPDX-License-Identifier: GPL-2.0
// C source included <cpuidle.h>; cpuidle_state_count is provided externally.

use std::os::raw::{c_int, c_uint};

unsafe extern "C" {
    fn cpuidle_state_count(cpu: c_uint) -> c_int;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main() -> c_int {
    let rv: c_int = unsafe { cpuidle_state_count(0) };
    return rv;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
