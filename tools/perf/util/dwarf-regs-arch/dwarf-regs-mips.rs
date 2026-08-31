// SPDX-License-Identifier: GPL-2.0
// C dependencies: errno.h, dwarf-regs.h,
// ../../../arch/mips/include/uapi/asm/perf_regs.h

pub unsafe extern "C" fn __get_dwarf_regnum_for_perf_regnum_mips(perf_regnum: i32) -> i32 {
    if perf_regnum == PERF_REG_MIPS_PC {
        return 37;
    }
    if perf_regnum < 0 || perf_regnum >= PERF_REG_MIPS_MAX {
        return -ENOENT;
    }

    perf_regnum
}
