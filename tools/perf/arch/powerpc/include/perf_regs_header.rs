/* SPDX-License-Identifier: GPL-2.0 */

/* Dependencies from the original header:
 * - linux/types.h
 * - arch/powerpc/include/uapi/asm/perf_regs.h
 */

unsafe extern "C" {
    pub fn perf_regs_load(regs: *mut u64);
}

pub const PERF_REGS_MASK: u64 = (1u64 << PERF_REG_POWERPC_MAX) - 1;
pub const PERF_REGS_MAX: u32 = PERF_REG_POWERPC_MAX;

#[cfg(target_arch = "powerpc64")]
pub const PERF_SAMPLE_REGS_ABI: u32 = PERF_SAMPLE_REGS_ABI_64;

#[cfg(not(target_arch = "powerpc64"))]
pub const PERF_SAMPLE_REGS_ABI: u32 = PERF_SAMPLE_REGS_ABI_32;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
