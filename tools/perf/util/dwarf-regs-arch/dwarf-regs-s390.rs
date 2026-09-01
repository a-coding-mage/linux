// SPDX-License-Identifier: GPL-2.0
// C includes translated as dependency intent:
// <errno.h>
// <dwarf-regs.h>
// "../../../arch/s390/include/uapi/asm/perf_regs.h"

pub unsafe extern "C" fn __get_dwarf_regnum_for_perf_regnum_s390(perf_regnum: libc::c_int) -> libc::c_int {
    let mut dwarf_s390_regnums: [libc::c_int; (PERF_REG_S390_PC + 1) as usize] =
        [0; (PERF_REG_S390_PC + 1) as usize];

    dwarf_s390_regnums[PERF_REG_S390_R0 as usize] = 0;
    dwarf_s390_regnums[PERF_REG_S390_R1 as usize] = 1;
    dwarf_s390_regnums[PERF_REG_S390_R2 as usize] = 2;
    dwarf_s390_regnums[PERF_REG_S390_R3 as usize] = 3;
    dwarf_s390_regnums[PERF_REG_S390_R4 as usize] = 4;
    dwarf_s390_regnums[PERF_REG_S390_R5 as usize] = 5;
    dwarf_s390_regnums[PERF_REG_S390_R6 as usize] = 6;
    dwarf_s390_regnums[PERF_REG_S390_R7 as usize] = 7;
    dwarf_s390_regnums[PERF_REG_S390_R8 as usize] = 8;
    dwarf_s390_regnums[PERF_REG_S390_R9 as usize] = 9;
    dwarf_s390_regnums[PERF_REG_S390_R10 as usize] = 10;
    dwarf_s390_regnums[PERF_REG_S390_R11 as usize] = 11;
    dwarf_s390_regnums[PERF_REG_S390_R12 as usize] = 12;
    dwarf_s390_regnums[PERF_REG_S390_R13 as usize] = 13;
    dwarf_s390_regnums[PERF_REG_S390_R14 as usize] = 14;
    dwarf_s390_regnums[PERF_REG_S390_R15 as usize] = 15;
    dwarf_s390_regnums[PERF_REG_S390_FP0 as usize] = 16;
    dwarf_s390_regnums[PERF_REG_S390_FP1 as usize] = 20;
    dwarf_s390_regnums[PERF_REG_S390_FP2 as usize] = 17;
    dwarf_s390_regnums[PERF_REG_S390_FP3 as usize] = 21;
    dwarf_s390_regnums[PERF_REG_S390_FP4 as usize] = 18;
    dwarf_s390_regnums[PERF_REG_S390_FP5 as usize] = 22;
    dwarf_s390_regnums[PERF_REG_S390_FP6 as usize] = 19;
    dwarf_s390_regnums[PERF_REG_S390_FP7 as usize] = 23;
    dwarf_s390_regnums[PERF_REG_S390_FP8 as usize] = 24;
    dwarf_s390_regnums[PERF_REG_S390_FP9 as usize] = 28;
    dwarf_s390_regnums[PERF_REG_S390_FP10 as usize] = 25;
    dwarf_s390_regnums[PERF_REG_S390_FP11 as usize] = 29;
    dwarf_s390_regnums[PERF_REG_S390_FP12 as usize] = 26;
    dwarf_s390_regnums[PERF_REG_S390_FP13 as usize] = 30;
    dwarf_s390_regnums[PERF_REG_S390_FP14 as usize] = 27;
    dwarf_s390_regnums[PERF_REG_S390_FP15 as usize] = 31;
    dwarf_s390_regnums[PERF_REG_S390_MASK as usize] = 64;
    dwarf_s390_regnums[PERF_REG_S390_PC as usize] = 65;

    if perf_regnum == 0 {
        return 0;
    }

    if perf_regnum < 0
        || perf_regnum > dwarf_s390_regnums.len() as libc::c_int
        || dwarf_s390_regnums[perf_regnum as usize] == 0
    {
        return -ENOENT;
    }

    dwarf_s390_regnums[perf_regnum as usize]
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
