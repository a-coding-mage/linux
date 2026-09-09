/* Machine-dependent software floating-point definitions.
   Sparc64 kernel version.
   Copyright (C) 1997,1998,1999 Free Software Foundation, Inc.
   This file is part of the GNU C Library.

   Rust translation of sfp-machine_64.h. */

pub const _FP_W_TYPE_SIZE: usize = 64;
// C types: unsigned long, signed long, and long.
pub type _FP_W_TYPE = ::core::primitive::u64;
pub type _FP_WS_TYPE = ::core::primitive::i64;
pub type _FP_I_TYPE = ::core::primitive::i64;

#[macro_export]
macro_rules! _FP_MUL_MEAT_S {
    ($r:expr, $x:expr, $y:expr) => {
        _FP_MUL_MEAT_1_imm!(_FP_WFRACBITS_S, $r, $x, $y)
    };
}
#[macro_export]
macro_rules! _FP_MUL_MEAT_D {
    ($r:expr, $x:expr, $y:expr) => {
        _FP_MUL_MEAT_1_wide!(_FP_WFRACBITS_D, $r, $x, $y, umul_ppmm)
    };
}
#[macro_export]
macro_rules! _FP_MUL_MEAT_Q {
    ($r:expr, $x:expr, $y:expr) => {
        _FP_MUL_MEAT_2_wide!(_FP_WFRACBITS_Q, $r, $x, $y, umul_ppmm)
    };
}

#[macro_export]
macro_rules! _FP_DIV_MEAT_S {
    ($r:expr, $x:expr, $y:expr) => {
        _FP_DIV_MEAT_1_imm!(S, $r, $x, $y, _FP_DIV_HELP_imm)
    };
}
#[macro_export]
macro_rules! _FP_DIV_MEAT_D {
    ($r:expr, $x:expr, $y:expr) => {
        _FP_DIV_MEAT_1_udiv_norm!(D, $r, $x, $y)
    };
}
#[macro_export]
macro_rules! _FP_DIV_MEAT_Q {
    ($r:expr, $x:expr, $y:expr) => {
        _FP_DIV_MEAT_2_udiv!(Q, $r, $x, $y)
    };
}

// The following identifiers are supplied by the surrounding soft-fp implementation.
#[macro_export]
macro_rules! _FP_NANFRAC_S { () => { ((_FP_QNANBIT_S << 1) - 1) }; }
#[macro_export]
macro_rules! _FP_NANFRAC_D { () => { ((_FP_QNANBIT_D << 1) - 1) }; }
#[macro_export]
macro_rules! _FP_NANFRAC_Q { () => { ((_FP_QNANBIT_Q << 1) - 1, -1) }; }
pub const _FP_NANSIGN_S: i32 = 0;
pub const _FP_NANSIGN_D: i32 = 0;
pub const _FP_NANSIGN_Q: i32 = 0;
pub const _FP_KEEPNANFRACP: i32 = 1;

/* If one NaN is signaling and the other is not, choose that one;
   otherwise choose X for _Qp_* and _Q_*, while CPU emulation chooses Y.
   The C token-pasting interface is represented by explicit field expressions. */
#[macro_export]
macro_rules! _FP_CHOOSENAN {
    ($fs:ident, $wc:ident, $r_s:expr, $r_c:expr, $x_s:expr, $y_s:expr,
     $x_frac:expr, $y_frac:expr, $op:expr, $x_nanbit:expr, $y_nanbit:expr,
     $copy_x:expr, $copy_y:expr, $fp_cls_nan:expr) => {{
        if (($y_frac & $y_nanbit) != 0) && (($x_frac & $x_nanbit) == 0) {
            $r_s = $x_s;
            $copy_x;
        } else {
            $r_s = $y_s;
            $copy_y;
        }
        $r_c = $fp_cls_nan;
    }};
}

/* Build-time condition: preserve the default current-thread rounding-mode expression. */
#[macro_export]
macro_rules! FP_ROUNDMODE {
    () => { ((current_thread_info().xfsr[0] >> 30) & 0x3) };
}

pub const FP_EX_INVALID: i32 = 1 << 4;
pub const FP_EX_OVERFLOW: i32 = 1 << 3;
pub const FP_EX_UNDERFLOW: i32 = 1 << 2;
pub const FP_EX_DIVZERO: i32 = 1 << 1;
pub const FP_EX_INEXACT: i32 = 1 << 0;

#[macro_export]
macro_rules! FP_HANDLE_EXCEPTIONS { () => { return _fex; }; }
#[macro_export]
macro_rules! FP_INHIBIT_RESULTS {
    () => { ((current_thread_info().xfsr[0] >> 23) & _fex) };
}
#[macro_export]
macro_rules! FP_TRAPPING_EXCEPTIONS {
    () => { ((current_thread_info().xfsr[0] >> 23) & 0x1f) };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
