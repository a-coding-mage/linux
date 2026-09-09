// SPDX-License-Identifier: GPL-2.0-or-later
/*
 */

// `word_type` is supplied by the corresponding external dependency.

pub unsafe fn __ucmpdi2(a: u64, b: u64) -> word_type {
    // This is the direct effect of initializing DWunion.ll and reading its
    // unsigned high and low 32-bit members.
    let a_high = (a >> 32) as u32;
    let b_high = (b >> 32) as u32;
    let a_low = a as u32;
    let b_low = b as u32;

    if a_high < b_high {
        0
    } else if a_high > b_high {
        2
    } else if a_low < b_low {
        0
    } else if a_low > b_low {
        2
    } else {
        1
    }
}

// EXPORT_SYMBOL(__ucmpdi2);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
