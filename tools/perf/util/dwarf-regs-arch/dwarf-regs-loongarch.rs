// SPDX-License-Identifier: GPL-2.0
// C dependencies: <errno.h>, <dwarf-regs.h>,
// "../../../arch/loongarch/include/uapi/asm/perf_regs.h"

use std::os::raw::c_int;

const ENOENT: c_int = 2;

extern "C" {
    static PERF_REG_LOONGARCH_MAX: c_int;
}

#[no_mangle]
pub unsafe extern "C" fn __get_dwarf_regnum_for_perf_regnum_loongarch(perf_regnum: c_int) -> c_int {
    if perf_regnum < 0 || perf_regnum >= PERF_REG_LOONGARCH_MAX {
        return -ENOENT;
    }

    perf_regnum
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
