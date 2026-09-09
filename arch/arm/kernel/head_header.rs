// SPDX-License-Identifier: GPL-2.0-only

use core::ffi::{c_char, c_int};

unsafe extern "C" {
    static mut __data_loc: [c_char; 0];
    static mut _edata_loc: [c_char; 0];
    static mut _sdata: [c_char; 0];

    // The C `__init` annotation is a build/linker attribute supplied externally.
    fn __inflate_kernel_data() -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
