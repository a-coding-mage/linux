/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/*
 * Processor flags
 *
 * Note: When adding a new flag, keep cpu_flags[] in
 * arch/sh/kernel/setup.c in sync so symbolic name
 * mapping of the processor flags has a chance of being
 * reasonably accurate.
 *
 * These flags are also available through the ELF
 * auxiliary vector as AT_HWCAP.
 */
pub const CPU_HAS_FPU: u32 = 0x0001; /* Hardware FPU support */
pub const CPU_HAS_P2_FLUSH_BUG: u32 = 0x0002; /* Need to flush the cache in P2 area */
pub const CPU_HAS_MMU_PAGE_ASSOC: u32 = 0x0004; /* SH3: TLB way selection bit support */
pub const CPU_HAS_DSP: u32 = 0x0008; /* SH-DSP: DSP support */
pub const CPU_HAS_PERF_COUNTER: u32 = 0x0010; /* Hardware performance counters */
pub const CPU_HAS_PTEA: u32 = 0x0020; /* PTEA register */
pub const CPU_HAS_LLSC: u32 = 0x0040; /* movli.l/movco.l */
pub const CPU_HAS_L2_CACHE: u32 = 0x0080; /* Secondary cache / URAM */
pub const CPU_HAS_OP32: u32 = 0x0100; /* 32-bit instruction support */
pub const CPU_HAS_PTEAEX: u32 = 0x0200; /* PTE ASID Extension support */
pub const CPU_HAS_CAS_L: u32 = 0x0400; /* cas.l atomic compare-and-swap */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
