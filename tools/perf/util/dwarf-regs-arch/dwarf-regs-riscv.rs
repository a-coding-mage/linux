// SPDX-License-Identifier: GPL-2.0
// C dependencies: <errno.h>, <dwarf-regs.h>,
// "../../../arch/riscv/include/uapi/asm/perf_regs.h"

pub unsafe extern "C" fn __get_dwarf_regnum_for_perf_regnum_riscv(perf_regnum: ::std::os::raw::c_int) -> ::std::os::raw::c_int {
    if perf_regnum < 0 || perf_regnum >= PERF_REG_RISCV_MAX {
        return -ENOENT;
    }

    perf_regnum
}
