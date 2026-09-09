// SPDX-License-Identifier: GPL-2.0-or-later
/*
    NetWinder Floating Point Emulator
    (c) Rebel.COM, 1998,1999
    (c) Philip Blundell, 2001

    Direct questions, comments to Scott Bambrough <scottb@netwinder.org>
*/

// Dependencies supplied by the surrounding implementation.

extern "C" {
    fn float32_exp(fm: float32) -> float32;
    fn float32_ln(fm: float32) -> float32;
    fn float32_sin(fm: float32) -> float32;
    fn float32_cos(fm: float32) -> float32;
    fn float32_arcsin(fm: float32) -> float32;
    fn float32_arctan(fm: float32) -> float32;
    fn float32_log(fm: float32) -> float32;
    fn float32_tan(fm: float32) -> float32;
    fn float32_arccos(fm: float32) -> float32;
    fn float32_pow(fn_: float32, fm: float32) -> float32;
    fn float32_pol(fn_: float32, fm: float32) -> float32;
    fn float32_add(round_data: *mut roundingData, fn_: float32, fm: float32) -> float32;
    fn float32_mul(round_data: *mut roundingData, fn_: float32, fm: float32) -> float32;
    fn float32_sub(round_data: *mut roundingData, fn_: float32, fm: float32) -> float32;
    fn float32_div(round_data: *mut roundingData, fn_: float32, fm: float32) -> float32;
    fn float32_rem(round_data: *mut roundingData, fn_: float32, fm: float32) -> float32;
    fn float32_round_to_int(round_data: *mut roundingData, fm: float32) -> float32;
    fn float32_sqrt(round_data: *mut roundingData, fm: float32) -> float32;
}

unsafe fn float32_rsf(round_data: *mut roundingData, rfn: float32, rfm: float32) -> float32 {
    float32_sub(round_data, rfm, rfn)
}

unsafe fn float32_rdv(round_data: *mut roundingData, rfn: float32, rfm: float32) -> float32 {
    float32_div(round_data, rfm, rfn)
}

static DYADIC_SINGLE: [Option<unsafe fn(*mut roundingData, float32, float32) -> float32>; 16] = {
    let mut table = [None; 16];
    table[ADF_CODE >> 20] = Some(float32_add);
    table[MUF_CODE >> 20] = Some(float32_mul);
    table[SUF_CODE >> 20] = Some(float32_sub);
    table[RSF_CODE >> 20] = Some(float32_rsf);
    table[DVF_CODE >> 20] = Some(float32_div);
    table[RDF_CODE >> 20] = Some(float32_rdv);
    table[RMF_CODE >> 20] = Some(float32_rem);
    table[FML_CODE >> 20] = Some(float32_mul);
    table[FDV_CODE >> 20] = Some(float32_div);
    table[FRD_CODE >> 20] = Some(float32_rdv);
    table
};

unsafe fn float32_mvf(_round_data: *mut roundingData, rfm: float32) -> float32 { rfm }

unsafe fn float32_mnf(_round_data: *mut roundingData, rfm: float32) -> float32 {
    rfm ^ 0x80000000
}

unsafe fn float32_abs(_round_data: *mut roundingData, rfm: float32) -> float32 {
    rfm & 0x7fffffff
}

static MONADIC_SINGLE: [Option<unsafe fn(*mut roundingData, float32) -> float32>; 16] = {
    let mut table = [None; 16];
    table[MVF_CODE >> 20] = Some(float32_mvf);
    table[MNF_CODE >> 20] = Some(float32_mnf);
    table[ABS_CODE >> 20] = Some(float32_abs);
    table[RND_CODE >> 20] = Some(float32_round_to_int);
    table[URD_CODE >> 20] = Some(float32_round_to_int);
    table[SQT_CODE >> 20] = Some(float32_sqrt);
    table[NRM_CODE >> 20] = Some(float32_mvf);
    table
};

pub unsafe fn SingleCPDO(
    round_data: *mut roundingData,
    opcode: u32,
    rfd: *mut FPREG,
) -> u32 {
    let fpa11 = GET_FPA11();
    let fm = getFm(opcode);
    let rfm: float32;

    if CONSTANT_FM(opcode) {
        rfm = getSingleConstant(fm);
    } else if (*fpa11).fType[fm as usize] == typeSingle {
        rfm = (*fpa11).fpreg[fm as usize].fSingle;
    } else {
        return 0;
    }

    let opc_mask_shift = (opcode & MASK_ARITHMETIC_OPCODE) >> 20;
    if !MONADIC_INSTRUCTION(opcode) {
        let fn_ = getFn(opcode);
        if (*fpa11).fType[fn_ as usize] == typeSingle {
            if let Some(operation) = DYADIC_SINGLE[opc_mask_shift as usize] {
                let rfn = (*fpa11).fpreg[fn_ as usize].fSingle;
                (*rfd).fSingle = operation(round_data, rfn, rfm);
            } else {
                return 0;
            }
        } else {
            return 0;
        }
    } else if let Some(operation) = MONADIC_SINGLE[opc_mask_shift as usize] {
        (*rfd).fSingle = operation(round_data, rfm);
    } else {
        return 0;
    }

    1
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
