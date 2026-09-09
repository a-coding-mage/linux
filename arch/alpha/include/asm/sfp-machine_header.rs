/* Machine-dependent software floating-point definitions.
   Alpha kernel version.
   Copyright (C) 1997,1998,1999 Free Software Foundation, Inc.
   This file is part of the GNU C Library.
   Contributed by Richard Henderson (rth@cygnus.com),
                  Jakub Jelinek (jakub@redhat.com) and
                  David S. Miller (davem@redhat.com).

   The GNU C Library is free software; you can redistribute it and/or
   modify it under the terms of the GNU Library General Public License
   as published by the Free Software Foundation; either version 2 of the
   License, or (at your option) any later version.

   The GNU C Library is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
   Library General Public License for more details.

   You should have received a copy of the GNU General Public
   License along with the GNU C Library; see the file COPYING.LIB.  If
   not, write to the Free Software Foundation, Inc.,
   59 Temple Place - Suite 330, Boston, MA 02111-1307, USA.  */

// C header guard: _SFP_MACHINE_H

pub const _FP_W_TYPE_SIZE: usize = 64;
pub type _FP_W_TYPE = ::core::ffi::c_ulong;
pub type _FP_WS_TYPE = ::core::ffi::c_long;
pub type _FP_I_TYPE = ::core::ffi::c_long;

macro_rules! _FP_MUL_MEAT_S {
    ($R:ident, $X:ident, $Y:ident) => {
        _FP_MUL_MEAT_1_imm!(_FP_WFRACBITS_S, $R, $X, $Y)
    };
}
macro_rules! _FP_MUL_MEAT_D {
    ($R:ident, $X:ident, $Y:ident) => {
        _FP_MUL_MEAT_1_wide!(_FP_WFRACBITS_D, $R, $X, $Y, umul_ppmm)
    };
}
macro_rules! _FP_MUL_MEAT_Q {
    ($R:ident, $X:ident, $Y:ident) => {
        _FP_MUL_MEAT_2_wide!(_FP_WFRACBITS_Q, $R, $X, $Y, umul_ppmm)
    };
}

macro_rules! _FP_DIV_MEAT_S {
    ($R:ident, $X:ident, $Y:ident) => {
        _FP_DIV_MEAT_1_imm!(S, $R, $X, $Y, _FP_DIV_HELP_imm)
    };
}
macro_rules! _FP_DIV_MEAT_D {
    ($R:ident, $X:ident, $Y:ident) => {
        _FP_DIV_MEAT_1_udiv!(D, $R, $X, $Y)
    };
}
macro_rules! _FP_DIV_MEAT_Q {
    ($R:ident, $X:ident, $Y:ident) => {
        _FP_DIV_MEAT_2_udiv!(Q, $R, $X, $Y)
    };
}

pub const _FP_NANFRAC_S: _FP_W_TYPE = _FP_QNANBIT_S;
pub const _FP_NANFRAC_D: _FP_W_TYPE = _FP_QNANBIT_D;
pub const _FP_NANFRAC_Q: _FP_W_TYPE = _FP_QNANBIT_Q;
pub const _FP_NANSIGN_S: i32 = 1;
pub const _FP_NANSIGN_D: i32 = 1;
pub const _FP_NANSIGN_Q: i32 = 1;

pub const _FP_KEEPNANFRACP: i32 = 1;

/* Alpha Architecture Handbook, 4.7.10.4 sais that
 * we should prefer any type of NaN in Fb, then Fa.
 */
macro_rules! _FP_CHOOSENAN {
    ($fs:ident, $wc:ident, $R:ident, $X:ident, $Y:ident, $OP:ident) => {{
        $R##_s = $Y##_s;
        _FP_FRAC_COPY_$wc!($R, $X);
        $R##_c = FP_CLS_NAN;
    }};
}

/* Obtain the current rounding mode. */
pub const FP_ROUNDMODE: i32 = mode;
pub const FP_RND_NEAREST: i32 = (FPCR_DYN_NORMAL >> FPCR_DYN_SHIFT);
pub const FP_RND_ZERO: i32 = (FPCR_DYN_CHOPPED >> FPCR_DYN_SHIFT);
pub const FP_RND_PINF: i32 = (FPCR_DYN_PLUS >> FPCR_DYN_SHIFT);
pub const FP_RND_MINF: i32 = (FPCR_DYN_MINUS >> FPCR_DYN_SHIFT);

/* Exception flags. */
pub const FP_EX_INVALID: i32 = IEEE_TRAP_ENABLE_INV;
pub const FP_EX_OVERFLOW: i32 = IEEE_TRAP_ENABLE_OVF;
pub const FP_EX_UNDERFLOW: i32 = IEEE_TRAP_ENABLE_UNF;
pub const FP_EX_DIVZERO: i32 = IEEE_TRAP_ENABLE_DZE;
pub const FP_EX_INEXACT: i32 = IEEE_TRAP_ENABLE_INE;
pub const FP_EX_DENORM: i32 = IEEE_TRAP_ENABLE_DNO;

macro_rules! FP_DENORM_ZERO {
    () => { swcr & IEEE_MAP_DMZ };
}

/* We write the results always */
pub const FP_INHIBIT_RESULTS: i32 = 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
