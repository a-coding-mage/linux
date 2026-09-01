/* SPDX-License-Identifier: GPL-2.0 */

/* C includes translated as external dependencies:
 * <stdlib.h>
 * <linux/types.h>
 * "../../../../arch/x86/include/uapi/asm/perf_regs.h"
 */

unsafe extern "C" {
    pub fn perf_regs_load(regs: *mut u64);
}

pub const PERF_REGS_MAX: u64 = PERF_REG_X86_XMM_MAX as u64;

/* Original C condition:
 * #ifndef HAVE_ARCH_X86_64_SUPPORT
 */
#[cfg(not(HAVE_ARCH_X86_64_SUPPORT))]
pub const PERF_REGS_MASK: u64 = (1u64 << (PERF_REG_X86_32_MAX as u32)) - 1;
#[cfg(not(HAVE_ARCH_X86_64_SUPPORT))]
pub const PERF_SAMPLE_REGS_ABI: u64 = PERF_SAMPLE_REGS_ABI_32 as u64;

#[cfg(HAVE_ARCH_X86_64_SUPPORT)]
pub const REG_NOSUPPORT: u64 = (1u64 << (PERF_REG_X86_DS as u32))
    | (1u64 << (PERF_REG_X86_ES as u32))
    | (1u64 << (PERF_REG_X86_FS as u32))
    | (1u64 << (PERF_REG_X86_GS as u32));
#[cfg(HAVE_ARCH_X86_64_SUPPORT)]
pub const PERF_REGS_MASK: u64 = (((1u64 << (PERF_REG_X86_64_MAX as u32)) - 1) & !REG_NOSUPPORT);
#[cfg(HAVE_ARCH_X86_64_SUPPORT)]
pub const PERF_SAMPLE_REGS_ABI: u64 = PERF_SAMPLE_REGS_ABI_64 as u64;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
