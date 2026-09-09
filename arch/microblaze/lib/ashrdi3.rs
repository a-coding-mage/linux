// SPDX-License-Identifier: GPL-2.0
// The C source includes the declarations supplied by libgcc.h.

pub unsafe fn __ashrdi3(u: i64, b: word_type) -> i64 {
    let mut uu: DWunion = core::mem::zeroed();
    let mut w: DWunion = core::mem::zeroed();
    let bm: i32;

    if b == 0 {
        return u;
    }

    uu.ll = u;
    bm = 32 - b as i32;

    if bm <= 0 {
        // w.s.high = 1..1 or 0..0
        w.s.high = uu.s.high >> 31;
        w.s.low = uu.s.high >> (-bm as u32);
    } else {
        let carries = (uu.s.high as u32) << bm as u32;

        w.s.high = uu.s.high >> b as u32;
        w.s.low = ((uu.s.low as u32) >> b as u32) | carries;
    }

    w.ll
}

// EXPORT_SYMBOL(__ashrdi3);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
