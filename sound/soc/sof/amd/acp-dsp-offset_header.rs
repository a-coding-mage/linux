/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause) */
/*
 * This file is provided under a dual BSD/GPLv2 license. When using or
 * redistributing this file, you may do so under either license.
 *
 * Copyright(c) 2021, 2023, 2024 Advanced Micro Devices, Inc. All rights reserved.
 *
 * Author: Ajit Kumar Pandey <AjitKumar.Pandey@amd.com>
 */

/* Registers from ACP_DMA_0 block */
pub const ACP_DMA_CNTL_0: u32 = 0x00;
pub const ACP_DMA_DSCR_STRT_IDX_0: u32 = 0x20;
pub const ACP_DMA_DSCR_CNT_0: u32 = 0x40;
pub const ACP_DMA_PRIO_0: u32 = 0x60;
pub const ACP_DMA_CUR_DSCR_0: u32 = 0x80;
pub const ACP_DMA_ERR_STS_0: u32 = 0xC0;
pub const ACP_DMA_DESC_BASE_ADDR: u32 = 0xE0;
pub const ACP_DMA_DESC_MAX_NUM_DSCR: u32 = 0xE4;
pub const ACP_DMA_CH_STS: u32 = 0xE8;
pub const ACP_DMA_CH_GROUP: u32 = 0xEC;
pub const ACP_DMA_CH_RST_STS: u32 = 0xF0;
pub const ACP70_DMA_CNTL_0: u32 = 0x00;
pub const ACP70_DMA_DSCR_STRT_IDX_0: u32 = 0x28;
pub const ACP70_DMA_DSCR_CNT_0: u32 = 0x50;
pub const ACP70_DMA_PRIO_0: u32 = 0x78;
pub const ACP70_DMA_CUR_DSCR_0: u32 = 0xA0;
pub const ACP70_DMA_ERR_STS_0: u32 = 0xF0;
pub const ACP70_DMA_DESC_BASE_ADDR: u32 = 0x118;
pub const ACP70_DMA_DESC_MAX_NUM_DSCR: u32 = 0x11C;
pub const ACP70_DMA_CH_STS: u32 = 0x120;
pub const ACP70_DMA_CH_GROUP: u32 = 0x124;
pub const ACP70_DMA_CH_RST_STS: u32 = 0x128;

/* Registers from ACP_DSP_0 block */
pub const ACP_DSP0_RUNSTALL: u32 = 0x414;

/* Registers from ACP_AXI2AXIATU block */
pub const ACPAXI2AXI_ATU_PAGE_SIZE_GRP_1: u32 = 0xC00;
pub const ACPAXI2AXI_ATU_BASE_ADDR_GRP_1: u32 = 0xC04;
pub const ACPAXI2AXI_ATU_PAGE_SIZE_GRP_2: u32 = 0xC08;
pub const ACPAXI2AXI_ATU_BASE_ADDR_GRP_2: u32 = 0xC0C;
pub const ACPAXI2AXI_ATU_PAGE_SIZE_GRP_3: u32 = 0xC10;
pub const ACPAXI2AXI_ATU_BASE_ADDR_GRP_3: u32 = 0xC14;
pub const ACPAXI2AXI_ATU_PAGE_SIZE_GRP_4: u32 = 0xC18;
pub const ACPAXI2AXI_ATU_BASE_ADDR_GRP_4: u32 = 0xC1C;
pub const ACPAXI2AXI_ATU_PAGE_SIZE_GRP_5: u32 = 0xC20;
pub const ACPAXI2AXI_ATU_BASE_ADDR_GRP_5: u32 = 0xC24;
pub const ACPAXI2AXI_ATU_PAGE_SIZE_GRP_6: u32 = 0xC28;
pub const ACPAXI2AXI_ATU_BASE_ADDR_GRP_6: u32 = 0xC2C;
pub const ACPAXI2AXI_ATU_PAGE_SIZE_GRP_7: u32 = 0xC30;
pub const ACPAXI2AXI_ATU_BASE_ADDR_GRP_7: u32 = 0xC34;
pub const ACPAXI2AXI_ATU_PAGE_SIZE_GRP_8: u32 = 0xC38;
pub const ACPAXI2AXI_ATU_BASE_ADDR_GRP_8: u32 = 0xC3C;
pub const ACPAXI2AXI_ATU_CTRL: u32 = 0xC40;
pub const ACP_SOFT_RESET: u32 = 0x1000;
pub const ACP_CONTROL: u32 = 0x1004;

pub const ACP3X_I2S_PIN_CONFIG: u32 = 0x1400;
pub const ACP5X_I2S_PIN_CONFIG: u32 = 0x1400;
pub const ACP6X_I2S_PIN_CONFIG: u32 = 0x1440;

/* Registers offsets from ACP_PGFSM block */
pub const ACP3X_PGFSM_BASE: u32 = 0x141C;
pub const ACP5X_PGFSM_BASE: u32 = 0x1424;
pub const ACP6X_PGFSM_BASE: u32 = 0x1024;
pub const ACP70_PGFSM_BASE: u32 = ACP6X_PGFSM_BASE;
pub const ACP7X_PGFSM_BASE: u32 = ACP6X_PGFSM_BASE;
pub const PGFSM_CONTROL_OFFSET: u32 = 0x0;
pub const PGFSM_STATUS_OFFSET: u32 = 0x4;
pub const ACP3X_CLKMUX_SEL: u32 = 0x1424;
pub const ACP5X_CLKMUX_SEL: u32 = 0x142C;
pub const ACP6X_CLKMUX_SEL: u32 = 0x102C;
pub const ACP70_CLKMUX_SEL: u32 = ACP6X_CLKMUX_SEL;
pub const ACP7X_CLKMUX_SEL: u32 = ACP6X_CLKMUX_SEL;

/* Registers from ACP_INTR block */
pub const ACP3X_EXT_INTR_STAT: u32 = 0x1808;
pub const ACP5X_EXT_INTR_STAT: u32 = 0x1808;
pub const ACP6X_EXTERNAL_INTR_ENB: u32 = 0x1A00;
pub const ACP6X_EXTERNAL_INTR_CNTL: u32 = 0x1A04;
pub const ACP6X_EXT_INTR_STAT: u32 = 0x1A0C;
pub const ACP6X_EXT_INTR_STAT1: u32 = 0x1A10;
pub const ACP70_EXTERNAL_INTR_ENB: u32 = ACP6X_EXTERNAL_INTR_ENB;
pub const ACP70_EXTERNAL_INTR_CNTL: u32 = ACP6X_EXTERNAL_INTR_CNTL;
pub const ACP70_EXT_INTR_STAT: u32 = ACP6X_EXT_INTR_STAT;
pub const ACP70_EXT_INTR_STAT1: u32 = ACP6X_EXT_INTR_STAT1;
pub const ACP7X_EXTERNAL_INTR_ENB: u32 = ACP6X_EXTERNAL_INTR_ENB;
pub const ACP7X_EXTERNAL_INTR_CNTL: u32 = 0x1A04;
pub const ACP7X_EXT_INTR_STAT: u32 = 0x1A1C;
pub const ACP7X_EXTERNAL_INTR_CNTL1: u32 = 0x1A08;
pub const ACP7X_EXT_INTR_STAT1: u32 = 0x1A20;

pub const ACP3X_DSP_SW_INTR_BASE: u32 = 0x1814;
pub const ACP5X_DSP_SW_INTR_BASE: u32 = 0x1814;
pub const ACP6X_DSP_SW_INTR_BASE: u32 = 0x1808;
pub const ACP70_DSP_SW_INTR_BASE: u32 = ACP6X_DSP_SW_INTR_BASE;
pub const ACP7X_DSP_SW_INTR_BASE: u32 = 0x1860;
pub const DSP_SW_INTR_CNTL_OFFSET: u32 = 0x0;
pub const DSP_SW_INTR_STAT_OFFSET: u32 = 0x4;
pub const ACP7X_DSP_SW_INTR_STAT: u32 = ACP7X_DSP_SW_INTR_BASE + DSP_SW_INTR_STAT_OFFSET;
pub const DSP_SW_INTR_TRIG_OFFSET: u32 = 0x8;
pub const ACP7X_DSP_SW_INTR_TRIG_OFFSET: u32 = 0x30;
pub const ACP3X_ERROR_STATUS: u32 = 0x18C4;
pub const ACP6X_ERROR_STATUS: u32 = 0x1A4C;
pub const ACP70_ERROR_STATUS: u32 = ACP6X_ERROR_STATUS;
pub const ACP7X_ERROR_STATUS: u32 = 0x1A88;
pub const ACP3X_AXI2DAGB_SEM_0: u32 = 0x1880;
pub const ACP5X_AXI2DAGB_SEM_0: u32 = 0x1884;
pub const ACP6X_AXI2DAGB_SEM_0: u32 = 0x1874;
pub const ACP70_AXI2DAGB_SEM_0: u32 = ACP6X_AXI2DAGB_SEM_0;
pub const ACP7X_AXI2DAGB_SEM_0: u32 = 0x18F4;

/* ACP common registers to report errors related to I2S & SoundWire interfaces */
pub const ACP3X_SW_I2S_ERROR_REASON: u32 = 0x18C8;
pub const ACP6X_SW0_I2S_ERROR_REASON: u32 = 0x18B4;
pub const ACP7X_SW0_I2S_ERROR_REASON: u32 = ACP6X_SW0_I2S_ERROR_REASON;
pub const ACP_SW1_I2S_ERROR_REASON: u32 = 0x1A50;

/* Registers from ACP_SHA block */
pub const ACP_SHA_DSP_FW_QUALIFIER: u32 = 0x1C70;
pub const ACP_SHA_DMA_CMD: u32 = 0x1CB0;
pub const ACP_SHA_MSG_LENGTH: u32 = 0x1CB4;
pub const ACP_SHA_DMA_STRT_ADDR: u32 = 0x1CB8;
pub const ACP_SHA_DMA_DESTINATION_ADDR: u32 = 0x1CBC;
pub const ACP_SHA_DMA_CMD_STS: u32 = 0x1CC0;
pub const ACP_SHA_DMA_ERR_STATUS: u32 = 0x1CC4;
pub const ACP_SHA_TRANSFER_BYTE_CNT: u32 = 0x1CC8;
pub const ACP_SHA_DMA_INCLUDE_HDR: u32 = 0x1CCC;
pub const ACP_SHA_PSP_ACK: u32 = 0x1C74;

pub const ACP_SCRATCH_REG_0: u32 = 0x10000;
pub const ACP6X_DSP_FUSION_RUNSTALL: u32 = 0x0644;
pub const ACP70_DSP_FUSION_RUNSTALL: u32 = ACP6X_DSP_FUSION_RUNSTALL;
pub const ACP7X_DSP_FUSION_RUNSTALL: u32 = ACP6X_DSP_FUSION_RUNSTALL;

/* Cache window registers */
pub const ACP_DSP0_CACHE_OFFSET0: u32 = 0x0420;
pub const ACP_DSP0_CACHE_SIZE0: u32 = 0x0424;

pub const ACP_SW0_EN: u32 = 0x3000;
pub const ACP_SW1_EN: u32 = 0x3C00;
pub const ACP70_PME_EN: u32 = 0x1400;
pub const ACP70_EXTERNAL_INTR_CNTL1: u32 = 0x1A08;
pub const ACP70_SW0_WAKE_EN: u32 = 0x1458;
pub const ACP70_SW1_WAKE_EN: u32 = 0x1460;
pub const ACP70_SDW_HOST_WAKE_MASK: u32 = 0x0C00000;
pub const ACP70_SDW0_HOST_WAKE_STAT: u32 = 1u32 << 24;
pub const ACP70_SDW1_HOST_WAKE_STAT: u32 = 1u32 << 25;
pub const ACP70_SDW0_PME_STAT: u32 = 1u32 << 26;
pub const ACP70_SDW1_PME_STAT: u32 = 1u32 << 27;
pub const ACP7X_DSP0_IDMA_ERROR_MASK: u32 = 0x4B0;
pub const ACP7X_IDMA_ERROR_MASK: u32 = 0x1FF9FF;
pub const ACP7X_ZSC_DSP_CTRL: u32 = 0x001014;
pub const ACP7X_PME_EN: u32 = ACP70_PME_EN;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
