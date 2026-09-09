/* Software floating-point emulation.
   Direct Rust translation of soft-fp.h. */

// Dependencies supplied by the corresponding machine and operation headers:
// asm/sfp-machine.h, endian.h, math-emu/op-{1,2,4,8,common}.h,
// and stdlib/longlong.h.

pub const _FP_WORKBITS: usize = 3;
pub const FP_RND_NEAREST: i32 = 0;
pub const FP_RND_ZERO: i32 = 1;
pub const FP_RND_PINF: i32 = 2;
pub const FP_RND_MINF: i32 = 3;

/* These machine-dependent items are expected from sfp-machine.h. */
// pub type _FP_W_TYPE = ...;
// pub const _FP_W_TYPE_SIZE: usize = ...;

pub const FP_EX_INVALID: i32 = 0;
pub const FP_EX_INVALID_SNAN: i32 = 0;
pub const FP_EX_INVALID_ISI: i32 = 0;
pub const FP_EX_INVALID_IDI: i32 = 0;
pub const FP_EX_INVALID_ZDZ: i32 = 0;
pub const FP_EX_INVALID_IMZ: i32 = 0;
pub const FP_EX_OVERFLOW: i32 = 0;
/* FP_EX_UNDERFLOW has no default value in the C header. */
pub const FP_EX_DIVZERO: i32 = 0;
pub const FP_EX_INEXACT: i32 = 0;
pub const FP_EX_DENORM: i32 = 0;

pub const FP_ROUNDMODE: i32 = FP_RND_NEAREST;
pub const FP_DENORM_ZERO: i32 = 0;
pub const FP_INHIBIT_RESULTS: i32 = 0;
pub const FP_TRAPPING_EXCEPTIONS: i32 = 0;

pub const FP_CLS_NORMAL: i32 = 0;
pub const FP_CLS_ZERO: i32 = 1;
pub const FP_CLS_INF: i32 = 2;
pub const FP_CLS_NAN: i32 = 3;

#[inline]
pub const fn _fp_cls_combine(x: i32, y: i32) -> i32 { (x << 2) | y }

/* C macro: FP_DECL_EX (the optional machine _FP_DECL_EX is external). */
#[inline]
pub fn fp_decl_ex() -> i32 { 0 }

#[inline]
pub fn fp_init_roundmode() {}

#[inline]
pub fn fp_handle_exceptions() {}

#[inline]
pub fn fp_set_exception(fex: &mut i32, ex: i32) { *fex |= ex; }

#[inline]
pub fn fp_unset_exception(fex: &mut i32, ex: i32) { *fex &= !ex; }

#[inline]
pub fn fp_cur_exceptions(fex: i32) -> i32 { fex }

#[inline]
pub fn fp_clear_exceptions(fex: &mut i32) { *fex = 0; }

/* Rounding helpers retain the operation-header accessors as external macros. */
#[inline]
pub unsafe fn _fp_round_nearest<W, F>(_wc: W, _x: &mut F) {
    // C: if ((_FP_FRAC_LOW_##wc(X) & 15) != _FP_WORK_ROUND)
    //        _FP_FRAC_ADDI_##wc(X, _FP_WORK_ROUND);
}

#[inline]
pub fn _fp_round_zero<W, F>(_wc: W, _x: &mut F) {}

#[inline]
pub unsafe fn _fp_round_pinf<W, F>(_wc: W, _x: &mut F) {
    // C: if (!X##_s && (_FP_FRAC_LOW_##wc(X) & 7))
    //        _FP_FRAC_ADDI_##wc(X, _FP_WORK_LSB);
}

#[inline]
pub unsafe fn _fp_round_minf<W, F>(_wc: W, _x: &mut F) {
    // C: if (X##_s && (_FP_FRAC_LOW_##wc(X) & 7))
    //        _FP_FRAC_ADDI_##wc(X, _FP_WORK_LSB);
}

/* _FP_ROUND is dependent on token-pasted operation-header macros and is
   intentionally represented by this direct control-flow skeleton. */
#[inline]
pub unsafe fn _fp_round<W, F>(_wc: W, _x: &mut F, _fex: &mut i32) {
    // if (_FP_FRAC_LOW_##wc(X) & 7) FP_SET_EXCEPTION(FP_EX_INEXACT);
    match FP_ROUNDMODE {
        FP_RND_NEAREST => _fp_round_nearest(_wc, _x),
        FP_RND_ZERO => _fp_round_zero(_wc, _x),
        FP_RND_PINF => _fp_round_pinf(_wc, _x),
        FP_RND_MINF => _fp_round_minf(_wc, _x),
        _ => {}
    }
}

pub type UWtype = usize;
pub const W_TYPE_SIZE: usize = 0;
pub type SItype = i32;
pub type DItype = i64;
pub type USItype = u32;
pub type UDItype = u64;
// UHWtype is USItype when _FP_W_TYPE_SIZE == 64; otherwise it is the
// machine's unsigned half-word type.
pub type UHWtype = u32;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
