// SPDX-License-Identifier: GPL-2.0
// Dependencies corresponding to <linux/types.h>, <linux/errno.h>, and
// <linux/uaccess.h> are supplied by the surrounding translation unit.

#[cfg(DEBUG)]
unsafe extern "C" {
    fn printk(format: *const core::ffi::c_char, ...) -> core::ffi::c_int;
}

pub unsafe fn fres(frD: *mut core::ffi::c_void, frB: *mut core::ffi::c_void) -> i32 {
    #[cfg(DEBUG)]
    {
        // Corresponds to: printk("%s: %p %p\n", __func__, frD, frB);
        // The surrounding kernel translation supplies the printk interface.
        unsafe {
            printk(
                c"%s: %p %p\n".as_ptr(),
                c"fres".as_ptr(),
                frD,
                frB,
            );
        }
    }
    -crate::ENOSYS
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
