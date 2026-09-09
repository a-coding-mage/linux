/* Software floating-point emulation.
   Definitions for IEEE Double Precision
   Copyright (C) 1997,1998,1999 Free Software Foundation, Inc.
   This file is part of the GNU C Library.
   Contributed by Richard Henderson (rth@cygnus.com),
                  Jakub Jelinek (jj@ultra.linux.cz),
                  David S. Miller (davem@redhat.com) and
                  Peter Maydell (pmaydell@chiark.greenend.org.uk).

   The GNU C Library is free software; you can redistribute it and/or
   modify it under the terms of the GNU Library General Public License.
*/

/* The original header requires _FP_W_TYPE_SIZE >= 32.  Build-time selection
   of the word size is supplied by the surrounding floating-point emulation. */

#[cfg(fp_w_type_size_lt_64)]
pub const _FP_FRACTBITS_D: usize = 2 * _FP_W_TYPE_SIZE;
#[cfg(not(fp_w_type_size_lt_64))]
pub const _FP_FRACTBITS_D: usize = _FP_W_TYPE_SIZE;

pub const _FP_FRACBITS_D: usize = 53;
pub const _FP_FRACXBITS_D: usize = _FP_FRACTBITS_D - _FP_FRACBITS_D;
pub const _FP_WFRACBITS_D: usize = _FP_WORKBITS + _FP_FRACBITS_D;
pub const _FP_WFRACXBITS_D: usize = _FP_FRACTBITS_D - _FP_WFRACBITS_D;
pub const _FP_EXPBITS_D: usize = 11;
pub const _FP_EXPBIAS_D: i32 = 1023;
pub const _FP_EXPMAX_D: i32 = 2047;

pub const _FP_QNANBIT_D: _FP_W_TYPE =
    (1 as _FP_W_TYPE) << ((_FP_FRACBITS_D - 2) % _FP_W_TYPE_SIZE);
pub const _FP_IMPLBIT_D: _FP_W_TYPE =
    (1 as _FP_W_TYPE) << ((_FP_FRACBITS_D - 1) % _FP_W_TYPE_SIZE);
pub const _FP_OVERFLOW_D: _FP_W_TYPE =
    (1 as _FP_W_TYPE) << (_FP_WFRACBITS_D % _FP_W_TYPE_SIZE);

#[repr(C)]
pub union _FP_UNION_D {
    pub flt: f64,
    /* C bit-fields are represented by their containing raw word; callers use
       the same masks and shifts as the original implementation. */
    pub bits: u64,
}

#[cfg(fp_w_type_size_lt_64)]
macro_rules! FP_DECL_D { ($X:ident) => { _FP_DECL!(2, $X) }; }
#[cfg(not(fp_w_type_size_lt_64))]
macro_rules! FP_DECL_D { ($X:ident) => { _FP_DECL!(1, $X) }; }

#[cfg(fp_w_type_size_lt_64)]
macro_rules! FP_UNPACK_RAW_D { ($X:ident, $val:expr) => { _FP_UNPACK_RAW_2!(D, $X, $val) }; }
#[cfg(not(fp_w_type_size_lt_64))]
macro_rules! FP_UNPACK_RAW_D { ($X:ident, $val:expr) => { _FP_UNPACK_RAW_1!(D, $X, $val) }; }
#[cfg(fp_w_type_size_lt_64)]
macro_rules! FP_UNPACK_RAW_DP { ($X:ident, $val:expr) => { _FP_UNPACK_RAW_2_P!(D, $X, $val) }; }
#[cfg(not(fp_w_type_size_lt_64))]
macro_rules! FP_UNPACK_RAW_DP { ($X:ident, $val:expr) => { _FP_UNPACK_RAW_1_P!(D, $X, $val) }; }

#[cfg(fp_w_type_size_lt_64)]
macro_rules! FP_PACK_RAW_D { ($val:expr, $X:expr) => { _FP_PACK_RAW_2!(D, $val, $X) }; }
#[cfg(not(fp_w_type_size_lt_64))]
macro_rules! FP_PACK_RAW_D { ($val:expr, $X:expr) => { _FP_PACK_RAW_1!(D, $val, $X) }; }
#[cfg(fp_w_type_size_lt_64)]
macro_rules! FP_PACK_RAW_DP { ($val:expr, $X:expr) => { if !FP_INHIBIT_RESULTS { _FP_PACK_RAW_2_P!(D, $val, $X); } }; }
#[cfg(not(fp_w_type_size_lt_64))]
macro_rules! FP_PACK_RAW_DP { ($val:expr, $X:expr) => { if !FP_INHIBIT_RESULTS { _FP_PACK_RAW_1_P!(D, $val, $X); } }; }

#[cfg(fp_w_type_size_lt_64)]
macro_rules! FP_UNPACK_D { ($X:ident, $val:expr) => {{ _FP_UNPACK_RAW_2!(D, $X, $val); _FP_UNPACK_CANONICAL!(D, 2, $X); }}; }
#[cfg(not(fp_w_type_size_lt_64))]
macro_rules! FP_UNPACK_D { ($X:ident, $val:expr) => {{ _FP_UNPACK_RAW_1!(D, $X, $val); _FP_UNPACK_CANONICAL!(D, 1, $X); }}; }
#[cfg(fp_w_type_size_lt_64)]
macro_rules! FP_UNPACK_DP { ($X:ident, $val:expr) => {{ _FP_UNPACK_RAW_2_P!(D, $X, $val); _FP_UNPACK_CANONICAL!(D, 2, $X); }}; }
#[cfg(not(fp_w_type_size_lt_64))]
macro_rules! FP_UNPACK_DP { ($X:ident, $val:expr) => {{ _FP_UNPACK_RAW_1_P!(D, $X, $val); _FP_UNPACK_CANONICAL!(D, 1, $X); }}; }

#[cfg(fp_w_type_size_lt_64)]
macro_rules! FP_PACK_D { ($val:expr, $X:expr) => {{ _FP_PACK_CANONICAL!(D, 2, $X); _FP_PACK_RAW_2!(D, $val, $X); }}; }
#[cfg(not(fp_w_type_size_lt_64))]
macro_rules! FP_PACK_D { ($val:expr, $X:expr) => {{ _FP_PACK_CANONICAL!(D, 1, $X); _FP_PACK_RAW_1!(D, $val, $X); }}; }
#[cfg(fp_w_type_size_lt_64)]
macro_rules! FP_PACK_DP { ($val:expr, $X:expr) => {{ _FP_PACK_CANONICAL!(D, 2, $X); if !FP_INHIBIT_RESULTS { _FP_PACK_RAW_2_P!(D, $val, $X); } }}; }
#[cfg(not(fp_w_type_size_lt_64))]
macro_rules! FP_PACK_DP { ($val:expr, $X:expr) => {{ _FP_PACK_CANONICAL!(D, 1, $X); if !FP_INHIBIT_RESULTS { _FP_PACK_RAW_1_P!(D, $val, $X); } }}; }

#[cfg(fp_w_type_size_lt_64)]
macro_rules! FP_ISSIGNAN_D { ($X:expr) => { _FP_ISSIGNAN!(D, 2, $X) }; }
#[cfg(not(fp_w_type_size_lt_64))]
macro_rules! FP_ISSIGNAN_D { ($X:expr) => { _FP_ISSIGNAN!(D, 1, $X) }; }
#[cfg(fp_w_type_size_lt_64)]
macro_rules! FP_NEG_D { ($R:expr,$X:expr) => { _FP_NEG!(D,2,$R,$X) }; }
#[cfg(not(fp_w_type_size_lt_64))]
macro_rules! FP_NEG_D { ($R:expr,$X:expr) => { _FP_NEG!(D,1,$R,$X) }; }
#[cfg(fp_w_type_size_lt_64)]
macro_rules! FP_ADD_D { ($R:expr,$X:expr,$Y:expr) => { _FP_ADD!(D,2,$R,$X,$Y) }; }
#[cfg(not(fp_w_type_size_lt_64))]
macro_rules! FP_ADD_D { ($R:expr,$X:expr,$Y:expr) => { _FP_ADD!(D,1,$R,$X,$Y) }; }
#[cfg(fp_w_type_size_lt_64)]
macro_rules! FP_SUB_D { ($R:expr,$X:expr,$Y:expr) => { _FP_SUB!(D,2,$R,$X,$Y) }; }
#[cfg(not(fp_w_type_size_lt_64))]
macro_rules! FP_SUB_D { ($R:expr,$X:expr,$Y:expr) => { _FP_SUB!(D,1,$R,$X,$Y) }; }
#[cfg(fp_w_type_size_lt_64)]
macro_rules! FP_MUL_D { ($R:expr,$X:expr,$Y:expr) => { _FP_MUL!(D,2,$R,$X,$Y) }; }
#[cfg(not(fp_w_type_size_lt_64))]
macro_rules! FP_MUL_D { ($R:expr,$X:expr,$Y:expr) => { _FP_MUL!(D,1,$R,$X,$Y) }; }
#[cfg(fp_w_type_size_lt_64)]
macro_rules! FP_DIV_D { ($R:expr,$X:expr,$Y:expr) => { _FP_DIV!(D,2,$R,$X,$Y) }; }
#[cfg(not(fp_w_type_size_lt_64))]
macro_rules! FP_DIV_D { ($R:expr,$X:expr,$Y:expr) => { _FP_DIV!(D,1,$R,$X,$Y) }; }
#[cfg(fp_w_type_size_lt_64)]
macro_rules! FP_SQRT_D { ($R:expr,$X:expr) => { _FP_SQRT!(D,2,$R,$X) }; }
#[cfg(not(fp_w_type_size_lt_64))]
macro_rules! FP_SQRT_D { ($R:expr,$X:expr) => { _FP_SQRT!(D,1,$R,$X) }; }

#[cfg(fp_w_type_size_lt_64)]
macro_rules! _FP_SQRT_MEAT_D { ($R:expr,$S:expr,$T:expr,$X:expr,$Q:expr) => { _FP_SQRT_MEAT_2!($R,$S,$T,$X,$Q) }; }
#[cfg(not(fp_w_type_size_lt_64))]
macro_rules! _FP_SQRT_MEAT_D { ($R:expr,$S:expr,$T:expr,$X:expr,$Q:expr) => { _FP_SQRT_MEAT_1!($R,$S,$T,$X,$Q) }; }

#[cfg(fp_w_type_size_lt_64)] macro_rules! FP_CMP_D { ($r:expr,$X:expr,$Y:expr,$un:expr) => { _FP_CMP!(D,2,$r,$X,$Y,$un) }; }
#[cfg(not(fp_w_type_size_lt_64))] macro_rules! FP_CMP_D { ($r:expr,$X:expr,$Y:expr,$un:expr) => { _FP_CMP!(D,1,$r,$X,$Y,$un) }; }
#[cfg(fp_w_type_size_lt_64)] macro_rules! FP_CMP_EQ_D { ($r:expr,$X:expr,$Y:expr) => { _FP_CMP_EQ!(D,2,$r,$X,$Y) }; }
#[cfg(not(fp_w_type_size_lt_64))] macro_rules! FP_CMP_EQ_D { ($r:expr,$X:expr,$Y:expr) => { _FP_CMP_EQ!(D,1,$r,$X,$Y) }; }
#[cfg(fp_w_type_size_lt_64)] macro_rules! FP_TO_INT_D { ($r:expr,$X:expr,$rsz:expr,$rsg:expr) => { _FP_TO_INT!(D,2,$r,$X,$rsz,$rsg) }; }
#[cfg(not(fp_w_type_size_lt_64))] macro_rules! FP_TO_INT_D { ($r:expr,$X:expr,$rsz:expr,$rsg:expr) => { _FP_TO_INT!(D,1,$r,$X,$rsz,$rsg) }; }
#[cfg(fp_w_type_size_lt_64)] macro_rules! FP_TO_INT_ROUND_D { ($r:expr,$X:expr,$rsz:expr,$rsg:expr) => { _FP_TO_INT_ROUND!(D,2,$r,$X,$rsz,$rsg) }; }
#[cfg(not(fp_w_type_size_lt_64))] macro_rules! FP_TO_INT_ROUND_D { ($r:expr,$X:expr,$rsz:expr,$rsg:expr) => { _FP_TO_INT_ROUND!(D,1,$r,$X,$rsz,$rsg) }; }
#[cfg(fp_w_type_size_lt_64)] macro_rules! FP_FROM_INT_D { ($X:expr,$r:expr,$rs:expr,$rt:expr) => { _FP_FROM_INT!(D,2,$X,$r,$rs,$rt) }; }
#[cfg(not(fp_w_type_size_lt_64))] macro_rules! FP_FROM_INT_D { ($X:expr,$r:expr,$rs:expr,$rt:expr) => { _FP_FROM_INT!(D,1,$X,$r,$rs,$rt) }; }
#[cfg(fp_w_type_size_lt_64)] macro_rules! _FP_FRAC_HIGH_D { ($X:expr) => { _FP_FRAC_HIGH_2!($X) }; }
#[cfg(not(fp_w_type_size_lt_64))] macro_rules! _FP_FRAC_HIGH_D { ($X:expr) => { _FP_FRAC_HIGH_1!($X) }; }
#[cfg(fp_w_type_size_lt_64)] macro_rules! _FP_FRAC_HIGH_RAW_D { ($X:expr) => { _FP_FRAC_HIGH_2!($X) }; }
#[cfg(not(fp_w_type_size_lt_64))] macro_rules! _FP_FRAC_HIGH_RAW_D { ($X:expr) => { _FP_FRAC_HIGH_1!($X) }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
