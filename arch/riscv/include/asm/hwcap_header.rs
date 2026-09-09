/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copied from arch/arm64/include/asm/hwcap.h
 *
 * Copyright (C) 2012 ARM Ltd.
 * Copyright (C) 2017 SiFive
 */

// Dependency intent: declarations from <uapi/asm/hwcap.h> are supplied elsewhere.

pub const RISCV_ISA_EXT_A: usize = 'a' as usize - 'a' as usize;
pub const RISCV_ISA_EXT_C: usize = 'c' as usize - 'a' as usize;
pub const RISCV_ISA_EXT_D: usize = 'd' as usize - 'a' as usize;
pub const RISCV_ISA_EXT_F: usize = 'f' as usize - 'a' as usize;
pub const RISCV_ISA_EXT_H: usize = 'h' as usize - 'a' as usize;
pub const RISCV_ISA_EXT_I: usize = 'i' as usize - 'a' as usize;
pub const RISCV_ISA_EXT_M: usize = 'm' as usize - 'a' as usize;
pub const RISCV_ISA_EXT_Q: usize = 'q' as usize - 'a' as usize;
pub const RISCV_ISA_EXT_V: usize = 'v' as usize - 'a' as usize;

/*
 * These macros represent the logical IDs of each multi-letter RISC-V ISA
 * extension and are used in the ISA bitmap. The logical IDs start from
 * RISCV_ISA_EXT_BASE, which allows the 0-25 range to be reserved for single
 * letter extensions. The maximum, RISCV_ISA_EXT_MAX, is defined in order
 * to allocate the bitmap and may be increased when necessary.
 *
 * New extensions should just be added to the bottom, rather than added
 * alphabetically, in order to avoid unnecessary shuffling.
 */
pub const RISCV_ISA_EXT_BASE: usize = 26;

pub const RISCV_ISA_EXT_SSCOFPMF: usize = 26;
pub const RISCV_ISA_EXT_SSTC: usize = 27;
pub const RISCV_ISA_EXT_SVINVAL: usize = 28;
pub const RISCV_ISA_EXT_SVPBMT: usize = 29;
pub const RISCV_ISA_EXT_ZBB: usize = 30;
pub const RISCV_ISA_EXT_ZICBOM: usize = 31;
pub const RISCV_ISA_EXT_ZIHINTPAUSE: usize = 32;
pub const RISCV_ISA_EXT_SVNAPOT: usize = 33;
pub const RISCV_ISA_EXT_ZICBOZ: usize = 34;
pub const RISCV_ISA_EXT_SMAIA: usize = 35;
pub const RISCV_ISA_EXT_SSAIA: usize = 36;
pub const RISCV_ISA_EXT_ZBA: usize = 37;
pub const RISCV_ISA_EXT_ZBS: usize = 38;
pub const RISCV_ISA_EXT_ZICNTR: usize = 39;
pub const RISCV_ISA_EXT_ZICSR: usize = 40;
pub const RISCV_ISA_EXT_ZIFENCEI: usize = 41;
pub const RISCV_ISA_EXT_ZIHPM: usize = 42;
pub const RISCV_ISA_EXT_SMSTATEEN: usize = 43;
pub const RISCV_ISA_EXT_ZICOND: usize = 44;
pub const RISCV_ISA_EXT_ZBC: usize = 45;
pub const RISCV_ISA_EXT_ZBKB: usize = 46;
pub const RISCV_ISA_EXT_ZBKC: usize = 47;
pub const RISCV_ISA_EXT_ZBKX: usize = 48;
pub const RISCV_ISA_EXT_ZKND: usize = 49;
pub const RISCV_ISA_EXT_ZKNE: usize = 50;
pub const RISCV_ISA_EXT_ZKNH: usize = 51;
pub const RISCV_ISA_EXT_ZKR: usize = 52;
pub const RISCV_ISA_EXT_ZKSED: usize = 53;
pub const RISCV_ISA_EXT_ZKSH: usize = 54;
pub const RISCV_ISA_EXT_ZKT: usize = 55;
pub const RISCV_ISA_EXT_ZVBB: usize = 56;
pub const RISCV_ISA_EXT_ZVBC: usize = 57;
pub const RISCV_ISA_EXT_ZVKB: usize = 58;
pub const RISCV_ISA_EXT_ZVKG: usize = 59;
pub const RISCV_ISA_EXT_ZVKNED: usize = 60;
pub const RISCV_ISA_EXT_ZVKNHA: usize = 61;
pub const RISCV_ISA_EXT_ZVKNHB: usize = 62;
pub const RISCV_ISA_EXT_ZVKSED: usize = 63;
pub const RISCV_ISA_EXT_ZVKSH: usize = 64;
pub const RISCV_ISA_EXT_ZVKT: usize = 65;
pub const RISCV_ISA_EXT_ZFH: usize = 66;
pub const RISCV_ISA_EXT_ZFHMIN: usize = 67;
pub const RISCV_ISA_EXT_ZIHINTNTL: usize = 68;
pub const RISCV_ISA_EXT_ZVFH: usize = 69;
pub const RISCV_ISA_EXT_ZVFHMIN: usize = 70;
pub const RISCV_ISA_EXT_ZFA: usize = 71;
pub const RISCV_ISA_EXT_ZTSO: usize = 72;
pub const RISCV_ISA_EXT_ZACAS: usize = 73;
pub const RISCV_ISA_EXT_ZVE32X: usize = 74;
pub const RISCV_ISA_EXT_ZVE32F: usize = 75;
pub const RISCV_ISA_EXT_ZVE64X: usize = 76;
pub const RISCV_ISA_EXT_ZVE64F: usize = 77;
pub const RISCV_ISA_EXT_ZVE64D: usize = 78;
pub const RISCV_ISA_EXT_ZIMOP: usize = 79;
pub const RISCV_ISA_EXT_ZCA: usize = 80;
pub const RISCV_ISA_EXT_ZCB: usize = 81;
pub const RISCV_ISA_EXT_ZCD: usize = 82;
pub const RISCV_ISA_EXT_ZCF: usize = 83;
pub const RISCV_ISA_EXT_ZCMOP: usize = 84;
pub const RISCV_ISA_EXT_ZAWRS: usize = 85;
pub const RISCV_ISA_EXT_SVVPTC: usize = 86;
pub const RISCV_ISA_EXT_SMMPM: usize = 87;
pub const RISCV_ISA_EXT_SMNPM: usize = 88;
pub const RISCV_ISA_EXT_SSNPM: usize = 89;
pub const RISCV_ISA_EXT_ZABHA: usize = 90;
pub const RISCV_ISA_EXT_ZICCRSE: usize = 91;
pub const RISCV_ISA_EXT_SVADE: usize = 92;
pub const RISCV_ISA_EXT_SVADU: usize = 93;
pub const RISCV_ISA_EXT_ZFBFMIN: usize = 94;
pub const RISCV_ISA_EXT_ZVFBFMIN: usize = 95;
pub const RISCV_ISA_EXT_ZVFBFWMA: usize = 96;
pub const RISCV_ISA_EXT_ZAAMO: usize = 97;
pub const RISCV_ISA_EXT_ZALRSC: usize = 98;
pub const RISCV_ISA_EXT_ZICBOP: usize = 99;
pub const RISCV_ISA_EXT_SVRSW60T59B: usize = 100;
pub const RISCV_ISA_EXT_ZALASR: usize = 101;
pub const RISCV_ISA_EXT_ZILSD: usize = 102;
pub const RISCV_ISA_EXT_ZCLSD: usize = 103;
pub const RISCV_ISA_EXT_ZICFILP: usize = 104;
pub const RISCV_ISA_EXT_ZICFISS: usize = 105;
pub const RISCV_ISA_EXT_SSCSRIND: usize = 106;
pub const RISCV_ISA_EXT_SMCSRIND: usize = 107;
pub const RISCV_ISA_EXT_SMCNTRPMF: usize = 108;
pub const RISCV_ISA_EXT_SSCCFG: usize = 109;
pub const RISCV_ISA_EXT_SMCDELEG: usize = 110;
pub const RISCV_ISA_EXT_SSQOSID: usize = 111;
pub const RISCV_ISA_EXT_ZICCLSM: usize = 112;
pub const RISCV_ISA_EXT_ZICCAMOA: usize = 113;
pub const RISCV_ISA_EXT_ZICCIF: usize = 114;
pub const RISCV_ISA_EXT_ZA64RS: usize = 115;

pub const RISCV_ISA_EXT_XLINUXENVCFG: usize = 127;
pub const RISCV_ISA_EXT_MAX: usize = 128;
pub const RISCV_ISA_EXT_INVALID: u32 = u32::MAX;

#[cfg(feature = "CONFIG_RISCV_M_MODE")]
pub const RISCV_ISA_EXT_SxAIA: usize = RISCV_ISA_EXT_SMAIA;
#[cfg(feature = "CONFIG_RISCV_M_MODE")]
pub const RISCV_ISA_EXT_SUPM: usize = RISCV_ISA_EXT_SMNPM;
#[cfg(feature = "CONFIG_RISCV_M_MODE")]
pub const RISCV_ISA_EXT_SxCSRIND: usize = RISCV_ISA_EXT_SMCSRIND;

#[cfg(not(feature = "CONFIG_RISCV_M_MODE"))]
pub const RISCV_ISA_EXT_SxAIA: usize = RISCV_ISA_EXT_SSAIA;
#[cfg(not(feature = "CONFIG_RISCV_M_MODE"))]
pub const RISCV_ISA_EXT_SUPM: usize = RISCV_ISA_EXT_SSNPM;
#[cfg(not(feature = "CONFIG_RISCV_M_MODE"))]
pub const RISCV_ISA_EXT_SxCSRIND: usize = RISCV_ISA_EXT_SSCSRIND;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
