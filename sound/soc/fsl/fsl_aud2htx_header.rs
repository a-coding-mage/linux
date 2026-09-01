/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright 2020 NXP
 */

/* Dependencies from the original header:
 * SNDRV_PCM_FMTBIT_S24_LE, SNDRV_PCM_FMTBIT_S32_LE,
 * SNDRV_PCM_FMTBIT_IEC958_SUBFRAME_LE, platform_device, regmap, clk,
 * and snd_dmaengine_dai_dma_data are supplied by other translation units.
 */

pub const FSL_AUD2HTX_FORMATS: u32 = SNDRV_PCM_FMTBIT_S24_LE
    | SNDRV_PCM_FMTBIT_S32_LE
    | SNDRV_PCM_FMTBIT_IEC958_SUBFRAME_LE;

/* AUD2HTX Register Map */
pub const AUD2HTX_CTRL: u32 = 0x0; /* AUD2HTX Control Register */
pub const AUD2HTX_CTRL_EXT: u32 = 0x4; /* AUD2HTX Control Extended Register */
pub const AUD2HTX_WR: u32 = 0x8; /* AUD2HTX Write Register */
pub const AUD2HTX_STATUS: u32 = 0xC; /* AUD2HTX Status Register */
pub const AUD2HTX_IRQ_NOMASK: u32 = 0x10; /* AUD2HTX Nonmasked Interrupt Flags Register */
pub const AUD2HTX_IRQ_MASKED: u32 = 0x14; /* AUD2HTX Masked Interrupt Flags Register */
pub const AUD2HTX_IRQ_MASK: u32 = 0x18; /* AUD2HTX IRQ Masks Register */

/* AUD2HTX Control Register */
pub const AUD2HTX_CTRL_EN: u32 = 1u32 << 0;

/* AUD2HTX Control Extended Register */
pub const AUD2HTX_CTRE_DE: u32 = 1u32 << 0;
pub const AUD2HTX_CTRE_DT_SHIFT: u32 = 0x1;
pub const AUD2HTX_CTRE_DT_WIDTH: u32 = 0x2;
pub const AUD2HTX_CTRE_DT_MASK: u32 =
    ((1u32 << AUD2HTX_CTRE_DT_WIDTH) - 1) << AUD2HTX_CTRE_DT_SHIFT;
pub const AUD2HTX_CTRE_WL_SHIFT: u32 = 16;
pub const AUD2HTX_CTRE_WL_WIDTH: u32 = 5;
pub const AUD2HTX_CTRE_WL_MASK: u32 =
    ((1u32 << AUD2HTX_CTRE_WL_WIDTH) - 1) << AUD2HTX_CTRE_WL_SHIFT;
pub const AUD2HTX_CTRE_WH_SHIFT: u32 = 24;
pub const AUD2HTX_CTRE_WH_WIDTH: u32 = 5;
pub const AUD2HTX_CTRE_WH_MASK: u32 =
    ((1u32 << AUD2HTX_CTRE_WH_WIDTH) - 1) << AUD2HTX_CTRE_WH_SHIFT;

/* AUD2HTX IRQ Masks Register */
pub const AUD2HTX_WM_HIGH_IRQ_MASK: u32 = 1u32 << 2;
pub const AUD2HTX_WM_LOW_IRQ_MASK: u32 = 1u32 << 1;
pub const AUD2HTX_OVF_MASK: u32 = 1u32 << 0;

pub const AUD2HTX_FIFO_DEPTH: u32 = 0x20;
pub const AUD2HTX_WTMK_LOW: u32 = 0x10;
pub const AUD2HTX_WTMK_HIGH: u32 = 0x10;
pub const AUD2HTX_MAXBURST: u32 = 0x10;

/**
 * fsl_aud2htx: AUD2HTX private data
 *
 * @pdev: platform device pointer
 * @regmap: regmap handler
 * @bus_clk: clock source to access register
 * @dma_params_rx: DMA parameters for receive channel
 * @dma_params_tx: DMA parameters for transmit channel
 */
#[repr(C)]
pub struct fsl_aud2htx {
    pub pdev: *mut platform_device,
    pub regmap: *mut regmap,
    pub bus_clk: *mut clk,

    pub dma_params_rx: snd_dmaengine_dai_dma_data,
    pub dma_params_tx: snd_dmaengine_dai_dma_data,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
