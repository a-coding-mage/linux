/* Software floating-point emulation.
   Definitions for IEEE Single Precision.
   Copyright (C) 1997,1998,1999 Free Software Foundation, Inc.
   This file is part of the GNU C Library.
   Contributed by Richard Henderson (rth@cygnus.com),
                  Jakub Jelinek (jj@ultra.linux.cz),
                  David S. Miller (davem@redhat.com) and
                  Peter Maydell (pmaydell@chiark.greenend.org.uk).

   The GNU C Library is free software; you can redistribute it and/or
   modify it under the terms of the GNU Library General Public License
   as published by the Free Software Foundation; either version 2 of
   the License, or (at your option) any later version.

   The GNU C Library is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
   Library General Public License for more details.

   You should have received a copy of the GNU Library General Public
   License along with the GNU C Library; see the file COPYING.LIB.  If
   not, write to the Free Software Foundation, Inc.,
   59 Temple Place - Suite 330, Boston, MA 02111-1307, USA.  */

// Original header guard: __MATH_EMU_SINGLE_H__.
// The source requires _FP_W_TYPE_SIZE >= 32.

pub const _FP_FRACBITS_S: usize = 24;
pub const _FP_FRACXBITS_S: usize = _FP_W_TYPE_SIZE - _FP_FRACBITS_S;
pub const _FP_WFRACBITS_S: usize = _FP_WORKBITS + _FP_FRACBITS_S;
pub const _FP_WFRACXBITS_S: usize = _FP_W_TYPE_SIZE - _FP_WFRACBITS_S;
pub const _FP_EXPBITS_S: usize = 8;
pub const _FP_EXPBIAS_S: usize = 127;
pub const _FP_EXPMAX_S: usize = 255;
pub const _FP_QNANBIT_S: _FP_W_TYPE = (_FP_W_TYPE::from(1u8)) << (_FP_FRACBITS_S - 2);
pub const _FP_IMPLBIT_S: _FP_W_TYPE = (_FP_W_TYPE::from(1u8)) << (_FP_FRACBITS_S - 1);
pub const _FP_OVERFLOW_S: _FP_W_TYPE = (_FP_W_TYPE::from(1u8)) << _FP_WFRACBITS_S;

// The implementation of _FP_MUL_MEAT_S and _FP_DIV_MEAT_S is chosen by the
// target machine.

#[repr(C)]
pub union _FP_UNION_S {
    pub flt: f32,
    pub bits: _FP_UNION_S_BITS,
}

#[repr(C, packed)]
pub struct _FP_UNION_S_BITS {
    // C bit-fields are represented by their containing word; callers must
    // apply the target byte order and masks when accessing these fields.
    pub raw: u32,
}

#[macro_export]
macro_rules! FP_DECL_S { ($X:ident) => { _FP_DECL!(1, $X) }; }
#[macro_export]
macro_rules! FP_UNPACK_RAW_S { ($X:ident, $val:expr) => { _FP_UNPACK_RAW_1!(S, $X, $val) }; }
#[macro_export]
macro_rules! FP_UNPACK_RAW_SP { ($X:ident, $val:expr) => { _FP_UNPACK_RAW_1_P!(S, $X, $val) }; }
#[macro_export]
macro_rules! FP_PACK_RAW_S { ($val:expr, $X:ident) => { _FP_PACK_RAW_1!(S, $val, $X) }; }
#[macro_export]
macro_rules! FP_PACK_RAW_SP {
    ($val:expr, $X:ident) => {{
        if !FP_INHIBIT_RESULTS { _FP_PACK_RAW_1_P!(S, $val, $X); }
    }};
}
#[macro_export]
macro_rules! FP_UNPACK_S {
    ($X:ident, $val:expr) => {{
        _FP_UNPACK_RAW_1!(S, $X, $val);
        _FP_UNPACK_CANONICAL!(S, 1, $X);
    }};
}
#[macro_export]
macro_rules! FP_UNPACK_SP {
    ($X:ident, $val:expr) => {{
        _FP_UNPACK_RAW_1_P!(S, $X, $val);
        _FP_UNPACK_CANONICAL!(S, 1, $X);
    }};
}
#[macro_export]
macro_rules! FP_PACK_S {
    ($val:expr, $X:ident) => {{
        _FP_PACK_CANONICAL!(S, 1, $X);
        _FP_PACK_RAW_1!(S, $val, $X);
    }};
}
#[macro_export]
macro_rules! FP_PACK_SP {
    ($val:expr, $X:ident) => {{
        _FP_PACK_CANONICAL!(S, 1, $X);
        if !FP_INHIBIT_RESULTS { _FP_PACK_RAW_1_P!(S, $val, $X); }
    }};
}

#[macro_export] macro_rules! FP_ISSIGNAN_S { ($X:expr) => { _FP_ISSIGNAN!(S, 1, $X) }; }
#[macro_export] macro_rules! FP_NEG_S { ($R:expr, $X:expr) => { _FP_NEG!(S, 1, $R, $X) }; }
#[macro_export] macro_rules! FP_ADD_S { ($R:expr, $X:expr, $Y:expr) => { _FP_ADD!(S, 1, $R, $X, $Y) }; }
#[macro_export] macro_rules! FP_SUB_S { ($R:expr, $X:expr, $Y:expr) => { _FP_SUB!(S, 1, $R, $X, $Y) }; }
#[macro_export] macro_rules! FP_MUL_S { ($R:expr, $X:expr, $Y:expr) => { _FP_MUL!(S, 1, $R, $X, $Y) }; }
#[macro_export] macro_rules! FP_DIV_S { ($R:expr, $X:expr, $Y:expr) => { _FP_DIV!(S, 1, $R, $X, $Y) }; }
#[macro_export] macro_rules! FP_SQRT_S { ($R:expr, $X:expr) => { _FP_SQRT!(S, 1, $R, $X) }; }
#[macro_export] macro_rules! _FP_SQRT_MEAT_S { ($R:expr, $S:expr, $T:expr, $X:expr, $Q:expr) => { _FP_SQRT_MEAT_1!($R, $S, $T, $X, $Q) }; }
#[macro_export] macro_rules! FP_CMP_S { ($r:expr, $X:expr, $Y:expr, $un:expr) => { _FP_CMP!(S, 1, $r, $X, $Y, $un) }; }
#[macro_export] macro_rules! FP_CMP_EQ_S { ($r:expr, $X:expr, $Y:expr) => { _FP_CMP_EQ!(S, 1, $r, $X, $Y) }; }
#[macro_export] macro_rules! FP_TO_INT_S { ($r:expr, $X:expr, $rsz:expr, $rsg:expr) => { _FP_TO_INT!(S, 1, $r, $X, $rsz, $rsg) }; }
#[macro_export] macro_rules! FP_TO_INT_ROUND_S { ($r:expr, $X:expr, $rsz:expr, $rsg:expr) => { _FP_TO_INT_ROUND!(S, 1, $r, $X, $rsz, $rsg) }; }
#[macro_export] macro_rules! FP_FROM_INT_S { ($X:expr, $r:expr, $rs:expr, $rt:expr) => { _FP_FROM_INT!(S, 1, $X, $r, $rs, $rt) }; }
#[macro_export] macro_rules! _FP_FRAC_HIGH_S { ($X:expr) => { _FP_FRAC_HIGH_1!($X) }; }
#[macro_export] macro_rules! _FP_FRAC_HIGH_RAW_S { ($X:expr) => { _FP_FRAC_HIGH_1!($X) }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
