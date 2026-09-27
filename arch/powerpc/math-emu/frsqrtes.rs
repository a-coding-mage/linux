// SPDX-License-Identifier: GPL-2.0
// C dependencies: linux/types.h, linux/errno.h, and linux/uaccess.h.

#[cfg(DEBUG)]
unsafe extern "C" {
    fn printk(fmt: *const kernel::ffi::c_char, ...) -> kernel::ffi::c_int;
}

pub fn frsqrtes(frD: *mut kernel::ffi::c_void, frB: *mut kernel::ffi::c_void) -> kernel::ffi::c_int {
    #[cfg(DEBUG)]
    unsafe {
        static FORMAT: &[u8] = b"frsqrtes: %p %p\n\0";
        let _ = printk(FORMAT.as_ptr() as *const kernel::ffi::c_char, frD, frB);
    }

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
