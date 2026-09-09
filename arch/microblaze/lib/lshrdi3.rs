// SPDX-License-Identifier: GPL-2.0
// Dependency supplied externally: libgcc.h (including word_type and DWunion).

pub unsafe fn __lshrdi3(u: i64, b: word_type) -> i64 {
    let mut uu: DWunion = core::mem::zeroed();
    let mut w: DWunion = core::mem::zeroed();
    let bm: word_type;

    if b == 0 {
        return u;
    }

    uu.ll = u;
    bm = 32 - b;

    if bm <= 0 {
        w.s.high = 0;
        w.s.low = (uu.s.high as u32) >> (-bm as u32);
    } else {
        let carries: u32 = (uu.s.high as u32).wrapping_shl(bm as u32);

        w.s.high = (uu.s.high as u32) >> (b as u32);
        w.s.low = ((uu.s.low as u32) >> (b as u32)) | carries;
    }

    w.ll
}

// EXPORT_SYMBOL(__lshrdi3);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
