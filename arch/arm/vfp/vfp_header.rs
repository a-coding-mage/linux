/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  linux/arch/arm/vfp/vfp.h
 *
 *  Copyright (C) 2004 ARM Limited.
 *  Written by Deep Blue Solutions Limited.
 */

#[inline]
pub fn vfp_shiftright32jamming(mut val: u32, shift: u32) -> u32 {
    if shift != 0 {
        if shift < 32 {
            val = (val >> shift) | (((val << (32 - shift)) != 0) as u32);
        } else {
            val = (val != 0) as u32;
        }
    }
    val
}

#[inline]
pub fn vfp_shiftright64jamming(mut val: u64, shift: u32) -> u64 {
    if shift != 0 {
        if shift < 64 {
            val = (val >> shift) | (((val << (64 - shift)) != 0) as u64);
        } else {
            val = (val != 0) as u64;
        }
    }
    val
}

#[inline]
pub fn vfp_hi64to32jamming(val: u64) -> u32 {
    ((val >> 32) as u32) | (((val & 0xffff_ffff) < 1) as u32 ^ 1)
}

#[inline]
pub unsafe fn add128(resh: *mut u64, resl: *mut u64, nh: u64, nl: u64, mh: u64, ml: u64) {
    let r = (nl as u128) + (ml as u128) + ((nh as u128) << 64) + ((mh as u128) << 64);
    *resl = r as u64;
    *resh = (r >> 64) as u64;
}

#[inline]
pub unsafe fn sub128(resh: *mut u64, resl: *mut u64, nh: u64, nl: u64, mh: u64, ml: u64) {
    let n = ((nh as u128) << 64) | nl as u128;
    let m = ((mh as u128) << 64) | ml as u128;
    let r = n.wrapping_sub(m);
    *resl = r as u64;
    *resh = (r >> 64) as u64;
}

#[inline]
pub unsafe fn mul64to128(resh: *mut u64, resl: *mut u64, n: u64, m: u64) {
    let r = (n as u128) * (m as u128);
    *resl = r as u64;
    *resh = (r >> 64) as u64;
}

#[inline]
pub unsafe fn shift64left(resh: *mut u64, resl: *mut u64, n: u64) {
    *resh = n >> 63;
    *resl = n << 1;
}

#[inline]
pub fn vfp_hi64multiply64(n: u64, m: u64) -> u64 {
    let r = (n as u128) * (m as u128);
    (r >> 64) as u64 | ((r as u64 != 0) as u64)
}

#[inline]
pub fn vfp_estimate_div128to64(nh: u64, nl: u64, m: u64) -> u64 {
    if nh >= m { return !0u64; }
    (((((nh as u128) << 64) | nl as u128) / m as u128) as u64)
}

/* Operations on unpacked elements */
#[inline] pub const fn vfp_sign_negate(sign: u16) -> u16 { sign ^ 0x8000 }

#[repr(C)]
pub struct vfp_single { pub exponent: i16, pub sign: u16, pub significand: u32 }

extern "C" {
    pub fn vfp_get_float(reg: u32) -> i32;
    pub fn vfp_put_float(val: i32, reg: u32);
}

pub const VFP_SINGLE_MANTISSA_BITS: u32 = 23;
pub const VFP_SINGLE_EXPONENT_BITS: u32 = 8;
pub const VFP_SINGLE_LOW_BITS: u32 = 32 - VFP_SINGLE_MANTISSA_BITS - 2;
pub const VFP_SINGLE_LOW_BITS_MASK: u32 = (1 << VFP_SINGLE_LOW_BITS) - 1;
pub const VFP_SINGLE_SIGNIFICAND_QNAN: u32 = 1 << (VFP_SINGLE_MANTISSA_BITS - 1 + VFP_SINGLE_LOW_BITS);

#[inline] pub fn vfp_single_packed_sign(v: u32) -> u32 { v & 0x8000_0000 }
#[inline] pub fn vfp_single_packed_negate(v: u32) -> u32 { v ^ 0x8000_0000 }
#[inline] pub fn vfp_single_packed_abs(v: u32) -> u32 { v & !0x8000_0000 }
#[inline] pub fn vfp_single_packed_exponent(v: u32) -> u32 { (v >> VFP_SINGLE_MANTISSA_BITS) & ((1 << VFP_SINGLE_EXPONENT_BITS) - 1) }
#[inline] pub fn vfp_single_packed_mantissa(v: u32) -> u32 { v & ((1 << VFP_SINGLE_MANTISSA_BITS) - 1) }

#[inline] pub unsafe fn vfp_single_unpack(s: *mut vfp_single, val: i32) {
    (*s).sign = vfp_single_packed_sign(val as u32) >> 16;
    (*s).exponent = vfp_single_packed_exponent(val as u32) as i16;
    let mut significand = (val as u32) << (32 - VFP_SINGLE_MANTISSA_BITS) >> 2;
    if (*s).exponent != 0 && (*s).exponent != 255 { significand |= 0x4000_0000; }
    (*s).significand = significand;
}

#[inline] pub unsafe fn vfp_single_pack(s: *mut vfp_single) -> i32 {
    ((((*s).sign as u32) << 16) + (((*s).exponent as u32) << VFP_SINGLE_MANTISSA_BITS) + ((*s).significand >> VFP_SINGLE_LOW_BITS)) as i32
}

pub const VFP_NUMBER: i32 = 1 << 0; pub const VFP_ZERO: i32 = 1 << 1; pub const VFP_DENORMAL: i32 = 1 << 2;
pub const VFP_INFINITY: i32 = 1 << 3; pub const VFP_NAN: i32 = 1 << 4; pub const VFP_NAN_SIGNAL: i32 = 1 << 5;
pub const VFP_QNAN: i32 = VFP_NAN; pub const VFP_SNAN: i32 = VFP_NAN | VFP_NAN_SIGNAL;

#[inline] pub unsafe fn vfp_single_type(s: *mut vfp_single) -> i32 {
    let mut t = VFP_NUMBER;
    if (*s).exponent == 255 { if (*s).significand == 0 { t = VFP_INFINITY; } else if (*s).significand & VFP_SINGLE_SIGNIFICAND_QNAN != 0 { t = VFP_QNAN; } else { t = VFP_SNAN; } }
    else if (*s).exponent == 0 { if (*s).significand == 0 { t |= VFP_ZERO; } else { t |= VFP_DENORMAL; } }
    t
}

extern "C" { pub fn __vfp_single_normaliseround(sd: i32, vs: *mut vfp_single, fpscr: u32, exceptions: u32) -> u32; }

#[repr(C)] pub struct vfp_double { pub exponent: i16, pub sign: u16, pub significand: u64 }
pub const VFP_REG_ZERO: u32 = 16;
extern "C" { pub fn vfp_get_double(reg: u32) -> u64; pub fn vfp_put_double(val: u64, reg: u32); }
pub const VFP_DOUBLE_MANTISSA_BITS: u32 = 52; pub const VFP_DOUBLE_EXPONENT_BITS: u32 = 11;
pub const VFP_DOUBLE_LOW_BITS: u32 = 64 - VFP_DOUBLE_MANTISSA_BITS - 2;
pub const VFP_DOUBLE_LOW_BITS_MASK: u64 = (1 << VFP_DOUBLE_LOW_BITS) - 1;
pub const VFP_DOUBLE_SIGNIFICAND_QNAN: u64 = 1u64 << (VFP_DOUBLE_MANTISSA_BITS - 1 + VFP_DOUBLE_LOW_BITS);
#[inline] pub fn vfp_double_packed_sign(v: u64) -> u64 { v & (1u64 << 63) }
#[inline] pub fn vfp_double_packed_negate(v: u64) -> u64 { v ^ (1u64 << 63) }
#[inline] pub fn vfp_double_packed_abs(v: u64) -> u64 { v & !(1u64 << 63) }
#[inline] pub fn vfp_double_packed_exponent(v: u64) -> u64 { (v >> VFP_DOUBLE_MANTISSA_BITS) & ((1 << VFP_DOUBLE_EXPONENT_BITS) - 1) }
#[inline] pub fn vfp_double_packed_mantissa(v: u64) -> u64 { v & ((1u64 << VFP_DOUBLE_MANTISSA_BITS) - 1) }

#[inline] pub unsafe fn vfp_double_unpack(s: *mut vfp_double, val: i64) {
    (*s).sign = vfp_double_packed_sign(val as u64) >> 48; (*s).exponent = vfp_double_packed_exponent(val as u64) as i16;
    let mut x = (val as u64) << (64 - VFP_DOUBLE_MANTISSA_BITS) >> 2;
    if (*s).exponent != 0 && (*s).exponent != 2047 { x |= 1u64 << 62; } (*s).significand = x;
}
#[inline] pub unsafe fn vfp_double_pack(s: *mut vfp_double) -> i64 { (((*s).sign as u64 << 48) + ((*s).exponent as u64 << VFP_DOUBLE_MANTISSA_BITS) + ((*s).significand >> VFP_DOUBLE_LOW_BITS)) as i64 }
#[inline] pub unsafe fn vfp_double_type(s: *mut vfp_double) -> i32 { let mut t=VFP_NUMBER; if (*s).exponent==2047 { if (*s).significand==0 {t=VFP_INFINITY;} else if (*s).significand&VFP_DOUBLE_SIGNIFICAND_QNAN!=0 {t=VFP_QNAN;} else {t=VFP_SNAN;} } else if (*s).exponent==0 {if (*s).significand==0 {t|=VFP_ZERO;} else {t|=VFP_DENORMAL;}} t }
extern "C" { pub fn vfp_double_normaliseround(dd: i32, vd: *mut vfp_double, fpscr: u32, exceptions: u32, func: *const i8) -> u32; pub fn vfp_estimate_sqrt_significand(exponent: u32, significand: u32) -> u32; }
pub const VFP_NAN_FLAG: u32 = 0x100; pub const VFP_EXCEPTION_ERROR: u32 = !0u32 & !VFP_NAN_FLAG;
pub const OP_SCALAR: u32 = 1 << 0; pub const OP_SD: u32 = 1 << 1; pub const OP_DD: u32 = 1 << 1; pub const OP_SM: u32 = 1 << 2;
#[repr(C)] pub struct op { pub fn_: Option<unsafe extern "C" fn(dd: i32, dn: i32, dm: i32, fpscr: u32) -> u32>, pub flags: u32 }
extern "C" { pub fn vfp_save_state(location: *mut core::ffi::c_void, fpexc: u32); pub fn vfp_load_state(location: *const core::ffi::c_void) -> u32; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
