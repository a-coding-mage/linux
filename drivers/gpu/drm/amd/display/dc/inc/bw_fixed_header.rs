/*
 * Copyright 2015 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 */

pub const BW_FIXED_BITS_PER_FRACTIONAL_PART: u32 = 24;

#[inline]
pub fn BW_FIXED_GET_INTEGER_PART(x: i64) -> i64 {
    x >> BW_FIXED_BITS_PER_FRACTIONAL_PART
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bw_fixed {
    pub value: i64,
}

pub const BW_FIXED_MIN_I32: i64 = -(1_i64 << (63 - BW_FIXED_BITS_PER_FRACTIONAL_PART));
pub const BW_FIXED_MAX_I32: i64 = (1_i64 << (63 - BW_FIXED_BITS_PER_FRACTIONAL_PART)) - 1;

#[inline]
pub fn bw_min2(arg1: bw_fixed, arg2: bw_fixed) -> bw_fixed {
    if arg1.value <= arg2.value { arg1 } else { arg2 }
}

#[inline]
pub fn bw_max2(arg1: bw_fixed, arg2: bw_fixed) -> bw_fixed {
    if arg2.value <= arg1.value { arg1 } else { arg2 }
}

#[inline]
pub fn bw_min3(v1: bw_fixed, v2: bw_fixed, v3: bw_fixed) -> bw_fixed {
    bw_min2(bw_min2(v1, v2), v3)
}

#[inline]
pub fn bw_max3(v1: bw_fixed, v2: bw_fixed, v3: bw_fixed) -> bw_fixed {
    bw_max2(bw_max2(v1, v2), v3)
}

extern "C" {
    pub fn bw_int_to_fixed_nonconst(value: i64) -> bw_fixed;
    pub fn bw_frc_to_fixed(num: i64, denum: i64) -> bw_fixed;
    pub fn bw_mul(arg1: bw_fixed, arg2: bw_fixed) -> bw_fixed;
    pub fn bw_floor2(arg: bw_fixed, significance: bw_fixed) -> bw_fixed;
    pub fn bw_ceil2(arg: bw_fixed, significance: bw_fixed) -> bw_fixed;
    fn div64_u64_rem(n: u64, base: u64, rem: *mut u64);
}

#[inline]
pub fn bw_int_to_fixed(value: i64) -> bw_fixed {
    // __builtin_constant_p/BUILD_BUG_ON are compiler/build-time facilities;
    // the conversion and range intent are preserved here.
    if value > BW_FIXED_MAX_I32 || value < BW_FIXED_MIN_I32 {
        unsafe { core::hint::unreachable_unchecked() }
    }
    bw_fixed { value: value.wrapping_shl(BW_FIXED_BITS_PER_FRACTIONAL_PART) }
}

#[inline]
pub fn bw_fixed_to_int(value: bw_fixed) -> i32 {
    BW_FIXED_GET_INTEGER_PART(value.value) as i32
}

#[inline]
pub fn fixed31_32_to_bw_fixed(mut raw: i64) -> bw_fixed {
    let mut result = bw_fixed { value: 0 };
    if raw < 0 {
        raw = raw.wrapping_neg();
        result.value = (raw >> (32 - BW_FIXED_BITS_PER_FRACTIONAL_PART)).wrapping_neg();
    } else {
        result.value = raw >> (32 - BW_FIXED_BITS_PER_FRACTIONAL_PART);
    }
    result
}

#[inline]
pub fn bw_add(arg1: bw_fixed, arg2: bw_fixed) -> bw_fixed {
    bw_fixed { value: arg1.value.wrapping_add(arg2.value) }
}

#[inline]
pub fn bw_sub(arg1: bw_fixed, arg2: bw_fixed) -> bw_fixed {
    bw_fixed { value: arg1.value.wrapping_sub(arg2.value) }
}

#[inline]
pub fn bw_div(arg1: bw_fixed, arg2: bw_fixed) -> bw_fixed {
    unsafe { bw_frc_to_fixed(arg1.value, arg2.value) }
}

#[inline]
pub fn bw_mod(arg1: bw_fixed, arg2: bw_fixed) -> bw_fixed {
    let mut res = bw_fixed { value: 0 };
    unsafe { div64_u64_rem(arg1.value as u64, arg2.value as u64, &mut res.value as *mut i64 as *mut u64); }
    res
}

#[inline]
pub fn bw_equ(arg1: bw_fixed, arg2: bw_fixed) -> bool { arg1.value == arg2.value }
#[inline]
pub fn bw_neq(arg1: bw_fixed, arg2: bw_fixed) -> bool { arg1.value != arg2.value }
#[inline]
pub fn bw_leq(arg1: bw_fixed, arg2: bw_fixed) -> bool { arg1.value <= arg2.value }
#[inline]
pub fn bw_meq(arg1: bw_fixed, arg2: bw_fixed) -> bool { arg1.value >= arg2.value }
#[inline]
pub fn bw_ltn(arg1: bw_fixed, arg2: bw_fixed) -> bool { arg1.value < arg2.value }
#[inline]
pub fn bw_mtn(arg1: bw_fixed, arg2: bw_fixed) -> bool { arg1.value > arg2.value }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
