// SPDX-License-Identifier: GPL-2.0-only

// The C source includes `test_fortify.h`, which supplies the surrounding
// declarations and invokes this macro in its test harness.
macro_rules! TEST {
    () => {{
        unsafe {
            std::ptr::copy_nonoverlapping(
                large_src.as_ptr() as *const u8,
                instance.buf.as_mut_ptr() as *mut u8,
                std::mem::size_of_val(&large_src),
            );
        }
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
