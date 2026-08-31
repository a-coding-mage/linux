// SPDX-License-Identifier: GPL-2.0
// C includes translated as external dependencies:
// <errno.h>
// <dwarf-regs.h>
// "../../../arch/arm64/include/uapi/asm/perf_regs.h"

extern "C" {
    static PERF_REG_ARM64_MAX: i32;
}

const ENOENT: i32 = 2;

#[no_mangle]
pub unsafe extern "C" fn __get_dwarf_regnum_for_perf_regnum_arm64(perf_regnum: i32) -> i32 {
    if perf_regnum < 0 || perf_regnum >= PERF_REG_ARM64_MAX {
        return -ENOENT;
    }

    perf_regnum
}
