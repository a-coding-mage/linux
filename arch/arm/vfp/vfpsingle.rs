/*
 *  linux/arch/arm/vfp/vfpsingle.c
 *
 * This code is derived in part from John R. Housers softfloat library, which
 * carries the following notice:
 *
 * ===========================================================================
 * This C source file is part of the SoftFloat IEC/IEEE Floating-point
 * Arithmetic Package, Release 2.
 *
 * Written by John R. Hauser.  This work was made possible in part by the
 * International Computer Science Institute, located at Suite 600, 1947 Center
 * Street, Berkeley, California 94704.  Funding was partially provided by the
 * National Science Foundation under grant MIP-9311980.  The original version
 * of this code was written as part of a project to build a fixed-point vector
 * processor in collaboration with the University of California at Berkeley,
 * overseen by Profs. Nelson Morgan and John Wawrzynek.  More information
 * is available through the web page `http://HTTP.CS.Berkeley.EDU/~jhauser/
 * arithmetic/softfloat.html'.
 *
 * THIS SOFTWARE IS DISTRIBUTED AS IS, FOR FREE.  Although reasonable effort
 * has been made to avoid it, THIS SOFTWARE MAY CONTAIN FAULTS THAT WILL AT
 * TIMES RESULT IN INCORRECT BEHAVIOR.  USE OF THIS SOFTWARE IS RESTRICTED TO
 * PERSONS AND ORGANIZATIONS WHO CAN AND WILL TAKE FULL RESPONSIBILITY FOR ANY
 * AND ALL LOSSES, COSTS, OR OTHER PROBLEMS ARISING FROM ITS USE.
 *
 * Derivative works are acceptable, even for commercial purposes, so long as
 * (1) they include prominent notice that the work is derivative, and (2) they
 * include prominent notice akin to these three paragraphs for those parts of
 * this code that are retained.
 * ===========================================================================
 */

static mut vfp_single_default_qnan: struct_vfp_single = struct_vfp_single {
    exponent: 255,
    sign: 0,
    significand: VFP_SINGLE_SIGNIFICAND_QNAN,
};

unsafe fn vfp_single_dump(str_: *const i8, s: *mut struct_vfp_single) {
    pr_debug!("VFP: %s: sign=%d exponent=%d significand=%08x\n",
        str_, (*s).sign != 0, (*s).exponent, (*s).significand);
}

unsafe fn vfp_single_normalise_denormal(vs: *mut struct_vfp_single) {
    let bits: i32 = 31 - fls((*vs).significand) as i32;
    vfp_single_dump(b"normalise_denormal: in\0".as_ptr() as *const i8, vs);
    if bits != 0 {
        (*vs).exponent -= bits - 1;
        (*vs).significand <<= bits as u32;
    }
    vfp_single_dump(b"normalise_denormal: out\0".as_ptr() as *const i8, vs);
}

unsafe fn vfp_single_normaliseround(sd: i32, vs: *mut struct_vfp_single,
                                    fpscr: u32, mut exceptions: u32) -> u32 {
    let mut significand: u32;
    let mut incr: u32;
    let rmode: u32;
    let mut exponent: i32;
    let mut shift: i32;
    let mut underflow: i32;

    vfp_single_dump(b"pack: in\0".as_ptr() as *const i8, vs);
    if (*vs).exponent == 255 && ((*vs).significand == 0 || exceptions != 0) { goto_pack!(); }
    if (*vs).significand == 0 {
        (*vs).exponent = 0;
        goto_pack!();
    }
    exponent = (*vs).exponent;
    significand = (*vs).significand;
    shift = 32 - fls(significand) as i32;
    if shift < 32 && shift != 0 {
        exponent -= shift;
        significand <<= shift as u32;
    }
    underflow = (exponent < 0) as i32;
    if underflow != 0 {
        significand = vfp_shiftright32jamming(significand, (-exponent) as u32);
        exponent = 0;
        if (significand & ((1u32 << (VFP_SINGLE_LOW_BITS + 1)) - 1)) == 0 { underflow = 0; }
    }
    incr = 0;
    rmode = fpscr & FPSCR_RMODE_MASK;
    if rmode == FPSCR_ROUND_NEAREST {
        incr = 1u32 << VFP_SINGLE_LOW_BITS;
        if significand & (1u32 << (VFP_SINGLE_LOW_BITS + 1)) == 0 { incr -= 1; }
    } else if rmode == FPSCR_ROUND_TOZERO {
        incr = 0;
    } else if (rmode == FPSCR_ROUND_PLUSINF) ^ ((*vs).sign != 0) {
        incr = (1u32 << (VFP_SINGLE_LOW_BITS + 1)) - 1;
    }
    pr_debug!("VFP: rounding increment = 0x%08x\n", incr);
    if significand.wrapping_add(incr) < significand {
        exponent += 1;
        significand = (significand >> 1) | (significand & 1);
        incr >>= 1;
    }
    if significand & ((1u32 << (VFP_SINGLE_LOW_BITS + 1)) - 1) != 0 { exceptions |= FPSCR_IXC; }
    significand = significand.wrapping_add(incr);
    if exponent >= 254 {
        exceptions |= FPSCR_OFC | FPSCR_IXC;
        if incr == 0 { (*vs).exponent = 253; (*vs).significand = 0x7fffffff; }
        else { (*vs).exponent = 255; (*vs).significand = 0; }
    } else {
        if significand >> (VFP_SINGLE_LOW_BITS + 1) == 0 { exponent = 0; }
        if exponent != 0 || significand > 0x80000000 { underflow = 0; }
        if underflow != 0 { exceptions |= FPSCR_UFC; }
        (*vs).exponent = exponent;
        (*vs).significand = significand >> 1;
    }
goto_pack!();
    vfp_single_dump(b"pack: final\0".as_ptr() as *const i8, vs);
    let d: i32 = vfp_single_pack(vs);
    vfp_put_float(d, sd);
    exceptions
}

unsafe fn vfp_propagate_nan(vsd: *mut struct_vfp_single, vsn: *mut struct_vfp_single,
                            vsm: *mut struct_vfp_single, fpscr: u32) -> u32 {
    let tn = vfp_single_type(vsn);
    let tm = if !vsm.is_null() { vfp_single_type(vsm) } else { 0 };
    let nan: *mut struct_vfp_single;
    if fpscr & FPSCR_DEFAULT_NAN != 0 { nan = &raw mut vfp_single_default_qnan; }
    else if tn == VFP_SNAN || (tm != VFP_SNAN && tn == VFP_QNAN) { nan = vsn; }
    else { nan = vsm; (*nan).significand |= VFP_SINGLE_SIGNIFICAND_QNAN; }
    *vsd = *nan;
    if tn == VFP_SNAN || tm == VFP_SNAN { FPSCR_IOC } else { VFP_NAN_FLAG }
}

unsafe fn vfp_single_fabs(sd: i32, _unused: i32, m: i32, _fpscr: u32) -> u32 { vfp_put_float(vfp_single_packed_abs(m), sd); 0 }
unsafe fn vfp_single_fcpy(sd: i32, _unused: i32, m: i32, _fpscr: u32) -> u32 { vfp_put_float(m, sd); 0 }
unsafe fn vfp_single_fneg(sd: i32, _unused: i32, m: i32, _fpscr: u32) -> u32 { vfp_put_float(vfp_single_packed_negate(m), sd); 0 }

static sqrt_oddadjust: [u16; 16] = [0x0004,0x0022,0x005d,0x00b1,0x011d,0x019f,0x0236,0x02e0,0x039c,0x0468,0x0545,0x0631,0x072b,0x0832,0x0946,0x0a67];
static sqrt_evenadjust: [u16; 16] = [0x0a2d,0x08af,0x075a,0x0629,0x051a,0x0429,0x0356,0x029e,0x0200,0x0179,0x0109,0x00af,0x0068,0x0034,0x0012,0x0002];

pub unsafe fn vfp_estimate_sqrt_significand(exponent: u32, mut significand: u32) -> u32 {
    let mut a = significand << 1;
    let index = ((a >> 27) & 15) as usize;
    let mut z;
    if exponent & 1 != 0 {
        z = 0x4000 + (a >> 17) - sqrt_oddadjust[index] as u32;
        z = ((a / z) << 14) + (z << 15); a >>= 1;
    } else {
        z = 0x8000 + (a >> 17) - sqrt_evenadjust[index] as u32;
        z = a / z + z;
        z = if z >= 0x20000 { 0xffff8000 } else { z << 15 };
        if z <= a { return (a as i32 >> 1) as u32; }
    }
    let v: u64 = (a as u64) << 31;
    v / z as u64 + (z >> 1) as u64 as u32
}

unsafe fn vfp_single_fsqrt(sd: i32, _unused: i32, m: i32, fpscr: u32) -> u32 {
    let mut vsm = struct_vfp_single::default(); let mut vsd = struct_vfp_single::default();
    vfp_single_unpack(&mut vsm, m); let tm = vfp_single_type(&vsm);
    if tm & (VFP_NAN | VFP_INFINITY) != 0 {
        let mut ret; let vsp: *mut struct_vfp_single;
        if tm & VFP_NAN != 0 { vsp = &mut vsd; ret = vfp_propagate_nan(vsp, &mut vsm, core::ptr::null_mut(), fpscr); }
        else if vsm.sign == 0 { vsp = &mut vsm; ret = 0; }
        else { vsp = &raw mut vfp_single_default_qnan; ret = FPSCR_IOC; }
        vfp_put_float(vfp_single_pack(vsp), sd); return ret;
    }
    if tm & VFP_ZERO != 0 { vfp_put_float(vfp_single_pack(&mut vsm), sd); return 0; }
    if tm & VFP_DENORMAL != 0 { vfp_single_normalise_denormal(&mut vsm); }
    if vsm.sign != 0 { vfp_put_float(vfp_single_pack(&raw mut vfp_single_default_qnan), sd); return FPSCR_IOC; }
    vsd.sign = 0; vsd.exponent = ((vsm.exponent - 127) >> 1) + 127;
    vsd.significand = vfp_estimate_sqrt_significand(vsm.exponent, vsm.significand) + 2;
    if vsd.significand & VFP_SINGLE_LOW_BITS_MASK <= 5 {
        if vsd.significand < 2 { vsd.significand = 0xffffffff; }
        else { vsm.significand <<= (!(vsm.exponent & 1)) as u32; let term = vsd.significand as u64 * vsd.significand as u64; let mut rem = ((vsm.significand as u64) << 32) as i64 - term as i64; while rem < 0 { vsd.significand -= 1; rem += ((vsd.significand as u64) << 1 | 1) as i64; } if rem != 0 { vsd.significand |= 1; } }
    }
    vsd.significand = vfp_shiftright32jamming(vsd.significand, 1);
    vfp_single_normaliseround(sd, &mut vsd, fpscr, 0)
}

unsafe fn vfp_compare(sd: i32, signal_on_qnan: i32, m: i32, _fpscr: u32) -> u32 {
    let d = vfp_get_float(sd); let mut ret = 0;
    if vfp_single_packed_exponent(m) == 255 && vfp_single_packed_mantissa(m) != 0 { ret |= FPSCR_C | FPSCR_V; if signal_on_qnan != 0 || vfp_single_packed_mantissa(m) & (1 << (VFP_SINGLE_MANTISSA_BITS - 1)) == 0 { ret |= FPSCR_IOC; } }
    if vfp_single_packed_exponent(d) == 255 && vfp_single_packed_mantissa(d) != 0 { ret |= FPSCR_C | FPSCR_V; if signal_on_qnan != 0 || vfp_single_packed_mantissa(d) & (1 << (VFP_SINGLE_MANTISSA_BITS - 1)) == 0 { ret |= FPSCR_IOC; } }
    if ret == 0 { if d == m || vfp_single_packed_abs(d | m) == 0 { ret |= FPSCR_Z | FPSCR_C; } else if vfp_single_packed_sign(d ^ m) != 0 { if vfp_single_packed_sign(d) != 0 { ret |= FPSCR_N; } else { ret |= FPSCR_C; } } else if (vfp_single_packed_sign(d) != 0) ^ (d < m) { ret |= FPSCR_N; } else if (vfp_single_packed_sign(d) != 0) ^ (d > m) { ret |= FPSCR_C; } }
    ret
}
unsafe fn vfp_single_fcmp(sd: i32, _unused: i32, m: i32, fpscr: u32) -> u32 { vfp_compare(sd, 0, m, fpscr) }
unsafe fn vfp_single_fcmpe(sd: i32, _unused: i32, m: i32, fpscr: u32) -> u32 { vfp_compare(sd, 1, m, fpscr) }
unsafe fn vfp_single_fcmpz(sd: i32, _unused: i32, _m: i32, fpscr: u32) -> u32 { vfp_compare(sd, 0, 0, fpscr) }
unsafe fn vfp_single_fcmpez(sd: i32, _unused: i32, _m: i32, fpscr: u32) -> u32 { vfp_compare(sd, 1, 0, fpscr) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
