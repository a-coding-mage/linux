/* SPDX-License-Identifier: MIT */
/*
 * Copyright © 2022 Intel Corporation
 */

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum vlv_iosf_sb_unit {
    VLV_IOSF_SB_BUNIT,
    VLV_IOSF_SB_CCK,
    VLV_IOSF_SB_CCU,
    VLV_IOSF_SB_DPIO,
    VLV_IOSF_SB_DPIO_2,
    VLV_IOSF_SB_FLISDSI,
    VLV_IOSF_SB_GPIO,
    VLV_IOSF_SB_NC,
    VLV_IOSF_SB_PUNIT,
}

/* See configdb bunit SB addr map */
pub const BUNIT_REG_BISOC: u32 = 0x11;

/* PUNIT_REG_*SSPM0 */
pub const fn _SSPM0_SSC(val: u32) -> u32 { val << 0 }
pub const SSPM0_SSC_MASK: u32 = _SSPM0_SSC(0x3);
pub const SSPM0_SSC_PWR_ON: u32 = _SSPM0_SSC(0x0);
pub const SSPM0_SSC_CLK_GATE: u32 = _SSPM0_SSC(0x1);
pub const SSPM0_SSC_RESET: u32 = _SSPM0_SSC(0x2);
pub const SSPM0_SSC_PWR_GATE: u32 = _SSPM0_SSC(0x3);
pub const fn _SSPM0_SSS(val: u32) -> u32 { val << 24 }
pub const SSPM0_SSS_MASK: u32 = _SSPM0_SSS(0x3);
pub const SSPM0_SSS_PWR_ON: u32 = _SSPM0_SSS(0x0);
pub const SSPM0_SSS_CLK_GATE: u32 = _SSPM0_SSS(0x1);
pub const SSPM0_SSS_RESET: u32 = _SSPM0_SSS(0x2);
pub const SSPM0_SSS_PWR_GATE: u32 = _SSPM0_SSS(0x3);

/* PUNIT_REG_*SSPM1 */
pub const SSPM1_FREQSTAT_SHIFT: u32 = 24;
pub const SSPM1_FREQSTAT_MASK: u32 = 0x1f << SSPM1_FREQSTAT_SHIFT;
pub const SSPM1_FREQGUAR_SHIFT: u32 = 8;
pub const SSPM1_FREQGUAR_MASK: u32 = 0x1f << SSPM1_FREQGUAR_SHIFT;
pub const SSPM1_FREQ_SHIFT: u32 = 0;
pub const SSPM1_FREQ_MASK: u32 = 0x1f << SSPM1_FREQ_SHIFT;

pub const PUNIT_REG_VEDSSPM0: u32 = 0x32;
pub const PUNIT_REG_VEDSSPM1: u32 = 0x33;
pub const PUNIT_REG_DSPSSPM: u32 = 0x36;
pub const DSPFREQSTAT_SHIFT_CHV: u32 = 24;
pub const DSPFREQSTAT_MASK_CHV: u32 = 0x1f << DSPFREQSTAT_SHIFT_CHV;
pub const DSPFREQGUAR_SHIFT_CHV: u32 = 8;
pub const DSPFREQGUAR_MASK_CHV: u32 = 0x1f << DSPFREQGUAR_SHIFT_CHV;
pub const DSPFREQSTAT_SHIFT: u32 = 30;
pub const DSPFREQSTAT_MASK: u32 = 0x3 << DSPFREQSTAT_SHIFT;
pub const DSPFREQGUAR_SHIFT: u32 = 14;
pub const DSPFREQGUAR_MASK: u32 = 0x3 << DSPFREQGUAR_SHIFT;
pub const DSP_MAXFIFO_PM5_STATUS: u32 = 1 << 22; /* chv */
pub const DSP_AUTO_CDCLK_GATE_DISABLE: u32 = 1 << 7; /* chv */
pub const DSP_MAXFIFO_PM5_ENABLE: u32 = 1 << 6; /* chv */
pub const fn _DP_SSC(val: u32, pipe: u32) -> u32 { val << (2 * pipe) }
pub const fn DP_SSC_MASK(pipe: u32) -> u32 { _DP_SSC(0x3, pipe) }
pub const fn DP_SSC_PWR_ON(pipe: u32) -> u32 { _DP_SSC(0x0, pipe) }
pub const fn DP_SSC_CLK_GATE(pipe: u32) -> u32 { _DP_SSC(0x1, pipe) }
pub const fn DP_SSC_RESET(pipe: u32) -> u32 { _DP_SSC(0x2, pipe) }
pub const fn DP_SSC_PWR_GATE(pipe: u32) -> u32 { _DP_SSC(0x3, pipe) }
pub const fn _DP_SSS(val: u32, pipe: u32) -> u32 { val << (2 * pipe + 16) }
pub const fn DP_SSS_MASK(pipe: u32) -> u32 { _DP_SSS(0x3, pipe) }
pub const fn DP_SSS_PWR_ON(pipe: u32) -> u32 { _DP_SSS(0x0, pipe) }
pub const fn DP_SSS_CLK_GATE(pipe: u32) -> u32 { _DP_SSS(0x1, pipe) }
pub const fn DP_SSS_RESET(pipe: u32) -> u32 { _DP_SSS(0x2, pipe) }
pub const fn DP_SSS_PWR_GATE(pipe: u32) -> u32 { _DP_SSS(0x3, pipe) }

pub const PUNIT_REG_ISPSSPM0: u32 = 0x39;
pub const PUNIT_REG_ISPSSPM1: u32 = 0x3a;
pub const PUNIT_REG_PWRGT_CTRL: u32 = 0x60;
pub const PUNIT_REG_PWRGT_STATUS: u32 = 0x61;
pub const fn PUNIT_PWRGT_MASK(pw_idx: u32) -> u32 { 3 << (pw_idx * 2) }
pub const fn PUNIT_PWRGT_PWR_ON(pw_idx: u32) -> u32 { 0 << (pw_idx * 2) }
pub const fn PUNIT_PWRGT_CLK_GATE(pw_idx: u32) -> u32 { 1 << (pw_idx * 2) }
pub const fn PUNIT_PWRGT_RESET(pw_idx: u32) -> u32 { 2 << (pw_idx * 2) }
pub const fn PUNIT_PWRGT_PWR_GATE(pw_idx: u32) -> u32 { 3 << (pw_idx * 2) }

pub const PUNIT_PWGT_IDX_RENDER: u32 = 0;
pub const PUNIT_PWGT_IDX_MEDIA: u32 = 1;
pub const PUNIT_PWGT_IDX_DISP2D: u32 = 3;
pub const PUNIT_PWGT_IDX_DPIO_CMN_BC: u32 = 5;
pub const PUNIT_PWGT_IDX_DPIO_TX_B_LANES_01: u32 = 6;
pub const PUNIT_PWGT_IDX_DPIO_TX_B_LANES_23: u32 = 7;
pub const PUNIT_PWGT_IDX_DPIO_TX_C_LANES_01: u32 = 8;
pub const PUNIT_PWGT_IDX_DPIO_TX_C_LANES_23: u32 = 9;
pub const PUNIT_PWGT_IDX_DPIO_RX0: u32 = 10;
pub const PUNIT_PWGT_IDX_DPIO_RX1: u32 = 11;
pub const PUNIT_PWGT_IDX_DPIO_CMN_D: u32 = 12;

pub const PUNIT_REG_GPU_LFM: u32 = 0xd3;
pub const PUNIT_REG_GPU_FREQ_REQ: u32 = 0xd4;
pub const PUNIT_REG_GPU_FREQ_STS: u32 = 0xd8;
pub const GPLLENABLE: u32 = 1 << 4;
pub const GENFREQSTATUS: u32 = 1 << 0;
pub const PUNIT_REG_MEDIA_TURBO_FREQ_REQ: u32 = 0xdc;
pub const PUNIT_REG_CZ_TIMESTAMP: u32 = 0xce;
pub const PUNIT_FUSE_BUS2: u32 = 0xf6; /* bits 47:40 */
pub const PUNIT_FUSE_BUS1: u32 = 0xf5; /* bits 55:48 */
pub const FB_GFX_FMAX_AT_VMAX_FUSE: u32 = 0x136;
pub const FB_GFX_FREQ_FUSE_MASK: u32 = 0xff;
pub const FB_GFX_FMAX_AT_VMAX_2SS4EU_FUSE_SHIFT: u32 = 24;
pub const FB_GFX_FMAX_AT_VMAX_2SS6EU_FUSE_SHIFT: u32 = 16;
pub const FB_GFX_FMAX_AT_VMAX_2SS8EU_FUSE_SHIFT: u32 = 8;
pub const FB_GFX_FMIN_AT_VMIN_FUSE: u32 = 0x137;
pub const FB_GFX_FMIN_AT_VMIN_FUSE_SHIFT: u32 = 8;
pub const PUNIT_REG_DDR_SETUP2: u32 = 0x139;
pub const FORCE_DDR_FREQ_REQ_ACK: u32 = 1 << 8;
pub const FORCE_DDR_LOW_FREQ: u32 = 1 << 1;
pub const FORCE_DDR_HIGH_FREQ: u32 = 1 << 0;
pub const PUNIT_GPU_STATUS_REG: u32 = 0xdb;
pub const PUNIT_GPU_STATUS_MAX_FREQ_SHIFT: u32 = 16;
pub const PUNIT_GPU_STATUS_MAX_FREQ_MASK: u32 = 0xff;
pub const PUNIT_GPU_STATIS_GFX_MIN_FREQ_SHIFT: u32 = 8;
pub const PUNIT_GPU_STATUS_GFX_MIN_FREQ_MASK: u32 = 0xff;
pub const PUNIT_GPU_DUTYCYCLE_REG: u32 = 0xdf;
pub const PUNIT_GPU_DUTYCYCLE_RPE_FREQ_SHIFT: u32 = 8;
pub const PUNIT_GPU_DUTYCYCLE_RPE_FREQ_MASK: u32 = 0xff;
pub const IOSF_NC_FB_GFX_FREQ_FUSE: u32 = 0x1c;
pub const FB_GFX_MAX_FREQ_FUSE_SHIFT: u32 = 3;
pub const FB_GFX_MAX_FREQ_FUSE_MASK: u32 = 0x000007f8;
pub const FB_GFX_FGUARANTEED_FREQ_FUSE_SHIFT: u32 = 11;
pub const FB_GFX_FGUARANTEED_FREQ_FUSE_MASK: u32 = 0x0007f800;
pub const IOSF_NC_FB_GFX_FMAX_FUSE_HI: u32 = 0x34;
pub const FB_FMAX_VMIN_FREQ_HI_MASK: u32 = 0x00000007;
pub const IOSF_NC_FB_GFX_FMAX_FUSE_LO: u32 = 0x30;
pub const FB_FMAX_VMIN_FREQ_LO_SHIFT: u32 = 27;
pub const FB_FMAX_VMIN_FREQ_LO_MASK: u32 = 0xf8000000;
pub const VLV_TURBO_SOC_OVERRIDE: u32 = 0x04;
pub const VLV_OVERRIDE_EN: u32 = 1;
pub const VLV_SOC_TDP_EN: u32 = 1 << 1;
pub const VLV_BIAS_CPU_125_SOC_875: u32 = 6 << 2;
pub const CHV_BIAS_CPU_50_SOC_50: u32 = 3 << 2;

/* vlv2 north clock has */
pub const CCK_FUSE_REG: u32 = 0x8;
pub const CCK_FUSE_HPLL_FREQ_MASK: u32 = 0x3;
pub const CCK_REG_DSI_PLL_FUSE: u32 = 0x44;
pub const CCK_REG_DSI_PLL_CONTROL: u32 = 0x48;
pub const DSI_PLL_VCO_EN: u32 = 1 << 31;
pub const DSI_PLL_LDO_GATE: u32 = 1 << 30;
pub const DSI_PLL_P1_POST_DIV_SHIFT: u32 = 17;
pub const DSI_PLL_P1_POST_DIV_MASK: u32 = 0x1ff << 17;
pub const DSI_PLL_P2_MUX_DSI0_DIV2: u32 = 1 << 13;
pub const DSI_PLL_P3_MUX_DSI1_DIV2: u32 = 1 << 12;
pub const DSI_PLL_MUX_MASK: u32 = 3 << 9;
pub const DSI_PLL_MUX_DSI0_DSIPLL: u32 = 0 << 10;
pub const DSI_PLL_MUX_DSI0_CCK: u32 = 1 << 10;
pub const DSI_PLL_MUX_DSI1_DSIPLL: u32 = 0 << 9;
pub const DSI_PLL_MUX_DSI1_CCK: u32 = 1 << 9;
pub const DSI_PLL_CLK_GATE_MASK: u32 = 0xf << 5;
pub const DSI_PLL_CLK_GATE_DSI0_DSIPLL: u32 = 1 << 8;
pub const DSI_PLL_CLK_GATE_DSI1_DSIPLL: u32 = 1 << 7;
pub const DSI_PLL_CLK_GATE_DSI0_CCK: u32 = 1 << 6;
pub const DSI_PLL_CLK_GATE_DSI1_CCK: u32 = 1 << 5;
pub const DSI_PLL_LOCK: u32 = 1 << 0;
pub const CCK_REG_DSI_PLL_DIVIDER: u32 = 0x4c;
pub const DSI_PLL_LFSR: u32 = 1 << 31;
pub const DSI_PLL_FRACTION_EN: u32 = 1 << 30;
pub const DSI_PLL_FRAC_COUNTER_SHIFT: u32 = 27;
pub const DSI_PLL_FRAC_COUNTER_MASK: u32 = 7 << 27;
pub const DSI_PLL_USYNC_CNT_SHIFT: u32 = 18;
pub const DSI_PLL_USYNC_CNT_MASK: u32 = 0x1ff << 18;
pub const DSI_PLL_N1_DIV_SHIFT: u32 = 16;
pub const DSI_PLL_N1_DIV_MASK: u32 = 3 << 16;
pub const DSI_PLL_M1_DIV_SHIFT: u32 = 0;
pub const DSI_PLL_M1_DIV_MASK: u32 = 0x1ff << 0;
pub const CCK_CZ_CLOCK_CONTROL: u32 = 0x62;
pub const CCK_GPLL_CLOCK_CONTROL: u32 = 0x67;
pub const CCK_DISPLAY_CLOCK_CONTROL: u32 = 0x6b;
pub const CCK_DISPLAY_REF_CLOCK_CONTROL: u32 = 0x6c;
pub const CCK_TRUNK_FORCE_ON: u32 = 1 << 17;
pub const CCK_TRUNK_FORCE_OFF: u32 = 1 << 16;
pub const CCK_FREQUENCY_STATUS: u32 = 0x1f << 8;
pub const CCK_FREQUENCY_STATUS_SHIFT: u32 = 8;
pub const CCK_FREQUENCY_VALUES: u32 = 0x1f << 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
