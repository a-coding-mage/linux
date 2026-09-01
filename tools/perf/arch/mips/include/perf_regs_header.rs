/* SPDX-License-Identifier: GPL-2.0 */

// C header dependencies:
// - <stdlib.h>
// - <linux/types.h>
// - "../../../../arch/mips/include/uapi/asm/perf_regs.h"

pub const PERF_REGS_MAX: u64 = PERF_REG_MIPS_MAX as u64;

pub const PERF_REGS_MASK: u64 = (1_u64 << PERF_REG_MIPS_MAX) - 1;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
