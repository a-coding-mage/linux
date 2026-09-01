/* SPDX-License-Identifier: GPL-2.0 */

/* C dependencies: <stdlib.h>, <linux/types.h>. */
/* C include alias intent: perf_event_arm_regs was defined as perf_event_arm64_regs
 * while including ../../../../arch/arm64/include/uapi/asm/perf_regs.h. */

unsafe extern "C" {
    pub fn perf_regs_load(regs: *mut u64);
}

pub const PERF_REGS_MASK: u64 = ((1u64 << PERF_REG_ARM64_MAX) - 1);
pub const PERF_REGS_MAX: u32 = PERF_REG_ARM64_MAX;
pub const PERF_SAMPLE_REGS_ABI: u32 = PERF_SAMPLE_REGS_ABI_64;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
