/* SPDX-License-Identifier: GPL-2.0 */
// Copyright (C) 2019 Hangzhou C-SKY Microsystems co.,ltd.

// C dependencies:
// - <stdlib.h>
// - <linux/types.h>
// - "../../../../arch/csky/include/uapi/asm/perf_regs.h"

pub const PERF_REGS_MASK: u64 = ((1u64 << PERF_REG_CSKY_MAX) - 1);
pub const PERF_REGS_MAX: _ = PERF_REG_CSKY_MAX;
pub const PERF_SAMPLE_REGS_ABI: _ = PERF_SAMPLE_REGS_ABI_32;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
