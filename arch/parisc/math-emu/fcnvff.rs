// SPDX-License-Identifier: GPL-2.0-or-later
/* Linux/PA-RISC floating-point emulation code. */

/* The floating-point representation helpers and exception symbols are supplied
 * by the surrounding PA-RISC math-emulation code. */

pub unsafe fn sgl_to_dbl_fcnvff(
    srcptr: *mut u32,
    _nullptr: *mut u32,
    dstptr: *mut u32,
    _status: *mut u32,
) -> i32 {
    let mut src: u32 = *srcptr;
    let mut resultp1: u32 = 0;
    let mut resultp2: u32 = 0;
    let mut src_exponent: i32;

    src_exponent = Sgl_exponent(src);
    resultp1 = Sgl_all(src);
    if src_exponent == SGL_INFINITY_EXPONENT {
        if Sgl_iszero_mantissa(src) {
            Dbl_setinfinity_exponentmantissa(&mut resultp1, &mut resultp2);
            Dbl_copytoptr(resultp1, resultp2, dstptr);
            return NOEXCEPTION;
        } else {
            if Sgl_isone_signaling(src) {
                if Is_invalidtrap_enabled() {
                    return INVALIDEXCEPTION;
                } else {
                    Set_invalidflag();
                    Sgl_set_quiet(&mut src);
                }
            }
            Dbl_setinfinity_exponent(&mut resultp1);
            Sgl_to_dbl_mantissa(src, &mut resultp1, &mut resultp2);
            Dbl_copytoptr(resultp1, resultp2, dstptr);
            return NOEXCEPTION;
        }
    }
    if src_exponent == 0 {
        if Sgl_isnotzero_mantissa(src) {
            Sgl_clear_signexponent(&mut src);
            Sgl_leftshiftby1(&mut src);
            Sgl_normalize(&mut src, &mut src_exponent);
            Sgl_to_dbl_exponent(src_exponent, &mut resultp1);
            Sgl_to_dbl_mantissa(src, &mut resultp1, &mut resultp2);
        } else {
            Dbl_setzero_exponentmantissa(&mut resultp1, &mut resultp2);
        }
        Dbl_copytoptr(resultp1, resultp2, dstptr);
        return NOEXCEPTION;
    }
    Sgl_to_dbl_exponent(src_exponent, &mut resultp1);
    Sgl_to_dbl_mantissa(Sgl_mantissa(src), &mut resultp1, &mut resultp2);
    Dbl_copytoptr(resultp1, resultp2, dstptr);
    NOEXCEPTION
}

pub unsafe fn dbl_to_sgl_fcnvff(
    srcptr: *mut u32,
    _nullptr: *mut u32,
    dstptr: *mut u32,
    _status: *mut u32,
) -> i32 {
    let mut srcp1: u32 = 0;
    let mut srcp2: u32 = 0;
    let mut result: u32 = 0;
    let mut src_exponent: i32;
    let mut dest_exponent: i32 = 0;
    let mut dest_mantissa: i32 = 0;
    let mut inexact = false;
    let mut guardbit = false;
    let mut stickybit = false;
    let mut lsb_odd = false;
    let mut is_tiny = false;

    Dbl_copyfromptr(srcptr, &mut srcp1, &mut srcp2);
    src_exponent = Dbl_exponent(srcp1);
    result = Dbl_allp1(srcp1);
    if src_exponent == DBL_INFINITY_EXPONENT {
        if Dbl_iszero_mantissa(srcp1, srcp2) {
            Sgl_setinfinity_exponentmantissa(&mut result);
            *dstptr = result;
            return NOEXCEPTION;
        }
        if Dbl_isone_signaling(srcp1) {
            if Is_invalidtrap_enabled() {
                return INVALIDEXCEPTION;
            } else {
                Set_invalidflag();
                Dbl_set_quiet(&mut srcp1);
            }
        }
        Sgl_setinfinity_exponent(&mut result);
        Sgl_set_mantissa(&mut result, Dallp1(srcp1) << 3 | Dallp2(srcp2) >> 29);
        if Sgl_iszero_mantissa(result) { Sgl_set_quiet(&mut result); }
        *dstptr = result;
        return NOEXCEPTION;
    }
    Dbl_to_sgl_exponent(src_exponent, &mut dest_exponent);
    if dest_exponent > 0 {
        Dbl_to_sgl_mantissa(srcp1, srcp2, &mut dest_mantissa, &mut inexact, &mut guardbit, &mut stickybit, &mut lsb_odd);
    } else {
        if Dbl_iszero_exponentmantissa(srcp1, srcp2) {
            Sgl_setzero_exponentmantissa(&mut result);
            *dstptr = result;
            return NOEXCEPTION;
        }
        if Is_underflowtrap_enabled() {
            Dbl_to_sgl_mantissa(srcp1, srcp2, &mut dest_mantissa, &mut inexact, &mut guardbit, &mut stickybit, &mut lsb_odd);
        } else {
            Dbl_to_sgl_denormalized(srcp1, srcp2, &mut dest_exponent, &mut dest_mantissa, &mut inexact, &mut guardbit, &mut stickybit, &mut lsb_odd, &mut is_tiny);
        }
    }
    if inexact {
        match Rounding_mode() {
            ROUNDPLUS => { if Sgl_iszero_sign(result) { dest_mantissa += 1; } }
            ROUNDMINUS => { if Sgl_isone_sign(result) { dest_mantissa += 1; } }
            ROUNDNEAREST => { if guardbit && (stickybit || lsb_odd) { dest_mantissa += 1; } }
            _ => {}
        }
    }
    Sgl_set_exponentmantissa(&mut result, dest_mantissa);
    if (dest_exponent > 0 || Is_underflowtrap_enabled()) && Sgl_isone_hidden(result) { dest_exponent += 1; }
    if dest_exponent >= SGL_INFINITY_EXPONENT {
        if Is_overflowtrap_enabled() {
            if dest_exponent >= SGL_INFINITY_EXPONENT + SGL_WRAP { return UNIMPLEMENTEDEXCEPTION; }
            Sgl_setwrapped_exponent(&mut result, dest_exponent, ovfl);
            *dstptr = result;
            if inexact { if Is_inexacttrap_enabled() { return OVERFLOWEXCEPTION | INEXACTEXCEPTION; } else { Set_inexactflag(); } }
            return OVERFLOWEXCEPTION;
        }
        Set_overflowflag(); inexact = true; Sgl_setoverflow(&mut result);
    } else if dest_exponent <= 0 {
        if Is_underflowtrap_enabled() {
            if dest_exponent <= -SGL_WRAP { return UNIMPLEMENTEDEXCEPTION; }
            Sgl_setwrapped_exponent(&mut result, dest_exponent, unfl);
            *dstptr = result;
            if inexact { if Is_inexacttrap_enabled() { return UNDERFLOWEXCEPTION | INEXACTEXCEPTION; } else { Set_inexactflag(); } }
            return UNDERFLOWEXCEPTION;
        }
        if inexact && is_tiny { Set_underflowflag(); }
    } else { Sgl_set_exponent(&mut result, dest_exponent); }
    *dstptr = result;
    if inexact { if Is_inexacttrap_enabled() { return INEXACTEXCEPTION; } else { Set_inexactflag(); } }
    NOEXCEPTION
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
