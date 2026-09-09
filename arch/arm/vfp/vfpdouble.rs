/*
 *  linux/arch/arm/vfp/vfpdouble.c
 *
 * This code is derived in part from John R. Housers softfloat library.
 * The original license and provenance notice are retained in the C source.
 */

static mut vfp_double_default_qnan: vfp_double = vfp_double {
    exponent: 2047,
    sign: 0,
    significand: VFP_DOUBLE_SIGNIFICAND_QNAN,
};

unsafe fn vfp_double_dump(str_: *const i8, d: *mut vfp_double) {
    pr_debug!("VFP: %s: sign=%d exponent=%d significand=%016llx\n", str_, ((*d).sign != 0) as i32, (*d).exponent, (*d).significand);
}

unsafe fn vfp_double_normalise_denormal(vd: *mut vfp_double) {
    let mut bits: i32 = 31 - fls((*vd).significand >> 32);
    if bits == 31 { bits = 63 - fls((*vd).significand); }
    vfp_double_dump(b"normalise_denormal: in\0".as_ptr() as *const i8, vd);
    if bits != 0 {
        (*vd).exponent -= bits - 1;
        (*vd).significand <<= bits;
    }
    vfp_double_dump(b"normalise_denormal: out\0".as_ptr() as *const i8, vd);
}

unsafe fn vfp_double_normaliseround(dd: i32, vd: *mut vfp_double, fpscr: u32, mut exceptions: u32, func: *const i8) -> u32 {
    vfp_double_dump(b"pack: in\0".as_ptr() as *const i8, vd);
    if (*vd).exponent == 2047 && ((*vd).significand == 0 || exceptions != 0) { return vfp_double_pack_result(dd, vd, exceptions, func); }
    if (*vd).significand == 0 { (*vd).exponent = 0; return vfp_double_pack_result(dd, vd, exceptions, func); }
    let mut exponent = (*vd).exponent;
    let mut significand = (*vd).significand;
    let mut shift = 32 - fls(significand >> 32);
    if shift == 32 { shift = 64 - fls(significand); }
    if shift != 0 { exponent -= shift; significand <<= shift; }
    let mut underflow = exponent < 0;
    if underflow {
        significand = vfp_shiftright64jamming(significand, -exponent);
        exponent = 0;
        if (significand & ((1u64 << (VFP_DOUBLE_LOW_BITS + 1)) - 1)) == 0 { underflow = false; }
    }
    let mut incr: u64 = 0;
    let rmode = fpscr & FPSCR_RMODE_MASK;
    if rmode == FPSCR_ROUND_NEAREST {
        incr = 1u64 << VFP_DOUBLE_LOW_BITS;
        if significand & (1u64 << (VFP_DOUBLE_LOW_BITS + 1)) == 0 { incr -= 1; }
    } else if rmode == FPSCR_ROUND_TOZERO { incr = 0; }
    else if (rmode == FPSCR_ROUND_PLUSINF) ^ ((*vd).sign != 0) { incr = (1u64 << (VFP_DOUBLE_LOW_BITS + 1)) - 1; }
    pr_debug!("VFP: rounding increment = 0x%08llx\n", incr);
    if significand.wrapping_add(incr) < significand { exponent += 1; significand = (significand >> 1) | (significand & 1); incr >>= 1; }
    if significand & ((1u64 << (VFP_DOUBLE_LOW_BITS + 1)) - 1) != 0 { exceptions |= FPSCR_IXC; }
    significand = significand.wrapping_add(incr);
    if exponent >= 2046 {
        exceptions |= FPSCR_OFC | FPSCR_IXC;
        if incr == 0 { (*vd).exponent = 2045; (*vd).significand = 0x7fffffffffffffff; }
        else { (*vd).exponent = 2047; (*vd).significand = 0; }
    } else {
        if significand >> (VFP_DOUBLE_LOW_BITS + 1) == 0 { exponent = 0; }
        if exponent != 0 || significand > 0x8000000000000000 { underflow = false; }
        if underflow { exceptions |= FPSCR_UFC; }
        (*vd).exponent = exponent; (*vd).significand = significand >> 1;
    }
    vfp_double_pack_result(dd, vd, exceptions, func)
}

unsafe fn vfp_double_pack_result(dd: i32, vd: *mut vfp_double, exceptions: u32, func: *const i8) -> u32 {
    vfp_double_dump(b"pack: final\0".as_ptr() as *const i8, vd);
    let d = vfp_double_pack(vd);
    pr_debug!("VFP: %s: d(d%d)=%016llx exceptions=%08x\n", func, dd, d, exceptions);
    vfp_put_double(d, dd); exceptions
}

unsafe fn vfp_propagate_nan(vdd: *mut vfp_double, vdn: *mut vfp_double, vdm: *mut vfp_double, fpscr: u32) -> u32 {
    let tn = vfp_double_type(vdn); let tm = if !vdm.is_null() { vfp_double_type(vdm) } else { 0 };
    let nan: *mut vfp_double;
    if fpscr & FPSCR_DEFAULT_NAN != 0 { nan = &mut vfp_double_default_qnan; }
    else { nan = if tn == VFP_SNAN || (tm != VFP_SNAN && tn == VFP_QNAN) { vdn } else { vdm }; (*nan).significand |= VFP_DOUBLE_SIGNIFICAND_QNAN; }
    *vdd = *nan;
    if tn == VFP_SNAN || tm == VFP_SNAN { FPSCR_IOC } else { VFP_NAN_FLAG }
}

unsafe fn vfp_double_fabs(dd: i32, _unused: i32, dm: i32, _fpscr: u32) -> u32 { vfp_put_double(vfp_double_packed_abs(vfp_get_double(dm)), dd); 0 }
unsafe fn vfp_double_fcpy(dd: i32, _unused: i32, dm: i32, _fpscr: u32) -> u32 { vfp_put_double(vfp_get_double(dm), dd); 0 }
unsafe fn vfp_double_fneg(dd: i32, _unused: i32, dm: i32, _fpscr: u32) -> u32 { vfp_put_double(vfp_double_packed_negate(vfp_get_double(dm)), dd); 0 }

unsafe fn vfp_double_fsqrt(dd: i32, _unused: i32, dm: i32, fpscr: u32) -> u32 {
    let mut vdm = vfp_double { exponent: 0, sign: 0, significand: 0 }; let mut vdd = vdm;
    vfp_double_unpack(&mut vdm, vfp_get_double(dm)); let tm = vfp_double_type(&mut vdm);
    if tm & (VFP_NAN|VFP_INFINITY) != 0 { let (vdp, ret): (*mut vfp_double, u32) = if tm & VFP_NAN != 0 { (&mut vdd, vfp_propagate_nan(&mut vdd, &mut vdm, core::ptr::null_mut(), fpscr)) } else if vdm.sign == 0 { (&mut vdm, 0) } else { (&mut vfp_double_default_qnan, FPSCR_IOC) }; vfp_put_double(vfp_double_pack(vdp), dd); return ret; }
    if tm & VFP_ZERO != 0 { vfp_put_double(vfp_double_pack(&mut vdm), dd); return 0; }
    if tm & VFP_DENORMAL != 0 { vfp_double_normalise_denormal(&mut vdm); }
    if vdm.sign != 0 { vfp_put_double(vfp_double_pack(&mut vfp_double_default_qnan), dd); return FPSCR_IOC; }
    vdd.sign = 0; vdd.exponent = ((vdm.exponent - 1023) >> 1) + 1023;
    vdd.significand = (vfp_estimate_sqrt_significand(vdm.exponent, vdm.significand >> 32) as u64) << 31;
    vdm.significand >>= 1 + (vdm.exponent & 1);
    vdd.significand = vdd.significand.wrapping_add(2 + vfp_estimate_div128to64(vdm.significand, 0, vdd.significand));
    if vdd.significand & VFP_DOUBLE_LOW_BITS_MASK <= 5 { if vdd.significand < 2 { vdd.significand = !0; } else { let (mut termh, mut terml, mut remh, mut reml) = (0,0,0,0); vdm.significand <<= 2; mul64to128(&mut termh,&mut terml,vdd.significand,vdd.significand); sub128(&mut remh,&mut reml,vdm.significand,0,termh,terml); while (remh as i64) < 0 { vdd.significand -= 1; shift64left(&mut termh,&mut terml,vdd.significand); terml |= 1; add128(&mut remh,&mut reml,remh,reml,termh,terml); } vdd.significand |= (remh | reml) != 0; } }
    vdd.significand = vfp_shiftright64jamming(vdd.significand, 1); vfp_double_normaliseround(dd, &mut vdd, fpscr, 0, b"fsqrt\0".as_ptr() as *const i8)
}

unsafe fn vfp_compare(dd: i32, signal_on_qnan: i32, dm: i32, _fpscr: u32) -> u32 {
    let mut ret = 0; let m = vfp_get_double(dm);
    if vfp_double_packed_exponent(m) == 2047 && vfp_double_packed_mantissa(m) != 0 { ret |= FPSCR_C | FPSCR_V; if signal_on_qnan != 0 || vfp_double_packed_mantissa(m) & (1u64 << (VFP_DOUBLE_MANTISSA_BITS - 1)) == 0 { ret |= FPSCR_IOC; } }
    let d = vfp_get_double(dd);
    if vfp_double_packed_exponent(d) == 2047 && vfp_double_packed_mantissa(d) != 0 { ret |= FPSCR_C | FPSCR_V; if signal_on_qnan != 0 || vfp_double_packed_mantissa(d) & (1u64 << (VFP_DOUBLE_MANTISSA_BITS - 1)) == 0 { ret |= FPSCR_IOC; } }
    if ret == 0 {
        if d == m || vfp_double_packed_abs(d | m) == 0 { ret |= FPSCR_Z | FPSCR_C; }
        else if vfp_double_packed_sign(d ^ m) != 0 { if vfp_double_packed_sign(d) != 0 { ret |= FPSCR_N; } else { ret |= FPSCR_C; } }
        else if (vfp_double_packed_sign(d) != 0) ^ (d < m) { ret |= FPSCR_N; }
        else if (vfp_double_packed_sign(d) != 0) ^ (d > m) { ret |= FPSCR_C; }
    } ret
}
unsafe fn vfp_double_fcmp(dd: i32, _unused: i32, dm: i32, fpscr: u32) -> u32 { vfp_compare(dd, 0, dm, fpscr) }
unsafe fn vfp_double_fcmpe(dd: i32, _unused: i32, dm: i32, fpscr: u32) -> u32 { vfp_compare(dd, 1, dm, fpscr) }
unsafe fn vfp_double_fcmpz(dd: i32, _unused: i32, _dm: i32, fpscr: u32) -> u32 { vfp_compare(dd, 0, VFP_REG_ZERO, fpscr) }
unsafe fn vfp_double_fcmpez(dd: i32, _unused: i32, _dm: i32, fpscr: u32) -> u32 { vfp_compare(dd, 1, VFP_REG_ZERO, fpscr) }

unsafe fn vfp_double_fcvts(sd: i32, _unused: i32, dm: i32, fpscr: u32) -> u32 {
    let mut vdm = vfp_double { exponent: 0, sign: 0, significand: 0 }; let mut vsd = vfp_single { exponent: 0, sign: 0, significand: 0 };
    vfp_double_unpack(&mut vdm, vfp_get_double(dm)); let tm = vfp_double_type(&mut vdm); let mut exceptions = if tm == VFP_SNAN { FPSCR_IOC } else { 0 };
    if tm & VFP_DENORMAL != 0 { vfp_double_normalise_denormal(&mut vdm); }
    vsd.sign = vdm.sign; vsd.significand = vfp_hi64to32jamming(vdm.significand);
    if tm & (VFP_INFINITY|VFP_NAN) != 0 { vsd.exponent = 255; if tm == VFP_QNAN { vsd.significand |= VFP_SINGLE_SIGNIFICAND_QNAN; } vfp_put_float(vfp_single_pack(&mut vsd), sd); exceptions }
    else { if tm & VFP_ZERO != 0 { vsd.exponent = 0; } else { vsd.exponent = vdm.exponent - (1023 - 127); } vfp_single_normaliseround(sd, &mut vsd, fpscr, exceptions, b"fcvts\0".as_ptr() as *const i8) }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
