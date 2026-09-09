/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Clock domain register offsets for TI81XX.
 *
 * Copyright (C) 2010 Texas Instruments, Inc. - https://www.ti.com/
 * Copyright (C) 2013 SKTB SKiT, http://www.skitlab.ru/
 */

/* TI81XX common CM module offsets */
pub const TI81XX_CM_ACTIVE_MOD: u32 = 0x0400; /* 256B */
pub const TI81XX_CM_DEFAULT_MOD: u32 = 0x0500; /* 256B */
pub const TI81XX_CM_ALWON_MOD: u32 = 0x1400; /* 1KB */
pub const TI81XX_CM_SGX_MOD: u32 = 0x0900; /* 256B */

/* TI816X CM module offsets */
pub const TI816X_CM_IVAHD0_MOD: u32 = 0x0600; /* 256B */
pub const TI816X_CM_IVAHD1_MOD: u32 = 0x0700; /* 256B */
pub const TI816X_CM_IVAHD2_MOD: u32 = 0x0800; /* 256B */

/* ALWON */
pub const TI81XX_CM_ALWON_L3_SLOW_CLKDM: u32 = 0x0000;
pub const TI81XX_CM_ALWON_L3_MED_CLKDM: u32 = 0x0004;
pub const TI81XX_CM_ETHERNET_CLKDM: u32 = 0x0004;
pub const TI81XX_CM_MMU_CLKDM: u32 = 0x000C;
pub const TI81XX_CM_MMUCFG_CLKDM: u32 = 0x0010;
pub const TI81XX_CM_ALWON_MPU_CLKDM: u32 = 0x001C;
pub const TI81XX_CM_ALWON_L3_FAST_CLKDM: u32 = 0x0030;

/* ACTIVE */
pub const TI816X_CM_ACTIVE_GEM_CLKDM: u32 = 0x0000;

/* IVAHD0 */
pub const TI816X_CM_IVAHD0_CLKDM: u32 = 0x0000;

/* IVAHD1 */
pub const TI816X_CM_IVAHD1_CLKDM: u32 = 0x0000;

/* IVAHD2 */
pub const TI816X_CM_IVAHD2_CLKDM: u32 = 0x0000;

/* SGX */
pub const TI816X_CM_SGX_CLKDM: u32 = 0x0000;

/* DEFAULT */
pub const TI816X_CM_DEFAULT_L3_MED_CLKDM: u32 = 0x0004;
pub const TI816X_CM_DEFAULT_PCI_CLKDM: u32 = 0x0010;
pub const TI816X_CM_DEFAULT_L3_SLOW_CLKDM: u32 = 0x0014;
pub const TI816X_CM_DEFAULT_DUCATI_CLKDM: u32 = 0x0018;
pub const TI816X_CM_DEFAULT_SATA_CLKDM: u32 = 0x0060;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
