// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2019 Hangzhou C-SKY Microsystems co.,ltd.

// C header dependencies:
// - <stdlib.h>
// - <linux/types.h>
// - ../../../../arch/riscv/include/uapi/asm/perf_regs.h

pub const PERF_REGS_MASK: u64 = (1u64 << PERF_REG_RISCV_MAX) - 1;
pub const PERF_REGS_MAX: u32 = PERF_REG_RISCV_MAX;

// Original C selects this with __riscv_xlen when it is defined:
// - __riscv_xlen == 64: PERF_SAMPLE_REGS_ABI_64
// - otherwise:          PERF_SAMPLE_REGS_ABI_32
// If __riscv_xlen is not defined, it uses PERF_SAMPLE_REGS_NONE.
#[cfg(target_pointer_width = "64")]
pub const PERF_SAMPLE_REGS_ABI: u32 = PERF_SAMPLE_REGS_ABI_64;

#[cfg(not(target_pointer_width = "64"))]
pub const PERF_SAMPLE_REGS_ABI: u32 = PERF_SAMPLE_REGS_ABI_32;
