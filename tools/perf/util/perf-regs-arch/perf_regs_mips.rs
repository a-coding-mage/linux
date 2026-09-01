// SPDX-License-Identifier: GPL-2.0

// Dependencies from:
// ../perf_regs.h
// ../../arch/mips/include/perf_regs.h

pub unsafe fn __perf_reg_mask_mips(_intr: bool) -> u64 {
    PERF_REGS_MASK
}

pub unsafe fn __perf_reg_name_mips(id: i32) -> *const ::core::ffi::c_char {
    match id {
        PERF_REG_MIPS_PC => b"PC\0".as_ptr() as *const ::core::ffi::c_char,
        PERF_REG_MIPS_R1 => b"$1\0".as_ptr() as *const ::core::ffi::c_char,
        PERF_REG_MIPS_R2 => b"$2\0".as_ptr() as *const ::core::ffi::c_char,
        PERF_REG_MIPS_R3 => b"$3\0".as_ptr() as *const ::core::ffi::c_char,
        PERF_REG_MIPS_R4 => b"$4\0".as_ptr() as *const ::core::ffi::c_char,
        PERF_REG_MIPS_R5 => b"$5\0".as_ptr() as *const ::core::ffi::c_char,
        PERF_REG_MIPS_R6 => b"$6\0".as_ptr() as *const ::core::ffi::c_char,
        PERF_REG_MIPS_R7 => b"$7\0".as_ptr() as *const ::core::ffi::c_char,
        PERF_REG_MIPS_R8 => b"$8\0".as_ptr() as *const ::core::ffi::c_char,
        PERF_REG_MIPS_R9 => b"$9\0".as_ptr() as *const ::core::ffi::c_char,
        PERF_REG_MIPS_R10 => b"$10\0".as_ptr() as *const ::core::ffi::c_char,
        PERF_REG_MIPS_R11 => b"$11\0".as_ptr() as *const ::core::ffi::c_char,
        PERF_REG_MIPS_R12 => b"$12\0".as_ptr() as *const ::core::ffi::c_char,
        PERF_REG_MIPS_R13 => b"$13\0".as_ptr() as *const ::core::ffi::c_char,
        PERF_REG_MIPS_R14 => b"$14\0".as_ptr() as *const ::core::ffi::c_char,
        PERF_REG_MIPS_R15 => b"$15\0".as_ptr() as *const ::core::ffi::c_char,
        PERF_REG_MIPS_R16 => b"$16\0".as_ptr() as *const ::core::ffi::c_char,
        PERF_REG_MIPS_R17 => b"$17\0".as_ptr() as *const ::core::ffi::c_char,
        PERF_REG_MIPS_R18 => b"$18\0".as_ptr() as *const ::core::ffi::c_char,
        PERF_REG_MIPS_R19 => b"$19\0".as_ptr() as *const ::core::ffi::c_char,
        PERF_REG_MIPS_R20 => b"$20\0".as_ptr() as *const ::core::ffi::c_char,
        PERF_REG_MIPS_R21 => b"$21\0".as_ptr() as *const ::core::ffi::c_char,
        PERF_REG_MIPS_R22 => b"$22\0".as_ptr() as *const ::core::ffi::c_char,
        PERF_REG_MIPS_R23 => b"$23\0".as_ptr() as *const ::core::ffi::c_char,
        PERF_REG_MIPS_R24 => b"$24\0".as_ptr() as *const ::core::ffi::c_char,
        PERF_REG_MIPS_R25 => b"$25\0".as_ptr() as *const ::core::ffi::c_char,
        PERF_REG_MIPS_R28 => b"$28\0".as_ptr() as *const ::core::ffi::c_char,
        PERF_REG_MIPS_R29 => b"$29\0".as_ptr() as *const ::core::ffi::c_char,
        PERF_REG_MIPS_R30 => b"$30\0".as_ptr() as *const ::core::ffi::c_char,
        PERF_REG_MIPS_R31 => b"$31\0".as_ptr() as *const ::core::ffi::c_char,
        _ => ::core::ptr::null(),
    }
}

pub unsafe fn __perf_reg_ip_mips() -> u64 {
    PERF_REG_MIPS_PC as u64
}

pub unsafe fn __perf_reg_sp_mips() -> u64 {
    PERF_REG_MIPS_R29 as u64
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
