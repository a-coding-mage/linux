// SPDX-License-Identifier: GPL-2.0
// Dependencies corresponding to the original Linux and math-emu includes are
// supplied by the surrounding translation unit.

pub unsafe fn fctiw(frD: *mut u32, frB: *mut core::ffi::c_void) -> i32 {
    FP_DECL_D!(B);
    FP_DECL_EX!();
    let mut r: u32;

    FP_UNPACK_DP!(B, frB);
    FP_TO_INT_D!(r, B, 32, 1);
    *frD.add(1) = r;

    #[cfg(feature = "DEBUG")]
    {
        printk!("%s: D %p, B %p: ", c"fctiw".as_ptr(), frD, frB);
        dump_double!(frD);
        printk!("\n");
    }

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
