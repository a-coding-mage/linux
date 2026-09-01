// SPDX-License-Identifier: GPL-2.0
// C dependency intent: #include <elfutils/debuginfod.h>

#[repr(C)]
pub struct debuginfod_client {
    _unused: [u8; 0],
}

extern "C" {
    fn debuginfod_begin() -> *mut debuginfod_client;
}

fn main() -> i32 {
    let c: *mut debuginfod_client = unsafe { debuginfod_begin() };
    (!c.is_null()) as i32
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
