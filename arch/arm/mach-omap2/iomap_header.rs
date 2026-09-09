/*
 * IO mappings for OMAP2+
 *
 * IO definitions for TI OMAP processors and boards
 *
 * Copied from arch/arm/mach-sa1100/include/mach/io.h
 * Copyright (C) 1997-1999 Russell King
 *
 * Copyright (C) 2009-2012 Texas Instruments
 * Added OMAP4/5 support - Santosh Shilimkar <santosh.shilimkar@ti.com>
 *
 * This program is free software; you can redistribute it and/or modify it
 * under the terms of the GNU General Public License as published by the
 * Free Software Foundation; either version 2 of the License, or (at your
 * option) any later version.
 */

pub const OMAP2_L3_IO_OFFSET: u32 = 0x90000000;
macro_rules! OMAP2_L3_IO_ADDRESS { ($pa:expr) => { IOMEM(($pa) + OMAP2_L3_IO_OFFSET) }; }

pub const OMAP2_L4_IO_OFFSET: u32 = 0xb2000000;
macro_rules! OMAP2_L4_IO_ADDRESS { ($pa:expr) => { IOMEM(($pa) + OMAP2_L4_IO_OFFSET) }; }

pub const OMAP4_L3_IO_OFFSET: u32 = 0xb4000000;
macro_rules! OMAP4_L3_IO_ADDRESS { ($pa:expr) => { IOMEM(($pa) + OMAP4_L3_IO_OFFSET) }; }

pub const AM33XX_L4_WK_IO_OFFSET: u32 = 0xb5000000;
macro_rules! AM33XX_L4_WK_IO_ADDRESS { ($pa:expr) => { IOMEM(($pa) + AM33XX_L4_WK_IO_OFFSET) }; }

pub const OMAP4_L3_PER_IO_OFFSET: u32 = 0xb1100000;
macro_rules! OMAP4_L3_PER_IO_ADDRESS { ($pa:expr) => { IOMEM(($pa) + OMAP4_L3_PER_IO_OFFSET) }; }

pub const OMAP2_EMU_IO_OFFSET: u32 = 0xaa800000;
macro_rules! OMAP2_EMU_IO_ADDRESS { ($pa:expr) => { IOMEM(($pa) + OMAP2_EMU_IO_OFFSET) }; }

/* Omap2 specific IO mapping */
pub const L3_24XX_PHYS: u32 = L3_24XX_BASE;
pub const L3_24XX_VIRT: u32 = L3_24XX_PHYS + OMAP2_L3_IO_OFFSET;
pub const L3_24XX_SIZE: u32 = SZ_1M;
pub const L4_24XX_PHYS: u32 = L4_24XX_BASE;
pub const L4_24XX_VIRT: u32 = L4_24XX_PHYS + OMAP2_L4_IO_OFFSET;
pub const L4_24XX_SIZE: u32 = SZ_1M;
pub const L4_WK_243X_PHYS: u32 = L4_WK_243X_BASE;
pub const L4_WK_243X_VIRT: u32 = L4_WK_243X_PHYS + OMAP2_L4_IO_OFFSET;
pub const L4_WK_243X_SIZE: u32 = SZ_1M;
pub const OMAP243X_GPMC_PHYS: u32 = OMAP243X_GPMC_BASE;
pub const OMAP243X_GPMC_VIRT: u32 = OMAP243X_GPMC_PHYS + OMAP2_L3_IO_OFFSET;
pub const OMAP243X_GPMC_SIZE: u32 = SZ_1M;
pub const OMAP243X_SDRC_PHYS: u32 = OMAP243X_SDRC_BASE;
pub const OMAP243X_SDRC_VIRT: u32 = OMAP243X_SDRC_PHYS + OMAP2_L3_IO_OFFSET;
pub const OMAP243X_SDRC_SIZE: u32 = SZ_1M;
pub const OMAP243X_SMS_PHYS: u32 = OMAP243X_SMS_BASE;
pub const OMAP243X_SMS_VIRT: u32 = OMAP243X_SMS_PHYS + OMAP2_L3_IO_OFFSET;
pub const OMAP243X_SMS_SIZE: u32 = SZ_1M;

/* 2420 IVA */
pub const DSP_MEM_2420_PHYS: u32 = OMAP2420_DSP_MEM_BASE;
pub const DSP_MEM_2420_VIRT: u32 = 0xfc100000;
pub const DSP_MEM_2420_SIZE: u32 = 0x28000;
pub const DSP_IPI_2420_PHYS: u32 = OMAP2420_DSP_IPI_BASE;
pub const DSP_IPI_2420_VIRT: u32 = 0xfc128000;
pub const DSP_IPI_2420_SIZE: u32 = SZ_4K;
pub const DSP_MMU_2420_PHYS: u32 = OMAP2420_DSP_MMU_BASE;
pub const DSP_MMU_2420_VIRT: u32 = 0xfc129000;
pub const DSP_MMU_2420_SIZE: u32 = SZ_4K;

/* 2430 IVA2.1 - currently unmapped */

/* Omap3 specific IO mapping */
pub const L3_34XX_PHYS: u32 = L3_34XX_BASE;
pub const L3_34XX_VIRT: u32 = L3_34XX_PHYS + OMAP2_L3_IO_OFFSET;
pub const L3_34XX_SIZE: u32 = SZ_1M;
pub const L4_34XX_PHYS: u32 = L4_34XX_BASE;
pub const L4_34XX_VIRT: u32 = L4_34XX_PHYS + OMAP2_L4_IO_OFFSET;
pub const L4_34XX_SIZE: u32 = SZ_4M;

/* AM33XX specific IO mapping */
pub const L4_WK_AM33XX_PHYS: u32 = L4_WK_AM33XX_BASE;
pub const L4_WK_AM33XX_VIRT: u32 = L4_WK_AM33XX_PHYS + AM33XX_L4_WK_IO_OFFSET;
pub const L4_WK_AM33XX_SIZE: u32 = SZ_4M;

/* Need to look at the Size 4M for L4. VPOM3430 was not working for Int controller */
pub const L4_PER_34XX_PHYS: u32 = L4_PER_34XX_BASE;
pub const L4_PER_34XX_VIRT: u32 = L4_PER_34XX_PHYS + OMAP2_L4_IO_OFFSET;
pub const L4_PER_34XX_SIZE: u32 = SZ_1M;
pub const L4_EMU_34XX_PHYS: u32 = L4_EMU_34XX_BASE;
pub const L4_EMU_34XX_VIRT: u32 = L4_EMU_34XX_PHYS + OMAP2_EMU_IO_OFFSET;
pub const L4_EMU_34XX_SIZE: u32 = SZ_8M;
pub const OMAP34XX_GPMC_PHYS: u32 = OMAP34XX_GPMC_BASE;
pub const OMAP34XX_GPMC_VIRT: u32 = OMAP34XX_GPMC_PHYS + OMAP2_L3_IO_OFFSET;
pub const OMAP34XX_GPMC_SIZE: u32 = SZ_1M;
pub const OMAP343X_SMS_PHYS: u32 = OMAP343X_SMS_BASE;
pub const OMAP343X_SMS_VIRT: u32 = OMAP343X_SMS_PHYS + OMAP2_L3_IO_OFFSET;
pub const OMAP343X_SMS_SIZE: u32 = SZ_1M;
pub const OMAP343X_SDRC_PHYS: u32 = OMAP343X_SDRC_BASE;
pub const OMAP343X_SDRC_VIRT: u32 = OMAP343X_SDRC_PHYS + OMAP2_L3_IO_OFFSET;
pub const OMAP343X_SDRC_SIZE: u32 = SZ_1M;

/* 3430 IVA - currently unmapped */

/* Omap4 specific IO mapping */
pub const L3_44XX_PHYS: u32 = L3_44XX_BASE;
pub const L3_44XX_VIRT: u32 = L3_44XX_PHYS + OMAP4_L3_IO_OFFSET;
pub const L3_44XX_SIZE: u32 = SZ_1M;
pub const L4_44XX_PHYS: u32 = L4_44XX_BASE;
pub const L4_44XX_VIRT: u32 = L4_44XX_PHYS + OMAP2_L4_IO_OFFSET;
pub const L4_44XX_SIZE: u32 = SZ_4M;
pub const L4_PER_44XX_PHYS: u32 = L4_PER_44XX_BASE;
pub const L4_PER_44XX_VIRT: u32 = L4_PER_44XX_PHYS + OMAP2_L4_IO_OFFSET;
pub const L4_PER_44XX_SIZE: u32 = SZ_4M;
pub const L4_ABE_44XX_PHYS: u32 = L4_ABE_44XX_BASE;
pub const L4_ABE_44XX_VIRT: u32 = L4_ABE_44XX_PHYS + OMAP2_L4_IO_OFFSET;
pub const L4_ABE_44XX_SIZE: u32 = SZ_1M;

/* Omap5 specific IO mapping */
pub const L3_54XX_PHYS: u32 = L3_54XX_BASE;
pub const L3_54XX_VIRT: u32 = L3_54XX_PHYS + OMAP4_L3_IO_OFFSET;
pub const L3_54XX_SIZE: u32 = SZ_1M;
pub const L4_54XX_PHYS: u32 = L4_54XX_BASE;
pub const L4_54XX_VIRT: u32 = L4_54XX_PHYS + OMAP2_L4_IO_OFFSET;
pub const L4_54XX_SIZE: u32 = SZ_4M;
pub const L4_WK_54XX_PHYS: u32 = L4_WK_54XX_BASE;
pub const L4_WK_54XX_VIRT: u32 = L4_WK_54XX_PHYS + OMAP2_L4_IO_OFFSET;
pub const L4_WK_54XX_SIZE: u32 = SZ_2M;
pub const L4_PER_54XX_PHYS: u32 = L4_PER_54XX_BASE;
pub const L4_PER_54XX_VIRT: u32 = L4_PER_54XX_PHYS + OMAP2_L4_IO_OFFSET;
pub const L4_PER_54XX_SIZE: u32 = SZ_4M;

/* DRA7xx specific IO mapping */
pub const L3_MAIN_SN_DRA7XX_PHYS: u32 = L3_MAIN_SN_DRA7XX_BASE;
pub const L3_MAIN_SN_DRA7XX_VIRT: u32 = L3_MAIN_SN_DRA7XX_PHYS + OMAP4_L3_IO_OFFSET;
pub const L3_MAIN_SN_DRA7XX_SIZE: u32 = SZ_1M;
pub const L4_PER1_DRA7XX_PHYS: u32 = L4_PER1_DRA7XX_BASE;
pub const L4_PER1_DRA7XX_VIRT: u32 = L4_PER1_DRA7XX_PHYS + OMAP2_L4_IO_OFFSET;
pub const L4_PER1_DRA7XX_SIZE: u32 = SZ_1M;
pub const L4_CFG_MPU_DRA7XX_PHYS: u32 = L4_CFG_MPU_DRA7XX_BASE;
pub const L4_CFG_MPU_DRA7XX_VIRT: u32 = L4_CFG_MPU_DRA7XX_PHYS + OMAP2_L4_IO_OFFSET;
pub const L4_CFG_MPU_DRA7XX_SIZE: u32 = SZ_1M;
pub const L4_PER2_DRA7XX_PHYS: u32 = L4_PER2_DRA7XX_BASE;
pub const L4_PER2_DRA7XX_VIRT: u32 = L4_PER2_DRA7XX_PHYS + OMAP2_L4_IO_OFFSET;
pub const L4_PER2_DRA7XX_SIZE: u32 = SZ_1M;
pub const L4_PER3_DRA7XX_PHYS: u32 = L4_PER3_DRA7XX_BASE;
pub const L4_PER3_DRA7XX_VIRT: u32 = L4_PER3_DRA7XX_PHYS + OMAP2_L4_IO_OFFSET;
pub const L4_PER3_DRA7XX_SIZE: u32 = SZ_2M;
pub const L4_CFG_DRA7XX_PHYS: u32 = L4_CFG_DRA7XX_BASE;
pub const L4_CFG_DRA7XX_VIRT: u32 = L4_CFG_DRA7XX_PHYS + OMAP2_L4_IO_OFFSET;
pub const L4_CFG_DRA7XX_SIZE: u32 = SZ_1M + SZ_2M;
pub const L4_WKUP_DRA7XX_PHYS: u32 = L4_WKUP_DRA7XX_BASE;
pub const L4_WKUP_DRA7XX_VIRT: u32 = L4_WKUP_DRA7XX_PHYS + OMAP2_L4_IO_OFFSET;
pub const L4_WKUP_DRA7XX_SIZE: u32 = SZ_1M;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
