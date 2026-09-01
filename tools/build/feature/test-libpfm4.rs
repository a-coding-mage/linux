// SPDX-License-Identifier: GPL-2.0
// C dependencies: <sys/types.h>, <perfmon/pfmlib.h>

unsafe extern "C" {
    fn pfm_initialize() -> ::std::os::raw::c_int;
}

fn main() {
    unsafe {
        pfm_initialize();
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
