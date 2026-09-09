// SPDX-License-Identifier: GPL-2.0-or-later
/*
    NetWinder Floating Point Emulator
    (c) Rebel.COM, 1998,1999
    (c) Philip Blundell, 2001

    Direct questions, comments to Scott Bambrough <scottb@netwinder.org>
*/

// C dependencies: fpa11.h and fpopcode.h.

extern "C" {
    fn SingleCPDO(round_data: *mut roundingData, opcode: u32, r_fd: *mut FPREG) -> u32;
    fn DoubleCPDO(round_data: *mut roundingData, opcode: u32, r_fd: *mut FPREG) -> u32;
    #[cfg(feature = "CONFIG_FPE_NWFPE_XP")]
    fn ExtendedCPDO(round_data: *mut roundingData, opcode: u32, r_fd: *mut FPREG) -> u32;

    fn getDestinationSize(opcode: u32) -> u32;
    fn SetRoundingMode(opcode: u32) -> u32;
    fn SetRoundingPrecision(opcode: u32) -> u32;
    fn MONADIC_INSTRUCTION(opcode: u32) -> bool;
    fn getFn(opcode: u32) -> u32;
    fn CONSTANT_FM(opcode: u32) -> bool;
    fn getFm(opcode: u32) -> u32;
    fn getFd(opcode: u32) -> u32;
    fn float64_to_float32(round_data: *mut roundingData, value: f64) -> f32;
    fn float32_to_float64(value: f32) -> f64;
    #[cfg(feature = "CONFIG_FPE_NWFPE_XP")]
    fn floatx80_to_float32(round_data: *mut roundingData, value: floatx80) -> f32;
    #[cfg(feature = "CONFIG_FPE_NWFPE_XP")]
    fn floatx80_to_float64(round_data: *mut roundingData, value: floatx80) -> f64;
    #[cfg(feature = "CONFIG_FPE_NWFPE_XP")]
    fn float32_to_floatx80(value: f32) -> floatx80;
    #[cfg(feature = "CONFIG_FPE_NWFPE_XP")]
    fn float64_to_floatx80(value: f64) -> floatx80;
    fn float_raise(exception: u32);
}

// GET_FPA11 is a macro supplied by fpa11.h.
pub unsafe fn EmulateCPDO(opcode: u32) -> u32 {
    let fpa11: *mut FPA11 = GET_FPA11();
    let mut round_data: roundingData = core::mem::zeroed();

    /* Get the destination size.  If not valid let Linux perform
       an invalid instruction trap. */
    let n_dest = getDestinationSize(opcode);
    if typeNone == n_dest {
        return 0;
    }

    (*(&mut round_data)).mode = SetRoundingMode(opcode);
    round_data.precision = SetRoundingPrecision(opcode);
    round_data.exception = 0;

    /* Compare the size of the operands in Fn and Fm.
       Choose the largest size and perform operations in that size,
       in order to make use of all the precision of the operands.
       If Fm is a constant, we just grab a constant of a size
       matching the size of the operand in Fn. */
    let mut n_type: u32;
    if MONADIC_INSTRUCTION(opcode) {
        n_type = n_dest;
    } else {
        n_type = (*fpa11).fType[getFn(opcode) as usize];
    }

    if !CONSTANT_FM(opcode) {
        let fm = getFm(opcode);
        if n_type < (*fpa11).fType[fm as usize] {
            n_type = (*fpa11).fType[fm as usize];
        }
    }

    let fd = getFd(opcode);
    let r_fd: *mut FPREG = &mut (*fpa11).fpreg[fd as usize];
    let n_rc = match n_type {
        typeSingle => SingleCPDO(&mut round_data, opcode, r_fd),
        typeDouble => DoubleCPDO(&mut round_data, opcode, r_fd),
        #[cfg(feature = "CONFIG_FPE_NWFPE_XP")]
        typeExtended => ExtendedCPDO(&mut round_data, opcode, r_fd),
        _ => 0,
    };

    /* The CPDO functions used to always set the destination type
       to be the same as their working size. */
    if n_rc != 0 {
        /* If the operation succeeded, check to see if the result in the
           destination register is the correct size.  If not force it
           to be. */
        (*fpa11).fType[fd as usize] = n_dest;

        #[cfg(feature = "CONFIG_FPE_NWFPE_XP")]
        if n_dest != n_type {
            match n_dest {
                typeSingle => {
                    if typeDouble == n_type {
                        (*r_fd).fSingle = float64_to_float32(&mut round_data, (*r_fd).fDouble);
                    } else {
                        (*r_fd).fSingle = floatx80_to_float32(&mut round_data, (*r_fd).fExtended);
                    }
                }
                typeDouble => {
                    if typeSingle == n_type {
                        (*r_fd).fDouble = float32_to_float64((*r_fd).fSingle);
                    } else {
                        (*r_fd).fDouble = floatx80_to_float64(&mut round_data, (*r_fd).fExtended);
                    }
                }
                typeExtended => {
                    if typeSingle == n_type {
                        (*r_fd).fExtended = float32_to_floatx80((*r_fd).fSingle);
                    } else {
                        (*r_fd).fExtended = float64_to_floatx80((*r_fd).fDouble);
                    }
                }
                _ => {}
            }
        }

        #[cfg(not(feature = "CONFIG_FPE_NWFPE_XP"))]
        if n_dest != n_type {
            if n_dest == typeSingle {
                (*r_fd).fSingle = float64_to_float32(&mut round_data, (*r_fd).fDouble);
            } else {
                (*r_fd).fDouble = float32_to_float64((*r_fd).fSingle);
            }
        }
    }

    if round_data.exception != 0 {
        float_raise(round_data.exception);
    }

    n_rc
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
