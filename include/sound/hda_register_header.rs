/* SPDX-License-Identifier: GPL-2.0 */
/* HD-audio controller (Azalia) registers and helpers. */

// Dependencies supplied by other translation units: linux I/O, hdaudio types,
// GENMASK, BIT, SD_LPIB, snd_hdac_stream_readl, and le32_to_cpu.

pub const AZX_REG_GCAP: u32 = 0x00;
pub const AZX_GCAP_64OK: u32 = 1 << 0;
pub const AZX_GCAP_NSDO: u32 = 3 << 1;
pub const AZX_GCAP_BSS: u32 = 31 << 3;
pub const AZX_GCAP_ISS: u32 = 15 << 8;
pub const AZX_GCAP_OSS: u32 = 15 << 12;
pub const AZX_REG_VMIN: u32 = 0x02;
pub const AZX_REG_VMAJ: u32 = 0x03;
pub const AZX_REG_OUTPAY: u32 = 0x04;
pub const AZX_REG_INPAY: u32 = 0x06;
pub const AZX_REG_GCTL: u32 = 0x08;
pub const AZX_GCTL_RESET: u32 = 1 << 0;
pub const AZX_GCTL_FCNTRL: u32 = 1 << 1;
pub const AZX_GCTL_UNSOL: u32 = 1 << 8;
pub const AZX_REG_WAKEEN: u32 = 0x0c;
pub const AZX_REG_STATESTS: u32 = 0x0e;
pub const AZX_REG_GSTS: u32 = 0x10;
pub const AZX_GSTS_FSTS: u32 = 1 << 1;
pub const AZX_REG_GCAP2: u32 = 0x12;
pub const AZX_REG_LLCH: u32 = 0x14;
pub const AZX_REG_OUTSTRMPAY: u32 = 0x18;
pub const AZX_REG_INSTRMPAY: u32 = 0x1A;
pub const AZX_REG_INTCTL: u32 = 0x20;
pub const AZX_REG_INTSTS: u32 = 0x24;
pub const AZX_REG_WALLCLK: u32 = 0x30;
pub const AZX_REG_OLD_SSYNC: u32 = 0x34;
pub const AZX_REG_SSYNC: u32 = 0x38;
pub const AZX_REG_CORBLBASE: u32 = 0x40;
pub const AZX_REG_CORBUBASE: u32 = 0x44;
pub const AZX_REG_CORBWP: u32 = 0x48;
pub const AZX_REG_CORBRP: u32 = 0x4a;
pub const AZX_CORBRP_RST: u32 = 1 << 15;
pub const AZX_REG_CORBCTL: u32 = 0x4c;
pub const AZX_CORBCTL_RUN: u32 = 1 << 1;
pub const AZX_CORBCTL_CMEIE: u32 = 1 << 0;
pub const AZX_REG_CORBSTS: u32 = 0x4d;
pub const AZX_CORBSTS_CMEI: u32 = 1 << 0;
pub const AZX_REG_CORBSIZE: u32 = 0x4e;
pub const AZX_REG_RIRBLBASE: u32 = 0x50;
pub const AZX_REG_RIRBUBASE: u32 = 0x54;
pub const AZX_REG_RIRBWP: u32 = 0x58;
pub const AZX_RIRBWP_RST: u32 = 1 << 15;
pub const AZX_REG_RINTCNT: u32 = 0x5a;
pub const AZX_REG_RIRBCTL: u32 = 0x5c;
pub const AZX_RBCTL_IRQ_EN: u32 = 1 << 0;
pub const AZX_RBCTL_DMA_EN: u32 = 1 << 1;
pub const AZX_RBCTL_OVERRUN_EN: u32 = 1 << 2;
pub const AZX_REG_RIRBSTS: u32 = 0x5d;
pub const AZX_RBSTS_IRQ: u32 = 1 << 0;
pub const AZX_RBSTS_OVERRUN: u32 = 1 << 2;
pub const AZX_REG_RIRBSIZE: u32 = 0x5e;
pub const AZX_REG_IC: u32 = 0x60;
pub const AZX_REG_IR: u32 = 0x64;
pub const AZX_REG_IRS: u32 = 0x68;
pub const AZX_IRS_VALID: u32 = 1 << 1;
pub const AZX_IRS_BUSY: u32 = 1 << 0;
pub const AZX_REG_DPLBASE: u32 = 0x70;
pub const AZX_REG_DPUBASE: u32 = 0x74;
pub const AZX_DPLBASE_ENABLE: u32 = 0x1;
pub const SDI0: u32 = 0; pub const SDI1: u32 = 1; pub const SDI2: u32 = 2; pub const SDI3: u32 = 3;
pub const SDO0: u32 = 4; pub const SDO1: u32 = 5; pub const SDO2: u32 = 6; pub const SDO3: u32 = 7;
pub const AZX_REG_SD_CTL: u32 = 0x00; pub const AZX_REG_SD_CTL_3B: u32 = 0x02; pub const AZX_REG_SD_STS: u32 = 0x03;
pub const AZX_REG_SD_LPIB: u32 = 0x04; pub const AZX_REG_SD_CBL: u32 = 0x08; pub const AZX_REG_SD_LVI: u32 = 0x0c;
pub const AZX_REG_SD_FIFOW: u32 = 0x0e; pub const AZX_REG_SD_FIFOSIZE: u32 = 0x10; pub const AZX_REG_SD_FORMAT: u32 = 0x12;
pub const AZX_REG_SD_FIFOL: u32 = 0x14; pub const AZX_REG_SD_BDLPL: u32 = 0x18; pub const AZX_REG_SD_BDLPU: u32 = 0x1c;
pub const AZX_SD_FIFOSIZE_MASK: u32 = GENMASK(15, 0);
pub const AZX_REG_GTS_BASE: u32 = 0x520;
pub const AZX_REG_GTSCC: u32 = AZX_REG_GTS_BASE + 0x00; pub const AZX_REG_WALFCC: u32 = AZX_REG_GTS_BASE + 0x04;
pub const AZX_REG_TSCCL: u32 = AZX_REG_GTS_BASE + 0x08; pub const AZX_REG_TSCCU: u32 = AZX_REG_GTS_BASE + 0x0C;
pub const AZX_REG_LLPFOC: u32 = AZX_REG_GTS_BASE + 0x14; pub const AZX_REG_LLPCL: u32 = AZX_REG_GTS_BASE + 0x18; pub const AZX_REG_LLPCU: u32 = AZX_REG_GTS_BASE + 0x1C;
pub const AZX_REG_HSW_EM4: u32 = 0x100c; pub const AZX_REG_HSW_EM5: u32 = 0x1010;
pub const AZX_REG_VS_EM1: u32 = 0x1000; pub const AZX_REG_VS_INRC: u32 = 0x1004; pub const AZX_REG_VS_OUTRC: u32 = 0x1008;
pub const AZX_REG_VS_FIFOTRK: u32 = 0x100C; pub const AZX_REG_VS_FIFOTRK2: u32 = 0x1010; pub const AZX_REG_VS_EM2: u32 = 0x1030;
pub const AZX_REG_VS_EM3L: u32 = 0x1038; pub const AZX_REG_VS_EM3U: u32 = 0x103C; pub const AZX_REG_VS_EM4L: u32 = 0x1040;
pub const AZX_REG_VS_EM4U: u32 = 0x1044; pub const AZX_REG_VS_LTRP: u32 = 0x1048; pub const AZX_REG_VS_D0I3C: u32 = 0x104A;
pub const AZX_REG_VS_PCE: u32 = 0x104B; pub const AZX_REG_VS_L2MAGC: u32 = 0x1050; pub const AZX_REG_VS_L2LAHPT: u32 = 0x1054;
pub const AZX_REG_VS_SDXDPIB_XBASE: u32 = 0x1084; pub const AZX_REG_VS_SDXDPIB_XINTERVAL: u32 = 0x20;
pub const AZX_REG_VS_SDXEFIFOS_XBASE: u32 = 0x1094; pub const AZX_REG_VS_SDXEFIFOS_XINTERVAL: u32 = 0x20;
pub const AZX_REG_VS_LTRP_GB_MASK: u32 = GENMASK(6, 0); pub const AZX_PCIREG_TCSEL: u32 = 0x44;
pub const BDL_SIZE: u32 = 4096; pub const AZX_MAX_BDL_ENTRIES: u32 = BDL_SIZE / 16; pub const AZX_MAX_FRAG: u32 = 32; pub const AZX_MAX_BUF_SIZE: u32 = 4*1024*1024;
pub const RIRB_INT_RESPONSE: u32 = 0x01; pub const RIRB_INT_OVERRUN: u32 = 0x04; pub const RIRB_INT_MASK: u32 = 0x05;
pub const STATESTS_INT_MASK: u32 = (1 << HDA_MAX_CODECS) - 1;
pub const SD_CTL_STREAM_RESET: u32 = 0x01; pub const SD_CTL_DMA_START: u32 = 0x02; pub const SD_CTL_STRIPE: u32 = 3 << 16; pub const SD_CTL_TRAFFIC_PRIO: u32 = 1 << 18; pub const SD_CTL_DIR: u32 = 1 << 19; pub const SD_CTL_STREAM_TAG_MASK: u32 = 0xf << 20; pub const SD_CTL_STREAM_TAG_SHIFT: u32 = 20;
pub const SD_INT_DESC_ERR: u32 = 0x10; pub const SD_INT_FIFO_ERR: u32 = 0x08; pub const SD_INT_COMPLETE: u32 = 0x04; pub const SD_INT_MASK: u32 = SD_INT_DESC_ERR | SD_INT_FIFO_ERR | SD_INT_COMPLETE; pub const SD_CTL_STRIPE_MASK: u32 = 0x3; pub const SD_STS_FIFO_READY: u32 = 0x20;
pub const AZX_INT_ALL_STREAM: u32 = 0x3fffffff; pub const AZX_INT_CTRL_EN: u32 = 0x40000000; pub const AZX_INT_GLOBAL_EN: u32 = 0x80000000;
pub const AZX_MAX_CORB_ENTRIES: u32 = 256; pub const AZX_MAX_RIRB_ENTRIES: u32 = 256;
pub const AZX_REG_CAP_HDR: u32 = 0x0; pub const AZX_CAP_HDR_VER_OFF: u32 = 28; pub const AZX_CAP_HDR_VER_MASK: u32 = 0xF << AZX_CAP_HDR_VER_OFF; pub const AZX_CAP_HDR_ID_OFF: u32 = 16; pub const AZX_CAP_HDR_ID_MASK: u32 = 0xFFF << AZX_CAP_HDR_ID_OFF; pub const AZX_CAP_HDR_NXT_PTR_MASK: u32 = 0xFFFF;
pub const AZX_SPB_CAP_ID: u32 = 0x4; pub const AZX_REG_SPB_BASE_ADDR: u32 = 0x700; pub const AZX_REG_SPB_SPBFCH: u32 = 0x00; pub const AZX_REG_SPB_SPBFCCTL: u32 = 0x04; pub const AZX_SPB_BASE: u32 = 0x08; pub const AZX_SPB_INTERVAL: u32 = 0x08; pub const AZX_SPB_SPIB: u32 = 0x00; pub const AZX_SPB_MAXFIFO: u32 = 0x04;
pub const AZX_GTS_CAP_ID: u32 = 0x1; pub const AZX_REG_GTS_GTSCH: u32 = 0x00; pub const AZX_REG_GTS_GTSCD: u32 = 0x04; pub const AZX_REG_GTS_GTSCTLAC: u32 = 0x0C; pub const AZX_GTS_BASE: u32 = 0x20; pub const AZX_GTS_INTERVAL: u32 = 0x20;
pub const AZX_PP_CAP_ID: u32 = 0x3; pub const AZX_REG_PP_PPCH: u32 = 0x10; pub const AZX_REG_PP_PPCTL: u32 = 0x04; pub const AZX_PPCTL_PIE: u32 = 1<<31; pub const AZX_PPCTL_GPROCEN: u32 = 1<<30; pub const AZX_REG_PP_PPSTS: u32 = 0x08; pub const AZX_PPHC_BASE: u32 = 0x10; pub const AZX_PPHC_INTERVAL: u32 = 0x10; pub const AZX_REG_PPHCLLPL: u32 = 0x0; pub const AZX_REG_PPHCLLPU: u32 = 0x4; pub const AZX_REG_PPHCLDPL: u32 = 0x8; pub const AZX_REG_PPHCLDPU: u32 = 0xC;
pub const AZX_PPLC_BASE: u32 = 0x10; pub const AZX_PPLC_MULTI: u32 = 0x10; pub const AZX_PPLC_INTERVAL: u32 = 0x10; pub const AZX_REG_PPLCCTL: u32 = 0x0; pub const AZX_PPLCCTL_STRM_BITS: u32 = 4; pub const AZX_PPLCCTL_STRM_SHIFT: u32 = 20; pub const AZX_PPLCCTL_RUN: u32 = 1<<1; pub const AZX_PPLCCTL_STRST: u32 = 1<<0; pub const AZX_REG_PPLCFMT: u32 = 0x4; pub const AZX_REG_PPLCLLPL: u32 = 0x8; pub const AZX_REG_PPLCLLPU: u32 = 0xC;
pub const AZX_ML_CAP_ID: u32 = 0x2; pub const AZX_REG_ML_MLCH: u32 = 0x00; pub const AZX_REG_ML_MLCD: u32 = 0x04; pub const AZX_ML_BASE: u32 = 0x40; pub const AZX_ML_INTERVAL: u32 = 0x40;
pub const AZX_REG_ML_LCAP: u32 = 0x00; pub const AZX_ML_HDA_LCAP_ALT: u32 = BIT(28); pub const AZX_ML_HDA_LCAP_ALT_HDA: u32 = 0; pub const AZX_ML_HDA_LCAP_ALT_HDA_EXT: u32 = 1; pub const AZX_ML_HDA_LCAP_INTC: u32 = BIT(27); pub const AZX_ML_HDA_LCAP_OFLS: u32 = BIT(26); pub const AZX_ML_HDA_LCAP_LSS: u32 = BIT(23); pub const AZX_ML_HDA_LCAP_SLCOUNT: u32 = GENMASK(22,20);
pub const AZX_REG_ML_LCTL: u32 = 0x04; pub const AZX_ML_LCTL_INTSTS: u32 = BIT(31); pub const AZX_ML_LCTL_CPA: u32 = BIT(23); pub const AZX_ML_LCTL_CPA_SHIFT: u32 = 23; pub const AZX_ML_LCTL_SPA: u32 = BIT(16); pub const AZX_ML_LCTL_SPA_SHIFT: u32 = 16; pub const AZX_ML_LCTL_INTEN: u32 = BIT(5); pub const AZX_ML_LCTL_OFLEN: u32 = BIT(4); pub const AZX_ML_LCTL_SCF: u32 = GENMASK(3,0); pub const AZX_REG_ML_LOSIDV: u32 = 0x08; pub const AZX_ML_LOSIDV_STREAM_MASK: u32 = 0xFFFE; pub const AZX_REG_ML_LSDIID: u32 = 0x0C;
#[inline] pub const fn azx_reg_ml_lsdiid_offset(x: u32) -> u32 { 0x0C + x * 0x02 }
pub const AZX_REG_ML_LPSOO: u32 = 0x10; pub const AZX_REG_ML_LPSIO: u32 = 0x12; pub const AZX_REG_ML_LWALFC: u32 = 0x18; pub const AZX_REG_ML_LOUTPAY: u32 = 0x20; pub const AZX_REG_ML_LINPAY: u32 = 0x30; pub const AZX_REG_ML_LSYNC: u32 = 0x1C; pub const AZX_REG_ML_LSYNC_CMDSYNC: u32 = BIT(24); pub const AZX_REG_ML_LSYNC_CMDSYNC_SHIFT: u32 = 24; pub const AZX_REG_ML_LSYNC_SYNCGO: u32 = BIT(23); pub const AZX_REG_ML_LSYNC_SYNCPU: u32 = BIT(20); pub const AZX_REG_ML_LSYNC_SYNCPRD: u32 = GENMASK(19,0); pub const AZX_REG_ML_LEPTR: u32 = 0x20; pub const AZX_REG_ML_LEPTR_ID: u32 = GENMASK(31,24); pub const AZX_REG_ML_LEPTR_ID_SHIFT: u32 = 24; pub const AZX_REG_ML_LEPTR_ID_SDW: u32 = 0x00; pub const AZX_REG_ML_LEPTR_ID_INTEL_SSP: u32 = 0xC0; pub const AZX_REG_ML_LEPTR_ID_INTEL_DMIC: u32 = 0xC1; pub const AZX_REG_ML_LEPTR_ID_INTEL_UAOL: u32 = 0xC2; pub const AZX_REG_ML_LEPTR_VER: u32 = GENMASK(23,20); pub const AZX_REG_ML_LEPTR_PTR: u32 = GENMASK(19,0);
pub const AZX_DRSM_CAP_ID: u32 = 0x5; pub const AZX_REG_DRSM_CTL: u32 = 0x4; pub const AZX_DRSM_BASE: u32 = 0x08; pub const AZX_DRSM_INTERVAL: u32 = 0x08;
pub const GTSCC_TSCCD_MASK: u32 = 0x80000000; pub const GTSCC_TSCCD_SHIFT: u32 = BIT(31); pub const GTSCC_TSCCI_MASK: u32 = 0x20; pub const GTSCC_CDMAS_DMA_DIR_SHIFT: u32 = 4; pub const WALFCC_CIF_MASK: u32 = 0x1FF; pub const WALFCC_FN_SHIFT: u32 = 9; pub const HDA_CLK_CYCLES_PER_FRAME: u32 = 512; pub const HDA_MAX_CYCLE_VALUE: u32 = 499; pub const HDA_MAX_CYCLE_OFFSET: u32 = 10; pub const HDA_MAX_CYCLE_READ_RETRY: u32 = 10; pub const TSCCU_CCU_SHIFT: u32 = 32; pub const LLPC_CCU_SHIFT: u32 = 32;
pub unsafe fn snd_hdac_stream_get_pos_lpib(stream: *mut hdac_stream) -> u32 { snd_hdac_stream_readl(stream, SD_LPIB) }
pub unsafe fn snd_hdac_stream_get_pos_posbuf(stream: *mut hdac_stream) -> u32 { le32_to_cpu(*(*stream).posbuf) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
