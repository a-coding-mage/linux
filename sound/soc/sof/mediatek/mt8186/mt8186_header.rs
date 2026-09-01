/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause) */

/*
 * Copyright (c) 2022 MediaTek Corporation. All rights reserved.
 *
 *  Header file for the mt8186 DSP register definition
 */

#[repr(C)]
pub struct mtk_adsp_chip_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_sof_dev {
    _private: [u8; 0],
}

pub const fn BIT(n: u32) -> u32 {
    1u32 << n
}

pub const DSP_REG_BAR: u32 = 4;
pub const DSP_SECREG_BAR: u32 = 5;
pub const DSP_BUSREG_BAR: u32 = 6;

/*****************************************************************************
 *                  R E G I S T E R       TABLE
 *****************************************************************************/
/* dsp cfg */
pub const ADSP_CFGREG_SW_RSTN: u32 = 0x0000;
pub const SW_DBG_RSTN_C0: u32 = BIT(0);
pub const SW_RSTN_C0: u32 = BIT(4);
pub const ADSP_HIFI_IO_CONFIG: u32 = 0x000C;
pub const TRACEMEMREADY: u32 = BIT(15);
pub const RUNSTALL: u32 = BIT(31);
pub const ADSP_IRQ_MASK: u32 = 0x0030;
pub const ADSP_DVFSRC_REQ: u32 = 0x0040;
pub const ADSP_DDREN_REQ_0: u32 = 0x0044;
pub const ADSP_SEMAPHORE: u32 = 0x0064;
pub const ADSP_WDT_CON_C0: u32 = 0x007C;
pub const ADSP_MBOX_IRQ_EN: u32 = 0x009C;
pub const DSP_MBOX0_IRQ_EN: u32 = BIT(0);
pub const DSP_MBOX1_IRQ_EN: u32 = BIT(1);
pub const DSP_MBOX2_IRQ_EN: u32 = BIT(2);
pub const DSP_MBOX3_IRQ_EN: u32 = BIT(3);
pub const DSP_MBOX4_IRQ_EN: u32 = BIT(4);
pub const DSP_PDEBUGPC: u32 = 0x013C;
pub const DSP_PDEBUGDATA: u32 = 0x0140;
pub const DSP_PDEBUGINST: u32 = 0x0144;
pub const DSP_PDEBUGLS0STAT: u32 = 0x0148;
pub const DSP_PDEBUGSTATUS: u32 = 0x014C;
pub const DSP_PFAULTINFO: u32 = 0x0150;
pub const ADSP_CK_EN: u32 = 0x1000;
pub const CORE_CLK_EN: u32 = BIT(0);
pub const COREDBG_EN: u32 = BIT(1);
pub const TIMER_EN: u32 = BIT(3);
pub const DMA_EN: u32 = BIT(4);
pub const UART_EN: u32 = BIT(5);
pub const ADSP_UART_CTRL: u32 = 0x1010;
pub const UART_BCLK_CG: u32 = BIT(0);
pub const UART_RSTN: u32 = BIT(3);

/* dsp sec */
pub const ADSP_PRID: u32 = 0x0;
pub const ADSP_ALTVEC_C0: u32 = 0x04;
pub const ADSP_ALTVECSEL: u32 = 0x0C;
pub const MT8188_ADSP_ALTVECSEL_C0: u32 = BIT(0);
pub const MT8186_ADSP_ALTVECSEL_C0: u32 = BIT(1);

/*
 * On MT8188, BIT(1) is not evaluated and on MT8186 BIT(0) is not evaluated:
 * We can simplify the driver by safely setting both bits regardless of the SoC.
 */
pub const ADSP_ALTVECSEL_C0: u32 = MT8188_ADSP_ALTVECSEL_C0 | MT8186_ADSP_ALTVECSEL_C0;

/* dsp bus */
pub const ADSP_SRAM_POOL_CON: u32 = 0x190;
pub const DSP_SRAM_POOL_PD_MASK: u32 = 0xF00F; /* [0:3] and [12:15] */
pub const DSP_C0_EMI_MAP_ADDR: u32 = 0xA00; /* ADSP Core0 To EMI Address Remap */
pub const DSP_C0_DMAEMI_MAP_ADDR: u32 = 0xA08; /* DMA0 To EMI Address Remap */

/* DSP memories */
pub const MBOX_OFFSET: u32 = 0x500000; /* DRAM */
pub const MBOX_SIZE: u32 = 0x1000; /* consistent with which in memory.h of sof fw */
pub const DSP_DRAM_SIZE: u32 = 0xA00000; /* 16M */

/*remap dram between AP and DSP view, 4KB aligned*/
pub const SRAM_PHYS_BASE_FROM_DSP_VIEW: u32 = 0x4E100000; /* MT8186 DSP view */
pub const DRAM_PHYS_BASE_FROM_DSP_VIEW: u32 = 0x60000000; /* MT8186 DSP view */
pub const DRAM_REMAP_SHIFT: u32 = 12;
pub const DRAM_REMAP_MASK: u32 = 0xFFF;

pub const SIZE_SHARED_DRAM_DL: u32 = 0x40000; /*Shared buffer for Downlink*/
pub const SIZE_SHARED_DRAM_UL: u32 = 0x40000; /*Shared buffer for Uplink*/
pub const TOTAL_SIZE_SHARED_DRAM_FROM_TAIL: u32 = SIZE_SHARED_DRAM_DL + SIZE_SHARED_DRAM_UL;

unsafe extern "C" {
    pub fn mt8186_sof_hifixdsp_boot_sequence(sdev: *mut snd_sof_dev, boot_addr: u32);
    pub fn mt8186_sof_hifixdsp_shutdown(sdev: *mut snd_sof_dev);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
