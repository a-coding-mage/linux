// SPDX-License-Identifier: GPL-2.0

extern "C" {
    static mut __FPU_FPSCR: u32;
}

#[cfg(feature = "DEBUG")]
extern "C" {
    fn printk(fmt: *const u8, ...) -> i32;
}

pub unsafe fn mtfsb0(crbD: i32) -> i32 {
    if (crbD != 1) && (crbD != 2) {
        __FPU_FPSCR &= !(1u32 << (31 - crbD));
    }

    #[cfg(feature = "DEBUG")]
    {
        static FUNC: &[u8] = b"mtfsb0\0";
        static FORMAT: &[u8] = b"%s: %d %08lx\n\0";
        printk(
            FORMAT.as_ptr(),
            FUNC.as_ptr(),
            crbD,
            __FPU_FPSCR as u64,
        );
    }

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
