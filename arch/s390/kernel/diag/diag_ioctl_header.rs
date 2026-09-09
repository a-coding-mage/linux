/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::{c_long, c_ulong};

extern "C" {
    pub fn diag324_pibbuf(arg: c_ulong) -> c_long;
    pub fn diag324_piblen(arg: c_ulong) -> c_long;

    pub fn diag310_memtop_stride(arg: c_ulong) -> c_long;
    pub fn diag310_memtop_len(arg: c_ulong) -> c_long;
    pub fn diag310_memtop_buf(arg: c_ulong) -> c_long;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
