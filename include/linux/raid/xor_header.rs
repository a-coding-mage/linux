/* SPDX-License-Identifier: GPL-2.0 */

// Original C header guard: _XOR_H

use core::ffi::c_void;

extern "C" {
    pub fn xor_gen(
        dest: *mut c_void,
        srcs: *mut *mut c_void,
        src_cnt: u32,
        bytes: u32,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
