/* SPDX-License-Identifier: GPL-2.0-only */
/* MIPS floating point support. Translated from ieee754.h. */

#[repr(C)]
pub union ieee754dp {
    pub fields: ieee754dp_fields,
    pub bits: u64,
}

#[repr(C)]
pub struct ieee754dp_fields {
    /* C bitfields: sign:1, bexp:11, mant:52; layout is target byte-order dependent. */
    pub sign: u32,
    pub bexp: u32,
    pub mant: u64,
}

#[repr(C)]
pub union ieee754sp {
    pub fields: ieee754sp_fields,
    pub bits: u32,
}

#[repr(C)]
pub struct ieee754sp_fields {
    /* C bitfields: sign:1, bexp:8, mant:23; layout is target byte-order dependent. */
    pub sign: u32,
    pub bexp: u32,
    pub mant: u32,
}

extern "C" {
    pub fn ieee754sp_class(x: ieee754sp) -> i32;
    pub fn ieee754sp_abs(x: ieee754sp) -> ieee754sp;
    pub fn ieee754sp_neg(x: ieee754sp) -> ieee754sp;
    pub fn ieee754sp_add(x: ieee754sp, y: ieee754sp) -> ieee754sp;
    pub fn ieee754sp_sub(x: ieee754sp, y: ieee754sp) -> ieee754sp;
    pub fn ieee754sp_mul(x: ieee754sp, y: ieee754sp) -> ieee754sp;
    pub fn ieee754sp_div(x: ieee754sp, y: ieee754sp) -> ieee754sp;
    pub fn ieee754sp_fint(x: i32) -> ieee754sp;
    pub fn ieee754sp_flong(x: i64) -> ieee754sp;
    pub fn ieee754sp_fdp(x: ieee754dp) -> ieee754sp;
    pub fn ieee754sp_rint(x: ieee754sp) -> ieee754sp;
    pub fn ieee754sp_tint(x: ieee754sp) -> i32;
    pub fn ieee754sp_tlong(x: ieee754sp) -> i64;
    pub fn ieee754sp_cmp(x: ieee754sp, y: ieee754sp, cop: i32, sig: i32) -> i32;
    pub fn ieee754sp_sqrt(x: ieee754sp) -> ieee754sp;
    pub fn ieee754sp_maddf(z: ieee754sp, x: ieee754sp, y: ieee754sp) -> ieee754sp;
    pub fn ieee754sp_msubf(z: ieee754sp, x: ieee754sp, y: ieee754sp) -> ieee754sp;
    pub fn ieee754sp_madd(z: ieee754sp, x: ieee754sp, y: ieee754sp) -> ieee754sp;
    pub fn ieee754sp_msub(z: ieee754sp, x: ieee754sp, y: ieee754sp) -> ieee754sp;
    pub fn ieee754sp_nmadd(z: ieee754sp, x: ieee754sp, y: ieee754sp) -> ieee754sp;
    pub fn ieee754sp_nmsub(z: ieee754sp, x: ieee754sp, y: ieee754sp) -> ieee754sp;
    pub fn ieee754sp_2008class(x: ieee754sp) -> i32;
    pub fn ieee754sp_fmin(x: ieee754sp, y: ieee754sp) -> ieee754sp;
    pub fn ieee754sp_fmina(x: ieee754sp, y: ieee754sp) -> ieee754sp;
    pub fn ieee754sp_fmax(x: ieee754sp, y: ieee754sp) -> ieee754sp;
    pub fn ieee754sp_fmaxa(x: ieee754sp, y: ieee754sp) -> ieee754sp;

    pub fn ieee754dp_class(x: ieee754dp) -> i32;
    pub fn ieee754dp_add(x: ieee754dp, y: ieee754dp) -> ieee754dp;
    pub fn ieee754dp_sub(x: ieee754dp, y: ieee754dp) -> ieee754dp;
    pub fn ieee754dp_mul(x: ieee754dp, y: ieee754dp) -> ieee754dp;
    pub fn ieee754dp_div(x: ieee754dp, y: ieee754dp) -> ieee754dp;
    pub fn ieee754dp_abs(x: ieee754dp) -> ieee754dp;
    pub fn ieee754dp_neg(x: ieee754dp) -> ieee754dp;
    pub fn ieee754dp_fint(x: i32) -> ieee754dp;
    pub fn ieee754dp_flong(x: i64) -> ieee754dp;
    pub fn ieee754dp_fsp(x: ieee754sp) -> ieee754dp;
    pub fn ieee754dp_rint(x: ieee754dp) -> ieee754dp;
    pub fn ieee754dp_tint(x: ieee754dp) -> i32;
    pub fn ieee754dp_tlong(x: ieee754dp) -> i64;
    pub fn ieee754dp_cmp(x: ieee754dp, y: ieee754dp, cop: i32, sig: i32) -> i32;
    pub fn ieee754dp_sqrt(x: ieee754dp) -> ieee754dp;
    pub fn ieee754dp_maddf(z: ieee754dp, x: ieee754dp, y: ieee754dp) -> ieee754dp;
    pub fn ieee754dp_msubf(z: ieee754dp, x: ieee754dp, y: ieee754dp) -> ieee754dp;
    pub fn ieee754dp_madd(z: ieee754dp, x: ieee754dp, y: ieee754dp) -> ieee754dp;
    pub fn ieee754dp_msub(z: ieee754dp, x: ieee754dp, y: ieee754dp) -> ieee754dp;
    pub fn ieee754dp_nmadd(z: ieee754dp, x: ieee754dp, y: ieee754dp) -> ieee754dp;
    pub fn ieee754dp_nmsub(z: ieee754dp, x: ieee754dp, y: ieee754dp) -> ieee754dp;
    pub fn ieee754dp_2008class(x: ieee754dp) -> i32;
    pub fn ieee754dp_fmin(x: ieee754dp, y: ieee754dp) -> ieee754dp;
    pub fn ieee754dp_fmina(x: ieee754dp, y: ieee754dp) -> ieee754dp;
    pub fn ieee754dp_fmax(x: ieee754dp, y: ieee754dp) -> ieee754dp;
    pub fn ieee754dp_fmaxa(x: ieee754dp, y: ieee754dp) -> ieee754dp;
    pub fn ieee754sp_dump(s: *mut i8, x: ieee754sp) -> ieee754sp;
    pub fn ieee754dp_dump(s: *mut i8, x: ieee754dp) -> ieee754dp;
}

pub const IEEE754_CLASS_NORM: i32 = 0x00;
pub const IEEE754_CLASS_ZERO: i32 = 0x01;
pub const IEEE754_CLASS_DNORM: i32 = 0x02;
pub const IEEE754_CLASS_INF: i32 = 0x03;
pub const IEEE754_CLASS_SNAN: i32 = 0x04;
pub const IEEE754_CLASS_QNAN: i32 = 0x05;

pub const IEEE754_INEXACT: u32 = 0x01;
pub const IEEE754_UNDERFLOW: u32 = 0x02;
pub const IEEE754_OVERFLOW: u32 = 0x04;
pub const IEEE754_ZERO_DIVIDE: u32 = 0x08;
pub const IEEE754_INVALID_OPERATION: u32 = 0x10;

pub const IEEE754_CLT: u32 = 0x01;
pub const IEEE754_CEQ: u32 = 0x02;
pub const IEEE754_CGT: u32 = 0x04;
pub const IEEE754_CUN: u32 = 0x08;

#[repr(C)]
pub struct _ieee754_csr {
    /* C bitfields: fcc:7, nod:1, c:1, pad0:3, abs2008:1, nan2008:1,
       cx:6, mx:5, sx:5, rm:2. */
    pub fcc: u32,
    pub nod: u32,
    pub c: u32,
    pub pad0: u32,
    pub abs2008: u32,
    pub nan2008: u32,
    pub cx: u32,
    pub mx: u32,
    pub sx: u32,
    pub rm: u32,
}

/* linux/compiler.h, asm/byteorder.h, linux/kernel.h, linux/types.h,
   linux/sched.h, and asm/bitfield.h provide these dependencies in the source. */
extern "C" {
    pub static mut current: *mut kernel_task;
    pub static __ieee754dp_spcvals: [ieee754dp; 18];
    pub static __ieee754sp_spcvals: [ieee754sp; 18];
}

#[repr(C)]
pub struct kernel_task {
    pub thread: kernel_thread,
}
#[repr(C)]
pub struct kernel_thread {
    pub fpu: kernel_fpu,
}
#[repr(C)]
pub struct kernel_fpu {
    pub fcr31: u32,
}

pub const IEEE754_SPCVAL_PZERO: usize = 0;
pub const IEEE754_SPCVAL_NZERO: usize = 1;
pub const IEEE754_SPCVAL_PONE: usize = 2;
pub const IEEE754_SPCVAL_NONE: usize = 3;
pub const IEEE754_SPCVAL_PTEN: usize = 4;
pub const IEEE754_SPCVAL_NTEN: usize = 5;
pub const IEEE754_SPCVAL_PINFINITY: usize = 6;
pub const IEEE754_SPCVAL_NINFINITY: usize = 7;
pub const IEEE754_SPCVAL_INDEF_LEG: usize = 8;
pub const IEEE754_SPCVAL_INDEF_2008: usize = 9;
pub const IEEE754_SPCVAL_PMAX: usize = 10;
pub const IEEE754_SPCVAL_NMAX: usize = 11;
pub const IEEE754_SPCVAL_PMIN: usize = 12;
pub const IEEE754_SPCVAL_NMIN: usize = 13;
pub const IEEE754_SPCVAL_PMIND: usize = 14;
pub const IEEE754_SPCVAL_NMIND: usize = 15;
pub const IEEE754_SPCVAL_P1E31: usize = 16;
pub const IEEE754_SPCVAL_P1E63: usize = 17;

#[inline]
pub unsafe fn ieee754dp_inf(sn: usize) -> ieee754dp { __ieee754dp_spcvals[IEEE754_SPCVAL_PINFINITY + sn] }
#[inline]
pub unsafe fn ieee754dp_zero(sn: usize) -> ieee754dp { __ieee754dp_spcvals[IEEE754_SPCVAL_PZERO + sn] }
#[inline]
pub unsafe fn ieee754dp_one(sn: usize) -> ieee754dp { __ieee754dp_spcvals[IEEE754_SPCVAL_PONE + sn] }
#[inline]
pub unsafe fn ieee754dp_ten(sn: usize) -> ieee754dp { __ieee754dp_spcvals[IEEE754_SPCVAL_PTEN + sn] }
#[inline]
pub unsafe fn ieee754dp_indef() -> ieee754dp { __ieee754dp_spcvals[IEEE754_SPCVAL_INDEF_LEG + ieee754_csr().nan2008 as usize] }
#[inline]
pub unsafe fn ieee754dp_max(sn: usize) -> ieee754dp { __ieee754dp_spcvals[IEEE754_SPCVAL_PMAX + sn] }
#[inline]
pub unsafe fn ieee754dp_min(sn: usize) -> ieee754dp { __ieee754dp_spcvals[IEEE754_SPCVAL_PMIN + sn] }
#[inline]
pub unsafe fn ieee754dp_mind(sn: usize) -> ieee754dp { __ieee754dp_spcvals[IEEE754_SPCVAL_PMIND + sn] }
#[inline]
pub unsafe fn ieee754dp_1e31() -> ieee754dp { __ieee754dp_spcvals[IEEE754_SPCVAL_P1E31] }
#[inline]
pub unsafe fn ieee754dp_1e63() -> ieee754dp { __ieee754dp_spcvals[IEEE754_SPCVAL_P1E63] }

#[inline]
pub unsafe fn ieee754sp_inf(sn: usize) -> ieee754sp { __ieee754sp_spcvals[IEEE754_SPCVAL_PINFINITY + sn] }
#[inline]
pub unsafe fn ieee754sp_zero(sn: usize) -> ieee754sp { __ieee754sp_spcvals[IEEE754_SPCVAL_PZERO + sn] }
#[inline]
pub unsafe fn ieee754sp_one(sn: usize) -> ieee754sp { __ieee754sp_spcvals[IEEE754_SPCVAL_PONE + sn] }
#[inline]
pub unsafe fn ieee754sp_ten(sn: usize) -> ieee754sp { __ieee754sp_spcvals[IEEE754_SPCVAL_PTEN + sn] }
#[inline]
pub unsafe fn ieee754sp_indef() -> ieee754sp { __ieee754sp_spcvals[IEEE754_SPCVAL_INDEF_LEG + ieee754_csr().nan2008 as usize] }
#[inline]
pub unsafe fn ieee754sp_max(sn: usize) -> ieee754sp { __ieee754sp_spcvals[IEEE754_SPCVAL_PMAX + sn] }
#[inline]
pub unsafe fn ieee754sp_min(sn: usize) -> ieee754sp { __ieee754sp_spcvals[IEEE754_SPCVAL_PMIN + sn] }
#[inline]
pub unsafe fn ieee754sp_mind(sn: usize) -> ieee754sp { __ieee754sp_spcvals[IEEE754_SPCVAL_PMIND + sn] }
#[inline]
pub unsafe fn ieee754sp_1e31() -> ieee754sp { __ieee754sp_spcvals[IEEE754_SPCVAL_P1E31] }
#[inline]
pub unsafe fn ieee754sp_1e63() -> ieee754sp { __ieee754sp_spcvals[IEEE754_SPCVAL_P1E63] }

pub const IEEE754_RT_SP: i32 = 0;
pub const IEEE754_RT_DP: i32 = 1;
pub const IEEE754_RT_XP: i32 = 2;
pub const IEEE754_RT_SI: i32 = 3;
pub const IEEE754_RT_DI: i32 = 4;

#[inline]
pub unsafe fn ieee754_csr() -> &'static mut _ieee754_csr {
    &mut *((&mut (*(*current)).thread.fpu.fcr31) as *mut u32 as *mut _ieee754_csr)
}

#[inline]
pub unsafe fn ieee754_getrm() -> u32 { ieee754_csr().rm }
#[inline]
pub unsafe fn ieee754_setrm(rm: u32) -> u32 { ieee754_csr().rm = rm; rm }
#[inline]
pub unsafe fn ieee754_getcx() -> u32 { ieee754_csr().cx }
#[inline]
pub unsafe fn ieee754_cxtest(n: u32) -> i32 { (ieee754_csr().cx & n) as i32 }
#[inline]
pub unsafe fn ieee754_getsx() -> u32 { ieee754_csr().sx }
#[inline]
pub unsafe fn ieee754_clrsx() -> u32 { ieee754_csr().sx = 0; 0 }
#[inline]
pub unsafe fn ieee754_sxtest(n: u32) -> i32 { (ieee754_csr().sx & n) as i32 }

#[inline]
pub unsafe fn ieee754si_indef() -> i32 { if ieee754_csr().nan2008 != 0 { 0 } else { i32::MAX } }
#[inline]
pub unsafe fn ieee754di_indef() -> i64 { if ieee754_csr().nan2008 != 0 { 0 } else { i64::MAX } }
#[inline]
pub unsafe fn ieee754si_overflow(xs: i32) -> i32 { if ieee754_csr().nan2008 != 0 && xs != 0 { i32::MIN } else { i32::MAX } }
#[inline]
pub unsafe fn ieee754di_overflow(xs: i32) -> i64 { if ieee754_csr().nan2008 != 0 && xs != 0 { i64::MIN } else { i64::MAX } }

#[inline]
pub unsafe fn ieee754dp_fix(x: ieee754dp) -> i32 { ieee754dp_tint(x) }
#[inline]
pub unsafe fn ieee754sp_fix(x: ieee754sp) -> i32 { ieee754sp_tint(x) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
