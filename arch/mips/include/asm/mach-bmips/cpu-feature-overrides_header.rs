/* SPDX-License-Identifier: GPL-2.0 */
// Header guard: __ASM_MACH_BMIPS_CPU_FEATURE_OVERRIDES_H

/* Invariants across all BMIPS processors */
pub const cpu_has_vtag_icache: i32 = 0;
pub const cpu_icache_snoops_remote_store: i32 = 1;

/* Processor ISA compatibility is MIPS32R1 */
pub const cpu_has_mips32r1: i32 = 1;
pub const cpu_has_mips32r2: i32 = 0;
pub const cpu_has_mips64r1: i32 = 0;
pub const cpu_has_mips64r2: i32 = 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
