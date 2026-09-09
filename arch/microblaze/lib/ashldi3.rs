// SPDX-License-Identifier: GPL-2.0
// Dependency supplied by the kernel export interface: <linux/export.h>
// Dependency supplied by libgcc.h.

pub unsafe fn __ashldi3(u: i64, b: word_type) -> i64 {
    let mut uu: DWunion = core::mem::MaybeUninit::uninit().assume_init();
    let mut w: DWunion = core::mem::MaybeUninit::uninit().assume_init();
    let bm: word_type;

    if b == 0 {
        return u;
    }

    uu.ll = u;
    bm = 32 - b;

    if bm <= 0 {
        w.s.low = 0;
        w.s.high = (uu.s.low as u32) << (-bm);
    } else {
        let carries: u32 = (uu.s.low as u32) >> bm;

        w.s.low = (uu.s.low as u32) << b;
        w.s.high = ((uu.s.high as u32) << b) | carries;
    }

    w.ll
}

// EXPORT_SYMBOL(__ashldi3);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
