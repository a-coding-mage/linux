// SPDX-License-Identifier: GPL-2.0

pub const WM8766_LDA1: u32 = 0x00;
pub const WM8766_RDA1: u32 = 0x01;
pub const WM8766_DAC_CTRL: u32 = 0x02;
pub const WM8766_INT_CTRL: u32 = 0x03;
pub const WM8766_LDA2: u32 = 0x04;
pub const WM8766_RDA2: u32 = 0x05;
pub const WM8766_LDA3: u32 = 0x06;
pub const WM8766_RDA3: u32 = 0x07;
pub const WM8766_MASTDA: u32 = 0x08;
pub const WM8766_DAC_CTRL2: u32 = 0x09;
pub const WM8766_DAC_CTRL3: u32 = 0x0a;
pub const WM8766_MUTE1: u32 = 0x0c;
pub const WM8766_MUTE2: u32 = 0x0f;
pub const WM8766_RESET: u32 = 0x1f;

/* LDAx/RDAx/MASTDA */
pub const WM8766_ATT_MASK: u32 = 0x0ff;
pub const WM8766_UPDATE: u32 = 0x100;
/* DAC_CTRL */
pub const WM8766_MUTEALL: u32 = 0x001;
pub const WM8766_DEEMPALL: u32 = 0x002;
pub const WM8766_PWDN: u32 = 0x004;
pub const WM8766_ATC: u32 = 0x008;
pub const WM8766_IZD: u32 = 0x010;
pub const WM8766_PL_LEFT_MASK: u32 = 0x060;
pub const WM8766_PL_LEFT_MUTE: u32 = 0x000;
pub const WM8766_PL_LEFT_LEFT: u32 = 0x020;
pub const WM8766_PL_LEFT_RIGHT: u32 = 0x040;
pub const WM8766_PL_LEFT_LRMIX: u32 = 0x060;
pub const WM8766_PL_RIGHT_MASK: u32 = 0x180;
pub const WM8766_PL_RIGHT_MUTE: u32 = 0x000;
pub const WM8766_PL_RIGHT_LEFT: u32 = 0x080;
pub const WM8766_PL_RIGHT_RIGHT: u32 = 0x100;
pub const WM8766_PL_RIGHT_LRMIX: u32 = 0x180;
/* INT_CTRL */
pub const WM8766_FMT_MASK: u32 = 0x003;
pub const WM8766_FMT_RJUST: u32 = 0x000;
pub const WM8766_FMT_LJUST: u32 = 0x001;
pub const WM8766_FMT_I2S: u32 = 0x002;
pub const WM8766_FMT_DSP: u32 = 0x003;
pub const WM8766_LRP: u32 = 0x004;
pub const WM8766_BCP: u32 = 0x008;
pub const WM8766_IWL_MASK: u32 = 0x030;
pub const WM8766_IWL_16: u32 = 0x000;
pub const WM8766_IWL_20: u32 = 0x010;
pub const WM8766_IWL_24: u32 = 0x020;
pub const WM8766_IWL_32: u32 = 0x030;
pub const WM8766_PHASE_MASK: u32 = 0x1c0;
/* DAC_CTRL2 */
pub const WM8766_ZCD: u32 = 0x001;
pub const WM8766_DZFM_MASK: u32 = 0x006;
pub const WM8766_DMUTE_MASK: u32 = 0x038;
pub const WM8766_DEEMP_MASK: u32 = 0x1c0;
/* DAC_CTRL3 */
pub const WM8766_DACPD_MASK: u32 = 0x00e;
pub const WM8766_PWRDNALL: u32 = 0x010;
pub const WM8766_MS: u32 = 0x020;
pub const WM8766_RATE_MASK: u32 = 0x1c0;
pub const WM8766_RATE_128: u32 = 0x000;
pub const WM8766_RATE_192: u32 = 0x040;
pub const WM8766_RATE_256: u32 = 0x080;
pub const WM8766_RATE_384: u32 = 0x0c0;
pub const WM8766_RATE_512: u32 = 0x100;
pub const WM8766_RATE_768: u32 = 0x140;
/* MUTE1 */
pub const WM8766_MPD1: u32 = 0x040;
/* MUTE2 */
pub const WM8766_MPD2: u32 = 0x020;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
