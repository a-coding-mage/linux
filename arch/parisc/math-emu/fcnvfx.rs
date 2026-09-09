// SPDX-License-Identifier: GPL-2.0-or-later
/* Linux/PA-RISC floating-point emulation code. */
// Dependencies supplied by float.h, sgl_float.h, dbl_float.h, and cnv_float.h

pub unsafe fn sgl_to_sgl_fcnvfx(srcptr: *mut sgl_floating_point, _nullptr: *mut sgl_floating_point, dstptr: *mut i32, _status: *mut sgl_floating_point) -> i32 {
    let src = *srcptr;
    let src_exponent = Sgl_exponent(src) - SGL_BIAS;
    let mut inexact = false;
    let mut result: i32;
    if src_exponent > SGL_FX_MAX_EXP {
        if src_exponent > SGL_FX_MAX_EXP + 1 || Sgl_isnotzero_mantissa(src) || Sgl_iszero_sign(src) {
            result = if Sgl_iszero_sign(src) { 0x7fffffff } else { 0x80000000u32 as i32 };
            if Is_invalidtrap_enabled() { return INVALIDEXCEPTION; }
            Set_invalidflag(); *dstptr = result; return NOEXCEPTION;
        }
    }
    if src_exponent >= 0 {
        let mut temp = src;
        Sgl_clear_signexponent_set_hidden(&mut temp);
        Int_from_sgl_mantissa(&mut temp, src_exponent);
        result = if Sgl_isone_sign(src) { -(Sgl_all(temp) as i32) } else { Sgl_all(temp) as i32 };
        if Sgl_isinexact_to_fix(src, src_exponent) {
            inexact = true;
            match Rounding_mode() { ROUNDPLUS => if Sgl_iszero_sign(src) { result += 1 }, ROUNDMINUS => if Sgl_isone_sign(src) { result -= 1 }, ROUNDNEAREST => if Sgl_isone_roundbit(src, src_exponent) && (Sgl_isone_stickybit(src, src_exponent) || Sgl_isone_lowmantissa(temp)) { if Sgl_iszero_sign(src) { result += 1 } else { result -= 1 } }, _ => {} }
        }
    } else {
        result = 0;
        if Sgl_isnotzero_exponentmantissa(src) {
            inexact = true;
            match Rounding_mode() { ROUNDPLUS => if Sgl_iszero_sign(src) { result += 1 }, ROUNDMINUS => if Sgl_isone_sign(src) { result -= 1 }, ROUNDNEAREST => if src_exponent == -1 && Sgl_isnotzero_mantissa(src) { if Sgl_iszero_sign(src) { result += 1 } else { result -= 1 } }, _ => {} }
        }
    }
    *dstptr = result;
    if inexact { if Is_inexacttrap_enabled() { return INEXACTEXCEPTION; } else { Set_inexactflag(); } }
    NOEXCEPTION
}

pub unsafe fn sgl_to_dbl_fcnvfx(srcptr: *mut sgl_floating_point, _nullptr: *mut u32, dstptr: *mut dbl_integer, _status: *mut u32) -> i32 {
    let src = *srcptr; let src_exponent = Sgl_exponent(src) - SGL_BIAS; let mut inexact = false; let (mut resultp1, mut resultp2): (i32, u32);
    if src_exponent > DBL_FX_MAX_EXP { if src_exponent > DBL_FX_MAX_EXP + 1 || Sgl_isnotzero_mantissa(src) || Sgl_iszero_sign(src) { if Sgl_iszero_sign(src) { resultp1=0x7fffffff; resultp2=0xffffffff } else { resultp1=0x80000000u32 as i32; resultp2=0 }; if Is_invalidtrap_enabled(){return INVALIDEXCEPTION;} Set_invalidflag(); Dint_copytoptr(resultp1,resultp2,dstptr); return NOEXCEPTION; } Dint_set_minint(&mut resultp1,&mut resultp2); Dint_copytoptr(resultp1,resultp2,dstptr); return NOEXCEPTION; }
    if src_exponent >= 0 { let mut temp=src; Sgl_clear_signexponent_set_hidden(&mut temp); Dint_from_sgl_mantissa(temp,src_exponent,&mut resultp1,&mut resultp2); if Sgl_isone_sign(src){Dint_setone_sign(&mut resultp1,&mut resultp2);} if Sgl_isinexact_to_fix(src,src_exponent){inexact=true; match Rounding_mode(){ROUNDPLUS=>if Sgl_iszero_sign(src){Dint_increment(&mut resultp1,&mut resultp2)},ROUNDMINUS=>if Sgl_isone_sign(src){Dint_decrement(&mut resultp1,&mut resultp2)},ROUNDNEAREST=>if Sgl_isone_roundbit(src,src_exponent)&&(Sgl_isone_stickybit(src,src_exponent)||Dint_isone_lowp2(resultp2)){if Sgl_iszero_sign(src){Dint_increment(&mut resultp1,&mut resultp2)}else{Dint_decrement(&mut resultp1,&mut resultp2)}},_=>{}}}} else {Dint_setzero(&mut resultp1,&mut resultp2); if Sgl_isnotzero_exponentmantissa(src){inexact=true; match Rounding_mode(){ROUNDPLUS=>if Sgl_iszero_sign(src){Dint_increment(&mut resultp1,&mut resultp2)},ROUNDMINUS=>if Sgl_isone_sign(src){Dint_decrement(&mut resultp1,&mut resultp2)},ROUNDNEAREST=>if src_exponent==-1&&Sgl_isnotzero_mantissa(src){if Sgl_iszero_sign(src){Dint_increment(&mut resultp1,&mut resultp2)}else{Dint_decrement(&mut resultp1,&mut resultp2)}},_=>{}}}}
    Dint_copytoptr(resultp1,resultp2,dstptr); if inexact {if Is_inexacttrap_enabled(){return INEXACTEXCEPTION}else{Set_inexactflag()}} NOEXCEPTION
}

pub unsafe fn dbl_to_sgl_fcnvfx(srcptr: *mut dbl_floating_point, _nullptr: *mut u32, dstptr: *mut i32, _status: *mut u32) -> i32 {
    let (mut srcp1,mut srcp2)=(0u32,0u32); Dbl_copyfromptr(srcptr,&mut srcp1,&mut srcp2); let src_exponent=Dbl_exponent(srcp1)-DBL_BIAS; let mut inexact=false; let mut result:i32;
    if src_exponent>SGL_FX_MAX_EXP && Dbl_isoverflow_to_int(src_exponent,srcp1,srcp2){result=if Dbl_iszero_sign(srcp1){0x7fffffff}else{0x80000000u32 as i32};if Is_invalidtrap_enabled(){return INVALIDEXCEPTION}Set_invalidflag();*dstptr=result;return NOEXCEPTION}
    if src_exponent>=0 {let(mut tempp1,mut tempp2)=(srcp1,srcp2);Dbl_clear_signexponent_set_hidden(&mut tempp1);Int_from_dbl_mantissa(tempp1,tempp2,src_exponent);result=if Dbl_isone_sign(srcp1)&&src_exponent<=SGL_FX_MAX_EXP{-(Dbl_allp1(tempp1) as i32)}else{Dbl_allp1(tempp1) as i32};if Dbl_isinexact_to_fix(srcp1,srcp2,src_exponent){inexact=true;match Rounding_mode(){ROUNDPLUS=>if Dbl_iszero_sign(srcp1){result+=1},ROUNDMINUS=>if Dbl_isone_sign(srcp1){result-=1},ROUNDNEAREST=>if Dbl_isone_roundbit(srcp1,srcp2,src_exponent)&&(Dbl_isone_stickybit(srcp1,srcp2,src_exponent)||Dbl_isone_lowmantissap1(tempp1)){if Dbl_iszero_sign(srcp1){result+=1}else{result-=1}},_=>{}}}}else{result=0;if Dbl_isnotzero_exponentmantissa(srcp1,srcp2){inexact=true;match Rounding_mode(){ROUNDPLUS=>if Dbl_iszero_sign(srcp1){result+=1},ROUNDMINUS=>if Dbl_isone_sign(srcp1){result-=1},ROUNDNEAREST=>if src_exponent==-1&&Dbl_isnotzero_mantissa(srcp1,srcp2){if Dbl_iszero_sign(srcp1){result+=1}else{result-=1}},_=>{}}}} *dstptr=result;if inexact{if Is_inexacttrap_enabled(){return INEXACTEXCEPTION}else{Set_inexactflag()}}NOEXCEPTION
}

pub unsafe fn dbl_to_dbl_fcnvfx(srcptr: *mut dbl_floating_point, _nullptr: *mut u32, dstptr: *mut dbl_integer, _status: *mut u32) -> i32 {
    let(mut srcp1,mut srcp2)=(0u32,0u32);Dbl_copyfromptr(srcptr,&mut srcp1,&mut srcp2);let src_exponent=Dbl_exponent(srcp1)-DBL_BIAS;let mut inexact=false;let(mut resultp1,mut resultp2):(i32,u32);
    if src_exponent>DBL_FX_MAX_EXP {if src_exponent>DBL_FX_MAX_EXP+1||Dbl_isnotzero_mantissa(srcp1,srcp2)||Dbl_iszero_sign(srcp1){if Dbl_iszero_sign(srcp1){resultp1=0x7fffffff;resultp2=0xffffffff}else{resultp1=0x80000000u32 as i32;resultp2=0};if Is_invalidtrap_enabled(){return INVALIDEXCEPTION}Set_invalidflag();Dint_copytoptr(resultp1,resultp2,dstptr);return NOEXCEPTION}}
    if src_exponent>=0 {let(mut tempp1,mut tempp2)=(srcp1,srcp2);Dbl_clear_signexponent_set_hidden(&mut tempp1);Dint_from_dbl_mantissa(tempp1,tempp2,src_exponent,&mut resultp1,&mut resultp2);if Dbl_isone_sign(srcp1){Dint_setone_sign(&mut resultp1,&mut resultp2)}if Dbl_isinexact_to_fix(srcp1,srcp2,src_exponent){inexact=true;match Rounding_mode(){ROUNDPLUS=>if Dbl_iszero_sign(srcp1){Dint_increment(&mut resultp1,&mut resultp2)},ROUNDMINUS=>if Dbl_isone_sign(srcp1){Dint_decrement(&mut resultp1,&mut resultp2)},ROUNDNEAREST=>if Dbl_isone_roundbit(srcp1,srcp2,src_exponent)&&(Dbl_isone_stickybit(srcp1,srcp2,src_exponent)||Dint_isone_lowp2(resultp2)){if Dbl_iszero_sign(srcp1){Dint_increment(&mut resultp1,&mut resultp2)}else{Dint_decrement(&mut resultp1,&mut resultp2)}},_=>{}}}}else{Dint_setzero(&mut resultp1,&mut resultp2);if Dbl_isnotzero_exponentmantissa(srcp1,srcp2){inexact=true;match Rounding_mode(){ROUNDPLUS=>if Dbl_iszero_sign(srcp1){Dint_increment(&mut resultp1,&mut resultp2)},ROUNDMINUS=>if Dbl_isone_sign(srcp1){Dint_decrement(&mut resultp1,&mut resultp2)},ROUNDNEAREST=>if src_exponent==-1&&Dbl_isnotzero_mantissa(srcp1,srcp2){if Dbl_iszero_sign(srcp1){Dint_increment(&mut resultp1,&mut resultp2)}else{Dint_decrement(&mut resultp1,&mut resultp2)}},_=>{}}}}Dint_copytoptr(resultp1,resultp2,dstptr);if inexact{if Is_inexacttrap_enabled(){return INEXACTEXCEPTION}else{Set_inexactflag()}}NOEXCEPTION
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
