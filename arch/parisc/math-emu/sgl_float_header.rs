/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Linux/PA-RISC floating-point emulation code. */
// C header translated as macro_rules! items. External PA helpers/constants are
// intentionally referenced but not defined here.

macro_rules! Sgl_firstword { ($value:expr) => { Sall!($value) }; }
macro_rules! Sgl_secondword { ($value:expr) => { dummy_location } }
macro_rules! Sgl_thirdword { ($value:expr) => { dummy_location } }
macro_rules! Sgl_fourthword { ($value:expr) => { dummy_location } }
macro_rules! Sgl_sign { ($object:expr) => { Ssign!($object) }; }
macro_rules! Sgl_exponent { ($object:expr) => { Sexponent!($object) }; }
macro_rules! Sgl_signexponent { ($object:expr) => { Ssignexponent!($object) }; }
macro_rules! Sgl_mantissa { ($object:expr) => { Smantissa!($object) }; }
macro_rules! Sgl_exponentmantissa { ($object:expr) => { Sexponentmantissa!($object) }; }
macro_rules! Sgl_all { ($object:expr) => { Sall!($object) }; }

macro_rules! Sgl_and_signs { ($a:expr,$b:expr) => { Sall!($a) = (Sall!($b)|!((1u32)<<31)) & Sall!($a) }; }
macro_rules! Sgl_or_signs { ($a:expr,$b:expr) => { Sall!($a) = (Sall!($b)&((1u32)<<31)) | Sall!($a) }; }
macro_rules! Sgl_clear_exponent_set_hidden { ($v:expr) => { Deposit_sexponent!($v,1) }; }
macro_rules! Sgl_clear_signexponent_set_hidden { ($v:expr) => { Deposit_ssignexponent!($v,1) }; }
macro_rules! Sgl_clear_sign { ($v:expr) => { Sall!($v) &= !((1u32)<<31) }; }
macro_rules! Sgl_clear_signexponent { ($v:expr) => { Sall!($v) &= 0x007fffff }; }
macro_rules! Sgl_rightshift { ($v:expr,$n:expr) => { Sall!($v) >>= $n }; }
macro_rules! Sgl_leftshift { ($v:expr,$n:expr) => { Sall!($v) <<= $n }; }
macro_rules! Sgl_rightshift_exponentmantissa { ($v:expr,$n:expr) => { Sall!($v) = (Sexponentmantissa!($v) >> $n) | (Sall!($v)&((1u32)<<31)) }; }
macro_rules! Sgl_leftshiftby1_withextent { ($l:expr,$r:expr,$o:expr) => { Shiftdouble!(Sall!($l),Extall!($r),31,Sall!($o)) }; }
macro_rules! Sgl_rightshiftby1_withextent { ($l:expr,$r:expr,$d:expr) => { Shiftdouble!(Sall!($l),Extall!($r),1,Extall!($r)) }; }
macro_rules! Sgl_arithrightshiftby1 { ($v:expr) => { Sall!($v) = (Sall!($v) as i32 >> 1) as _ }; }
macro_rules! Sgl_signextendedsign { ($v:expr) => { Ssignedsign!($v) }; }
macro_rules! Sgl_isone_hidden { ($v:expr) => { Shidden!($v) }; }
macro_rules! Sgl_increment { ($v:expr) => { Sall!($v) += 1 }; }
macro_rules! Sgl_increment_mantissa { ($v:expr) => { Deposit_smantissa!($v,$v+1) }; }
macro_rules! Sgl_decrement { ($v:expr) => { Sall!($v) -= 1 }; }

macro_rules! Sgl_isone_sign { ($v:expr) => { Is_ssign!($v)!=0 }; }
macro_rules! Sgl_isone_hiddenoverflow { ($v:expr) => { Is_shiddenoverflow!($v)!=0 }; }
macro_rules! Sgl_isone_lowmantissa { ($v:expr) => { Is_slow!($v)!=0 }; }
macro_rules! Sgl_isone_signaling { ($v:expr) => { Is_ssignaling!($v)!=0 }; }
macro_rules! Sgl_is_signalingnan { ($v:expr) => { Ssignalingnan!($v)==0x1ff }; }
macro_rules! Sgl_isnotzero { ($v:expr) => { Sall!($v)!=0 }; }
macro_rules! Sgl_isnotzero_hiddenhigh7mantissa { ($v:expr) => { Shiddenhigh7mantissa!($v)!=0 }; }
macro_rules! Sgl_isnotzero_low4 { ($v:expr) => { Slow4!($v)!=0 }; }
macro_rules! Sgl_isnotzero_exponent { ($v:expr) => { Sexponent!($v)!=0 }; }
macro_rules! Sgl_isnotzero_mantissa { ($v:expr) => { Smantissa!($v)!=0 }; }
macro_rules! Sgl_isnotzero_exponentmantissa { ($v:expr) => { Sexponentmantissa!($v)!=0 }; }
macro_rules! Sgl_iszero { ($v:expr) => { Sall!($v)==0 }; }
macro_rules! Sgl_iszero_signaling { ($v:expr) => { Is_ssignaling!($v)==0 }; }
macro_rules! Sgl_iszero_hidden { ($v:expr) => { Is_shidden!($v)==0 }; }
macro_rules! Sgl_iszero_hiddenoverflow { ($v:expr) => { Is_shiddenoverflow!($v)==0 }; }
macro_rules! Sgl_iszero_hiddenhigh3mantissa { ($v:expr) => { Shiddenhigh3mantissa!($v)==0 }; }
macro_rules! Sgl_iszero_hiddenhigh7mantissa { ($v:expr) => { Shiddenhigh7mantissa!($v)==0 }; }
macro_rules! Sgl_iszero_sign { ($v:expr) => { Is_ssign!($v)==0 }; }
macro_rules! Sgl_iszero_exponent { ($v:expr) => { Sexponent!($v)==0 }; }
macro_rules! Sgl_iszero_mantissa { ($v:expr) => { Smantissa!($v)==0 }; }
macro_rules! Sgl_iszero_exponentmantissa { ($v:expr) => { Sexponentmantissa!($v)==0 }; }
macro_rules! Sgl_isinfinity_exponent { ($v:expr) => { Sgl_exponent!($v)==SGL_INFINITY_EXPONENT }; }
macro_rules! Sgl_isnotinfinity_exponent { ($v:expr) => { Sgl_exponent!($v)!=SGL_INFINITY_EXPONENT }; }
macro_rules! Sgl_isinfinity { ($v:expr) => { Sgl_exponent!($v)==SGL_INFINITY_EXPONENT && Sgl_mantissa!($v)==0 }; }
macro_rules! Sgl_isnan { ($v:expr) => { Sgl_exponent!($v)==SGL_INFINITY_EXPONENT && Sgl_mantissa!($v)!=0 }; }
macro_rules! Sgl_isnotnan { ($v:expr) => { Sgl_exponent!($v)!=SGL_INFINITY_EXPONENT || Sgl_mantissa!($v)==0 }; }
macro_rules! Sgl_islessthan { ($a:expr,$b:expr) => { Sall!($a)<Sall!($b) }; }
macro_rules! Sgl_isgreaterthan { ($a:expr,$b:expr) => { Sall!($a)>Sall!($b) }; }
macro_rules! Sgl_isnotlessthan { ($a:expr,$b:expr) => { Sall!($a)>=Sall!($b) }; }
macro_rules! Sgl_isequal { ($a:expr,$b:expr) => { Sall!($a)==Sall!($b) }; }

macro_rules! Sgl_leftshiftby8 { ($v:expr) => { Sall!($v)<<=8 }; }
macro_rules! Sgl_leftshiftby4 { ($v:expr) => { Sall!($v)<<=4 }; }
macro_rules! Sgl_leftshiftby3 { ($v:expr) => { Sall!($v)<<=3 }; }
macro_rules! Sgl_leftshiftby2 { ($v:expr) => { Sall!($v)<<=2 }; }
macro_rules! Sgl_leftshiftby1 { ($v:expr) => { Sall!($v)<<=1 }; }
macro_rules! Sgl_rightshiftby1 { ($v:expr) => { Sall!($v)>>=1 }; }
macro_rules! Sgl_rightshiftby4 { ($v:expr) => { Sall!($v)>>=4 }; }
macro_rules! Sgl_rightshiftby8 { ($v:expr) => { Sall!($v)>>=8 }; }
macro_rules! Sgl_ismagnitudeless { ($a:expr,$b:expr) => { $a<$b }; }
macro_rules! Sgl_copytoint_exponentmantissa { ($s:expr,$d:expr) => { $d=Sexponentmantissa!($s) }; }
macro_rules! Sgl_set_quiet { ($v:expr) => { Deposit_shigh2mantissa!($v,1) }; }
macro_rules! Sgl_set_exponent { ($v:expr,$e:expr) => { Deposit_sexponent!($v,$e) }; }
macro_rules! Sgl_set_mantissa { ($d:expr,$v:expr) => { Deposit_smantissa!($d,$v) }; }
macro_rules! Sgl_set_exponentmantissa { ($d:expr,$v:expr) => { Deposit_sexponentmantissa!($d,$v) }; }
macro_rules! Sgl_setinfinity_exponent { ($v:expr) => { Deposit_sexponent!($v,SGL_INFINITY_EXPONENT) }; }
macro_rules! Sgl_setinfinity_exponentmantissa { ($v:expr) => { Deposit_sexponentmantissa!($v,SGL_INFINITY_EXPONENT << (32-(1+SGL_EXP_LENGTH))) }; }
macro_rules! Sgl_setinfinitypositive { ($v:expr) => { Sall!($v)=(SGL_INFINITY_EXPONENT << (32-(1+SGL_EXP_LENGTH))) }; }
macro_rules! Sgl_setinfinitynegative { ($v:expr) => { Sall!($v)=(SGL_INFINITY_EXPONENT << (32-(1+SGL_EXP_LENGTH)))|((1u32)<<31) }; }
macro_rules! Sgl_setinfinity { ($v:expr,$s:expr) => { Sall!($v)=(SGL_INFINITY_EXPONENT << (32-(1+SGL_EXP_LENGTH)))|(($s as u32)<<31) }; }
macro_rules! Sgl_sethigh4bits { ($v:expr,$s:expr) => { Deposit_shigh4!($v,$s) }; }
macro_rules! Sgl_set_sign { ($v:expr,$s:expr) => { Deposit_ssign!($v,$s) }; }
macro_rules! Sgl_invert_sign { ($v:expr) => { Deposit_ssign!($v,!Ssign!($v)) }; }
macro_rules! Sgl_setone_sign { ($v:expr) => { Deposit_ssign!($v,1) }; }
macro_rules! Sgl_setone_lowmantissa { ($v:expr) => { Deposit_slow!($v,1) }; }
macro_rules! Sgl_setzero_sign { ($v:expr) => { Sall!($v)&=0x7fffffff }; }
macro_rules! Sgl_setzero_exponent { ($v:expr) => { Sall!($v)&=0x807fffff }; }
macro_rules! Sgl_setzero_mantissa { ($v:expr) => { Sall!($v)&=0xff800000 }; }
macro_rules! Sgl_setzero_exponentmantissa { ($v:expr) => { Sall!($v)&=0x80000000 }; }
macro_rules! Sgl_setzero { ($v:expr) => { Sall!($v)=0 }; }
macro_rules! Sgl_setnegativezero { ($v:expr) => { Sall!($v)=(1u32)<<31 }; }
macro_rules! Sgl_setwrapped_exponent { ($v:expr,$e:expr,$op:tt) => { Deposit_sexponent!($v,$e $op SGL_WRAP) }; }
macro_rules! Sgl_setlargestpositive { ($v:expr) => { Sall!($v)=((SGL_EMAX+SGL_BIAS)<<(32-(1+SGL_EXP_LENGTH)))|((1u32<<(32-(1+SGL_EXP_LENGTH)))-1) }; }
macro_rules! Sgl_setlargestnegative { ($v:expr) => { Sall!($v)=((SGL_EMAX+SGL_BIAS)<<(32-(1+SGL_EXP_LENGTH)))|((1u32<<(32-(1+SGL_EXP_LENGTH)))-1)|((1u32)<<31) }; }
macro_rules! Sgl_setnegativeinfinity { ($v:expr) => { Sall!($v)=((1u32<<SGL_EXP_LENGTH)|SGL_INFINITY_EXPONENT)<<(32-(1+SGL_EXP_LENGTH)) }; }
macro_rules! Sgl_setlargest { ($v:expr,$s:expr) => { Sall!($v)=(($s as u32)<<31)|(((SGL_EMAX+SGL_BIAS)<<(32-(1+SGL_EXP_LENGTH)))|((1u32<<(32-(1+SGL_EXP_LENGTH)))-1)) }; }
macro_rules! Sgl_setlargest_exponentmantissa { ($v:expr) => { Sall!($v)=(Sall!($v)&((1u32)<<31))|(((SGL_EMAX+SGL_BIAS)<<(32-(1+SGL_EXP_LENGTH)))|((1u32<<(32-(1+SGL_EXP_LENGTH)))-1)) }; }

macro_rules! Sgl_right_align { ($v:expr,$shift:expr,$extent:expr) => {{ if $shift<32 { Extall!($extent)=Sall!($v)<<(32-$shift); Sall!($v)>>=$shift; } else { Extall!($extent)=Sall!($v); Sall!($v)=0; } }}; }
macro_rules! Sgl_hiddenhigh3mantissa { ($v:expr) => { Shiddenhigh3mantissa!($v) }; }
macro_rules! Sgl_hidden { ($v:expr) => { Shidden!($v) }; }
macro_rules! Sgl_lowmantissa { ($v:expr) => { Slow!($v) }; }
macro_rules! Sgl_subtract { ($l:expr,$r:expr,$o:expr) => { Sall!($o)=Sall!($l)-Sall!($r) }; }
macro_rules! Sgl_subtract_withextension { ($l:expr,$r:expr,$e:expr,$o:expr) => {{ Sgl_subtract!($l,$r,$o); Extall!($e)=0-Extall!($e); if Extall!($e)!=0 { Sall!($o)-=1; } }}; }
macro_rules! Sgl_addition { ($l:expr,$r:expr,$o:expr) => { Sall!($o)=Sall!($l)+Sall!($r) }; }
macro_rules! Sgl_xortointp1 { ($l:expr,$r:expr,$o:expr) => { $o=Sall!($l)^Sall!($r) }; }
macro_rules! Sgl_xorfromintp1 { ($l:expr,$r:expr,$o:expr) => { Sall!($o)=$l^Sall!($r) }; }

macro_rules! Sgl_makequietnan { ($d:expr) => { Sall!($d)=(((SGL_EMAX+SGL_BIAS)+1)<<(32-(1+SGL_EXP_LENGTH)))|(1u32<<(32-(SGL_EXP_LENGTH+3))) }; }
macro_rules! Sgl_makesignalingnan { ($d:expr) => { Sall!($d)=(((SGL_EMAX+SGL_BIAS)+1)<<(32-(1+SGL_EXP_LENGTH)))|(1u32<<(32-(SGL_EXP_LENGTH+2))) }; }
macro_rules! Sgl_normalize { ($o:expr,$e:expr) => {{ while Sgl_iszero_hiddenhigh7mantissa!($o) { Sgl_leftshiftby8!($o); $e-=8; } if Sgl_iszero_hiddenhigh3mantissa!($o) { Sgl_leftshiftby4!($o); $e-=4; } while Sgl_iszero_hidden!($o) { Sgl_leftshiftby1!($o); $e-=1; } }}; }
macro_rules! Sgl_copytoptr { ($s:expr,$d:expr) => { *$d=$s }; }
macro_rules! Sgl_copyfromptr { ($s:expr,$d:expr) => { $d=*$s }; }

const SGLEXT_THRESHOLD: i32 = 48;

macro_rules! Sglext_setzero { ($a:expr,$b:expr) => { Sextallp1!($a)=0; Sextallp2!($b)=0 }; }
macro_rules! Sglext_isnotzero_mantissap2 { ($v:expr) => { Sextallp2!($v)!=0 }; }
macro_rules! Sglext_isone_lowp1 { ($v:expr) => { Sextlowp1!($v)!=0 }; }
macro_rules! Sglext_isone_highp2 { ($v:expr) => { Sexthighp2!($v)!=0 }; }
macro_rules! Sglext_isnotzero_low31p2 { ($v:expr) => { Sextlow31p2!($v)!=0 }; }
macro_rules! Sglext_iszero { ($a:expr,$b:expr) => { Sextallp1!($a)==0 && Sextallp2!($b)==0 }; }
macro_rules! Sglext_copy { ($a:expr,$b:expr,$c:expr,$d:expr) => { Sextallp1!($c)=Sextallp1!($a); Sextallp2!($d)=Sextallp2!($b) }; }
macro_rules! Sgl_copyto_sglext { ($s:expr,$d1:expr,$d2:expr) => { Sextallp1!($d1)=Sall!($s); Sextallp2!($d2)=0 }; }
macro_rules! Sglext_swap_lower { ($l:expr,$r:expr) => {{ Sextallp2!($l)^=Sextallp2!($r); Sextallp2!($r)^=Sextallp2!($l); Sextallp2!($l)^=Sextallp2!($r); }}; }
macro_rules! Sglext_setone_lowmantissap2 { ($v:expr) => { Deposit_dlowp2!($v,1) }; }
macro_rules! Sglext_xortointp1 { ($l:expr,$r:expr,$o:expr) => { Sgl_xortointp1!($l,$r,$o) }; }
macro_rules! Sglext_xorfromintp1 { ($l:expr,$r:expr,$o:expr) => { Sgl_xorfromintp1!($l,$r,$o) }; }
macro_rules! Sglext_copytoint_exponentmantissa { ($s:expr,$d:expr) => { Sgl_copytoint_exponentmantissa!($s,$d) }; }
macro_rules! Sglext_ismagnitudeless { ($l:expr,$r:expr) => { Sgl_ismagnitudeless!($l,$r) }; }
macro_rules! Sglext_set_sign { ($v:expr,$s:expr) => { Sgl_set_sign!($v,$s) }; }
macro_rules! Sglext_clear_signexponent_set_hidden { ($v:expr) => { Sgl_clear_signexponent_set_hidden!($v) }; }
macro_rules! Sglext_clear_signexponent { ($v:expr) => { Sgl_clear_signexponent!($v) }; }
macro_rules! Sglext_clear_sign { ($v:expr) => { Sgl_clear_sign!($v) }; }
macro_rules! Sglext_isone_hidden { ($v:expr) => { Sgl_isone_hidden!($v) }; }

// The remaining extended-format operations are retained as direct macro
// translations; their helper operations and rounding symbols are external.
macro_rules! Sglext_right_align { ($a:expr,$b:expr,$s:expr) => {{ let shiftamt=$s%32; let mut sticky=0; match $s/32 { 0 => { if shiftamt>0 { sticky=Sextallp2!($b)<<(32-shiftamt); Variable_shift_double!(Sextallp1!($a),Sextallp2!($b),shiftamt,Sextallp2!($b)); Sextallp1!($a)>>=shiftamt; } }, 1 => { if shiftamt>0 { sticky=(Sextallp1!($a)<<(32-shiftamt))|Sextallp2!($b); } else { sticky=Sextallp2!($b); } Sextallp2!($b)=Sextallp1!($a)>>shiftamt; Sextallp1!($a)=0; }, _ => {} } if sticky!=0 { Sglext_setone_lowmantissap2!($b); } }}; }
macro_rules! Sglext_subtract { ($la:expr,$lb:expr,$ra:expr,$rb:expr,$oa:expr,$ob:expr) => {{ if Sextallp2!($rb)>Sextallp2!($lb) { Sextallp1!($la)-=1; } Sextallp2!($ob)=Sextallp2!($lb)-Sextallp2!($rb); Sextallp1!($oa)=Sextallp1!($la)-Sextallp1!($ra); }}; }
macro_rules! Sglext_addition { ($la:expr,$lb:expr,$ra:expr,$rb:expr,$oa:expr,$ob:expr) => {{ let low=Sextallp2!($la)+Sextallp2!($rb); Sextallp2!($ob)=low; if low<Sextallp2!($rb) { Sextallp1!($oa)=Sextallp1!($la)+Sextallp1!($ra)+1; } else { Sextallp1!($oa)=Sextallp1!($la)+Sextallp1!($ra); } }}; }
macro_rules! Sglext_arithrightshiftby1 { ($a:expr,$b:expr) => {{ Shiftdouble!(Sextallp1!($a),Sextallp2!($b),1,Sextallp2!($b)); Sextallp1!($a)=(Sextallp1!($a) as i32>>1) as _; }}; }
macro_rules! Sglext_leftshiftby8 { ($a:expr,$b:expr) => {{ Shiftdouble!(Sextallp1!($a),Sextallp2!($b),24,Sextallp1!($a)); Sextallp2!($b)<<=8; }}; }
macro_rules! Sglext_leftshiftby4 { ($a:expr,$b:expr) => {{ Shiftdouble!(Sextallp1!($a),Sextallp2!($b),28,Sextallp1!($a)); Sextallp2!($b)<<=4; }}; }
macro_rules! Sglext_leftshiftby3 { ($a:expr,$b:expr) => {{ Shiftdouble!(Sextallp1!($a),Sextallp2!($b),29,Sextallp1!($a)); Sextallp2!($b)<<=3; }}; }
macro_rules! Sglext_leftshiftby2 { ($a:expr,$b:expr) => {{ Shiftdouble!(Sextallp1!($a),Sextallp2!($b),30,Sextallp1!($a)); Sextallp2!($b)<<=2; }}; }
macro_rules! Sglext_leftshiftby1 { ($a:expr,$b:expr) => {{ Shiftdouble!(Sextallp1!($a),Sextallp2!($b),31,Sextallp1!($a)); Sextallp2!($b)<<=1; }}; }
macro_rules! Sglext_rightshiftby4 { ($a:expr,$b:expr) => {{ Shiftdouble!(Sextallp1!($a),Sextallp2!($b),4,Sextallp2!($b)); Sextallp1!($a)>>=4; }}; }
macro_rules! Sglext_rightshiftby3 { ($a:expr,$b:expr) => {{ Shiftdouble!(Sextallp1!($a),Sextallp2!($b),3,Sextallp2!($b)); Sextallp1!($a)>>=3; }}; }
macro_rules! Sglext_rightshiftby1 { ($a:expr,$b:expr) => {{ Shiftdouble!(Sextallp1!($a),Sextallp2!($b),1,Sextallp2!($b)); Sextallp1!($a)>>=1; }}; }

// C's overflow/denormalization macros depend on the surrounding emulator's
// rounding constants and helper macros; preserve their intended call surface.
macro_rules! Sgl_setoverflow { ($v:expr) => {{ match Rounding_mode!() { ROUNDPLUS => { if Sgl_isone_sign!($v) { Sgl_setlargestnegative!($v); } else { Sgl_setinfinitypositive!($v); } }, ROUNDMINUS => { if Sgl_iszero_sign!($v) { Sgl_setlargestpositive!($v); } else { Sgl_setinfinitynegative!($v); } }, ROUNDNEAREST => Sgl_setinfinity_exponentmantissa!($v), ROUNDZERO => Sgl_setlargest_exponentmantissa!($v), _ => {} } }}; }
macro_rules! Sgl_denormalize { ($o:expr,$e:expr,$g:expr,$st:expr,$ix:expr) => {{ Sgl_clear_signexponent_set_hidden!($o); if $e >= (1-SGL_P) { $g=(Sall!($o)>>(-$e))&1; if $e<0 { $st|=Sall!($o)<<(32+$e); } $ix=$g|$st; Sall!($o)>>=(1-$e); } else { $g=0; $st|=Sall!($o); $ix=$st; Sgl_setzero!($o); } }}; }

macro_rules! Sglext_denormalize { ($p1:expr,$p2:expr,$e:expr,$tiny:expr) => {{
    let mut sticky;
    $tiny=true;
    if $e==0 && Sextallp2!($p2)!=0 { match Rounding_mode!() {
        ROUNDPLUS => { if Sgl_iszero_sign!($p1) && Sgl_isone_hiddenoverflow!($p1+1) { $tiny=false; } },
        ROUNDMINUS => { if Sgl_isone_sign!($p1) && Sgl_isone_hiddenoverflow!($p1+1) { $tiny=false; } },
        ROUNDNEAREST => { if Sglext_isone_highp2!($p2) && (Sglext_isone_lowp1!($p1) || Sglext_isnotzero_low31p2!($p2)) && Sgl_isone_hiddenoverflow!($p1+1) { $tiny=false; } },
        _ => {}
    }}
    Sglext_clear_signexponent_set_hidden!($p1);
    if $e >= (1-DBL_P) {
        if $e >= -31 { if $e > -31 { sticky=Sextallp2!($p2)<<(31+$e); Variable_shift_double!($p1,$p2,1-$e,$p2); Sextallp1!($p1)>>=1-$e; } else { sticky=Sextallp2!($p2); Sextallp2!($p2)=Sextallp1!($p1); Sextallp1!($p1)=0; } }
        else { sticky=(Sextallp1!($p1)<<(31+$e))|Sextallp2!($p2); Sextallp2!($p2)=Sextallp1!($p1)>>(-31-$e); Sextallp1!($p1)=0; }
    } else { sticky=Sextallp1!($p1)|Sextallp2!($p2); Sglext_setzero!($p1,$p2); }
    if sticky!=0 { Sglext_setone_lowmantissap2!($p2); }
    $e=0;
}}; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
