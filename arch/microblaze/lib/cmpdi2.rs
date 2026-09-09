// SPDX-License-Identifier: GPL-2.0
// Dependency supplied by the surrounding translation unit: libgcc.h

pub unsafe fn __cmpdi2(a: i64, b: i64) -> word_type {
    let au = DWunion { ll: a };
    let bu = DWunion { ll: b };

    if au.s.high < bu.s.high {
        return 0;
    } else if au.s.high > bu.s.high {
        return 2;
    }

    if (au.s.low as u32) < (bu.s.low as u32) {
        return 0;
    } else if (au.s.low as u32) > (bu.s.low as u32) {
        return 2;
    }

    1
}

// EXPORT_SYMBOL(__cmpdi2);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
