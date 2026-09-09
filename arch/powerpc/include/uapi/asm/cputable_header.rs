/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/* in AT_HWCAP */
pub const PPC_FEATURE_32: u32 = 0x80000000;
pub const PPC_FEATURE_64: u32 = 0x40000000;
pub const PPC_FEATURE_601_INSTR: u32 = 0x20000000;
pub const PPC_FEATURE_HAS_ALTIVEC: u32 = 0x10000000;
pub const PPC_FEATURE_HAS_FPU: u32 = 0x08000000;
pub const PPC_FEATURE_HAS_MMU: u32 = 0x04000000;
pub const PPC_FEATURE_HAS_4xxMAC: u32 = 0x02000000;
pub const PPC_FEATURE_UNIFIED_CACHE: u32 = 0x01000000;
pub const PPC_FEATURE_HAS_SPE: u32 = 0x00800000;
pub const PPC_FEATURE_HAS_EFP_SINGLE: u32 = 0x00400000;
pub const PPC_FEATURE_HAS_EFP_DOUBLE: u32 = 0x00200000;
pub const PPC_FEATURE_NO_TB: u32 = 0x00100000;
pub const PPC_FEATURE_POWER4: u32 = 0x00080000;
pub const PPC_FEATURE_POWER5: u32 = 0x00040000;
pub const PPC_FEATURE_POWER5_PLUS: u32 = 0x00020000;
pub const PPC_FEATURE_CELL: u32 = 0x00010000;
pub const PPC_FEATURE_BOOKE: u32 = 0x00008000;
pub const PPC_FEATURE_SMT: u32 = 0x00004000;
pub const PPC_FEATURE_ICACHE_SNOOP: u32 = 0x00002000;
pub const PPC_FEATURE_ARCH_2_05: u32 = 0x00001000;
pub const PPC_FEATURE_PA6T: u32 = 0x00000800;
pub const PPC_FEATURE_HAS_DFP: u32 = 0x00000400;
pub const PPC_FEATURE_POWER6_EXT: u32 = 0x00000200;
pub const PPC_FEATURE_ARCH_2_06: u32 = 0x00000100;
pub const PPC_FEATURE_HAS_VSX: u32 = 0x00000080;

pub const PPC_FEATURE_PSERIES_PERFMON_COMPAT: u32 = 0x00000040;

/* Reserved - do not use        0x00000004 */
pub const PPC_FEATURE_TRUE_LE: u32 = 0x00000002;
pub const PPC_FEATURE_PPC_LE: u32 = 0x00000001;

/* in AT_HWCAP2 */
pub const PPC_FEATURE2_ARCH_2_07: u32 = 0x80000000;
pub const PPC_FEATURE2_HTM: u32 = 0x40000000;
pub const PPC_FEATURE2_DSCR: u32 = 0x20000000;
pub const PPC_FEATURE2_EBB: u32 = 0x10000000;
pub const PPC_FEATURE2_ISEL: u32 = 0x08000000;
pub const PPC_FEATURE2_TAR: u32 = 0x04000000;
pub const PPC_FEATURE2_VEC_CRYPTO: u32 = 0x02000000;
pub const PPC_FEATURE2_HTM_NOSC: u32 = 0x01000000;
pub const PPC_FEATURE2_ARCH_3_00: u32 = 0x00800000; /* ISA 3.00 */
pub const PPC_FEATURE2_HAS_IEEE128: u32 = 0x00400000; /* VSX IEEE Binary Float 128-bit */
pub const PPC_FEATURE2_DARN: u32 = 0x00200000; /* darn random number insn */
pub const PPC_FEATURE2_SCV: u32 = 0x00100000; /* scv syscall */
pub const PPC_FEATURE2_HTM_NO_SUSPEND: u32 = 0x00080000; /* TM w/out suspended state */
pub const PPC_FEATURE2_ARCH_3_1: u32 = 0x00040000; /* ISA 3.1 */
pub const PPC_FEATURE2_MMA: u32 = 0x00020000; /* Matrix Multiply Assist */
pub const PPC_FEATURE2_ARCH_3_2: u32 = 0x00010000; /* ISA 3.2 */
pub const PPC_FEATURE2_DMF: u32 = 0x00008000; /* Dense Math Facility */

/*
 * IMPORTANT!
 * All future PPC_FEATURE definitions should be allocated in cooperation with
 * OPAL / skiboot firmware, in accordance with the ibm,powerpc-cpu-features
 * device tree binding.
 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
