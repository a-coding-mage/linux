/* SPDX-License-Identifier: GPL-2.0 */

// C header dependencies:
// - <stdlib.h>
// - <linux/types.h>
// - "../../../../arch/arm/include/uapi/asm/perf_regs.h"
// This translation expects u64, PERF_REG_ARM_MAX, and PERF_SAMPLE_REGS_ABI_32
// to be provided by the surrounding translated bindings.

extern "C" {
    pub fn perf_regs_load(regs: *mut u64);
}

pub const PERF_REGS_MASK: u64 = ((1u64 << PERF_REG_ARM_MAX) - 1);
pub const PERF_REGS_MAX: u32 = PERF_REG_ARM_MAX;
pub const PERF_SAMPLE_REGS_ABI: u32 = PERF_SAMPLE_REGS_ABI_32;
