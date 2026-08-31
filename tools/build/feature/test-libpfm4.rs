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
