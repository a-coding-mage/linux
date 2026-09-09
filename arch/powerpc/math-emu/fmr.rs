// SPDX-License-Identifier: GPL-2.0
// Dependencies corresponding to linux/types.h, linux/errno.h, and
// linux/uaccess.h are supplied by the surrounding repository.

pub unsafe fn fmr(frD: *mut u32, frB: *mut u32) -> i32 {
    *frD.add(0) = *frB.add(0);
    *frD.add(1) = *frB.add(1);

    // The C DEBUG build-time condition is preserved here; the referenced
    // kernel debugging facilities are supplied by the surrounding repository.
    #[cfg(feature = "DEBUG")]
    {
        // printk("%s: D %p, B %p: ", __func__, frD, frB);
        // dump_double(frD);
        // printk("\n");
    }

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
