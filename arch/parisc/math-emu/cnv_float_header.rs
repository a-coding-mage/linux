/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Linux/PA-RISC Project (http://www.parisc-linux.org/)
 *
 * Floating-point emulation code
 *  Copyright (C) 2001 Hewlett-Packard (Paul Bame) <bame@debian.org>
 */

// PA header file -- do not include this header file for non-PA builds.

/* Some more constants */
pub const SGL_FX_MAX_EXP: i32 = 30;
pub const DBL_FX_MAX_EXP: i32 = 62;
pub const QUAD_FX_MAX_EXP: i32 = 126;

macro_rules! Dintp1 { ($object:expr) => { $object }; }
macro_rules! Dintp2 { ($object:expr) => { $object }; }
macro_rules! Duintp1 { ($object:expr) => { $object }; }
macro_rules! Duintp2 { ($object:expr) => { $object }; }
macro_rules! Qintp0 { ($object:expr) => { $object }; }
macro_rules! Qintp1 { ($object:expr) => { $object }; }
macro_rules! Qintp2 { ($object:expr) => { $object }; }
macro_rules! Qintp3 { ($object:expr) => { $object }; }

/* Single format macros */
macro_rules! Sgl_to_dbl_exponent { ($src_exponent:expr, $dest:expr) => { Deposit_dexponent!($dest, $src_exponent + (DBL_BIAS - SGL_BIAS)); }; }
macro_rules! Sgl_to_dbl_mantissa { ($src_mantissa:expr, $destA:expr, $destB:expr) => {{ Deposit_dmantissap1!($destA, $src_mantissa >> 3); Dmantissap2!($destB) = $src_mantissa << 29; }}; }
macro_rules! Sgl_isinexact_to_fix { ($sgl_value:expr, $exponent:expr) => { if $exponent < (SGL_P - 1) { Sall!($sgl_value) << (SGL_EXP_LENGTH + 1 + $exponent) } else { FALSE } }; }
macro_rules! Int_isinexact_to_sgl { ($int_value:expr) => { ($int_value << (33 - SGL_EXP_LENGTH)) != 0 }; }
macro_rules! Sgl_roundnearest_from_int { ($int_value:expr, $sgl_value:expr) => {{ if $int_value & (1 << (SGL_EXP_LENGTH - 2)) != 0 { if (($int_value << (34 - SGL_EXP_LENGTH)) != 0) || Slow!($sgl_value) { Sall!($sgl_value) += 1; } } }}; }
macro_rules! Dint_isinexact_to_sgl { ($a:expr, $b:expr) => { ((Dintp1!($a) << (33 - SGL_EXP_LENGTH)) != 0) || Dintp2!($b) != 0 }; }
macro_rules! Sgl_roundnearest_from_dint { ($a:expr, $b:expr, $s:expr) => {{ if Dintp1!($a) & (1 << (SGL_EXP_LENGTH - 2)) != 0 { if ((Dintp1!($a) << (34 - SGL_EXP_LENGTH)) != 0) || Dintp2!($b) != 0 || Slow!($s) { Sall!($s) += 1; } } }}; }
macro_rules! Dint_isinexact_to_dbl { ($v:expr) => { Dintp2!($v) << (33 - DBL_EXP_LENGTH) }; }
macro_rules! Dbl_roundnearest_from_dint { ($b:expr, $a:expr, $db:expr) => {{ if Dintp2!($b) & (1 << (DBL_EXP_LENGTH - 2)) != 0 && ((Dintp2!($b) << (34 - DBL_EXP_LENGTH)) != 0 || Dlowp2!($db) != 0) { Dallp2!($db) += 1; if Dallp2!($db) == 0 { Dallp1!($a) += 1; } } }}; }
macro_rules! Sgl_isone_roundbit { ($v:expr, $e:expr) => { (Sall!($v) << (SGL_EXP_LENGTH + 1 + $e)) >> 31 }; }
macro_rules! Sgl_isone_stickybit { ($v:expr, $e:expr) => { if $e < (SGL_P - 2) { Sall!($v) << (SGL_EXP_LENGTH + 2 + $e) } else { FALSE } }; }

/* Double format macros */
macro_rules! Dbl_to_sgl_exponent { ($src:expr, $dest:expr) => { $dest = $src + (SGL_BIAS - DBL_BIAS); }; }
macro_rules! Dbl_to_sgl_mantissa { ($a:expr,$b:expr,$dest:expr,$inexact:expr,$guard:expr,$sticky:expr,$odd:expr) => {{ Shiftdouble!(Dmantissap1!($a), Dmantissap2!($b), 29, $dest); $guard = Dbit3p2!($b); $sticky = Dallp2!($b) << 4; $inexact = $guard | $sticky; $odd = Dbit2p2!($b); }}; }
macro_rules! Dbl_to_sgl_denormalized { ($a:expr,$b:expr,$exp:expr,$dest:expr,$inexact:expr,$guard:expr,$sticky:expr,$odd:expr,$tiny:expr) => {{
    Deposit_dexponent!($a, 1); $tiny = TRUE;
    if $exp >= -2 { if $exp == 0 {
        $inexact = Dallp2!($b) << 3; $guard = $inexact >> 31; $sticky = $inexact << 1;
        Shiftdouble!(Dmantissap1!($a), Dmantissap2!($b), 29, $dest); $odd = $dest << 31;
        if $inexact { match Rounding_mode!() {
            ROUNDPLUS => { if Dbl_iszero_sign!($a) { $dest += 1; if Sgl_isone_hidden!($dest) { $tiny = FALSE; } $dest -= 1; } },
            ROUNDMINUS => { if Dbl_isone_sign!($a) { $dest += 1; if Sgl_isone_hidden!($dest) { $tiny = FALSE; } $dest -= 1; } },
            ROUNDNEAREST => { if $guard && ($sticky || $odd) { $dest += 1; if Sgl_isone_hidden!($dest) { $tiny = FALSE; } $dest -= 1; } },
            _ => {}
        }}
        $guard = $odd; $sticky = $inexact; $inexact = $inexact || $guard; $dest >>= 1;
        Deposit_dsign!($a, 0); Shiftdouble!(Dallp1!($a), Dallp2!($b), 30, $dest); $odd = $dest << 31;
    } else {
        $inexact = Dallp2!($b) << (2 + $exp); $guard = $inexact >> 31; $sticky = $inexact << 1;
        Deposit_dsign!($a, 0); if $exp == -2 { $dest = Dallp1!($a); } else { Variable_shift_double!(Dallp1!($a), Dallp2!($b), 30 - $exp, $dest); } $odd = $dest << 31;
    }} else {
        Deposit_dsign!($a, 0); if $exp > (1 - SGL_P) {
            $dest = Dallp1!($a) >> (-2 - $exp); $inexact = Dallp1!($a) << (34 + $exp); $guard = $inexact >> 31; $sticky = ($inexact << 1) | Dallp2!($b); $inexact = $inexact | Dallp2!($b); $odd = $dest << 31;
        } else { $dest = 0; $inexact = Dallp1!($a) | Dallp2!($b); if $exp == (1 - SGL_P) { $guard = Dhidden!($a); $sticky = Dmantissap1!($a) | Dallp2!($b); } else { $guard = 0; $sticky = $inexact; } $odd = 0; }
    } $exp = 0;
}}; }

macro_rules! Dbl_isinexact_to_fix { ($a:expr,$b:expr,$e:expr) => { if $e < (DBL_P - 33) { Dallp2!($b) != 0 || (Dallp1!($a) << (DBL_EXP_LENGTH + 1 + $e)) != 0 } else if $e < (DBL_P - 1) { Dallp2!($b) << ($e + (33 - DBL_P)) } else { FALSE } }; }
macro_rules! Dbl_isoverflow_to_int { ($e:expr,$a:expr,$b:expr) => { ($e > SGL_FX_MAX_EXP + 1) || Dsign!($a) == 0 || Dmantissap1!($a) != 0 || (Dallp2!($b) >> 21) != 0 }; }
macro_rules! Dbl_isone_roundbit { ($a:expr,$b:expr,$e:expr) => { (if $e < (DBL_P - 33) { Dallp1!($a) >> ((30 - DBL_EXP_LENGTH) - $e) } else { Dallp2!($b) >> ((DBL_P - 2) - $e) }) & 1 }; }
macro_rules! Dbl_isone_stickybit { ($a:expr,$b:expr,$e:expr) => { if $e < (DBL_P - 34) { Dallp2!($b) != 0 || (Dallp1!($a) << (DBL_EXP_LENGTH + 2 + $e)) != 0 } else if $e < (DBL_P - 2) { Dallp2!($b) << ($e + (34 - DBL_P)) } else { FALSE } }; }

/* Int macros */
macro_rules! Int_from_sgl_mantissa { ($v:expr,$e:expr) => { Sall!($v) = ((Sall!($v) << SGL_EXP_LENGTH) as u32) >> (31 - $e); }; }
macro_rules! Int_from_dbl_mantissa { ($a:expr,$b:expr,$e:expr) => {{ Shiftdouble!(Dallp1!($a), Dallp2!($b), 22, Dallp1!($a)); if $e < 31 { Dallp1!($a) >>= 30 - $e; } else { Dallp1!($a) <<= 1; } }}; }
macro_rules! Int_negate { ($v:expr) => { $v = -$v; }; }

/* Dint macros */
macro_rules! Dint_setzero { ($a:expr,$b:expr) => {{ Dintp1!($a) = 0; Dintp2!($b) = 0; }}; }
macro_rules! Dint_setone_sign { ($a:expr,$b:expr) => {{ Dintp1!($a) = !Dintp1!($a); Dintp2!($b) = -Dintp2!($b); if Dintp2!($b) == 0 { Dintp1!($a) += 1; } }}; }
macro_rules! Dint_set_minint { ($a:expr,$b:expr) => {{ Dintp1!($a) = (1u32 << 31); Dintp2!($b) = 0; }}; }
macro_rules! Dint_isone_lowp2 { ($b:expr) => { Dintp2!($b) & 0o1 }; }
macro_rules! Dint_increment { ($a:expr,$b:expr) => {{ Dintp2!($b) += 1; if Dintp2!($b) == 0 { Dintp1!($a) += 1; } }}; }
macro_rules! Dint_decrement { ($a:expr,$b:expr) => {{ if Dintp2!($b) == 0 { Dintp1!($a) -= 1; } Dintp2!($b) -= 1; }}; }
macro_rules! Dint_negate { ($a:expr,$b:expr) => {{ Dintp1!($a) = !Dintp1!($a); Dintp2!($b) = -Dintp2!($b); if Dintp2!($b) == 0 { Dintp1!($a) += 1; } }}; }
macro_rules! Dint_copyfromptr { ($src:expr,$a:expr,$b:expr) => {{ Dintp1!($a) = (*$src).wd0; Dintp2!($b) = (*$src).wd1; }}; }
macro_rules! Dint_copytoptr { ($a:expr,$b:expr,$dest:expr) => {{ (*$dest).wd0 = Dintp1!($a); (*$dest).wd1 = Dintp2!($b); }}; }
macro_rules! Dint_from_sgl_mantissa { ($s:expr,$e:expr,$a:expr,$b:expr) => {{ Sall!($s) <<= SGL_EXP_LENGTH; if $e <= 31 { Dintp1!($a)=0; Dintp2!($b)=(Sall!($s) as u32)>>(31-$e); } else { Dintp1!($a)=Sall!($s)>>(63-$e); Dintp2!($b)=Sall!($s)<<($e-31); } }}; }
macro_rules! Dint_from_dbl_mantissa { ($a:expr,$b:expr,$e:expr,$da:expr,$db:expr) => {{ if $e < 32 { Dintp1!($da)=0; if $e <= 20 { Dintp2!($db)=Dallp1!($a)>>(20-$e); } else { Variable_shift_double!(Dallp1!($a),Dallp2!($b),52-$e,Dintp2!($db)); } } else if $e <= 52 { Dintp1!($da)=Dallp1!($a)>>(52-$e); if $e == 52 { Dintp2!($db)=Dallp2!($b); } else { Variable_shift_double!(Dallp1!($a),Dallp2!($b),52-$e,Dintp2!($db)); } } else { Variable_shift_double!(Dallp1!($a),Dallp2!($b),84-$e,Dintp1!($da)); Dintp2!($db)=Dallp2!($b)<<($e-52); } }}; }

/* other macros */
macro_rules! Find_ms_one_bit { ($value:expr,$position:expr) => {{ let mut var: i32; var = 8; while var >= 1 { if $value >> (32 - $position) != 0 { $position -= var; } else { $position += var; } var >>= 1; } if $value >> (32 - $position) == 0 { $position -= 1; } else { $position -= 2; } }}; }

/* Unsigned int macros */
macro_rules! Duint_copyfromptr { ($src:expr,$a:expr,$b:expr) => { Dint_copyfromptr!($src,$a,$b); }; }
macro_rules! Duint_copytoptr { ($a:expr,$b:expr,$dest:expr) => { Dint_copytoptr!($a,$b,$dest); }; }
macro_rules! Suint_isinexact_to_sgl { ($v:expr) => { $v << (32 - SGL_EXP_LENGTH) }; }
macro_rules! Sgl_roundnearest_from_suint { ($v:expr,$s:expr) => {{ if $v & (1 << (SGL_EXP_LENGTH - 1)) != 0 && (($v << (33 - SGL_EXP_LENGTH)) != 0 || Slow!($s)) { Sall!($s) += 1; } }}; }
macro_rules! Duint_isinexact_to_sgl { ($a:expr,$b:expr) => { (Duintp1!($a) << (32 - SGL_EXP_LENGTH)) != 0 || Duintp2!($b) != 0 }; }
macro_rules! Sgl_roundnearest_from_duint { ($a:expr,$b:expr,$s:expr) => {{ if Duintp1!($a) & (1 << (SGL_EXP_LENGTH - 1)) != 0 && ((Duintp1!($a) << (33 - SGL_EXP_LENGTH)) != 0 || Duintp2!($b) != 0 || Slow!($s)) { Sall!($s) += 1; } }}; }
macro_rules! Duint_isinexact_to_dbl { ($v:expr) => { Duintp2!($v) << (32 - DBL_EXP_LENGTH) }; }
macro_rules! Dbl_roundnearest_from_duint { ($b:expr,$a:expr,$db:expr) => {{ if Duintp2!($b) & (1 << (DBL_EXP_LENGTH - 1)) != 0 && ((Duintp2!($b) << (33 - DBL_EXP_LENGTH)) != 0 || Dlowp2!($db) != 0) { Dallp2!($db) += 1; if Dallp2!($db) == 0 { Dallp1!($a) += 1; } } }}; }
macro_rules! Suint_from_sgl_mantissa { ($src:expr,$e:expr,$result:expr) => { Sall!($result) = ((Sall!($src) << SGL_EXP_LENGTH) as u32) >> (31 - $e); }; }
macro_rules! Sgl_isinexact_to_unsigned { ($v:expr,$e:expr) => { Sgl_isinexact_to_fix!($v,$e) }; }
macro_rules! Duint_setzero { ($a:expr,$b:expr) => { Dint_setzero!($a,$b); }; }
macro_rules! Duint_increment { ($a:expr,$b:expr) => { Dint_increment!($a,$b); }; }
macro_rules! Duint_isone_lowp2 { ($b:expr) => { Dint_isone_lowp2!($b) }; }
macro_rules! Duint_from_sgl_mantissa { ($s:expr,$e:expr,$a:expr,$b:expr) => {{ let val = Sall!($s) << SGL_EXP_LENGTH; if $e <= 31 { Dintp1!($a)=0; Dintp2!($b)=val>>(31-$e); } else { Dintp1!($a)=val>>(63-$e); Dintp2!($b)=if $e<=62 { val<<($e-31) } else { 0 }; } }}; }
macro_rules! Suint_from_dbl_mantissa { ($a:expr,$b:expr,$e:expr,$dest:expr) => {{ Shiftdouble!(Dallp1!($a),Dallp2!($b),21,$dest); $dest = ($dest as u32) >> (31-$e); }}; }
macro_rules! Dbl_isinexact_to_unsigned { ($a:expr,$b:expr,$e:expr) => { Dbl_isinexact_to_fix!($a,$b,$e) }; }
macro_rules! Duint_from_dbl_mantissa { ($a:expr,$b:expr,$e:expr,$da:expr,$db:expr) => { Dint_from_dbl_mantissa!($a,$b,$e,$da,$db); }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
