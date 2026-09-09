// SPDX-License-Identifier: GPL-2.0
// C dependencies: linux/types.h, linux/errno.h, and linux/uaccess.h.

#[cfg(DEBUG)]
unsafe extern "C" {
    fn printk(fmt: *const core::ffi::c_char, ...) -> core::ffi::c_int;
}

pub fn frsqrtes(frD: *mut core::ffi::c_void, frB: *mut core::ffi::c_void) -> core::ffi::c_int {
    #[cfg(DEBUG)]
    unsafe {
        static FORMAT: &[u8] = b"frsqrtes: %p %p\n\0";
        let _ = printk(FORMAT.as_ptr() as *const core::ffi::c_char, frD, frB);
    }

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
