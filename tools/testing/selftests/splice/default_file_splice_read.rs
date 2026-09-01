// SPDX-License-Identifier: GPL-2.0
// C source used `_GNU_SOURCE` and included <fcntl.h> for splice(2).

use core::ffi::{c_int, c_uint};

type SizeT = usize;
type SsizeT = isize;
type LoffT = i64;

unsafe extern "C" {
    fn splice(
        fd_in: c_int,
        off_in: *mut LoffT,
        fd_out: c_int,
        off_out: *mut LoffT,
        len: SizeT,
        flags: c_uint,
    ) -> SsizeT;
}

fn main() {
    unsafe {
        splice(
            0,
            core::ptr::null_mut(),
            1,
            core::ptr::null_mut(),
            (1_i32 << 30) as SizeT,
            0,
        );
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
