/* Software floating-point emulation.  Rust translation of op-1.h. */

/* External types, constants, unions, and helper functions are supplied by
 * the surrounding emulation implementation. */

macro_rules! _FP_FRAC_DECL_1 { ($x:tt) => { let mut ::kernel::macros::paste!([<$x _f>]) = 0; } }
macro_rules! _FP_FRAC_COPY_1 { ($d:tt, $s:tt) => { ::kernel::macros::paste!([<$d _f>]) = ::kernel::macros::paste!([<$s _f>]); } }
macro_rules! _FP_FRAC_SET_1 { ($x:tt, $i:expr) => { ::kernel::macros::paste!([<$x _f>]) = $i; } }
macro_rules! _FP_FRAC_HIGH_1 { ($x:tt) => { ::kernel::macros::paste!([<$x _f>]) } }
macro_rules! _FP_FRAC_LOW_1 { ($x:tt) => { ::kernel::macros::paste!([<$x _f>]) } }
macro_rules! _FP_FRAC_WORD_1 { ($x:tt, $w:expr) => { ::kernel::macros::paste!([<$x _f>]) } }
macro_rules! _FP_FRAC_ADDI_1 { ($x:tt, $i:expr) => { ::kernel::macros::paste!([<$x _f>]) += $i; } }
macro_rules! _FP_FRAC_SLL_1 { ($x:tt, $n:expr) => {{ ::kernel::macros::paste!([<$x _f>]) <<= $n; }} }
macro_rules! _FP_FRAC_SRL_1 { ($x:tt, $n:expr) => { ::kernel::macros::paste!([<$x _f>]) >>= $n; } }
macro_rules! _FP_FRAC_SRS_1 { ($x:tt, $n:expr, $sz:expr) => {{
    ::kernel::macros::paste!([<$x _f>]) = (::kernel::macros::paste!([<$x _f>]) >> $n) | if $n == 1 { ::kernel::macros::paste!([<$x _f>]) & 1 } else { ((::kernel::macros::paste!([<$x _f>]) << (_FP_W_TYPE_SIZE - $n)) != 0) as _ };
}} }
macro_rules! _FP_FRAC_ADD_1 { ($r:tt, $x:tt, $y:tt) => { ::kernel::macros::paste!([<$r _f>]) = ::kernel::macros::paste!([<$x _f>]) + ::kernel::macros::paste!([<$y _f>]); } }
macro_rules! _FP_FRAC_SUB_1 { ($r:tt, $x:tt, $y:tt) => { ::kernel::macros::paste!([<$r _f>]) = ::kernel::macros::paste!([<$x _f>]) - ::kernel::macros::paste!([<$y _f>]); } }
macro_rules! _FP_FRAC_DEC_1 { ($x:tt, $y:tt) => { ::kernel::macros::paste!([<$x _f>]) -= ::kernel::macros::paste!([<$y _f>]); } }
macro_rules! _FP_FRAC_CLZ_1 { ($z:ident, $x:tt) => { __FP_CLZ!($z, ::kernel::macros::paste!([<$x _f>])); } }

macro_rules! _FP_FRAC_NEGP_1 { ($x:tt) => { ((::kernel::macros::paste!([<$x _f>])) as _FP_WS_TYPE) < 0 } }
macro_rules! _FP_FRAC_ZEROP_1 { ($x:tt) => { ::kernel::macros::paste!([<$x _f>]) == 0 } }
macro_rules! _FP_FRAC_OVERP_1 { ($fs:ident, $x:tt) => { ::kernel::macros::paste!([<$x _f>]) & _FP_OVERFLOW_$fs } }
macro_rules! _FP_FRAC_CLEAR_OVERP_1 { ($fs:ident, $x:tt) => { ::kernel::macros::paste!([<$x _f>]) &= !_FP_OVERFLOW_$fs; } }
macro_rules! _FP_FRAC_EQ_1 { ($x:tt, $y:tt) => { ::kernel::macros::paste!([<$x _f>]) == ::kernel::macros::paste!([<$y _f>]) } }
macro_rules! _FP_FRAC_GE_1 { ($x:tt, $y:tt) => { ::kernel::macros::paste!([<$x _f>]) >= ::kernel::macros::paste!([<$y _f>]) } }
macro_rules! _FP_FRAC_GT_1 { ($x:tt, $y:tt) => { ::kernel::macros::paste!([<$x _f>]) > ::kernel::macros::paste!([<$y _f>]) } }

const _FP_ZEROFRAC_1: i32 = 0;
const _FP_MINFRAC_1: i32 = 1;
/* Type and width are supplied by the emulation implementation. */

macro_rules! _FP_FRAC_ASSEMBLE_1 { ($r:ident, $x:tt, $rsize:expr) => { $r = ::kernel::macros::paste!([<$x _f>]); } }
macro_rules! _FP_FRAC_DISASSEMBLE_1 { ($x:tt, $r:expr, $rsize:expr) => { ::kernel::macros::paste!([<$x _f>]) = $r; } }

macro_rules! _FP_MUL_MEAT_1_imm { ($wfracbits:expr, $r:tt, $x:tt, $y:tt) => {{
    ::kernel::macros::paste!([<$r _f>]) = ::kernel::macros::paste!([<$x _f>]) * ::kernel::macros::paste!([<$y _f>]);
    _FP_FRAC_SRS_1!($r, $wfracbits - 1, 2 * $wfracbits);
}} }
macro_rules! _FP_MUL_MEAT_1_wide { ($wfracbits:expr, $r:tt, $x:tt, $y:tt, $doit:expr) => {{
    let (_z_f0, _z_f1) = ($doit)(::kernel::macros::paste!([<$x _f>]), ::kernel::macros::paste!([<$y _f>]));
    let mut _z = (_z_f0, _z_f1);
    _FP_FRAC_SRS_2!(_z, $wfracbits - 1, 2 * $wfracbits);
    ::kernel::macros::paste!([<$r _f>]) = _z.0;
}} }
macro_rules! _FP_MUL_MEAT_1_hard { ($wfracbits:expr, $r:tt, $x:tt, $y:tt) => {{
    let (_xh, _xl) = (::kernel::macros::paste!([<$x _f>]) >> (_FP_W_TYPE_SIZE / 2), ::kernel::macros::paste!([<$x _f>]) & (((1 as _FP_W_TYPE) << (_FP_W_TYPE_SIZE / 2)) - 1));
    let (_yh, _yl) = (::kernel::macros::paste!([<$y _f>]) >> (_FP_W_TYPE_SIZE / 2), ::kernel::macros::paste!([<$y _f>]) & (((1 as _FP_W_TYPE) << (_FP_W_TYPE_SIZE / 2)) - 1));
    let mut _z_f0 = _xl * _yl;
    let mut _a_f0 = _xh * _yl;
    let _a_f1 = _xl * _yh;
    let mut _z_f1 = _xh * _yh;
    _a_f0 += _a_f1;
    _a_f0 = _a_f0.wrapping_add(_a_f1);
    _z_f1 = _z_f1.wrapping_add((_a_f0 >> (_FP_W_TYPE_SIZE / 2)) << (_FP_W_TYPE_SIZE / 2));
    _z_f0 = _z_f0.wrapping_add(_a_f0 << (_FP_W_TYPE_SIZE / 2));
    let mut _z = (_z_f0, _z_f1);
    _FP_FRAC_SRS_2!(_z, $wfracbits - 1, 2 * $wfracbits);
    ::kernel::macros::paste!([<$r _f>]) = _z.0;
}} }

macro_rules! _FP_DIV_MEAT_1_imm { ($fs:ident, $r:tt, $x:tt, $y:tt, $doit:expr) => {{
    let (_q, _r) = ($doit)(::kernel::macros::paste!([<$x _f>]) << if ::kernel::macros::paste!([<$x _f>]) < ::kernel::macros::paste!([<$y _f>]) { ::kernel::macros::paste!([<$r _e>]) -= 1; _FP_WFRACBITS_$fs } else { _FP_WFRACBITS_$fs - 1 }, ::kernel::macros::paste!([<$y _f>]));
    ::kernel::macros::paste!([<$r _f>]) = _q | ((_r != 0) as _);
}} }
macro_rules! _FP_DIV_MEAT_1_udiv_norm { ($fs:ident, $r:tt, $x:tt, $y:tt) => {{
    let _y = ::kernel::macros::paste!([<$y _f>]) << _FP_WFRACXBITS_$fs;
    let (_nh, _nl) = if ::kernel::macros::paste!([<$x _f>]) < ::kernel::macros::paste!([<$y _f>]) { ::kernel::macros::paste!([<$r _e>]) -= 1; ( ::kernel::macros::paste!([<$x _f>]), 0 ) } else { ( ::kernel::macros::paste!([<$x _f>]) >> 1, ::kernel::macros::paste!([<$x _f>]) << (_FP_W_TYPE_SIZE - 1) ) };
    let (_q, _r) = udiv_qrnnd!(_nh, _nl, _y);
    ::kernel::macros::paste!([<$r _f>]) = _q | ((_r != 0) as _);
}} }
macro_rules! _FP_DIV_MEAT_1_udiv { ($fs:ident, $r:tt, $x:tt, $y:tt) => {{
    let (_nh, _nl) = if ::kernel::macros::paste!([<$x _f>]) < ::kernel::macros::paste!([<$y _f>]) { ::kernel::macros::paste!([<$r _e>]) -= 1; (::kernel::macros::paste!([<$x _f>]) >> _FP_WFRACXBITS_$fs, ::kernel::macros::paste!([<$x _f>]) << _FP_WFRACBITS_$fs) } else { (::kernel::macros::paste!([<$x _f>]) >> (_FP_WFRACXBITS_$fs + 1), ::kernel::macros::paste!([<$x _f>]) << (_FP_WFRACBITS_$fs - 1)) };
    let (_q, _r) = udiv_qrnnd!(_nh, _nl, ::kernel::macros::paste!([<$y _f>]));
    ::kernel::macros::paste!([<$r _f>]) = _q | ((_r != 0) as _);
}} }

macro_rules! _FP_SQRT_MEAT_1 { ($r:tt, $s:tt, $t:tt, $x:tt, $q:ident) => {{
    while $q != _FP_WORK_ROUND { ::kernel::macros::paste!([<$t _f>]) = ::kernel::macros::paste!([<$s _f>]) + $q; if ::kernel::macros::paste!([<$t _f>]) <= ::kernel::macros::paste!([<$x _f>]) { ::kernel::macros::paste!([<$s _f>]) = ::kernel::macros::paste!([<$t _f>]) + $q; ::kernel::macros::paste!([<$x _f>]) -= ::kernel::macros::paste!([<$t _f>]); ::kernel::macros::paste!([<$r _f>]) += $q; } _FP_FRAC_SLL_1!($x, 1); $q >>= 1; }
    if ::kernel::macros::paste!([<$x _f>]) != 0 { if ::kernel::macros::paste!([<$s _f>]) < ::kernel::macros::paste!([<$x _f>]) { ::kernel::macros::paste!([<$r _f>]) |= _FP_WORK_ROUND; } ::kernel::macros::paste!([<$r _f>]) |= _FP_WORK_STICKY; }
}} }

macro_rules! _FP_FRAC_CONV_1_1 { ($dfs:ident, $sfs:ident, $d:tt, $s:tt) => {{
    ::kernel::macros::paste!([<$d _f>]) = ::kernel::macros::paste!([<$s _f>]);
    if _FP_WFRACBITS_$sfs > _FP_WFRACBITS_$dfs { if ::kernel::macros::paste!([<$s _c>]) != FP_CLS_NAN { _FP_FRAC_SRS_1!($d, _FP_WFRACBITS_$sfs - _FP_WFRACBITS_$dfs, _FP_WFRACBITS_$sfs); } else { _FP_FRAC_SRL_1!($d, _FP_WFRACBITS_$sfs - _FP_WFRACBITS_$dfs); } } else { ::kernel::macros::paste!([<$d _f>]) <<= _FP_WFRACBITS_$dfs - _FP_WFRACBITS_$sfs; }
}} }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
