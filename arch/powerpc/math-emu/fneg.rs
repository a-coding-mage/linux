// SPDX-License-Identifier: GPL-2.0

// Dependencies supplied by the surrounding implementation:
// linux/types.h, linux/errno.h, and linux/uaccess.h

#[cfg(feature = "DEBUG")]
unsafe extern "C" {
    fn printk(fmt: *const i8, ...);
    fn dump_double(value: *const u32);
}

pub unsafe fn fneg(frD: *mut u32, frB: *mut u32) -> i32 {
    unsafe {
        *frD.add(0) = *frB.add(0) ^ 0x80000000;
        *frD.add(1) = *frB.add(1);

        // Corresponds to the C DEBUG build-time condition.
        #[cfg(feature = "DEBUG")]
        {
            static FUNCTION_NAME: &[u8] = b"fneg\0";
            static DEST_SOURCE_FORMAT: &[u8] = b"%s: D %p, B %p: \0";
            static NEWLINE: &[u8] = b"\n\0";

            printk(
                DEST_SOURCE_FORMAT.as_ptr() as *const i8,
                FUNCTION_NAME.as_ptr() as *const i8,
                frD,
                frB,
            );
            dump_double(frD);
            printk(NEWLINE.as_ptr() as *const i8);
        }

        0
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
