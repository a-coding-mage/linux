// SPDX-License-Identifier: GPL-2.0
// C dependency intent: #include <tracefs.h>

use std::ffi::c_char;

#[repr(C)]
pub struct tracefs_instance {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn tracefs_instance_create(name: *const c_char) -> *mut tracefs_instance;
    fn tracefs_instance_destroy(instance: *mut tracefs_instance);
}

fn main() {
    let inst: *mut tracefs_instance =
        unsafe { tracefs_instance_create(c"dummy".as_ptr() as *const c_char) };

    unsafe {
        tracefs_instance_destroy(inst);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
