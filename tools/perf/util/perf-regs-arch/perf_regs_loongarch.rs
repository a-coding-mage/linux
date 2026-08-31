// SPDX-License-Identifier: GPL-2.0

// C dependencies:
// #include "../perf_regs.h"
// #include "../../arch/loongarch/include/perf_regs.h"

use core::ffi::{c_char, c_int};

extern "C" {
    static PERF_REGS_MASK: u64;
    static PERF_REG_LOONGARCH_PC: u64;
    static PERF_REG_LOONGARCH_R1: u64;
    static PERF_REG_LOONGARCH_R2: u64;
    static PERF_REG_LOONGARCH_R3: u64;
    static PERF_REG_LOONGARCH_R4: u64;
    static PERF_REG_LOONGARCH_R5: u64;
    static PERF_REG_LOONGARCH_R6: u64;
    static PERF_REG_LOONGARCH_R7: u64;
    static PERF_REG_LOONGARCH_R8: u64;
    static PERF_REG_LOONGARCH_R9: u64;
    static PERF_REG_LOONGARCH_R10: u64;
    static PERF_REG_LOONGARCH_R11: u64;
    static PERF_REG_LOONGARCH_R12: u64;
    static PERF_REG_LOONGARCH_R13: u64;
    static PERF_REG_LOONGARCH_R14: u64;
    static PERF_REG_LOONGARCH_R15: u64;
    static PERF_REG_LOONGARCH_R16: u64;
    static PERF_REG_LOONGARCH_R17: u64;
    static PERF_REG_LOONGARCH_R18: u64;
    static PERF_REG_LOONGARCH_R19: u64;
    static PERF_REG_LOONGARCH_R20: u64;
    static PERF_REG_LOONGARCH_R21: u64;
    static PERF_REG_LOONGARCH_R22: u64;
    static PERF_REG_LOONGARCH_R23: u64;
    static PERF_REG_LOONGARCH_R24: u64;
    static PERF_REG_LOONGARCH_R25: u64;
    static PERF_REG_LOONGARCH_R26: u64;
    static PERF_REG_LOONGARCH_R27: u64;
    static PERF_REG_LOONGARCH_R28: u64;
    static PERF_REG_LOONGARCH_R29: u64;
    static PERF_REG_LOONGARCH_R30: u64;
    static PERF_REG_LOONGARCH_R31: u64;
}

#[no_mangle]
pub unsafe extern "C" fn __perf_reg_mask_loongarch(_intr: bool) -> u64 {
    PERF_REGS_MASK
}

#[no_mangle]
pub unsafe extern "C" fn __perf_reg_name_loongarch(id: c_int) -> *const c_char {
    match id as u64 {
        x if x == PERF_REG_LOONGARCH_PC => c"PC".as_ptr(),
        x if x == PERF_REG_LOONGARCH_R1 => c"%r1".as_ptr(),
        x if x == PERF_REG_LOONGARCH_R2 => c"%r2".as_ptr(),
        x if x == PERF_REG_LOONGARCH_R3 => c"%r3".as_ptr(),
        x if x == PERF_REG_LOONGARCH_R4 => c"%r4".as_ptr(),
        x if x == PERF_REG_LOONGARCH_R5 => c"%r5".as_ptr(),
        x if x == PERF_REG_LOONGARCH_R6 => c"%r6".as_ptr(),
        x if x == PERF_REG_LOONGARCH_R7 => c"%r7".as_ptr(),
        x if x == PERF_REG_LOONGARCH_R8 => c"%r8".as_ptr(),
        x if x == PERF_REG_LOONGARCH_R9 => c"%r9".as_ptr(),
        x if x == PERF_REG_LOONGARCH_R10 => c"%r10".as_ptr(),
        x if x == PERF_REG_LOONGARCH_R11 => c"%r11".as_ptr(),
        x if x == PERF_REG_LOONGARCH_R12 => c"%r12".as_ptr(),
        x if x == PERF_REG_LOONGARCH_R13 => c"%r13".as_ptr(),
        x if x == PERF_REG_LOONGARCH_R14 => c"%r14".as_ptr(),
        x if x == PERF_REG_LOONGARCH_R15 => c"%r15".as_ptr(),
        x if x == PERF_REG_LOONGARCH_R16 => c"%r16".as_ptr(),
        x if x == PERF_REG_LOONGARCH_R17 => c"%r17".as_ptr(),
        x if x == PERF_REG_LOONGARCH_R18 => c"%r18".as_ptr(),
        x if x == PERF_REG_LOONGARCH_R19 => c"%r19".as_ptr(),
        x if x == PERF_REG_LOONGARCH_R20 => c"%r20".as_ptr(),
        x if x == PERF_REG_LOONGARCH_R21 => c"%r21".as_ptr(),
        x if x == PERF_REG_LOONGARCH_R22 => c"%r22".as_ptr(),
        x if x == PERF_REG_LOONGARCH_R23 => c"%r23".as_ptr(),
        x if x == PERF_REG_LOONGARCH_R24 => c"%r24".as_ptr(),
        x if x == PERF_REG_LOONGARCH_R25 => c"%r25".as_ptr(),
        x if x == PERF_REG_LOONGARCH_R26 => c"%r26".as_ptr(),
        x if x == PERF_REG_LOONGARCH_R27 => c"%r27".as_ptr(),
        x if x == PERF_REG_LOONGARCH_R28 => c"%r28".as_ptr(),
        x if x == PERF_REG_LOONGARCH_R29 => c"%r29".as_ptr(),
        x if x == PERF_REG_LOONGARCH_R30 => c"%r30".as_ptr(),
        x if x == PERF_REG_LOONGARCH_R31 => c"%r31".as_ptr(),
        _ => core::ptr::null(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn __perf_reg_ip_loongarch() -> u64 {
    PERF_REG_LOONGARCH_PC
}

#[no_mangle]
pub unsafe extern "C" fn __perf_reg_sp_loongarch() -> u64 {
    PERF_REG_LOONGARCH_R3
}
