// SPDX-License-Identifier: GPL-2.0
//
// C dependencies:
// - linux/types.h provides u32
// - linux/errno.h and linux/uaccess.h are included by the original source

#[cfg(feature = "DEBUG")]
unsafe extern "C" {
    fn printk(fmt: *const core::ffi::c_char, ...);
    fn dump_double(value: *mut u32);
}

pub unsafe fn fabs(frD: *mut u32, frB: *mut u32) -> i32 {
    *frD.add(0) = *frB.add(0) & 0x7fffffff;
    *frD.add(1) = *frB.add(1);

    // The original DEBUG preprocessor condition is preserved as a Cargo
    // feature condition; enable the `DEBUG` feature to include this block.
    #[cfg(feature = "DEBUG")]
    {
        static FUNC: &[u8] = b"fabs\0";
        static FORMAT: &[u8] = b"%s: D %p, B %p: \0";
        static NEWLINE: &[u8] = b"\n\0";

        printk(
            FORMAT.as_ptr() as *const core::ffi::c_char,
            FUNC.as_ptr() as *const core::ffi::c_char,
            frD,
            frB,
        );
        dump_double(frD);
        printk(NEWLINE.as_ptr() as *const core::ffi::c_char);
    }

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
