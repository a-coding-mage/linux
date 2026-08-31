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
