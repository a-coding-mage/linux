// SPDX-License-Identifier: GPL-2.0

// C dependencies:
// #include "../perf_regs.h"
// #include "../../arch/arm/include/perf_regs.h"

use std::os::raw::{c_char, c_int};

extern "C" {
    static PERF_REGS_MASK: u64;
    static PERF_REG_ARM_R0: c_int;
    static PERF_REG_ARM_R1: c_int;
    static PERF_REG_ARM_R2: c_int;
    static PERF_REG_ARM_R3: c_int;
    static PERF_REG_ARM_R4: c_int;
    static PERF_REG_ARM_R5: c_int;
    static PERF_REG_ARM_R6: c_int;
    static PERF_REG_ARM_R7: c_int;
    static PERF_REG_ARM_R8: c_int;
    static PERF_REG_ARM_R9: c_int;
    static PERF_REG_ARM_R10: c_int;
    static PERF_REG_ARM_FP: c_int;
    static PERF_REG_ARM_IP: c_int;
    static PERF_REG_ARM_SP: c_int;
    static PERF_REG_ARM_LR: c_int;
    static PERF_REG_ARM_PC: c_int;
}

#[no_mangle]
pub unsafe extern "C" fn __perf_reg_mask_arm(_intr: bool) -> u64 {
    PERF_REGS_MASK
}

#[no_mangle]
pub unsafe extern "C" fn __perf_reg_name_arm(id: c_int) -> *const c_char {
    if id == PERF_REG_ARM_R0 {
        return b"r0\0".as_ptr() as *const c_char;
    }
    if id == PERF_REG_ARM_R1 {
        return b"r1\0".as_ptr() as *const c_char;
    }
    if id == PERF_REG_ARM_R2 {
        return b"r2\0".as_ptr() as *const c_char;
    }
    if id == PERF_REG_ARM_R3 {
        return b"r3\0".as_ptr() as *const c_char;
    }
    if id == PERF_REG_ARM_R4 {
        return b"r4\0".as_ptr() as *const c_char;
    }
    if id == PERF_REG_ARM_R5 {
        return b"r5\0".as_ptr() as *const c_char;
    }
    if id == PERF_REG_ARM_R6 {
        return b"r6\0".as_ptr() as *const c_char;
    }
    if id == PERF_REG_ARM_R7 {
        return b"r7\0".as_ptr() as *const c_char;
    }
    if id == PERF_REG_ARM_R8 {
        return b"r8\0".as_ptr() as *const c_char;
    }
    if id == PERF_REG_ARM_R9 {
        return b"r9\0".as_ptr() as *const c_char;
    }
    if id == PERF_REG_ARM_R10 {
        return b"r10\0".as_ptr() as *const c_char;
    }
    if id == PERF_REG_ARM_FP {
        return b"fp\0".as_ptr() as *const c_char;
    }
    if id == PERF_REG_ARM_IP {
        return b"ip\0".as_ptr() as *const c_char;
    }
    if id == PERF_REG_ARM_SP {
        return b"sp\0".as_ptr() as *const c_char;
    }
    if id == PERF_REG_ARM_LR {
        return b"lr\0".as_ptr() as *const c_char;
    }
    if id == PERF_REG_ARM_PC {
        return b"pc\0".as_ptr() as *const c_char;
    }

    std::ptr::null()
}

#[no_mangle]
pub unsafe extern "C" fn __perf_reg_ip_arm() -> u64 {
    PERF_REG_ARM_PC as u64
}

#[no_mangle]
pub unsafe extern "C" fn __perf_reg_sp_arm() -> u64 {
    PERF_REG_ARM_SP as u64
}
