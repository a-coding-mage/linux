/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Rust translation of parisc/math-emu/dbl_float.h.  PA primitives and
 * constants referenced below are supplied by the surrounding translation. */

/* The original file is a macro header; retain its source-level interface as
 * Rust declarative macros.  Expressions intentionally use wrapping integer
 * operations where C unsigned arithmetic is required. */

macro_rules! Dbl_firstword { ($v:expr) => { Dallp1!($v) }; }
macro_rules! Dbl_secondword { ($v:expr) => { Dallp2!($v) }; }
macro_rules! Dbl_thirdword { ($v:expr) => { dummy_location }; }
macro_rules! Dbl_fourthword { ($v:expr) => { dummy_location }; }
macro_rules! Dbl_sign { ($v:expr) => { Dsign!($v) }; }
macro_rules! Dbl_exponent { ($v:expr) => { Dexponent!($v) }; }
macro_rules! Dbl_signexponent { ($v:expr) => { Dsignexponent!($v) }; }
macro_rules! Dbl_mantissap1 { ($v:expr) => { Dmantissap1!($v) }; }
macro_rules! Dbl_mantissap2 { ($v:expr) => { Dmantissap2!($v) }; }
macro_rules! Dbl_exponentmantissap1 { ($v:expr) => { Dexponentmantissap1!($v) }; }
macro_rules! Dbl_allp1 { ($v:expr) => { Dallp1!($v) }; }
macro_rules! Dbl_allp2 { ($v:expr) => { Dallp2!($v) }; }

macro_rules! Dbl_and_signs { ($a:expr,$b:expr) => {{ Dallp1!($a) = (Dallp1!($b) | !(1u32<<31)) & Dallp1!($a); }}; }
macro_rules! Dbl_or_signs { ($a:expr,$b:expr) => {{ Dallp1!($a) = (Dallp1!($b)&(1u32<<31)) | Dallp1!($a); }}; }
macro_rules! Dbl_clear_exponent_set_hidden { ($v:expr) => { Deposit_dexponent!($v,1) }; }
macro_rules! Dbl_clear_signexponent_set_hidden { ($v:expr) => { Deposit_dsignexponent!($v,1) }; }
macro_rules! Dbl_clear_sign { ($v:expr) => {{ Dallp1!($v) &= !(1u32<<31); }}; }
macro_rules! Dbl_clear_signexponent { ($v:expr) => {{ Dallp1!($v) &= Dmantissap1!(!0u32); }}; }

macro_rules! Dbl_rightshift { ($a:expr,$b:expr,$n:expr) => {{ let n=$n; if n>=32 { Dallp2!($b)=Dallp1!($a)>>(n-32); Dallp1!($a)=0; } else if n>0 { Variable_shift_double!(Dallp1!($a),Dallp2!($b),n,Dallp2!($b)); Dallp1!($a)>>=n; } }}; }
macro_rules! Dbl_rightshift_exponentmantissa { ($a:expr,$b:expr,$n:expr) => {{ let n=$n; if n>=32 { Dallp2!($b)=Dexponentmantissap1!($a)>>(n-32); Dallp1!($a)&=1u32<<31; } else if n>0 { Variable_shift_double!(Dexponentmantissap1!($a),Dallp2!($b),n,Dallp2!($b)); Deposit_dexponentmantissap1!($a,Dexponentmantissap1!($a)>>n); } }}; }
macro_rules! Dbl_leftshift { ($a:expr,$b:expr,$n:expr) => {{ let n=$n; if n>=32 { Dallp1!($a)=Dallp2!($b)<<(n-32); Dallp2!($b)=0; } else if n>0 { Dallp1!($a)=(Dallp1!($a)<<n)|(Dallp2!($b)>>(32-n)); Dallp2!($b)<<=n; } }}; }
macro_rules! Dbl_arithrightshiftby1 { ($a:expr,$b:expr) => {{ Shiftdouble!(Dallp1!($a),Dallp2!($b),1,Dallp2!($b)); Dallp1!($a)=(Dallp1!($a) as i32 >> 1) as u32; }}; }
macro_rules! Dbl_signextendedsign { ($v:expr) => { Dsignedsign!($v) }; }
macro_rules! Dbl_isone_hidden { ($v:expr) => { Is_dhidden!($v)!=0 }; }
macro_rules! Dbl_isone_sign { ($v:expr) => { Is_dsign!($v)!=0 }; }
macro_rules! Dbl_isone_hiddenoverflow { ($v:expr) => { Is_dhiddenoverflow!($v)!=0 }; }
macro_rules! Dbl_isone_lowmantissap1 { ($v:expr) => { Is_dlowp1!($v)!=0 }; }
macro_rules! Dbl_isone_lowmantissap2 { ($v:expr) => { Is_dlowp2!($v)!=0 }; }
macro_rules! Dbl_isone_signaling { ($v:expr) => { Is_dsignaling!($v)!=0 }; }
macro_rules! Dbl_is_signalingnan { ($v:expr) => { Dsignalingnan!($v)==0xfff }; }
macro_rules! Dbl_isnotzero { ($a:expr,$b:expr) => { Dallp1!($a)!=0 || Dallp2!($b)!=0 }; }
macro_rules! Dbl_iszero { ($a:expr,$b:expr) => { Dallp1!($a)==0 && Dallp2!($b)==0 }; }
macro_rules! Dbl_iszero_allp1 { ($v:expr) => { Dallp1!($v)==0 }; }
macro_rules! Dbl_iszero_allp2 { ($v:expr) => { Dallp2!($v)==0 }; }
macro_rules! Dbl_iszero_hidden { ($v:expr) => { Is_dhidden!($v)==0 }; }
macro_rules! Dbl_iszero_sign { ($v:expr) => { Is_dsign!($v)==0 }; }
macro_rules! Dbl_iszero_exponent { ($v:expr) => { Dexponent!($v)==0 }; }
macro_rules! Dbl_isnotzero_exponent { ($v:expr) => { Dexponent!($v)!=0 }; }
macro_rules! Dbl_iszero_mantissa { ($a:expr,$b:expr) => { Dmantissap1!($a)==0 && Dmantissap2!($b)==0 }; }
macro_rules! Dbl_isnotzero_mantissa { ($a:expr,$b:expr) => { Dmantissap1!($a)!=0 || Dmantissap2!($b)!=0 }; }
macro_rules! Dbl_isinfinity_exponent { ($v:expr) => { Dexponent!($v)==DBL_INFINITY_EXPONENT }; }
macro_rules! Dbl_isnotinfinity_exponent { ($v:expr) => { Dexponent!($v)!=DBL_INFINITY_EXPONENT }; }
macro_rules! Dbl_isinfinity { ($a:expr,$b:expr) => { Dexponent!($a)==DBL_INFINITY_EXPONENT && Dmantissap1!($a)==0 && Dmantissap2!($b)==0 }; }
macro_rules! Dbl_isnan { ($a:expr,$b:expr) => { Dexponent!($a)==DBL_INFINITY_EXPONENT && (Dmantissap1!($a)!=0 || Dmantissap2!($b)!=0) }; }
macro_rules! Dbl_isnotnan { ($a:expr,$b:expr) => { Dexponent!($a)!=DBL_INFINITY_EXPONENT || (Dmantissap1!($a)==0 && Dmantissap2!($b)==0) }; }

macro_rules! Dbl_set_exponent { ($v:expr,$e:expr) => { Deposit_dexponent!($v,$e) }; }
macro_rules! Dbl_set_quiet { ($v:expr) => { Deposit_dhigh2mantissa!($v,1) }; }
macro_rules! Dbl_set_sign { ($v:expr,$s:expr) => { Deposit_dsign!($v,$s) }; }
macro_rules! Dbl_setone_sign { ($v:expr) => { Deposit_dsign!($v,1) }; }
macro_rules! Dbl_setzero_sign { ($v:expr) => {{ Dallp1!($v)&=0x7fffffff; }}; }
macro_rules! Dbl_setzerop1 { ($v:expr) => {{ Dallp1!($v)=0; }}; }
macro_rules! Dbl_setzerop2 { ($v:expr) => {{ Dallp2!($v)=0; }}; }
macro_rules! Dbl_setzero { ($a:expr,$b:expr) => {{ Dallp1!($a)=0; Dallp2!($b)=0; }}; }
macro_rules! Dbl_setnegativezero { ($v:expr) => {{ Dallp1!($v)=1u32<<31; Dallp2!($v)=0; }}; }
macro_rules! Dbl_setzero_mantissa { ($a:expr,$b:expr) => {{ Dallp1!($a)&=0xfff00000; Dallp2!($b)=0; }}; }
macro_rules! Dbl_setzero_exponentmantissa { ($a:expr,$b:expr) => {{ Dallp1!($a)&=0x80000000; Dallp2!($b)=0; }}; }
macro_rules! Dbl_increment { ($a:expr,$b:expr) => {{ Dallp2!($b)=Dallp2!($b).wrapping_add(1); if Dallp2!($b)==0 { Dallp1!($a)=Dallp1!($a).wrapping_add(1); } }}; }
macro_rules! Dbl_decrement { ($a:expr,$b:expr) => {{ if Dallp2!($b)==0 { Dallp1!($a)=Dallp1!($a).wrapping_sub(1); } Dallp2!($b)=Dallp2!($b).wrapping_sub(1); }}; }

const DBLEXT_THRESHOLD: i32 = 106;
macro_rules! Dblext_setzero { ($a:expr,$b:expr,$c:expr,$d:expr) => {{ Dextallp1!($a)=0; Dextallp2!($b)=0; Dextallp3!($c)=0; Dextallp4!($d)=0; }}; }
macro_rules! Dblext_iszero { ($a:expr,$b:expr,$c:expr,$d:expr) => { Dextallp1!($a)==0 && Dextallp2!($b)==0 && Dextallp3!($c)==0 && Dextallp4!($d)==0 }; }
macro_rules! Dblext_isnotzero_mantissap3 { ($v:expr) => { Dextallp3!($v)!=0 }; }
macro_rules! Dblext_isnotzero_mantissap4 { ($v:expr) => { Dextallp3!($v)!=0 }; }
macro_rules! Dblext_isone_lowp2 { ($v:expr) => { Dextlowp2!($v)!=0 }; }
macro_rules! Dblext_isone_highp3 { ($v:expr) => { Dexthighp3!($v)!=0 }; }
macro_rules! Dblext_isnotzero_low31p3 { ($v:expr) => { Dextlow31p3!($v)!=0 }; }

macro_rules! Dbl_copytoint_exponentmantissap1 { ($s:expr,$d:expr) => {{ $d=Dexponentmantissap1!($s); }}; }
macro_rules! Dbl_lowmantissap2 { ($v:expr) => { Dlowp2!($v) }; }
macro_rules! Dbl_hidden { ($v:expr) => { Dhidden!($v) }; }
macro_rules! Dbl_hiddenhigh3mantissa { ($v:expr) => { Dhiddenhigh3mantissa!($v) }; }
macro_rules! Dbl_ismagnitudeless { ($l:expr,$r:expr,$x:expr,$y:expr) => { $x <= $y && ($x < $y || Dallp2!($l)<Dallp2!($r)) }; }
macro_rules! Dbl_isequal { ($a:expr,$b:expr,$c:expr,$d:expr) => { Dallp1!($a)==Dallp1!($c) && Dallp2!($b)==Dallp2!($d) }; }
macro_rules! Dbl_islessthan { ($a:expr,$b:expr,$c:expr,$d:expr) => { Dallp1!($a)<Dallp1!($c) || (Dallp1!($a)==Dallp1!($c)&&Dallp2!($b)<Dallp2!($d)) }; }
macro_rules! Dbl_isgreaterthan { ($a:expr,$b:expr,$c:expr,$d:expr) => { Dallp1!($a)>Dallp1!($c) || (Dallp1!($a)==Dallp1!($c)&&Dallp2!($b)>Dallp2!($d)) }; }
macro_rules! Dbl_isnotlessthan { ($a:expr,$b:expr,$c:expr,$d:expr) => { Dallp1!($a)>Dallp1!($c) || (Dallp1!($a)==Dallp1!($c)&&Dallp2!($b)>=Dallp2!($d)) }; }
macro_rules! Dbl_isnotgreaterthan { ($a:expr,$b:expr,$c:expr,$d:expr) => { Dallp1!($a)<Dallp1!($c) || (Dallp1!($a)==Dallp1!($c)&&Dallp2!($b)<=Dallp2!($d)) }; }
macro_rules! Dbl_leftshiftby1 { ($a:expr,$b:expr) => {{ Shiftdouble!(Dallp1!($a),Dallp2!($b),31,Dallp1!($a)); Dallp2!($b)<<=1; }}; }
macro_rules! Dbl_leftshiftby2 { ($a:expr,$b:expr) => {{ Shiftdouble!(Dallp1!($a),Dallp2!($b),30,Dallp1!($a)); Dallp2!($b)<<=2; }}; }
macro_rules! Dbl_leftshiftby4 { ($a:expr,$b:expr) => {{ Shiftdouble!(Dallp1!($a),Dallp2!($b),28,Dallp1!($a)); Dallp2!($b)<<=4; }}; }
macro_rules! Dbl_leftshiftby8 { ($a:expr,$b:expr) => {{ Shiftdouble!(Dallp1!($a),Dallp2!($b),24,Dallp1!($a)); Dallp2!($b)<<=8; }}; }
macro_rules! Dbl_rightshiftby1 { ($a:expr,$b:expr) => {{ Shiftdouble!(Dallp1!($a),Dallp2!($b),1,Dallp2!($b)); Dallp1!($a)>>=1; }}; }
macro_rules! Dbl_rightshiftby2 { ($a:expr,$b:expr) => {{ Shiftdouble!(Dallp1!($a),Dallp2!($b),2,Dallp2!($b)); Dallp1!($a)>>=2; }}; }
macro_rules! Dbl_rightshiftby4 { ($a:expr,$b:expr) => {{ Shiftdouble!(Dallp1!($a),Dallp2!($b),4,Dallp2!($b)); Dallp1!($a)>>=4; }}; }
macro_rules! Dbl_rightshiftby8 { ($a:expr,$b:expr) => {{ Shiftdouble!(Dallp1!($a),Dallp2!($b),8,Dallp2!($b)); Dallp1!($a)>>=8; }}; }
macro_rules! Dbl_xortointp1 { ($l:expr,$r:expr,$o:expr) => {{ $o=Dallp1!($l)^Dallp1!($r); }}; }
macro_rules! Dbl_xorfromintp1 { ($l:expr,$r:expr,$o:expr) => {{ Dallp1!($o)=$l^Dallp1!($r); }}; }
macro_rules! Dbl_swap_lower { ($l:expr,$r:expr) => {{ Dallp2!($l)^=Dallp2!($r); Dallp2!($r)^=Dallp2!($l); Dallp2!($l)^=Dallp2!($r); }}; }
macro_rules! Dblext_xortointp1 { ($l:expr,$r:expr,$o:expr) => { Dbl_xortointp1!($l,$r,$o) }; }
macro_rules! Dblext_xorfromintp1 { ($l:expr,$r:expr,$o:expr) => { Dbl_xorfromintp1!($l,$r,$o) }; }
macro_rules! Dblext_copytoint_exponentmantissap1 { ($s:expr,$d:expr) => { Dbl_copytoint_exponentmantissap1!($s,$d) }; }
macro_rules! Dblext_ismagnitudeless { ($l:expr,$r:expr,$x:expr,$y:expr) => { Dbl_ismagnitudeless!($l,$r,$x,$y) }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
