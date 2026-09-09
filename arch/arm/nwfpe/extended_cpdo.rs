// SPDX-License-Identifier: GPL-2.0-or-later
/*
    NetWinder Floating Point Emulator
    (c) Rebel.COM, 1998,1999

    Direct questions, comments to Scott Bambrough <scottb@netwinder.org>

*/

// Dependencies supplied by the surrounding translation unit:
// fpa11.h, softfloat.h, and fpopcode.h

extern "C" {
    fn floatx80_exp(fm: floatx80) -> floatx80;
    fn floatx80_ln(fm: floatx80) -> floatx80;
    fn floatx80_sin(fm: floatx80) -> floatx80;
    fn floatx80_cos(fm: floatx80) -> floatx80;
    fn floatx80_arcsin(fm: floatx80) -> floatx80;
    fn floatx80_arctan(fm: floatx80) -> floatx80;
    fn floatx80_log(fm: floatx80) -> floatx80;
    fn floatx80_tan(fm: floatx80) -> floatx80;
    fn floatx80_arccos(fm: floatx80) -> floatx80;
    fn floatx80_pow(fn_: floatx80, fm: floatx80) -> floatx80;
    fn floatx80_pol(fn_: floatx80, fm: floatx80) -> floatx80;
}

unsafe fn floatx80_rsf(round_data: *mut roundingData, rfn: floatx80, rfm: floatx80) -> floatx80 {
    floatx80_sub(round_data, rfm, rfn)
}

unsafe fn floatx80_rdv(round_data: *mut roundingData, rfn: floatx80, rfm: floatx80) -> floatx80 {
    floatx80_div(round_data, rfm, rfn)
}

unsafe fn dyadic_extended(opcode_index: usize) -> Option<unsafe fn(*mut roundingData, floatx80, floatx80) -> floatx80> {
    match opcode_index {
        x if x == (ADF_CODE >> 20) as usize => Some(floatx80_add),
        x if x == (MUF_CODE >> 20) as usize => Some(floatx80_mul),
        x if x == (SUF_CODE >> 20) as usize => Some(floatx80_sub),
        x if x == (RSF_CODE >> 20) as usize => Some(floatx80_rsf),
        x if x == (DVF_CODE >> 20) as usize => Some(floatx80_div),
        x if x == (RDF_CODE >> 20) as usize => Some(floatx80_rdv),
        x if x == (RMF_CODE >> 20) as usize => Some(floatx80_rem),
        // strictly, these opcodes should not be implemented
        x if x == (FML_CODE >> 20) as usize => Some(floatx80_mul),
        x if x == (FDV_CODE >> 20) as usize => Some(floatx80_div),
        x if x == (FRD_CODE >> 20) as usize => Some(floatx80_rdv),
        _ => None,
    }
}

unsafe fn floatx80_mvf(_round_data: *mut roundingData, rfm: floatx80) -> floatx80 {
    rfm
}

unsafe fn floatx80_mnf(_round_data: *mut roundingData, mut rfm: floatx80) -> floatx80 {
    rfm.high ^= 0x8000;
    rfm
}

unsafe fn floatx80_abs(_round_data: *mut roundingData, mut rfm: floatx80) -> floatx80 {
    rfm.high &= 0x7fff;
    rfm
}

unsafe fn monadic_extended(opcode_index: usize) -> Option<unsafe fn(*mut roundingData, floatx80) -> floatx80> {
    match opcode_index {
        x if x == (MVF_CODE >> 20) as usize => Some(floatx80_mvf),
        x if x == (MNF_CODE >> 20) as usize => Some(floatx80_mnf),
        x if x == (ABS_CODE >> 20) as usize => Some(floatx80_abs),
        x if x == (RND_CODE >> 20) as usize => Some(floatx80_round_to_int),
        x if x == (URD_CODE >> 20) as usize => Some(floatx80_round_to_int),
        x if x == (SQT_CODE >> 20) as usize => Some(floatx80_sqrt),
        x if x == (NRM_CODE >> 20) as usize => Some(floatx80_mvf),
        _ => None,
    }
}

pub unsafe fn ExtendedCPDO(
    round_data: *mut roundingData,
    opcode: u32,
    rfd: *mut FPREG,
) -> u32 {
    let fpa11 = GET_FPA11();
    let fm = getFm(opcode);
    let rfm = if CONSTANT_FM(opcode) {
        getExtendedConstant(fm)
    } else {
        match (*fpa11).fType[fm as usize] {
            typeSingle => float32_to_floatx80((*fpa11).fpreg[fm as usize].fSingle),
            typeDouble => float64_to_floatx80((*fpa11).fpreg[fm as usize].fDouble),
            typeExtended => (*fpa11).fpreg[fm as usize].fExtended,
            _ => return 0,
        }
    };

    let opc_mask_shift = ((opcode & MASK_ARITHMETIC_OPCODE) >> 20) as usize;
    if !MONADIC_INSTRUCTION(opcode) {
        let fn_ = getFn(opcode);
        let rfn = match (*fpa11).fType[fn_ as usize] {
            typeSingle => float32_to_floatx80((*fpa11).fpreg[fn_ as usize].fSingle),
            typeDouble => float64_to_floatx80((*fpa11).fpreg[fn_ as usize].fDouble),
            typeExtended => (*fpa11).fpreg[fn_ as usize].fExtended,
            _ => return 0,
        };

        if let Some(operation) = dyadic_extended(opc_mask_shift) {
            (*rfd).fExtended = operation(round_data, rfn, rfm);
        } else {
            return 0;
        }
    } else if let Some(operation) = monadic_extended(opc_mask_shift) {
        (*rfd).fExtended = operation(round_data, rfm);
    } else {
        return 0;
    }

    1
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
