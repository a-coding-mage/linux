/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright 2012-2013 Freescale Semiconductor, Inc.
 */

/* Dependencies from the C header:
 * #include <linux/dma/imx-dma.h>
 * #include <sound/dmaengine_pcm.h>
 */

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use core::ffi::c_uint;

pub enum platform_device {}
pub enum regmap {}
pub enum clk {}
pub enum resource {}
pub enum snd_soc_dai_driver {}
pub enum snd_dmaengine_dai_dma_data {}
pub enum pm_qos_request {}
pub enum pinctrl {}
pub enum pinctrl_state {}
pub enum sdma_peripheral_config {}
pub enum snd_pcm_hw_constraint_list {}

pub const fn BIT(nr: c_uint) -> c_uint {
    1u32 << nr
}

pub const fn GENMASK(h: c_uint, l: c_uint) -> c_uint {
    (!0u32 << l) & (!0u32 >> (31 - h))
}

pub const FAL_SAI_NUM_RATES: usize = 20;
pub const FSL_SAI_FORMATS: c_uint = SNDRV_PCM_FMTBIT_S16_LE
    | SNDRV_PCM_FMTBIT_S20_3LE
    | SNDRV_PCM_FMTBIT_S24_LE
    | SNDRV_PCM_FMTBIT_S32_LE
    | SNDRV_PCM_FMTBIT_DSD_U8
    | SNDRV_PCM_FMTBIT_DSD_U16_LE
    | SNDRV_PCM_FMTBIT_DSD_U32_LE;

/* External PCM format bit constants supplied by sound headers. */
extern "C" {
    pub static SNDRV_PCM_FMTBIT_S16_LE: c_uint;
    pub static SNDRV_PCM_FMTBIT_S20_3LE: c_uint;
    pub static SNDRV_PCM_FMTBIT_S24_LE: c_uint;
    pub static SNDRV_PCM_FMTBIT_S32_LE: c_uint;
    pub static SNDRV_PCM_FMTBIT_DSD_U8: c_uint;
    pub static SNDRV_PCM_FMTBIT_DSD_U16_LE: c_uint;
    pub static SNDRV_PCM_FMTBIT_DSD_U32_LE: c_uint;
}

/* SAI Register Map Register */
pub const FSL_SAI_VERID: c_uint = 0x00; /* SAI Version ID Register */
pub const FSL_SAI_PARAM: c_uint = 0x04; /* SAI Parameter Register */
pub const fn FSL_SAI_TCSR(ofs: c_uint) -> c_uint { 0x00 + ofs } /* SAI Transmit Control */
pub const fn FSL_SAI_TCR1(ofs: c_uint) -> c_uint { 0x04 + ofs } /* SAI Transmit Configuration 1 */
pub const fn FSL_SAI_TCR2(ofs: c_uint) -> c_uint { 0x08 + ofs } /* SAI Transmit Configuration 2 */
pub const fn FSL_SAI_TCR3(ofs: c_uint) -> c_uint { 0x0c + ofs } /* SAI Transmit Configuration 3 */
pub const fn FSL_SAI_TCR4(ofs: c_uint) -> c_uint { 0x10 + ofs } /* SAI Transmit Configuration 4 */
pub const fn FSL_SAI_TCR5(ofs: c_uint) -> c_uint { 0x14 + ofs } /* SAI Transmit Configuration 5 */
pub const FSL_SAI_TDR0: c_uint = 0x20; /* SAI Transmit Data 0 */
pub const FSL_SAI_TDR1: c_uint = 0x24; /* SAI Transmit Data 1 */
pub const FSL_SAI_TDR2: c_uint = 0x28; /* SAI Transmit Data 2 */
pub const FSL_SAI_TDR3: c_uint = 0x2C; /* SAI Transmit Data 3 */
pub const FSL_SAI_TDR4: c_uint = 0x30; /* SAI Transmit Data 4 */
pub const FSL_SAI_TDR5: c_uint = 0x34; /* SAI Transmit Data 5 */
pub const FSL_SAI_TDR6: c_uint = 0x38; /* SAI Transmit Data 6 */
pub const FSL_SAI_TDR7: c_uint = 0x3C; /* SAI Transmit Data 7 */
pub const FSL_SAI_TFR0: c_uint = 0x40; /* SAI Transmit FIFO 0 */
pub const FSL_SAI_TFR1: c_uint = 0x44; /* SAI Transmit FIFO 1 */
pub const FSL_SAI_TFR2: c_uint = 0x48; /* SAI Transmit FIFO 2 */
pub const FSL_SAI_TFR3: c_uint = 0x4C; /* SAI Transmit FIFO 3 */
pub const FSL_SAI_TFR4: c_uint = 0x50; /* SAI Transmit FIFO 4 */
pub const FSL_SAI_TFR5: c_uint = 0x54; /* SAI Transmit FIFO 5 */
pub const FSL_SAI_TFR6: c_uint = 0x58; /* SAI Transmit FIFO 6 */
pub const FSL_SAI_TFR7: c_uint = 0x5C; /* SAI Transmit FIFO 7 */
pub const FSL_SAI_TMR: c_uint = 0x60; /* SAI Transmit Mask */
pub const FSL_SAI_TTCTL: c_uint = 0x70; /* SAI Transmit Timestamp Control Register */
pub const FSL_SAI_TTCTN: c_uint = 0x74; /* SAI Transmit Timestamp Counter Register */
pub const FSL_SAI_TBCTN: c_uint = 0x78; /* SAI Transmit Bit Counter Register */
pub const FSL_SAI_TTCAP: c_uint = 0x7C; /* SAI Transmit Timestamp Capture */
pub const fn FSL_SAI_RCSR(ofs: c_uint) -> c_uint { 0x80 + ofs } /* SAI Receive Control */
pub const fn FSL_SAI_RCR1(ofs: c_uint) -> c_uint { 0x84 + ofs } /* SAI Receive Configuration 1 */
pub const fn FSL_SAI_RCR2(ofs: c_uint) -> c_uint { 0x88 + ofs } /* SAI Receive Configuration 2 */
pub const fn FSL_SAI_RCR3(ofs: c_uint) -> c_uint { 0x8c + ofs } /* SAI Receive Configuration 3 */
pub const fn FSL_SAI_RCR4(ofs: c_uint) -> c_uint { 0x90 + ofs } /* SAI Receive Configuration 4 */
pub const fn FSL_SAI_RCR5(ofs: c_uint) -> c_uint { 0x94 + ofs } /* SAI Receive Configuration 5 */
pub const FSL_SAI_RDR0: c_uint = 0xa0; /* SAI Receive Data 0 */
pub const FSL_SAI_RDR1: c_uint = 0xa4; /* SAI Receive Data 1 */
pub const FSL_SAI_RDR2: c_uint = 0xa8; /* SAI Receive Data 2 */
pub const FSL_SAI_RDR3: c_uint = 0xac; /* SAI Receive Data 3 */
pub const FSL_SAI_RDR4: c_uint = 0xb0; /* SAI Receive Data 4 */
pub const FSL_SAI_RDR5: c_uint = 0xb4; /* SAI Receive Data 5 */
pub const FSL_SAI_RDR6: c_uint = 0xb8; /* SAI Receive Data 6 */
pub const FSL_SAI_RDR7: c_uint = 0xbc; /* SAI Receive Data 7 */
pub const FSL_SAI_RFR0: c_uint = 0xc0; /* SAI Receive FIFO 0 */
pub const FSL_SAI_RFR1: c_uint = 0xc4; /* SAI Receive FIFO 1 */
pub const FSL_SAI_RFR2: c_uint = 0xc8; /* SAI Receive FIFO 2 */
pub const FSL_SAI_RFR3: c_uint = 0xcc; /* SAI Receive FIFO 3 */
pub const FSL_SAI_RFR4: c_uint = 0xd0; /* SAI Receive FIFO 4 */
pub const FSL_SAI_RFR5: c_uint = 0xd4; /* SAI Receive FIFO 5 */
pub const FSL_SAI_RFR6: c_uint = 0xd8; /* SAI Receive FIFO 6 */
pub const FSL_SAI_RFR7: c_uint = 0xdc; /* SAI Receive FIFO 7 */
pub const FSL_SAI_RMR: c_uint = 0xe0; /* SAI Receive Mask */
pub const FSL_SAI_RTCTL: c_uint = 0xf0; /* SAI Receive Timestamp Control Register */
pub const FSL_SAI_RTCTN: c_uint = 0xf4; /* SAI Receive Timestamp Counter Register */
pub const FSL_SAI_RBCTN: c_uint = 0xf8; /* SAI Receive Bit Counter Register */
pub const FSL_SAI_RTCAP: c_uint = 0xfc; /* SAI Receive Timestamp Capture */

pub const FSL_SAI_MCTL: c_uint = 0x100; /* SAI MCLK Control Register */
pub const FSL_SAI_MDIV: c_uint = 0x104; /* SAI MCLK Divide Register */

pub const fn FSL_SAI_xCSR(tx: c_uint, ofs: c_uint) -> c_uint { if tx != 0 { FSL_SAI_TCSR(ofs) } else { FSL_SAI_RCSR(ofs) } }
pub const fn FSL_SAI_xCR1(tx: c_uint, ofs: c_uint) -> c_uint { if tx != 0 { FSL_SAI_TCR1(ofs) } else { FSL_SAI_RCR1(ofs) } }
pub const fn FSL_SAI_xCR2(tx: c_uint, ofs: c_uint) -> c_uint { if tx != 0 { FSL_SAI_TCR2(ofs) } else { FSL_SAI_RCR2(ofs) } }
pub const fn FSL_SAI_xCR3(tx: c_uint, ofs: c_uint) -> c_uint { if tx != 0 { FSL_SAI_TCR3(ofs) } else { FSL_SAI_RCR3(ofs) } }
pub const fn FSL_SAI_xCR4(tx: c_uint, ofs: c_uint) -> c_uint { if tx != 0 { FSL_SAI_TCR4(ofs) } else { FSL_SAI_RCR4(ofs) } }
pub const fn FSL_SAI_xCR5(tx: c_uint, ofs: c_uint) -> c_uint { if tx != 0 { FSL_SAI_TCR5(ofs) } else { FSL_SAI_RCR5(ofs) } }
pub const fn FSL_SAI_xDR0(tx: c_uint) -> c_uint { if tx != 0 { FSL_SAI_TDR0 } else { FSL_SAI_RDR0 } }
pub const fn FSL_SAI_xFR0(tx: c_uint) -> c_uint { if tx != 0 { FSL_SAI_TFR0 } else { FSL_SAI_RFR0 } }
pub const fn FSL_SAI_xMR(tx: c_uint) -> c_uint { if tx != 0 { FSL_SAI_TMR } else { FSL_SAI_RMR } }

/* SAI Transmit/Receive Control Register */
pub const FSL_SAI_CSR_TERE: c_uint = BIT(31);
pub const FSL_SAI_CSR_SE: c_uint = BIT(30);
pub const FSL_SAI_CSR_BCE: c_uint = BIT(28);
pub const FSL_SAI_CSR_FR: c_uint = BIT(25);
pub const FSL_SAI_CSR_SR: c_uint = BIT(24);
pub const FSL_SAI_CSR_xF_SHIFT: c_uint = 16;
pub const FSL_SAI_CSR_xF_W_SHIFT: c_uint = 18;
pub const FSL_SAI_CSR_xF_MASK: c_uint = 0x1f << FSL_SAI_CSR_xF_SHIFT;
pub const FSL_SAI_CSR_xF_W_MASK: c_uint = 0x7 << FSL_SAI_CSR_xF_W_SHIFT;
pub const FSL_SAI_CSR_WSF: c_uint = BIT(20);
pub const FSL_SAI_CSR_SEF: c_uint = BIT(19);
pub const FSL_SAI_CSR_FEF: c_uint = BIT(18);
pub const FSL_SAI_CSR_FWF: c_uint = BIT(17);
pub const FSL_SAI_CSR_FRF: c_uint = BIT(16);
pub const FSL_SAI_CSR_xIE_SHIFT: c_uint = 8;
pub const FSL_SAI_CSR_xIE_MASK: c_uint = 0x1f << FSL_SAI_CSR_xIE_SHIFT;
pub const FSL_SAI_CSR_WSIE: c_uint = BIT(12);
pub const FSL_SAI_CSR_SEIE: c_uint = BIT(11);
pub const FSL_SAI_CSR_FEIE: c_uint = BIT(10);
pub const FSL_SAI_CSR_FWIE: c_uint = BIT(9);
pub const FSL_SAI_CSR_FRIE: c_uint = BIT(8);
pub const FSL_SAI_CSR_FRDE: c_uint = BIT(0);

/* SAI Transmit and Receive Configuration 1 Register */
pub const fn FSL_SAI_CR1_RFW_MASK(x: c_uint) -> c_uint { x - 1 }

/* SAI Transmit and Receive Configuration 2 Register */
pub const FSL_SAI_CR2_SYNC: c_uint = BIT(30);
pub const FSL_SAI_CR2_BCS: c_uint = BIT(29);
pub const FSL_SAI_CR2_BCI: c_uint = BIT(28);
pub const FSL_SAI_CR2_MSEL_MASK: c_uint = 0x3 << 26;
pub const FSL_SAI_CR2_MSEL_BUS: c_uint = 0;
pub const FSL_SAI_CR2_MSEL_MCLK1: c_uint = BIT(26);
pub const FSL_SAI_CR2_MSEL_MCLK2: c_uint = BIT(27);
pub const FSL_SAI_CR2_MSEL_MCLK3: c_uint = BIT(26) | BIT(27);
pub const fn FSL_SAI_CR2_MSEL(ID: c_uint) -> c_uint { ID << 26 }
pub const FSL_SAI_CR2_BCP: c_uint = BIT(25);
pub const FSL_SAI_CR2_BCD_MSTR: c_uint = BIT(24);
pub const FSL_SAI_CR2_BYP: c_uint = BIT(23); /* BCLK bypass */
pub const FSL_SAI_CR2_DIV_MASK: c_uint = 0xff;

/* SAI Transmit and Receive Configuration 3 Register */
pub const fn FSL_SAI_CR3_TRCE(x: c_uint) -> c_uint { x << 16 }
pub const FSL_SAI_CR3_TRCE_MASK: c_uint = GENMASK(23, 16);
pub const fn FSL_SAI_CR3_WDFL(x: c_uint) -> c_uint { x }
pub const FSL_SAI_CR3_WDFL_MASK: c_uint = 0x1f;

/* SAI Transmit and Receive Configuration 4 Register */
pub const FSL_SAI_CR4_FCONT_MASK: c_uint = BIT(28);
pub const FSL_SAI_CR4_FCONT: c_uint = BIT(28);
pub const FSL_SAI_CR4_FCOMB_SHIFT: c_uint = BIT(26);
pub const FSL_SAI_CR4_FCOMB_SOFT: c_uint = BIT(27);
pub const FSL_SAI_CR4_FCOMB_MASK: c_uint = 0x3 << 26;
pub const FSL_SAI_CR4_FPACK_8: c_uint = 0x2 << 24;
pub const FSL_SAI_CR4_FPACK_16: c_uint = 0x3 << 24;
pub const fn FSL_SAI_CR4_FRSZ(x: c_uint) -> c_uint { (x - 1) << 16 }
pub const FSL_SAI_CR4_FRSZ_MASK: c_uint = 0x1f << 16;
pub const fn FSL_SAI_CR4_SYWD(x: c_uint) -> c_uint { (x - 1) << 8 }
pub const FSL_SAI_CR4_SYWD_MASK: c_uint = 0x1f << 8;
pub const FSL_SAI_CR4_CHMOD: c_uint = BIT(5);
pub const FSL_SAI_CR4_CHMOD_MASK: c_uint = BIT(5);
pub const FSL_SAI_CR4_MF: c_uint = BIT(4);
pub const FSL_SAI_CR4_FSE: c_uint = BIT(3);
pub const FSL_SAI_CR4_FSP: c_uint = BIT(1);
pub const FSL_SAI_CR4_FSD_MSTR: c_uint = BIT(0);

/* SAI Transmit and Receive Configuration 5 Register */
pub const fn FSL_SAI_CR5_WNW(x: c_uint) -> c_uint { (x - 1) << 24 }
pub const FSL_SAI_CR5_WNW_MASK: c_uint = 0x1f << 24;
pub const fn FSL_SAI_CR5_W0W(x: c_uint) -> c_uint { (x - 1) << 16 }
pub const FSL_SAI_CR5_W0W_MASK: c_uint = 0x1f << 16;
pub const fn FSL_SAI_CR5_FBT(x: c_uint) -> c_uint { x << 8 }
pub const FSL_SAI_CR5_FBT_MASK: c_uint = 0x1f << 8;

/* SAI MCLK Control Register */
pub const FSL_SAI_MCTL_MCLK_EN: c_uint = BIT(30); /* MCLK Enable */
pub const FSL_SAI_MCTL_MSEL_MASK: c_uint = 0x3 << 24;
pub const fn FSL_SAI_MCTL_MSEL(ID: c_uint) -> c_uint { ID << 24 }
pub const FSL_SAI_MCTL_MSEL_BUS: c_uint = 0;
pub const FSL_SAI_MCTL_MSEL_MCLK1: c_uint = BIT(24);
pub const FSL_SAI_MCTL_MSEL_MCLK2: c_uint = BIT(25);
pub const FSL_SAI_MCTL_MSEL_MCLK3: c_uint = BIT(24) | BIT(25);
pub const FSL_SAI_MCTL_DIV_EN: c_uint = BIT(23);
pub const FSL_SAI_MCTL_DIV_MASK: c_uint = 0xFF;

/* SAI VERID Register */
pub const FSL_SAI_VERID_MAJOR_SHIFT: c_uint = 24;
pub const FSL_SAI_VERID_MAJOR_MASK: c_uint = GENMASK(31, 24);
pub const FSL_SAI_VERID_MINOR_SHIFT: c_uint = 16;
pub const FSL_SAI_VERID_MINOR_MASK: c_uint = GENMASK(23, 16);
pub const FSL_SAI_VERID_FEATURE_SHIFT: c_uint = 0;
pub const FSL_SAI_VERID_FEATURE_MASK: c_uint = GENMASK(15, 0);
pub const FSL_SAI_VERID_EFIFO_EN: c_uint = BIT(0);
pub const FSL_SAI_VERID_TSTMP_EN: c_uint = BIT(1);

/* SAI PARAM Register */
pub const FSL_SAI_PARAM_SPF_SHIFT: c_uint = 16;
pub const FSL_SAI_PARAM_SPF_MASK: c_uint = GENMASK(19, 16);
pub const FSL_SAI_PARAM_WPF_SHIFT: c_uint = 8;
pub const FSL_SAI_PARAM_WPF_MASK: c_uint = GENMASK(11, 8);
pub const FSL_SAI_PARAM_DLN_MASK: c_uint = GENMASK(3, 0);

/* SAI MCLK Divide Register */
pub const FSL_SAI_MDIV_MASK: c_uint = 0xFFFFF;

/* SAI timestamp and bitcounter */
pub const FSL_SAI_xTCTL_TSEN_SHIFT: c_uint = 0;
pub const FSL_SAI_xTCTL_TSEN: c_uint = BIT(0);
pub const FSL_SAI_xTCTL_TSINC_SHIFT: c_uint = 1;
pub const FSL_SAI_xTCTL_TSINC: c_uint = BIT(1);
pub const FSL_SAI_xTCTL_RTSC_SHIFT: c_uint = 8;
pub const FSL_SAI_xTCTL_RTSC: c_uint = BIT(8);
pub const FSL_SAI_xTCTL_RBC_SHIFT: c_uint = 9;
pub const FSL_SAI_xTCTL_RBC: c_uint = BIT(9);

/* SAI type */
pub const FSL_SAI_DMA: c_uint = BIT(0);
pub const FSL_SAI_USE_AC97: c_uint = BIT(1);
pub const FSL_SAI_NET: c_uint = BIT(2);
pub const FSL_SAI_TRA_SYN: c_uint = BIT(3);
pub const FSL_SAI_REC_SYN: c_uint = BIT(4);
pub const FSL_SAI_USE_I2S_SLAVE: c_uint = BIT(5);

/* SAI clock sources */
pub const FSL_SAI_CLK_BUS: c_uint = 0;
pub const FSL_SAI_CLK_MAST1: c_uint = 1;
pub const FSL_SAI_CLK_MAST2: c_uint = 2;
pub const FSL_SAI_CLK_MAST3: c_uint = 3;

pub const FSL_SAI_MCLK_MAX: usize = 4;

/* SAI data transfer numbers per DMA request */
pub const FSL_SAI_MAXBURST_TX: c_uint = 6;
pub const FSL_SAI_MAXBURST_RX: c_uint = 6;

pub const PMQOS_CPU_LATENCY: c_uint = BIT(0);

/* Max number of dataline */
pub const FSL_SAI_DL_NUM: c_uint = 8;
/* default dataline type is zero */
pub const FSL_SAI_DL_DEFAULT: c_uint = 0;
pub const FSL_SAI_DL_I2S: c_uint = BIT(0);
pub const FSL_SAI_DL_PDM: c_uint = BIT(1);

pub const FSL_SAI_AMIX_BYPASS: c_uint = 0;
pub const FSL_SAI_AMIX_AUDMIX: c_uint = 1;
pub const FSL_SAI_AMIX_NONE: c_uint = 2;

#[repr(C)]
pub struct fsl_sai_soc_data {
    pub use_imx_pcm: bool,
    pub use_edma: bool,
    pub mclk0_is_mclk1: bool,
    pub mclk_with_tere: bool,
    pub fifo_depth: c_uint,
    pub pins: c_uint,
    pub reg_offset: c_uint,
    pub flags: c_uint,
    pub max_register: c_uint,
    pub max_burst: [c_uint; 2],
}

/**
 * struct fsl_sai_verid - version id data
 * @version: version number
 * @feature: feature specification number
 *           0000000000000000b - Standard feature set
 *           0000000000000000b - Standard feature set
 */
#[repr(C)]
pub struct fsl_sai_verid {
    pub version: u32,
    pub feature: u32,
}

/**
 * struct fsl_sai_param - parameter data
 * @slot_num: The maximum number of slots per frame
 * @fifo_depth: The number of words in each FIFO (depth)
 * @dataline: The number of datalines implemented
 */
#[repr(C)]
pub struct fsl_sai_param {
    pub slot_num: u32,
    pub fifo_depth: u32,
    pub dataline: u32,
}

#[repr(C)]
pub struct fsl_sai_dl_cfg {
    pub type_: c_uint,
    pub pins: [c_uint; 2],
    pub mask: [c_uint; 2],
    pub start_off: [c_uint; 2],
    pub next_off: [c_uint; 2],
}

#[repr(C)]
pub struct fsl_sai {
    pub pdev: *mut platform_device,
    pub regmap: *mut regmap,
    pub bus_clk: *mut clk,
    pub mclk_clk: [*mut clk; FSL_SAI_MCLK_MAX],
    pub pll8k_clk: *mut clk,
    pub pll11k_clk: *mut clk,
    pub res: *mut resource,

    pub is_consumer_mode: [bool; 2],
    pub is_lsb_first: bool,
    pub is_dsp_mode: [bool; 2],
    pub is_pdm_mode: bool,
    pub is_multi_fifo_dma: bool,
    pub synchronous: [bool; 2],
    pub dl_cfg: *mut fsl_sai_dl_cfg,
    pub dl_cfg_cnt: c_uint,
    pub mclk_direction_output: bool,
    pub is_bit_clock_swap: bool,

    pub mclk_id: [c_uint; 2],
    pub mclk_streams: c_uint,
    pub slots: [c_uint; 2],
    pub slot_width: [c_uint; 2],
    pub bclk_ratio: c_uint,

    pub soc_data: *const fsl_sai_soc_data,
    pub cpu_dai_drv: [snd_soc_dai_driver; 3],
    pub dma_params_rx: snd_dmaengine_dai_dma_data,
    pub dma_params_tx: snd_dmaengine_dai_dma_data,
    pub verid: fsl_sai_verid,
    pub param: fsl_sai_param,
    pub pm_qos_req: pm_qos_request,
    pub pinctrl: *mut pinctrl,
    pub pins_state: *mut pinctrl_state,
    pub audio_config: [sdma_peripheral_config; 2],
    pub constraint_rates: snd_pcm_hw_constraint_list,
    pub constraint_rates_list: [c_uint; FAL_SAI_NUM_RATES],
}

pub const TX: c_uint = 1;
pub const RX: c_uint = 0;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
