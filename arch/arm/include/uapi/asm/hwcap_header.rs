/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/*
 * HWCAP flags - for elf_hwcap (in kernel) and AT_HWCAP
 */
pub const HWCAP_SWP: i32 = 1 << 0;
pub const HWCAP_HALF: i32 = 1 << 1;
pub const HWCAP_THUMB: i32 = 1 << 2;
pub const HWCAP_26BIT: i32 = 1 << 3; /* Play it safe */
pub const HWCAP_FAST_MULT: i32 = 1 << 4;
pub const HWCAP_FPA: i32 = 1 << 5;
pub const HWCAP_VFP: i32 = 1 << 6;
pub const HWCAP_EDSP: i32 = 1 << 7;
pub const HWCAP_JAVA: i32 = 1 << 8;
pub const HWCAP_IWMMXT: i32 = 1 << 9;
pub const HWCAP_CRUNCH: i32 = 1 << 10; /* Obsolete */
pub const HWCAP_THUMBEE: i32 = 1 << 11;
pub const HWCAP_NEON: i32 = 1 << 12;
pub const HWCAP_VFPv3: i32 = 1 << 13;
pub const HWCAP_VFPv3D16: i32 = 1 << 14; /* also set for VFPv4-D16 */
pub const HWCAP_TLS: i32 = 1 << 15;
pub const HWCAP_VFPv4: i32 = 1 << 16;
pub const HWCAP_IDIVA: i32 = 1 << 17;
pub const HWCAP_IDIVT: i32 = 1 << 18;
pub const HWCAP_VFPD32: i32 = 1 << 19; /* set if VFP has 32 regs (not 16) */
pub const HWCAP_IDIV: i32 = HWCAP_IDIVA | HWCAP_IDIVT;
pub const HWCAP_LPAE: i32 = 1 << 20;
pub const HWCAP_EVTSTRM: i32 = 1 << 21;
pub const HWCAP_FPHP: i32 = 1 << 22;
pub const HWCAP_ASIMDHP: i32 = 1 << 23;
pub const HWCAP_ASIMDDP: i32 = 1 << 24;
pub const HWCAP_ASIMDFHM: i32 = 1 << 25;
pub const HWCAP_ASIMDBF16: i32 = 1 << 26;
pub const HWCAP_I8MM: i32 = 1 << 27;

/*
 * HWCAP2 flags - for elf_hwcap2 (in kernel) and AT_HWCAP2
 */
pub const HWCAP2_AES: i32 = 1 << 0;
pub const HWCAP2_PMULL: i32 = 1 << 1;
pub const HWCAP2_SHA1: i32 = 1 << 2;
pub const HWCAP2_SHA2: i32 = 1 << 3;
pub const HWCAP2_CRC32: i32 = 1 << 4;
pub const HWCAP2_SB: i32 = 1 << 5;
pub const HWCAP2_SSBS: i32 = 1 << 6;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
