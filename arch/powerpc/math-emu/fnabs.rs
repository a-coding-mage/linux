// SPDX-License-Identifier: GPL-2.0

// The original implementation includes Linux type, errno, and uaccess
// headers.  The required u32 type is available directly in Rust.

#[cfg(feature = "DEBUG")]
unsafe extern "C" {
    fn printk(fmt: *const core::ffi::c_char, ...) -> i32;
    fn dump_double(value: *const u32);
}

pub unsafe fn fnabs(frD: *mut u32, frB: *mut u32) -> i32 {
    *frD.add(0) = *frB.add(0) | 0x8000_0000;
    *frD.add(1) = *frB.add(1);

    #[cfg(feature = "DEBUG")]
    {
        static FUNC_NAME: &[u8] = b"fnabs\0";
        static FORMAT: &[u8] = b"%s: D %p, B %p: \0";
        static NEWLINE: &[u8] = b"\n\0";

        printk(
            FORMAT.as_ptr() as *const core::ffi::c_char,
            FUNC_NAME.as_ptr(),
            frD,
            frB,
        );
        dump_double(frD);
        printk(NEWLINE.as_ptr() as *const core::ffi::c_char);
    }

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
