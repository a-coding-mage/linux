/* Software floating-point emulation.
   Definitions for IEEE Quad Precision.
   Copyright (C) 1997,1998,1999 Free Software Foundation, Inc.
   This file is part of the GNU C Library.

   Rust translation of the C header. */

/* Original header guard: __MATH_EMU_QUAD_H__ */
/* Original condition: _FP_W_TYPE_SIZE must be at least 32. */

#[cfg(any())]
compile_error!("Here's a nickel, kid. Go buy yourself a real computer.");

/* These configuration symbols are supplied by the surrounding floating-point
   emulation implementation. */
#[cfg(any())]
pub const _FP_FRACTBITS_Q: usize = 4 * _FP_W_TYPE_SIZE;
#[cfg(any())]
pub const _FP_FRACTBITS_Q: usize = 2 * _FP_W_TYPE_SIZE;

pub const _FP_FRACBITS_Q: usize = 113;
pub const _FP_FRACXBITS_Q: usize = _FP_FRACTBITS_Q - _FP_FRACBITS_Q;
pub const _FP_WFRACBITS_Q: usize = _FP_WORKBITS + _FP_FRACBITS_Q;
pub const _FP_WFRACXBITS_Q: usize = _FP_FRACTBITS_Q - _FP_WFRACBITS_Q;
pub const _FP_EXPBITS_Q: usize = 15;
pub const _FP_EXPBIAS_Q: usize = 16383;
pub const _FP_EXPMAX_Q: usize = 32767;

#[macro_export]
macro_rules! _FP_QNANBIT_Q { () => { ((_FP_W_TYPE)1 << ((_FP_FRACBITS_Q - 2) % _FP_W_TYPE_SIZE)) }; }
#[macro_export]
macro_rules! _FP_IMPLBIT_Q { () => { ((_FP_W_TYPE)1 << ((_FP_FRACBITS_Q - 1) % _FP_W_TYPE_SIZE)) }; }
#[macro_export]
macro_rules! _FP_OVERFLOW_Q { () => { ((_FP_W_TYPE)1 << (_FP_WFRACBITS_Q % _FP_W_TYPE_SIZE)) }; }

/* The following union layouts preserve the C representation.  The selected
   layout depends on _FP_W_TYPE_SIZE and the target byte order. */
#[cfg(any())]
#[repr(C)]
pub union _FP_UNION_Q {
    pub flt: f64, /* C long double; exact ABI type is supplied by the target. */
    pub bits: _FP_UNION_Q_BITS,
}

#[cfg(any())]
#[repr(C, packed)]
pub struct _FP_UNION_Q_BITS {
    pub frac0: _FP_W_TYPE,
    pub frac1: _FP_W_TYPE,
    pub frac2: _FP_W_TYPE,
    pub frac3: _FP_W_TYPE,
    pub exp: u32,
    pub sign: u32,
}

/* When _FP_W_TYPE_SIZE >= 64, the C header instead uses two words.  The
   corresponding declarations and macro bodies are preserved below under a
   feature condition so the build can select the same branch. */
#[cfg(feature = "fp_w_type_size_ge_64")]
#[repr(C)]
pub union _FP_UNION_Q_64 {
    pub flt: f64,
    pub bits: _FP_UNION_Q_BITS_64,
}
#[cfg(feature = "fp_w_type_size_ge_64")]
#[repr(C)]
pub struct _FP_UNION_Q_BITS_64 {
    pub frac0: _FP_W_TYPE,
    pub frac1: _FP_W_TYPE,
    pub exp: u32,
    pub sign: u32,
}

/* In the >=64-bit branch every _4 suffix below is _2, and FP_DECL_Q uses 2;
   this is the direct conditional counterpart of the definitions below. */

/* C preprocessor mappings, retained as Rust macros for source-level use. */
#[macro_export] macro_rules! FP_DECL_Q { ($X:ident) => { _FP_DECL!(4, $X) }; }
#[macro_export] macro_rules! FP_UNPACK_RAW_Q { ($X:ident, $val:expr) => { _FP_UNPACK_RAW_4!(Q, $X, $val) }; }
#[macro_export] macro_rules! FP_UNPACK_RAW_QP { ($X:ident, $val:expr) => { _FP_UNPACK_RAW_4_P!(Q, $X, $val) }; }
#[macro_export] macro_rules! FP_PACK_RAW_Q { ($val:expr, $X:ident) => { _FP_PACK_RAW_4!(Q, $val, $X) }; }
#[macro_export] macro_rules! FP_PACK_RAW_QP { ($val:expr, $X:ident) => { if !FP_INHIBIT_RESULTS { _FP_PACK_RAW_4_P!(Q, $val, $X); } }; }
#[macro_export] macro_rules! FP_UNPACK_Q { ($X:ident, $val:expr) => {{ _FP_UNPACK_RAW_4!(Q, $X, $val); _FP_UNPACK_CANONICAL!(Q, 4, $X); }}; }
#[macro_export] macro_rules! FP_UNPACK_QP { ($X:ident, $val:expr) => {{ _FP_UNPACK_RAW_4_P!(Q, $X, $val); _FP_UNPACK_CANONICAL!(Q, 4, $X); }}; }
#[macro_export] macro_rules! FP_PACK_Q { ($val:expr, $X:ident) => {{ _FP_PACK_CANONICAL!(Q, 4, $X); _FP_PACK_RAW_4!(Q, $val, $X); }}; }
#[macro_export] macro_rules! FP_PACK_QP { ($val:expr, $X:ident) => {{ _FP_PACK_CANONICAL!(Q, 4, $X); if !FP_INHIBIT_RESULTS { _FP_PACK_RAW_4_P!(Q, $val, $X); } }}; }
#[macro_export] macro_rules! FP_ISSIGNAN_Q { ($X:ident) => { _FP_ISSIGNAN!(Q, 4, $X) }; }
#[macro_export] macro_rules! FP_NEG_Q { ($R:ident, $X:ident) => { _FP_NEG!(Q, 4, $R, $X) }; }
#[macro_export] macro_rules! FP_ADD_Q { ($R:ident, $X:ident, $Y:ident) => { _FP_ADD!(Q, 4, $R, $X, $Y) }; }
#[macro_export] macro_rules! FP_SUB_Q { ($R:ident, $X:ident, $Y:ident) => { _FP_SUB!(Q, 4, $R, $X, $Y) }; }
#[macro_export] macro_rules! FP_MUL_Q { ($R:ident, $X:ident, $Y:ident) => { _FP_MUL!(Q, 4, $R, $X, $Y) }; }
#[macro_export] macro_rules! FP_DIV_Q { ($R:ident, $X:ident, $Y:ident) => { _FP_DIV!(Q, 4, $R, $X, $Y) }; }
#[macro_export] macro_rules! FP_SQRT_Q { ($R:ident, $X:ident) => { _FP_SQRT!(Q, 4, $R, $X) }; }
#[macro_export] macro_rules! _FP_SQRT_MEAT_Q { ($R:ident, $S:ident, $T:ident, $X:ident, $Q:ident) => { _FP_SQRT_MEAT_4!($R, $S, $T, $X, $Q) }; }
#[macro_export] macro_rules! FP_CMP_Q { ($r:ident, $X:ident, $Y:ident, $un:ident) => { _FP_CMP!(Q, 4, $r, $X, $Y, $un) }; }
#[macro_export] macro_rules! FP_CMP_EQ_Q { ($r:ident, $X:ident, $Y:ident) => { _FP_CMP_EQ!(Q, 4, $r, $X, $Y) }; }
#[macro_export] macro_rules! FP_TO_INT_Q { ($r:ident, $X:ident, $rsz:ident, $rsg:ident) => { _FP_TO_INT!(Q, 4, $r, $X, $rsz, $rsg) }; }
#[macro_export] macro_rules! FP_TO_INT_ROUND_Q { ($r:ident, $X:ident, $rsz:ident, $rsg:ident) => { _FP_TO_INT_ROUND!(Q, 4, $r, $X, $rsz, $rsg) }; }
#[macro_export] macro_rules! FP_FROM_INT_Q { ($X:ident, $r:ident, $rs:ident, $rt:ident) => { _FP_FROM_INT!(Q, 4, $X, $r, $rs, $rt) }; }
#[macro_export] macro_rules! _FP_FRAC_HIGH_Q { ($X:ident) => { _FP_FRAC_HIGH_4!($X) }; }
#[macro_export] macro_rules! _FP_FRAC_HIGH_RAW_Q { ($X:ident) => { _FP_FRAC_HIGH_4!($X) }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
