// SPDX-License-Identifier: GPL-2.0-or-later
/*
 */

// <linux/export.h>
// <linux/libgcc.h>

pub type word_type = u32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct DWstruct {
    pub low: i32,
    pub high: i32,
}

#[repr(C)]
pub union DWunion {
    pub ll: i64,
    pub s: DWstruct,
}

#[no_mangle]
pub unsafe extern "C" fn __ashrdi3(u: i64, b: word_type) -> i64 {
    let mut uu = DWunion { ll: u };
    let mut w = DWunion { ll: 0 };
    let bm: word_type;

    if b == 0 {
        return u;
    }

    bm = 32u32.wrapping_sub(b);

    if (bm as i32) <= 0 {
        // w.s.high = 1..1 or 0..0
        (*(&mut w.s as *mut DWstruct)).high = (*(&uu.s as *const DWstruct)).high >> 31;
        (*(&mut w.s as *mut DWstruct)).low =
            (*(&uu.s as *const DWstruct)).high >> (bm.wrapping_neg());
    } else {
        let carries: u32 = (*(&uu.s as *const DWstruct)).high as u32;
        let carries = carries.wrapping_shl(bm);

        (*(&mut w.s as *mut DWstruct)).high =
            (*(&uu.s as *const DWstruct)).high >> b;
        (*(&mut w.s as *mut DWstruct)).low =
            (((*(&uu.s as *const DWstruct)).low as u32) >> b) | carries;
    }

    w.ll
}

// EXPORT_SYMBOL(__ashrdi3);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
