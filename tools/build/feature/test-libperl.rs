// SPDX-License-Identifier: GPL-2.0
// C dependency intent: #include <EXTERN.h>
// C dependency intent: #include <perl.h>

unsafe extern "C" {
    fn perl_alloc() -> *mut core::ffi::c_void;
}

fn main() {
    unsafe {
        perl_alloc();
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
