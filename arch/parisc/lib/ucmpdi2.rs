// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the Linux kernel: <linux/module.h>, <linux/libgcc.h>

#[repr(C)]
pub union ull_union {
    pub ull: u64,
    pub ui: ull_union_ui,
}

#[repr(C)]
pub struct ull_union_ui {
    pub high: u32,
    pub low: u32,
}

pub unsafe fn __ucmpdi2(a: u64, b: u64) -> word_type {
    let au = ull_union { ull: a };
    let bu = ull_union { ull: b };

    if (*(&au.ui as *const ull_union_ui)).high < (*(&bu.ui as *const ull_union_ui)).high {
        return 0;
    } else if (*(&au.ui as *const ull_union_ui)).high > (*(&bu.ui as *const ull_union_ui)).high {
        return 2;
    }
    if (*(&au.ui as *const ull_union_ui)).low < (*(&bu.ui as *const ull_union_ui)).low {
        return 0;
    } else if (*(&au.ui as *const ull_union_ui)).low > (*(&bu.ui as *const ull_union_ui)).low {
        return 2;
    }
    1
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
