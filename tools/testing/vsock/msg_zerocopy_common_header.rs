// SPDX-License-Identifier: GPL-2.0-only

// C header dependency: <stdbool.h>

// Fallback definitions from the C preprocessor when not already supplied.
pub const SOL_VSOCK: i32 = 287;
pub const VSOCK_RECVERR: i32 = 1;

unsafe extern "C" {
    pub fn vsock_recv_completion(fd: ::std::os::raw::c_int, zerocopied: *const bool);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
