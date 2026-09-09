/* SPDX-License-Identifier: GPL-2.0+
 *
 * Machine-dependent software floating-point definitions.
 * SuperH kernel version.
 * Copyright (C) 1997,1998,1999 Free Software Foundation, Inc.
 * This file is part of the GNU C Library.
 * Contributed by Richard Henderson (rth@cygnus.com),
 *              Jakub Jelinek (jj@ultra.linux.cz),
 *              David S. Miller (davem@redhat.com) and
 *              Peter Maydell (pmaydell@chiark.greenend.org.uk).
 */

// The original header guard was: _SFP_MACHINE_H

#[cfg(target_endian = "big")]
pub const __BYTE_ORDER: u32 = __BIG_ENDIAN;
#[cfg(target_endian = "big")]
pub const __LITTLE_ENDIAN: u32 = 0;
#[cfg(not(target_endian = "big"))]
pub const __BYTE_ORDER: u32 = __LITTLE_ENDIAN;
#[cfg(not(target_endian = "big"))]
pub const __BIG_ENDIAN: u32 = 0;

pub const _FP_W_TYPE_SIZE: usize = 32;
pub type _FP_W_TYPE = ::core::ffi::c_ulong;
pub type _FP_WS_TYPE = ::core::ffi::c_long;
pub type _FP_I_TYPE = ::core::ffi::c_long;

// These operations depend on the floating-point support definitions supplied by other headers.
macro_rules! _FP_MUL_MEAT_S {
    ($R:expr, $X:expr, $Y:expr) => { _FP_MUL_MEAT_1_wide!(_FP_WFRACBITS_S, $R, $X, $Y, umul_ppmm) };
}
macro_rules! _FP_MUL_MEAT_D {
    ($R:expr, $X:expr, $Y:expr) => { _FP_MUL_MEAT_2_wide!(_FP_WFRACBITS_D, $R, $X, $Y, umul_ppmm) };
}
macro_rules! _FP_MUL_MEAT_Q {
    ($R:expr, $X:expr, $Y:expr) => { _FP_MUL_MEAT_4_wide!(_FP_WFRACBITS_Q, $R, $X, $Y, umul_ppmm) };
}

macro_rules! _FP_DIV_MEAT_S {
    ($R:expr, $X:expr, $Y:expr) => { _FP_DIV_MEAT_1_udiv!(S, $R, $X, $Y) };
}
macro_rules! _FP_DIV_MEAT_D {
    ($R:expr, $X:expr, $Y:expr) => { _FP_DIV_MEAT_2_udiv!(D, $R, $X, $Y) };
}
macro_rules! _FP_DIV_MEAT_Q {
    ($R:expr, $X:expr, $Y:expr) => { _FP_DIV_MEAT_4_udiv!(Q, $R, $X, $Y) };
}

macro_rules! _FP_NANFRAC_S { () => { ((_FP_QNANBIT_S << 1) - 1) }; }
macro_rules! _FP_NANFRAC_D { () => { (((_FP_QNANBIT_D << 1) - 1), -1) }; }
macro_rules! _FP_NANFRAC_Q { () => { (((_FP_QNANBIT_Q << 1) - 1), -1, -1, -1) }; }
pub const _FP_NANSIGN_S: i32 = 0;
pub const _FP_NANSIGN_D: i32 = 0;
pub const _FP_NANSIGN_Q: i32 = 0;

pub const _FP_KEEPNANFRACP: i32 = 1;

/*
 * If one NaN is signaling and the other is not,
 * we choose that one, otherwise we choose X.
 *
 * C token-pasting (R##_s, X##_s, and the fs/wc-dependent names) is retained
 * through the explicit macro arguments below; the referenced operations and
 * constants are supplied by the surrounding floating-point implementation.
 */
macro_rules! _FP_CHOOSENAN {
    ($fs:ident, $wc:ident, $R_s:expr, $R_c:expr, $X_s:expr, $Y_s:expr, $R:expr, $Y:expr, $X:expr, $op:expr) => {{
        if (_FP_FRAC_HIGH_RAW!($fs, $X) & _FP_QNANBIT!($fs)) != 0
            && (_FP_FRAC_HIGH_RAW!($fs, $Y) & _FP_QNANBIT!($fs)) == 0
        {
            $R_s = $Y_s;
            _FP_FRAC_COPY!($wc, $R, $Y);
        } else {
            $R_s = $X_s;
            _FP_FRAC_COPY!($wc, $R, $X);
        }
        $R_c = FP_CLS_NAN;
    }};
}

// #define FP_ROUNDMODE FPSCR_RM
pub const FP_DENORM_ZERO: i32 = 1; // FPSCR_DN

/* Exception flags. */
pub const FP_EX_INVALID: i32 = 1 << 4;
pub const FP_EX_DIVZERO: i32 = 1 << 3;
pub const FP_EX_OVERFLOW: i32 = 1 << 2;
pub const FP_EX_UNDERFLOW: i32 = 1 << 1;
pub const FP_EX_INEXACT: i32 = 1 << 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
