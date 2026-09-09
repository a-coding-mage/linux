// SPDX-License-Identifier: GPL-2.0-or-later
/*
    NetWinder Floating Point Emulator
    (c) Rebel.COM, 1998,1999
    (c) Philip Blundell, 1999, 2001

    Direct questions, comments to Scott Bambrough <scottb@netwinder.org>
*/

// Dependencies supplied by the surrounding translation unit.

extern "C" {
    fn PerformFLT(opcode: u32) -> u32;
    fn PerformFIX(opcode: u32) -> u32;
}

unsafe fn PerformComparison(opcode: u32) -> u32 {
    let fpa11 = GET_FPA11();
    let fn_ = getFn(opcode);
    let fm = getFm(opcode);
    let e_flag = opcode & 0x400000;
    let n_flag = opcode & 0x200000;
    let mut flags: u32 = 0;

    // CONFIG_FPE_NWFPE_XP selects the extended-precision implementation.
    #[cfg(CONFIG_FPE_NWFPE_XP)]
    {
        let r_fn;
        let mut r_fm;

        r_fn = match (*fpa11).fType[fn_] {
            typeSingle => {
                if float32_is_nan((*fpa11).fpreg[fn_].fSingle) {
                    return comparison_unordered(opcode, e_flag, flags);
                }
                float32_to_floatx80((*fpa11).fpreg[fn_].fSingle)
            }
            typeDouble => {
                if float64_is_nan((*fpa11).fpreg[fn_].fDouble) {
                    return comparison_unordered(opcode, e_flag, flags);
                }
                float64_to_floatx80((*fpa11).fpreg[fn_].fDouble)
            }
            typeExtended => {
                if floatx80_is_nan((*fpa11).fpreg[fn_].fExtended) {
                    return comparison_unordered(opcode, e_flag, flags);
                }
                (*fpa11).fpreg[fn_].fExtended
            }
            _ => return 0,
        };

        r_fm = if CONSTANT_FM(opcode) {
            let value = getExtendedConstant(fm);
            if floatx80_is_nan(value) {
                return comparison_unordered(opcode, e_flag, flags);
            }
            value
        } else {
            match (*fpa11).fType[fm] {
                typeSingle => {
                    if float32_is_nan((*fpa11).fpreg[fm].fSingle) {
                        return comparison_unordered(opcode, e_flag, flags);
                    }
                    float32_to_floatx80((*fpa11).fpreg[fm].fSingle)
                }
                typeDouble => {
                    if float64_is_nan((*fpa11).fpreg[fm].fDouble) {
                        return comparison_unordered(opcode, e_flag, flags);
                    }
                    float64_to_floatx80((*fpa11).fpreg[fm].fDouble)
                }
                typeExtended => {
                    if floatx80_is_nan((*fpa11).fpreg[fm].fExtended) {
                        return comparison_unordered(opcode, e_flag, flags);
                    }
                    (*fpa11).fpreg[fm].fExtended
                }
                _ => return 0,
            }
        };

        if n_flag != 0 { r_fm.high ^= 0x8000; }
        if floatx80_lt(r_fn, r_fm) { flags |= CC_NEGATIVE; }
        if floatx80_eq(r_fn, r_fm) { flags |= CC_ZERO; }
        if floatx80_lt(r_fm, r_fn) { flags |= CC_CARRY; }
    }

    #[cfg(not(CONFIG_FPE_NWFPE_XP))]
    {
        if CONSTANT_FM(opcode) {
            if (*fpa11).fType[fn_] == typeSingle {
                let mut r_fm = getSingleConstant(fm);
                let r_fn = (*fpa11).fpreg[fn_].fSingle;
                if float32_is_nan(r_fn) { return comparison_unordered(opcode, e_flag, flags); }
                if n_flag != 0 { r_fm ^= 0x80000000; }
                if float32_lt_nocheck(r_fn, r_fm) { flags |= CC_NEGATIVE; }
                if float32_eq_nocheck(r_fn, r_fm) { flags |= CC_ZERO; }
                if float32_lt_nocheck(r_fm, r_fn) { flags |= CC_CARRY; }
            } else {
                let mut r_fm = getDoubleConstant(fm);
                let r_fn = (*fpa11).fpreg[fn_].fDouble;
                if float64_is_nan(r_fn) { return comparison_unordered(opcode, e_flag, flags); }
                if n_flag != 0 { r_fm ^= 0x8000000000000000u64; }
                if float64_lt_nocheck(r_fn, r_fm) { flags |= CC_NEGATIVE; }
                if float64_eq_nocheck(r_fn, r_fm) { flags |= CC_ZERO; }
                if float64_lt_nocheck(r_fm, r_fn) { flags |= CC_CARRY; }
            }
        } else if (*fpa11).fType[fn_] == typeSingle && (*fpa11).fType[fm] == typeSingle {
            let mut r_fm = (*fpa11).fpreg[fm].fSingle;
            let r_fn = (*fpa11).fpreg[fn_].fSingle;
            if float32_is_nan(r_fn) || float32_is_nan(r_fm) { return comparison_unordered(opcode, e_flag, flags); }
            if n_flag != 0 { r_fm ^= 0x80000000; }
            if float32_lt_nocheck(r_fn, r_fm) { flags |= CC_NEGATIVE; }
            if float32_eq_nocheck(r_fn, r_fm) { flags |= CC_ZERO; }
            if float32_lt_nocheck(r_fm, r_fn) { flags |= CC_CARRY; }
        } else {
            let mut r_fm = if (*fpa11).fType[fm] == typeSingle { float32_to_float64((*fpa11).fpreg[fm].fSingle) } else { (*fpa11).fpreg[fm].fDouble };
            let r_fn = if (*fpa11).fType[fn_] == typeSingle { float32_to_float64((*fpa11).fpreg[fn_].fSingle) } else { (*fpa11).fpreg[fn_].fDouble };
            if float64_is_nan(r_fn) || float64_is_nan(r_fm) { return comparison_unordered(opcode, e_flag, flags); }
            if n_flag != 0 { r_fm ^= 0x8000000000000000u64; }
            if float64_lt_nocheck(r_fn, r_fm) { flags |= CC_NEGATIVE; }
            if float64_eq_nocheck(r_fn, r_fm) { flags |= CC_ZERO; }
            if float64_lt_nocheck(r_fm, r_fn) { flags |= CC_CARRY; }
        }
    }

    writeConditionCodes(flags);
    1
}

unsafe fn comparison_unordered(opcode: u32, e_flag: u32, mut flags: u32) -> u32 {
    flags |= CC_OVERFLOW;
    flags &= !(CC_ZERO | CC_NEGATIVE);
    if BIT_AC & readFPSR() != 0 { flags |= CC_CARRY; }
    if e_flag != 0 { float_raise(float_flag_invalid); }
    writeConditionCodes(flags);
    1
}

pub unsafe fn EmulateCPRT(opcode: u32) -> u32 {
    if opcode & 0x800000 != 0 { return PerformComparison(opcode); }
    match (opcode & 0x700000) >> 20 {
        x if x == (FLT_CODE >> 20) => PerformFLT(opcode),
        x if x == (FIX_CODE >> 20) => PerformFIX(opcode),
        x if x == (WFS_CODE >> 20) => { writeFPSR(readRegister(getRd(opcode))); 1 },
        x if x == (RFS_CODE >> 20) => { writeRegister(getRd(opcode), readFPSR()); 1 },
        _ => 0,
    }
}

pub unsafe fn PerformFLT(opcode: u32) -> u32 {
    let fpa11 = GET_FPA11();
    let mut roundData = roundingData { mode: SetRoundingMode(opcode), precision: SetRoundingPrecision(opcode), exception: 0 };
    match opcode & MASK_ROUNDING_PRECISION {
        ROUND_SINGLE => { (*fpa11).fType[getFn(opcode)] = typeSingle; (*fpa11).fpreg[getFn(opcode)].fSingle = int32_to_float32(&mut roundData, readRegister(getRd(opcode))); },
        ROUND_DOUBLE => { (*fpa11).fType[getFn(opcode)] = typeDouble; (*fpa11).fpreg[getFn(opcode)].fDouble = int32_to_float64(readRegister(getRd(opcode))); },
        #[cfg(CONFIG_FPE_NWFPE_XP)] ROUND_EXTENDED => { (*fpa11).fType[getFn(opcode)] = typeExtended; (*fpa11).fpreg[getFn(opcode)].fExtended = int32_to_floatx80(readRegister(getRd(opcode))); },
        _ => return 0,
    }
    if roundData.exception != 0 { float_raise(roundData.exception); }
    1
}

pub unsafe fn PerformFIX(opcode: u32) -> u32 {
    let fpa11 = GET_FPA11();
    let fn_ = getFm(opcode);
    let mut roundData = roundingData { mode: SetRoundingMode(opcode), precision: SetRoundingPrecision(opcode), exception: 0 };
    match (*fpa11).fType[fn_] {
        typeSingle => writeRegister(getRd(opcode), float32_to_int32(&mut roundData, (*fpa11).fpreg[fn_].fSingle)),
        typeDouble => writeRegister(getRd(opcode), float64_to_int32(&mut roundData, (*fpa11).fpreg[fn_].fDouble)),
        #[cfg(CONFIG_FPE_NWFPE_XP)] typeExtended => writeRegister(getRd(opcode), floatx80_to_int32(&mut roundData, (*fpa11).fpreg[fn_].fExtended)),
        _ => return 0,
    }
    if roundData.exception != 0 { float_raise(roundData.exception); }
    1
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
