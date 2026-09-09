/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/* HWCAP flags */
pub const HWCAP_MIPS_R6: u32 = 1u32 << 0;
pub const HWCAP_MIPS_MSA: u32 = 1u32 << 1;
pub const HWCAP_MIPS_CRC32: u32 = 1u32 << 2;
pub const HWCAP_MIPS_MIPS16: u32 = 1u32 << 3;
pub const HWCAP_MIPS_MDMX: u32 = 1u32 << 4;
pub const HWCAP_MIPS_MIPS3D: u32 = 1u32 << 5;
pub const HWCAP_MIPS_SMARTMIPS: u32 = 1u32 << 6;
pub const HWCAP_MIPS_DSP: u32 = 1u32 << 7;
pub const HWCAP_MIPS_DSP2: u32 = 1u32 << 8;
pub const HWCAP_MIPS_DSP3: u32 = 1u32 << 9;
pub const HWCAP_MIPS_MIPS16E2: u32 = 1u32 << 10;
pub const HWCAP_LOONGSON_MMI: u32 = 1u32 << 11;
pub const HWCAP_LOONGSON_EXT: u32 = 1u32 << 12;
pub const HWCAP_LOONGSON_EXT2: u32 = 1u32 << 13;
pub const HWCAP_LOONGSON_CPUCFG: u32 = 1u32 << 14;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
