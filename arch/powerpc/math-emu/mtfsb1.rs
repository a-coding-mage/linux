// SPDX-License-Identifier: GPL-2.0

use core::ffi::{c_char, c_int, c_ulong};

extern "C" {
    static mut __FPU_FPSCR: c_ulong;
    #[cfg(feature = "DEBUG")]
    fn printk(format: *const c_char, ...) -> c_int;
}

#[no_mangle]
pub unsafe extern "C" fn mtfsb1(crbD: c_int) -> c_int {
    if (crbD != 1) && (crbD != 2) {
        __FPU_FPSCR |= 1 as c_ulong << (31 - crbD);
    }

    // The C source conditionally compiles this diagnostic under DEBUG.
    #[cfg(feature = "DEBUG")]
    {
        static FUNC: &[u8] = b"mtfsb1\0";
        static FORMAT: &[u8] = b"%s: %d %08lx\n\0";
        printk(
            FORMAT.as_ptr() as *const c_char,
            FUNC.as_ptr() as *const c_char,
            crbD,
            __FPU_FPSCR,
        );
    }

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
