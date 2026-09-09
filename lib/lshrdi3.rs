// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * lib/lshrdi3.c
 */

// `word_type` and `DWunion` are supplied by the corresponding dependency.

pub unsafe fn __lshrdi3(u: i64, b: word_type) -> i64 {
    let mut uu: DWunion = core::mem::zeroed();
    let mut w: DWunion = core::mem::zeroed();
    let bm: word_type;

    if b == 0 {
        return u;
    }

    uu.ll = u;
    bm = (32 as word_type).wrapping_sub(b);

    if bm <= 0 {
        w.s.high = 0;
        w.s.low = (uu.s.high as u32) >> bm.wrapping_neg();
    } else {
        let carries: u32 = (uu.s.high as u32) << bm;

        w.s.high = (uu.s.high as u32) >> b;
        w.s.low = ((uu.s.low as u32) >> b) | carries;
    }

    w.ll
}

// EXPORT_SYMBOL(__lshrdi3);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
