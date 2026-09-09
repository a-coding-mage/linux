/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Translation of multi_arith.h.  The fp_* types and status helpers are
 * supplied by the surrounding floating-point emulation code. */

#[inline]
pub unsafe fn fp_denormalize(reg: *mut crate::fp_ext, cnt: u32) {
    (*reg).exp += cnt as _;
    match cnt {
        0..=8 => {
            (*reg).lowmant = (*reg).mant.m32[1] << (8 - cnt);
            (*reg).mant.m32[1] = ((*reg).mant.m32[1] >> cnt)
                | ((*reg).mant.m32[0] << (32 - cnt));
            (*reg).mant.m32[0] >>= cnt;
        }
        9..=32 => {
            (*reg).lowmant = (*reg).mant.m32[1] >> (cnt - 8);
            if (*reg).mant.m32[1] << (40 - cnt) != 0 { (*reg).lowmant |= 1; }
            (*reg).mant.m32[1] = ((*reg).mant.m32[1] >> cnt)
                | ((*reg).mant.m32[0] << (32 - cnt));
            (*reg).mant.m32[0] >>= cnt;
        }
        33..=39 => {
            (*reg).lowmant = (*reg).mant.m32[0] >> (cnt - 32);
            if (*reg).mant.m32[1] << (40 - cnt) != 0 { (*reg).lowmant |= 1; }
            (*reg).mant.m32[1] = (*reg).mant.m32[0] >> (cnt - 32);
            (*reg).mant.m32[0] = 0;
        }
        40..=71 => {
            (*reg).lowmant = (*reg).mant.m32[0] >> (cnt - 40);
            if ((*reg).mant.m32[0] << (72 - cnt)) != 0 || (*reg).mant.m32[1] != 0 {
                (*reg).lowmant |= 1;
            }
            (*reg).mant.m32[1] = (*reg).mant.m32[0] >> (cnt - 32);
            (*reg).mant.m32[0] = 0;
        }
        _ => {
            (*reg).lowmant = ((*reg).mant.m32[0] != 0 || (*reg).mant.m32[1] != 0) as _;
            (*reg).mant.m32[0] = 0;
            (*reg).mant.m32[1] = 0;
        }
    }
}

#[inline]
pub unsafe fn fp_overnormalize(reg: *mut crate::fp_ext) -> i32 {
    let shift: u32;
    if (*reg).mant.m32[0] != 0 {
        shift = (*reg).mant.m32[0].leading_zeros();
        (*reg).mant.m32[0] = ((*reg).mant.m32[0] << shift)
            | ((*reg).mant.m32[1] >> (32 - shift));
        (*reg).mant.m32[1] <<= shift;
    } else {
        shift = (*reg).mant.m32[1].leading_zeros();
        (*reg).mant.m32[0] = (*reg).mant.m32[1] << shift;
        (*reg).mant.m32[1] = 0;
    }
    if (*reg).mant.m32[0] == 0 { (shift + 32) as i32 } else { shift as i32 }
}

#[inline]
pub unsafe fn fp_addmant(dest: *mut crate::fp_ext, src: *mut crate::fp_ext) -> i32 {
    let (v, c) = ((*dest).lowmant as u32).overflowing_add((*src).lowmant as u32);
    (*dest).lowmant = v as _;
    let (v, c2) = ((*dest).mant.m32[1]).overflowing_add((*src).mant.m32[1]);
    let (v, c3) = v.overflowing_add(c as u32);
    (*dest).mant.m32[1] = v;
    let (v, c4) = ((*dest).mant.m32[0]).overflowing_add((*src).mant.m32[0]);
    let (_, c5) = v.overflowing_add(c2 as u32 | c3 as u32);
    (*dest).mant.m32[0] = v.wrapping_add((c2 as u32) | (c3 as u32));
    (c4 || c5) as i32
}

#[inline]
pub unsafe fn fp_addcarry(reg: *mut crate::fp_ext) -> i32 {
    (*reg).exp += 1;
    if (*reg).exp == 0x7fff {
        if (*reg).mant.m64 != 0 { crate::fp_set_sr(crate::FPSR_EXC_INEX2); }
        (*reg).mant.m64 = 0;
        crate::fp_set_sr(crate::FPSR_EXC_OVFL);
        return 0;
    }
    (*reg).lowmant = ((*reg).mant.m32[1] << 7) | ((*reg).lowmant != 0) as u32;
    (*reg).mant.m32[1] = ((*reg).mant.m32[1] >> 1) | ((*reg).mant.m32[0] << 31);
    (*reg).mant.m32[0] = ((*reg).mant.m32[0] >> 1) | 0x80000000;
    1
}

#[inline]
pub unsafe fn fp_submant(dest: *mut crate::fp_ext, src1: *mut crate::fp_ext, src2: *mut crate::fp_ext) {
    let (v, b) = ((*src1).lowmant as u32).overflowing_sub((*src2).lowmant as u32);
    (*dest).lowmant = v as _;
    let (v, b2) = (*src1).mant.m32[1].overflowing_sub((*src2).mant.m32[1]);
    let (v, b3) = v.overflowing_sub(b as u32);
    (*dest).mant.m32[1] = v;
    (*dest).mant.m32[0] = (*src1).mant.m32[0].wrapping_sub((*src2).mant.m32[0]).wrapping_sub((b2 || b3) as u32);
}

#[inline]
pub unsafe fn fp_multiplymant(dest: *mut crate::fp_mant128, src1: *mut crate::fp_ext, src2: *mut crate::fp_ext) {
    let a = ((*src1).mant.m64 as u128) * ((*src2).mant.m64 as u128);
    (*dest).m64[0] = (a >> 64) as _;
    (*dest).m64[1] = a as _;
}

/* The following operations correspond to the original m68k asm macros. */
#[inline] pub unsafe fn fp_mul64(desth: &mut u32, destl: &mut u32, src1: u32, src2: u32) { let v = (src1 as u64) * src2 as u64; *desth = (v >> 32) as u32; *destl = v as u32; }
#[inline] pub unsafe fn fp_div64(quot: &mut u32, rem: &mut u32, srch: u32, srcl: u32, div: u32) { let v = ((srch as u64) << 32) | srcl as u64; *quot = (v / div as u64) as u32; *rem = (v % div as u64) as u32; }

/* Remaining declaration is retained as a low-level dependency-bearing helper. */
#[inline] pub unsafe fn fp_putmant128(dest: *mut crate::fp_ext, src: *mut crate::fp_mant128, shift: i32) {
    match shift { 0 => { (*dest).mant.m64 = (*src).m64[0]; (*dest).lowmant = (*src).m32[2] >> 24; if (*src).m32[3] != 0 || (*src).m32[2] << 8 != 0 { (*dest).lowmant |= 1; } }, 32 => { (*dest).mant.m32[0] = (*src).m32[1]; (*dest).mant.m32[1] = (*src).m32[2]; (*dest).lowmant = (*src).m32[3] >> 24; if (*src).m32[3] << 8 != 0 { (*dest).lowmant |= 1; } }, _ => {} }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
