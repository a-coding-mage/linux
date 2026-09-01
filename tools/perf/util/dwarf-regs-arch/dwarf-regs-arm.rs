// SPDX-License-Identifier: GPL-2.0
// C dependencies: <errno.h>, <dwarf-regs.h>,
// "../../../arch/arm/include/uapi/asm/perf_regs.h"

#[no_mangle]
pub extern "C" fn __get_dwarf_regnum_for_perf_regnum_arm(perf_regnum: i32) -> i32 {
    if perf_regnum < 0 || perf_regnum >= PERF_REG_ARM_MAX {
        return -ENOENT;
    }

    perf_regnum
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
