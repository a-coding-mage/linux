// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Copyright (c) by James Courtier-Dutton <James@superbug.demon.co.uk>
 *  Driver p17v chips
 */

/******************************************************************************/
/* Audigy2Value Tina (P17V) pointer-offset register set,                      */
/* accessed through the PTR2 and DATA2 registers                              */
/******************************************************************************/

/* 00 - 07: Not used */
pub const P17V_PLAYBACK_FIFO_PTR: u32 = 0x08; /* Current playback fifo pointer
                                              * and number of sound samples in cache.
                                              */
/* 09 - 12: Not used */
pub const P17V_CAPTURE_FIFO_PTR: u32 = 0x13; /* Current capture fifo pointer
                                             * and number of sound samples in cache.
                                             */
/* 14 - 17: Not used */
pub const P17V_PB_CHN_SEL: u32 = 0x18; /* P17v playback channel select */
pub const P17V_SE_SLOT_SEL_L: u32 = 0x19; /* Sound Engine slot select low */
pub const P17V_SE_SLOT_SEL_H: u32 = 0x1a; /* Sound Engine slot select high */
/* 1b - 1f: Not used */
/* 20 - 2f: Not used */
/* 30 - 3b: Not used */
pub const P17V_SPI: u32 = 0x3c; /* SPI interface register */
pub const P17V_I2C_ADDR: u32 = 0x3d; /* I2C Address */
pub const P17V_I2C_0: u32 = 0x3e; /* I2C Data */
pub const P17V_I2C_1: u32 = 0x3f; /* I2C Data */
/* I2C values */
pub const I2C_A_ADC_ADD_MASK: u32 = 0x000000fe; /*The address is a 7 bit address */
pub const I2C_A_ADC_RW_MASK: u32 = 0x00000001; /*bit mask for R/W */
pub const I2C_A_ADC_TRANS_MASK: u32 = 0x00000010; /*Bit mask for I2c address DAC value  */
pub const I2C_A_ADC_ABORT_MASK: u32 = 0x00000020; /*Bit mask for I2C transaction abort flag */
pub const I2C_A_ADC_LAST_MASK: u32 = 0x00000040; /*Bit mask for Last word transaction */
pub const I2C_A_ADC_BYTE_MASK: u32 = 0x00000080; /*Bit mask for Byte Mode */

pub const I2C_A_ADC_ADD: u32 = 0x00000034; /*This is the Device address for ADC  */
pub const I2C_A_ADC_READ: u32 = 0x00000001; /*To perform a read operation */
pub const I2C_A_ADC_START: u32 = 0x00000100; /*Start I2C transaction */
pub const I2C_A_ADC_ABORT: u32 = 0x00000200; /*I2C transaction abort */
pub const I2C_A_ADC_LAST: u32 = 0x00000400; /*I2C last transaction */
pub const I2C_A_ADC_BYTE: u32 = 0x00000800; /*I2C one byte mode */

pub const I2C_D_ADC_REG_MASK: u32 = 0xfe000000; /*ADC address register */
pub const I2C_D_ADC_DAT_MASK: u32 = 0x01ff0000; /*ADC data register */

pub const ADC_TIMEOUT: u32 = 0x00000007; /*ADC Timeout Clock Disable */
pub const ADC_IFC_CTRL: u32 = 0x0000000b; /*ADC Interface Control */
pub const ADC_MASTER: u32 = 0x0000000c; /*ADC Master Mode Control */
pub const ADC_POWER: u32 = 0x0000000d; /*ADC PowerDown Control */
pub const ADC_ATTEN_ADCL: u32 = 0x0000000e; /*ADC Attenuation ADCL */
pub const ADC_ATTEN_ADCR: u32 = 0x0000000f; /*ADC Attenuation ADCR */
pub const ADC_ALC_CTRL1: u32 = 0x00000010; /*ADC ALC Control 1 */
pub const ADC_ALC_CTRL2: u32 = 0x00000011; /*ADC ALC Control 2 */
pub const ADC_ALC_CTRL3: u32 = 0x00000012; /*ADC ALC Control 3 */
pub const ADC_NOISE_CTRL: u32 = 0x00000013; /*ADC Noise Gate Control */
pub const ADC_LIMIT_CTRL: u32 = 0x00000014; /*ADC Limiter Control */
pub const ADC_MUX: u32 = 0x00000015; /*ADC Mux offset */

/*
 * Disabled in the original C header with `#if 0`.
 * FIXME: Not tested yet.
 *
 * pub const ADC_GAIN_MASK: u32 = 0x000000ff; //Mask for ADC Gain
 * pub const ADC_ZERODB: u32 = 0x000000cf; //Value to set ADC to 0dB
 * pub const ADC_MUTE_MASK: u32 = 0x000000c0; //Mask for ADC mute
 * pub const ADC_MUTE: u32 = 0x000000c0; //Value to mute ADC
 * pub const ADC_OSR: u32 = 0x00000008; //Mask for ADC oversample rate select
 * pub const ADC_TIMEOUT_DISABLE: u32 = 0x00000008; //Value and mask to disable Timeout clock
 * pub const ADC_HPF_DISABLE: u32 = 0x00000100; //Value and mask to disable High pass filter
 * pub const ADC_TRANWIN_MASK: u32 = 0x00000070; //Mask for Length of Transient Window
 */

pub const ADC_MUX_MASK: u32 = 0x0000000f; //Mask for ADC Mux
pub const ADC_MUX_0: u32 = 0x00000001; //Value to select Unknown at ADC Mux (Not used)
pub const ADC_MUX_1: u32 = 0x00000002; //Value to select Unknown at ADC Mux (Not used)
pub const ADC_MUX_2: u32 = 0x00000004; //Value to select Mic at ADC Mux
pub const ADC_MUX_3: u32 = 0x00000008; //Value to select Line-In at ADC Mux

pub const P17V_START_AUDIO: u32 = 0x40; /* Start Audio bit */
/* 41 - 47: Reserved */
pub const P17V_START_CAPTURE: u32 = 0x48; /* Start Capture bit */
pub const P17V_CAPTURE_FIFO_BASE: u32 = 0x49; /* Record FIFO base address */
pub const P17V_CAPTURE_FIFO_SIZE: u32 = 0x4a; /* Record FIFO buffer size */
pub const P17V_CAPTURE_FIFO_INDEX: u32 = 0x4b; /* Record FIFO capture index */
pub const P17V_CAPTURE_VOL_H: u32 = 0x4c; /* P17v capture volume control */
pub const P17V_CAPTURE_VOL_L: u32 = 0x4d; /* P17v capture volume control */
/* 4e - 4f: Not used */
/* 50 - 5f: Not used */
pub const P17V_SRCSel: u32 = 0x60; /* SRC48 and SRCMulti sample rate select
                                    * and output select
                                    */
pub const P17V_MIXER_AC97_10K1_VOL_L: u32 = 0x61; /* 10K to Mixer_AC97 input volume control */
pub const P17V_MIXER_AC97_10K1_VOL_H: u32 = 0x62; /* 10K to Mixer_AC97 input volume control */
pub const P17V_MIXER_AC97_P17V_VOL_L: u32 = 0x63; /* P17V to Mixer_AC97 input volume control */
pub const P17V_MIXER_AC97_P17V_VOL_H: u32 = 0x64; /* P17V to Mixer_AC97 input volume control */
pub const P17V_MIXER_AC97_SRP_REC_VOL_L: u32 = 0x65; /* SRP Record to Mixer_AC97 input volume control */
pub const P17V_MIXER_AC97_SRP_REC_VOL_H: u32 = 0x66; /* SRP Record to Mixer_AC97 input volume control */
/* 67 - 68: Reserved */
pub const P17V_MIXER_Spdif_10K1_VOL_L: u32 = 0x69; /* 10K to Mixer_Spdif input volume control */
pub const P17V_MIXER_Spdif_10K1_VOL_H: u32 = 0x6A; /* 10K to Mixer_Spdif input volume control */
pub const P17V_MIXER_Spdif_P17V_VOL_L: u32 = 0x6B; /* P17V to Mixer_Spdif input volume control */
pub const P17V_MIXER_Spdif_P17V_VOL_H: u32 = 0x6C; /* P17V to Mixer_Spdif input volume control */
pub const P17V_MIXER_Spdif_SRP_REC_VOL_L: u32 = 0x6D; /* SRP Record to Mixer_Spdif input volume control */
pub const P17V_MIXER_Spdif_SRP_REC_VOL_H: u32 = 0x6E; /* SRP Record to Mixer_Spdif input volume control */
/* 6f - 70: Reserved */
pub const P17V_MIXER_I2S_10K1_VOL_L: u32 = 0x71; /* 10K to Mixer_I2S input volume control */
pub const P17V_MIXER_I2S_10K1_VOL_H: u32 = 0x72; /* 10K to Mixer_I2S input volume control */
pub const P17V_MIXER_I2S_P17V_VOL_L: u32 = 0x73; /* P17V to Mixer_I2S input volume control */
pub const P17V_MIXER_I2S_P17V_VOL_H: u32 = 0x74; /* P17V to Mixer_I2S input volume control */
pub const P17V_MIXER_I2S_SRP_REC_VOL_L: u32 = 0x75; /* SRP Record to Mixer_I2S input volume control */
pub const P17V_MIXER_I2S_SRP_REC_VOL_H: u32 = 0x76; /* SRP Record to Mixer_I2S input volume control */
/* 77 - 78: Reserved */
pub const P17V_MIXER_AC97_ENABLE: u32 = 0x79; /* Mixer AC97 input audio enable */
pub const P17V_MIXER_SPDIF_ENABLE: u32 = 0x7A; /* Mixer SPDIF input audio enable */
pub const P17V_MIXER_I2S_ENABLE: u32 = 0x7B; /* Mixer I2S input audio enable */
pub const P17V_AUDIO_OUT_ENABLE: u32 = 0x7C; /* Audio out enable */
pub const P17V_MIXER_ATT: u32 = 0x7D; /* SRP Mixer Attenuation Select */
pub const P17V_SRP_RECORD_SRR: u32 = 0x7E; /* SRP Record channel source Select */
pub const P17V_SOFT_RESET_SRP_MIXER: u32 = 0x7F; /* SRP and mixer soft reset */

pub const P17V_AC97_OUT_MASTER_VOL_L: u32 = 0x80; /* AC97 Output master volume control */
pub const P17V_AC97_OUT_MASTER_VOL_H: u32 = 0x81; /* AC97 Output master volume control */
pub const P17V_SPDIF_OUT_MASTER_VOL_L: u32 = 0x82; /* SPDIF Output master volume control */
pub const P17V_SPDIF_OUT_MASTER_VOL_H: u32 = 0x83; /* SPDIF Output master volume control */
pub const P17V_I2S_OUT_MASTER_VOL_L: u32 = 0x84; /* I2S Output master volume control */
pub const P17V_I2S_OUT_MASTER_VOL_H: u32 = 0x85; /* I2S Output master volume control */
/* 86 - 87: Not used */
pub const P17V_I2S_CHANNEL_SWAP_PHASE_INVERSE: u32 = 0x88; /* I2S out mono channel swap
                                                            * and phase inverse */
pub const P17V_SPDIF_CHANNEL_SWAP_PHASE_INVERSE: u32 = 0x89; /* SPDIF out mono channel swap
                                                              * and phase inverse */
/* 8A: Not used */
pub const P17V_SRP_P17V_ESR: u32 = 0x8B; /* SRP_P17V estimated sample rate and rate lock */
pub const P17V_SRP_REC_ESR: u32 = 0x8C; /* SRP_REC estimated sample rate and rate lock */
pub const P17V_SRP_BYPASS: u32 = 0x8D; /* srps channel bypass and srps bypass */
/* 8E - 92: Not used */
pub const P17V_I2S_SRC_SEL: u32 = 0x93; /* I2SIN mode sel */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
