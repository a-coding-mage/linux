// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the surrounding kernel/math-emulation code.

unsafe extern "C" {
    static mut __FPU_FPSCR: u32;
    fn printk(fmt: *const i8, ...);
}

pub unsafe fn mtfsfi(crfD: u32, IMM: u32) -> i32 {
    let mut mask: u32 = 0xf;

    if crfD == 0 {
        mask = 9;
    }

    let shift = (7u32.wrapping_sub(crfD)) << 2;
    __FPU_FPSCR &= !(mask << shift);
    __FPU_FPSCR |= (IMM & 0xf) << shift;

    // Equivalent to the C DEBUG preprocessor condition; enable the feature
    // when the surrounding build defines DEBUG.
    #[cfg(feature = "DEBUG")]
    {
        let function_name = b"mtfsfi\0";
        let format = b"%s: %d %x: %08lx\n\0";
        printk(
            format.as_ptr() as *const i8,
            function_name.as_ptr() as *const i8,
            crfD as i32,
            IMM,
            __FPU_FPSCR as u64,
        );
    }

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
