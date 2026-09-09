// SPDX-License-Identifier: GPL-2.0-or-later
/*
    NetWinder Floating Point Emulator
    (c) Rebel.COM, 1998,1999

    Direct questions, comments to Scott Bambrough <scottb@netwinder.org>
*/

// C dependencies: fpa11.h, softfloat.h, and fpopcode.h.

#[repr(C)]
pub union Float64Components {
    pub f64: float64,
    pub i: [u32; 2],
}

extern "C" {
    fn float64_exp(fm: float64) -> float64;
    fn float64_ln(fm: float64) -> float64;
    fn float64_sin(fm: float64) -> float64;
    fn float64_cos(fm: float64) -> float64;
    fn float64_arcsin(fm: float64) -> float64;
    fn float64_arctan(fm: float64) -> float64;
    fn float64_log(fm: float64) -> float64;
    fn float64_tan(fm: float64) -> float64;
    fn float64_arccos(fm: float64) -> float64;
    fn float64_pow(fn_: float64, fm: float64) -> float64;
    fn float64_pol(fn_: float64, fm: float64) -> float64;
}

unsafe fn float64_rsf(round_data: *mut roundingData, r_fn: float64, r_fm: float64) -> float64 {
    float64_sub(round_data, r_fm, r_fn)
}

unsafe fn float64_rdv(round_data: *mut roundingData, r_fn: float64, r_fm: float64) -> float64 {
    float64_div(round_data, r_fm, r_fn)
}

type DyadicDouble = unsafe fn(*mut roundingData, float64, float64) -> float64;
type MonadicDouble = unsafe fn(*mut roundingData, float64) -> float64;

static DYADIC_DOUBLE: [Option<DyadicDouble>; 16] = {
    let mut table: [Option<DyadicDouble>; 16] = [None; 16];
    table[ADF_CODE >> 20] = Some(float64_add);
    table[MUF_CODE >> 20] = Some(float64_mul);
    table[SUF_CODE >> 20] = Some(float64_sub);
    table[RSF_CODE >> 20] = Some(float64_rsf);
    table[DVF_CODE >> 20] = Some(float64_div);
    table[RDF_CODE >> 20] = Some(float64_rdv);
    table[RMF_CODE >> 20] = Some(float64_rem);
    // Strictly, these opcodes should not be implemented.
    table[FML_CODE >> 20] = Some(float64_mul);
    table[FDV_CODE >> 20] = Some(float64_div);
    table[FRD_CODE >> 20] = Some(float64_rdv);
    table
};

unsafe fn float64_mvf(_round_data: *mut roundingData, r_fm: float64) -> float64 {
    r_fm
}

unsafe fn float64_mnf(_round_data: *mut roundingData, r_fm: float64) -> float64 {
    let mut u = Float64Components { f64: r_fm };
    #[cfg(target_endian = "big")]
    {
        u.i[0] ^= 0x8000_0000;
    }
    #[cfg(target_endian = "little")]
    {
        u.i[1] ^= 0x8000_0000;
    }
    u.f64
}

unsafe fn float64_abs(_round_data: *mut roundingData, r_fm: float64) -> float64 {
    let mut u = Float64Components { f64: r_fm };
    #[cfg(target_endian = "big")]
    {
        u.i[0] &= 0x7fff_ffff;
    }
    #[cfg(target_endian = "little")]
    {
        u.i[1] &= 0x7fff_ffff;
    }
    u.f64
}

static MONADIC_DOUBLE: [Option<MonadicDouble>; 16] = {
    let mut table: [Option<MonadicDouble>; 16] = [None; 16];
    table[MVF_CODE >> 20] = Some(float64_mvf);
    table[MNF_CODE >> 20] = Some(float64_mnf);
    table[ABS_CODE >> 20] = Some(float64_abs);
    table[RND_CODE >> 20] = Some(float64_round_to_int);
    table[URD_CODE >> 20] = Some(float64_round_to_int);
    table[SQT_CODE >> 20] = Some(float64_sqrt);
    table[NRM_CODE >> 20] = Some(float64_mvf);
    table
};

#[no_mangle]
pub unsafe extern "C" fn DoubleCPDO(
    round_data: *mut roundingData,
    opcode: u32,
    r_fd: *mut FPREG,
) -> u32 {
    let fpa11 = GET_FPA11();
    let fm = getFm(opcode);
    let r_fm: float64;

    if CONSTANT_FM(opcode) {
        r_fm = getDoubleConstant(fm);
    } else {
        r_fm = match fpa11.fType[fm as usize] {
            typeSingle => float32_to_float64(fpa11.fpreg[fm as usize].fSingle),
            typeDouble => fpa11.fpreg[fm as usize].fDouble,
            _ => return 0,
        };
    }

    let opc_mask_shift = (opcode & MASK_ARITHMETIC_OPCODE) >> 20;
    if !MONADIC_INSTRUCTION(opcode) {
        let fn_ = getFn(opcode);
        let r_fn = match fpa11.fType[fn_ as usize] {
            typeSingle => float32_to_float64(fpa11.fpreg[fn_ as usize].fSingle),
            typeDouble => fpa11.fpreg[fn_ as usize].fDouble,
            _ => return 0,
        };
        if let Some(operation) = DYADIC_DOUBLE[opc_mask_shift as usize] {
            (*r_fd).fDouble = operation(round_data, r_fn, r_fm);
        } else {
            return 0;
        }
    } else if let Some(operation) = MONADIC_DOUBLE[opc_mask_shift as usize] {
        (*r_fd).fDouble = operation(round_data, r_fm);
    } else {
        return 0;
    }

    1
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
