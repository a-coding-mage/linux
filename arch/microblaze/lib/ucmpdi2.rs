// SPDX-License-Identifier: GPL-2.0
//
// Dependency intent from libgcc.h: word_type and DWunion are supplied by the
// surrounding translation unit.  The C EXPORT_SYMBOL declaration is likewise
// supplied by the kernel build environment.

#[no_mangle]
pub extern "C" fn __ucmpdi2(a: u64, b: u64) -> u32 {
    let au_high: u32 = (a >> 32) as u32;
    let bu_high: u32 = (b >> 32) as u32;
    let au_low: u32 = a as u32;
    let bu_low: u32 = b as u32;

    if au_high < bu_high {
        return 0;
    } else if au_high > bu_high {
        return 2;
    }
    if au_low < bu_low {
        return 0;
    } else if au_low > bu_low {
        return 2;
    }
    1
}

// EXPORT_SYMBOL(__ucmpdi2);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
