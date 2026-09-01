// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) Intel Corporation, 2022
 * Copyright Everest Semiconductor Co.,Ltd
 */

/* Definitions extracted from ASL file provided at
 * https://github.com/thesofproject/linux/files/9398723/ESSX8326.zip
 */

/***************************************************
 *  DSM arguments                                  *
 ***************************************************/

pub const PLATFORM_MAINMIC_TYPE_ARG: u32 = 0x00;
pub const PLATFORM_HPMIC_TYPE_ARG: u32 = 0x01;
pub const PLATFORM_SPK_TYPE_ARG: u32 = 0x02;
pub const PLATFORM_HPDET_INV_ARG: u32 = 0x03;
pub const PLATFORM_PCM_TYPE_ARG: u32 = 0x04;

pub const PLATFORM_MIC_DE_POP_ARG: u32 = 0x06;
pub const PLATFORM_CODEC_TYPE_ARG: u32 = 0x0E;
pub const PLATFORM_BUS_SLOT_ARG: u32 = 0x0F;

pub const HP_CODEC_LINEIN_PGA_GAIN_ARG: u32 = 0x10;
pub const MAIN_CODEC_LINEIN_PGA_GAIN_ARG: u32 = 0x20;

pub const HP_CODEC_D2SEPGA_GAIN_ARG: u32 = 0x11;
pub const MAIN_CODEC_D2SEPGA_GAIN_ARG: u32 = 0x21;

pub const HP_CODEC_ADC_VOLUME_ARG: u32 = 0x12;
pub const MAIN_CODEC_ADC_VOLUME_ARG: u32 = 0x22;

pub const HP_CODEC_ADC_ALC_ENABLE_ARG: u32 = 0x13;
pub const MAIN_CODEC_ADC_ALC_ENABLE_ARG: u32 = 0x23;

pub const HP_CODEC_ADC_ALC_TARGET_LEVEL_ARG: u32 = 0x14;
pub const MAIN_CODEC_ADC_ALC_TARGET_LEVEL_ARG: u32 = 0x24;

pub const HP_CODEC_ADC_ALC_MAXGAIN_ARG: u32 = 0x15;
pub const MAIN_CODEC_ADC_ALC_MAXGAIN_ARG: u32 = 0x25;

pub const HP_CODEC_ADC_ALC_MINGAIN_ARG: u32 = 0x16;
pub const MAIN_CODEC_ADC_ALC_MINGAIN_ARG: u32 = 0x26;

pub const HP_CODEC_ADC_ALC_HLDTIME_ARG: u32 = 0x17;
pub const MAIN_CODEC_ADC_ALC_HLDTIME_ARG: u32 = 0x27;

pub const HP_CODEC_ADC_ALC_DCYTIME_ARG: u32 = 0x18;
pub const MAIN_CODEC_ADC_ALC_DCYTIME_ARG: u32 = 0x28;

pub const HP_CODEC_ADC_ALC_ATKTIME_ARG: u32 = 0x19;
pub const MAIN_CODEC_ADC_ALC_ATKTIME_ARG: u32 = 0x29;

pub const HP_CODEC_ADC_ALC_NGTYPE_ARG: u32 = 0x1a;
pub const MAIN_CODEC_ADC_ALC_NGTYPE_ARG: u32 = 0x2a;

pub const HP_CODEC_ADC_ALC_NGTHLD_ARG: u32 = 0x1b;
pub const MAIN_CODEC_ADC_ALC_NGTHLD_ARG: u32 = 0x2b;

pub const MAIN_CODEC_ADC_GUI_STEP_ARG: u32 = 0x2c;
pub const MAIN_CODEC_ADC_GUI_GAIN_RANGE_ARG: u32 = 0x2c;

pub const HEADPHONE_DUMMY_REMOVE_ENABLE_ARG: u32 = 0x2e;

pub const HP_CODEC_DAC_HPMIX_HIGAIN_ARG: u32 = 0x40;
pub const SPK_CODEC_DAC_HPMIX_HIGAIN_ARG: u32 = 0x50;

pub const HP_CODEC_DAC_HPMIX_VOLUME_ARG: u32 = 0x41;
pub const SPK_CODEC_DAC_HPMIX_VOLUME_ARG: u32 = 0x51;

pub const HP_CODEC_DAC_HPOUT_VOLUME_ARG: u32 = 0x42;
pub const SPK_CODEC_DAC_HPOUT_VOLUME_ARG: u32 = 0x52;

pub const HP_CODEC_LDAC_VOLUME_ARG: u32 = 0x44;
pub const HP_CODEC_RDAC_VOLUME_ARG: u32 = 0x54;

pub const SPK_CODEC_LDAC_VOLUME_ARG: u32 = 0x45;
pub const SPK_CODEC_RDAC_VOLUME_ARG: u32 = 0x55;

pub const HP_CODEC_DAC_AUTOMUTE_ARG: u32 = 0x46;
pub const SPK_CODEC_DAC_AUTOMUTE_ARG: u32 = 0x56;

pub const HP_CODEC_DAC_MONO_ARG: u32 = 0x4A;
pub const SPK_CODEC_DAC_MONO_ARG: u32 = 0x5A;

pub const HP_CTL_IO_LEVEL_ARG: u32 = 0x4B;
pub const SPK_CTL_IO_LEVEL_ARG: u32 = 0x5B;

pub const CODEC_GPIO0_FUNC_ARG: u32 = 0x80;
pub const CODEC_GPIO1_FUNC_ARG: u32 = 0x81;
pub const CODEC_GPIO2_FUNC_ARG: u32 = 0x82;
pub const CODEC_GPIO3_FUNC_ARG: u32 = 0x83;
pub const CODEC_GPIO4_FUNC_ARG: u32 = 0x84;

pub const PLATFORM_MCLK_LRCK_FREQ_ARG: u32 = 0x85;

/***************************************************
 *  Values for arguments                           *
 ***************************************************/

/* Main and HP Mic */
pub const PLATFORM_MIC_DMIC_HIGH_LEVEL: u32 = 0xAA;
pub const PLATFORM_MIC_DMIC_LOW_LEVEL: u32 = 0x55;
pub const PLATFORM_MIC_AMIC_LIN1RIN1: u32 = 0xBB;
pub const PLATFORM_MIC_AMIC_LIN2RIN2: u32 = 0xCC;

/* Speaker */
pub const PLATFORM_SPK_NONE: u32 = 0x00;
pub const PLATFORM_SPK_MONO: u32 = 0x01;
pub const PLATFORM_SPK_STEREO: u32 = 0x02;

/* Jack Detection */
pub const PLATFORM_HPDET_NORMAL: u32 = 0x00;
pub const PLATFORM_HPDET_INVERTED: u32 = 0x01;

/* PCM type (Port number + protocol) */
/*
 * RETURNED VALUE = 0x00,   PCM PORT0, I2S
 *       0x01,   PCM PORT0, LJ
 *       0x02,   PCM PORT0, RJ
 *       0x03,   PCM PORT0, DSP-A
 *       0x04,   PCM PORT0, DSP-B
 *       0x10,   PCM PORT1, I2S
 *       0x11,   PCM PORT1, LJ
 *       0x12,   PCM PORT1, RJ
 *       0x13,   PCM PORT1, DSP-A
 *       0x14,   PCM PORT1, DSP-B
 *       0xFF,   Use default
 *
 * This is not used in Linux (defined by topology) and in
 * Windows it's always DSP-A
 */

/* Depop */
pub const PLATFORM_MIC_DE_POP_OFF: u32 = 0x00;
pub const PLATFORM_MIC_DE_POP_ON: u32 = 0x01;

/* Codec type */
pub const PLATFORM_CODEC_8316: u32 = 16;
pub const PLATFORM_CODEC_8326: u32 = 26;
pub const PLATFORM_CODEC_8336: u32 = 36;
pub const PLATFORM_CODEC_8395: u32 = 95;
pub const PLATFORM_CODEC_8396: u32 = 96;

/* Bus slot (on the host) */
/* BIT[3:0] FOR BUS NUMBER, BIT[7:4] FOR SLOT NUMBER
 * BIT[3:0] 0 for I2S0, 1 for IS21, 2 for I2S2.
 *
 * On Intel platforms this refers to SSP0..2. This information
 * is not really useful for Linux, the information is already
 * inferred from NHLT but can be used to double-check NHLT
 */

/* Volume - Gain */
pub const LINEIN_GAIN_0db: u32 = 0x00; /* gain =  0db */
pub const LINEIN_GAIN_3db: u32 = 0x01; /* gain = +3db */
pub const LINEIN_GAIN_6db: u32 = 0x02; /* gain = +6db */
pub const LINEIN_GAIN_9db: u32 = 0x03; /* gain = +9db */
pub const LINEIN_GAIN_12db: u32 = 0x04; /* gain = +12db */
pub const LINEIN_GAIN_15db: u32 = 0x05; /* gain = +15db */
pub const LINEIN_GAIN_18db: u32 = 0x06; /* gain = +18db */
pub const LINEIN_GAIN_21db: u32 = 0x07; /* gain = +21db */
pub const LINEIN_GAIN_24db: u32 = 0x08; /* gain = +24db */
pub const LINEIN_GAIN_27db: u32 = 0x09; /* gain = +27db */
pub const LINEIN_GAIN_30db: u32 = 0x0a; /* gain = +30db */

pub const ADC_GUI_STEP_3db: u32 = 0x03; /* gain = +3db */
pub const ADC_GUI_STEP_6db: u32 = 0x06; /* gain = +6db */
pub const ADC_GUI_STEP_10db: u32 = 0x0a; /* gain = +10db */

pub const D2SEPGA_GAIN_0db: u32 = 0x00; /* gain =   0db */
pub const D2SEPGA_GAIN_15db: u32 = 0x01; /* gain = +15db */

/* ADC volume: base = 0db, -0.5db/setp, 0xc0 <-> -96db */

pub const ADC_ALC_DISABLE: u32 = 0x00;
pub const ADC_ALC_ENABLE: u32 = 0x01;

pub const ADC_ALC_TARGET_LEVEL_m16_5db: u32 = 0x00; /* gain = -16.5db */
pub const ADC_ALC_TARGET_LEVEL_m15db: u32 = 0x01; /* gain = -15db */
pub const ADC_ALC_TARGET_LEVEL_m13_5db: u32 = 0x02; /* gain = -13.5db */
pub const ADC_ALC_TARGET_LEVEL_m12db: u32 = 0x03; /* gain = -12db */
pub const ADC_ALC_TARGET_LEVEL_m10_5db: u32 = 0x04; /* gain = -10.5db */
pub const ADC_ALC_TARGET_LEVEL_m9db: u32 = 0x05; /* gain = -9db */
pub const ADC_ALC_TARGET_LEVEL_m7_5db: u32 = 0x06; /* gain = -7.5db */
pub const ADC_ALC_TARGET_LEVEL_m6db: u32 = 0x07; /* gain = -6db */
pub const ADC_ALC_TARGET_LEVEL_m4_5db: u32 = 0x08; /* gain = -4.5db */
pub const ADC_ALC_TARGET_LEVEL_m_3db: u32 = 0x09; /* gain = -3db */
pub const ADC_ALC_TARGET_LEVEL_m1_5db: u32 = 0x0a; /* gain = -1.5db */

pub const ADC_ALC_MAXGAIN_m6_5db: u32 = 0x00; /* gain = -6.5db */
pub const ADC_ALC_MAXGAIN_m5db: u32 = 0x01; /* gain = -5db */
pub const ADC_ALC_MAXGAIN_m3_5db: u32 = 0x02; /* gain = -3.5db */
pub const ADC_ALC_MAXGAIN_m2db: u32 = 0x03; /* gain = -2db */
pub const ADC_ALC_MAXGAIN_m0_5db: u32 = 0x04; /* gain = -0.5db */
pub const ADC_ALC_MAXGAIN_1db: u32 = 0x05; /* gain = +1db */
pub const ADC_ALC_MAXGAIN_2_5db: u32 = 0x06; /* gain = +2.5db */
pub const ADC_ALC_MAXGAIN_4db: u32 = 0x07; /* gain = +4db */
pub const ADC_ALC_MAXGAIN_5_5db: u32 = 0x08; /* gain = +5.5db */
pub const ADC_ALC_MAXGAIN_7db: u32 = 0x09; /* gain = +7db */
pub const ADC_ALC_MAXGAIN_8_5db: u32 = 0x0a; /* gain = +8.5db */
pub const ADC_ALC_MAXGAIN_10db: u32 = 0x0b; /* gain = +10db */
pub const ADC_ALC_MAXGAIN_11_5db: u32 = 0x0c; /* gain = +11.5db */
pub const ADC_ALC_MAXGAIN_13db: u32 = 0x0d; /* gain = +13db */
pub const ADC_ALC_MAXGAIN_14_5db: u32 = 0x0e; /* gain = +14.5db */
pub const ADC_ALC_MAXGAIN_16db: u32 = 0x0f; /* gain = +16db */
pub const ADC_ALC_MAXGAIN_17_5db: u32 = 0x10; /* gain = +17.5db */
pub const ADC_ALC_MAXGAIN_19db: u32 = 0x11; /* gain = +19db */
pub const ADC_ALC_MAXGAIN_20_5db: u32 = 0x12; /* gain = +20.5db */
pub const ADC_ALC_MAXGAIN_22db: u32 = 0x13; /* gain = +22db */
pub const ADC_ALC_MAXGAIN_23_5db: u32 = 0x14; /* gain = +23.5db */
pub const ADC_ALC_MAXGAIN_25db: u32 = 0x15; /* gain = +25db */
pub const ADC_ALC_MAXGAIN_26_5db: u32 = 0x16; /* gain = +26.5db */
pub const ADC_ALC_MAXGAIN_28db: u32 = 0x17; /* gain = +28db */
pub const ADC_ALC_MAXGAIN_29_5db: u32 = 0x18; /* gain = +29.5db */
pub const ADC_ALC_MAXGAIN_31db: u32 = 0x19; /* gain = +31db */
pub const ADC_ALC_MAXGAIN_32_5db: u32 = 0x1a; /* gain = +32.5db */
pub const ADC_ALC_MAXGAIN_34db: u32 = 0x1b; /* gain = +34db */
pub const ADC_ALC_MAXGAIN_35_5db: u32 = 0x1c; /* gain = +35.5db */

pub const ADC_ALC_MINGAIN_m12db: u32 = 0x00; /* gain = -12db */
pub const ADC_ALC_MINGAIN_m10_5db: u32 = 0x01; /* gain = -10.5db */
pub const ADC_ALC_MINGAIN_m9db: u32 = 0x02; /* gain = -9db */
pub const ADC_ALC_MINGAIN_m7_5db: u32 = 0x03; /* gain = -7.5db */
pub const ADC_ALC_MINGAIN_m6db: u32 = 0x04; /* gain = -6db */
pub const ADC_ALC_MINGAIN_m4_51db: u32 = 0x05; /* gain = -4.51db */
pub const ADC_ALC_MINGAIN_m3db: u32 = 0x06; /* gain = -3db */
pub const ADC_ALC_MINGAIN_m1_5db: u32 = 0x07; /* gain = -1.5db */
pub const ADC_ALC_MINGAIN_0db: u32 = 0x08; /* gain = 0db */
pub const ADC_ALC_MINGAIN_1_5db: u32 = 0x09; /* gain = +1.5db */
pub const ADC_ALC_MINGAIN_3db: u32 = 0x0a; /* gain = +3db */
pub const ADC_ALC_MINGAIN_4_5db: u32 = 0x0b; /* gain = +4.5db */
pub const ADC_ALC_MINGAIN_6db: u32 = 0x0c; /* gain = +6db */
pub const ADC_ALC_MINGAIN_7_5db: u32 = 0x0d; /* gain = +7.5db */
pub const ADC_ALC_MINGAIN_9db: u32 = 0x0e; /* gain = +9db */
pub const ADC_ALC_MINGAIN_10_5db: u32 = 0x0f; /* gain = +10.5db */
pub const ADC_ALC_MINGAIN_12db: u32 = 0x10; /* gain = +12db */
pub const ADC_ALC_MINGAIN_13_5db: u32 = 0x11; /* gain = +13.5db */
pub const ADC_ALC_MINGAIN_15db: u32 = 0x12; /* gain = +15db */
pub const ADC_ALC_MINGAIN_16_5db: u32 = 0x13; /* gain = +16.5db */
pub const ADC_ALC_MINGAIN_18db: u32 = 0x14; /* gain = +18db */
pub const ADC_ALC_MINGAIN_19_5db: u32 = 0x15; /* gain = +19.5db */
pub const ADC_ALC_MINGAIN_21db: u32 = 0x16; /* gain = +21db */
pub const ADC_ALC_MINGAIN_22_5db: u32 = 0x17; /* gain = +22.5db */
pub const ADC_ALC_MINGAIN_24db: u32 = 0x18; /* gain = +24db */
pub const ADC_ALC_MINGAIN_25_5db: u32 = 0x19; /* gain = +25.5db */
pub const ADC_ALC_MINGAIN_27db: u32 = 0x1a; /* gain = +27db */
pub const ADC_ALC_MINGAIN_28_5db: u32 = 0x1b; /* gain = +28.5db */
pub const ADC_ALC_MINGAIN_30db: u32 = 0x1c; /* gain = +30db */

/* ADC volume: step 1dB */

/* ALC Hold, Decay, Attack */
pub const ADC_ALC_HLDTIME_0_US: u32 = 0x00;
pub const ADC_ALC_HLDTIME_0000266_US: u32 = 0x01; //time = 2.67ms
pub const ADC_ALC_HLDTIME_0000533_US: u32 = 0x02; //time = 5.33ms
pub const ADC_ALC_HLDTIME_0001066_US: u32 = 0x03; //time = 10.66ms
pub const ADC_ALC_HLDTIME_0002132_US: u32 = 0x04; //time = 21.32ms
pub const ADC_ALC_HLDTIME_0004264_US: u32 = 0x05; //time = 42.64ms
pub const ADC_ALC_HLDTIME_0008538_US: u32 = 0x06; //time = 85.38ms
pub const ADC_ALC_HLDTIME_0017076_US: u32 = 0x07; //time = 170.76ms
pub const ADC_ALC_HLDTIME_0034152_US: u32 = 0x08; //time = 341.52ms
pub const ADC_ALC_HLDTIME_0680000_US: u32 = 0x09; //time = 0.68s
pub const ADC_ALC_HLDTIME_1360000_US: u32 = 0x0a; //time = 1.36s

pub const ADC_ALC_DCYTIME_000410_US: u32 = 0x00; //time = 410us
pub const ADC_ALC_DCYTIME_000820_US: u32 = 0x01; //time = 820us
pub const ADC_ALC_DCYTIME_001640_US: u32 = 0x02; //time = 1.64ms
pub const ADC_ALC_DCYTIME_003280_US: u32 = 0x03; //time = 3.28ms
pub const ADC_ALC_DCYTIME_006560_US: u32 = 0x04; //time = 6.56ms
pub const ADC_ALC_DCYTIME_013120_US: u32 = 0x05; //time = 13.12ms
pub const ADC_ALC_DCYTIME_026240_US: u32 = 0x06; //time = 26.24ms
pub const ADC_ALC_DCYTIME_058480_US: u32 = 0x07; //time = 52.48ms
pub const ADC_ALC_DCYTIME_104960_US: u32 = 0x08; //time = 104.96ms
pub const ADC_ALC_DCYTIME_209920_US: u32 = 0x09; //time = 209.92ms
pub const ADC_ALC_DCYTIME_420000_US: u32 = 0x0a; //time = 420ms

pub const ADC_ALC_ATKTIME_000104_US: u32 = 0x00; //time = 104us
pub const ADC_ALC_ATKTIME_000208_US: u32 = 0x01; //time = 208us
pub const ADC_ALC_ATKTIME_000416_US: u32 = 0x02; //time = 416ms
pub const ADC_ALC_ATKTIME_003832_US: u32 = 0x03; //time = 832ms
pub const ADC_ALC_ATKTIME_001664_US: u32 = 0x04; //time = 1.664ms
pub const ADC_ALC_ATKTIME_003328_US: u32 = 0x05; //time = 3.328ms
pub const ADC_ALC_ATKTIME_006656_US: u32 = 0x06; //time = 6.656ms
pub const ADC_ALC_ATKTIME_013312_US: u32 = 0x07; //time = 13.312ms
pub const ADC_ALC_ATKTIME_026624_US: u32 = 0x08; //time = 26.624ms
pub const ADC_ALC_ATKTIME_053248_US: u32 = 0x09; //time = 53.248ms
pub const ADC_ALC_ATKTIME_106496_US: u32 = 0x0a; //time = 106.496ms

/* ALC Noise Gate */
pub const ADC_ALC_NGTYPE_DISABLE: u32 = 0x00; //noise gate disable
pub const ADC_ALC_NGTYPE_ENABLE_HOLD: u32 = 0x01; //noise gate enable, hold gain type
pub const ADC_ALC_NGTYPE_ENABLE_MUTE: u32 = 0x03; //noise gate enable, mute type

pub const ADC_ALC_NGTHLD_m76_5db: u32 = 0x00; /* Threshold = -76.5db */
pub const ADC_ALC_NGTHLD_m75db: u32 = 0x01; /* Threshold = -75db   */
pub const ADC_ALC_NGTHLD_m73_5db: u32 = 0x02; /* Threshold = -73.5db */
pub const ADC_ALC_NGTHLD_m72db: u32 = 0x03; /* Threshold = -72db   */
pub const ADC_ALC_NGTHLD_m70_5db: u32 = 0x04; /* Threshold = -70.5db */
pub const ADC_ALC_NGTHLD_m69db: u32 = 0x05; /* Threshold = -69db   */
pub const ADC_ALC_NGTHLD_m67_5db: u32 = 0x06; /* Threshold = -67.5db */
pub const ADC_ALC_NGTHLD_m66db: u32 = 0x07; /* Threshold = -66db   */
pub const ADC_ALC_NGTHLD_m64_5db: u32 = 0x08; /* Threshold = -64.5db */
pub const ADC_ALC_NGTHLD_m63db: u32 = 0x09; /* Threshold = -63db   */
pub const ADC_ALC_NGTHLD_m61_5db: u32 = 0x0a; /* Threshold = -61.5db */
pub const ADC_ALC_NGTHLD_m60db: u32 = 0x0b; /* Threshold = -60db   */
pub const ADC_ALC_NGTHLD_m58_5db: u32 = 0x0c; /* Threshold = -58.5db */
pub const ADC_ALC_NGTHLD_m57db: u32 = 0x0d; /* Threshold = -57db   */
pub const ADC_ALC_NGTHLD_m55_5db: u32 = 0x0e; /* Threshold = -55.5db */
pub const ADC_ALC_NGTHLD_m54db: u32 = 0x0f; /* Threshold = -54db   */
pub const ADC_ALC_NGTHLD_m52_5db: u32 = 0x10; /* Threshold = -52.5db */
pub const ADC_ALC_NGTHLD_m51db: u32 = 0x11; /* Threshold = -51db   */
pub const ADC_ALC_NGTHLD_m49_5db: u32 = 0x12; /* Threshold = -49.5db */
pub const ADC_ALC_NGTHLD_m48db: u32 = 0x13; /* Threshold = -48db   */
pub const ADC_ALC_NGTHLD_m46_5db: u32 = 0x14; /* Threshold = -46.5db */
pub const ADC_ALC_NGTHLD_m45db: u32 = 0x15; /* Threshold = -45db   */
pub const ADC_ALC_NGTHLD_m43_5db: u32 = 0x16; /* Threshold = -43.5db */
pub const ADC_ALC_NGTHLD_m42db: u32 = 0x17; /* Threshold = -42db   */
pub const ADC_ALC_NGTHLD_m40_5db: u32 = 0x18; /* Threshold = -40.5db */
pub const ADC_ALC_NGTHLD_m39db: u32 = 0x19; /* Threshold = -39db   */
pub const ADC_ALC_NGTHLD_m37_5db: u32 = 0x1a; /* Threshold = -37.5db */
pub const ADC_ALC_NGTHLD_m36db: u32 = 0x1b; /* Threshold = -36db   */
pub const ADC_ALC_NGTHLD_m34_5db: u32 = 0x1c; /* Threshold = -34.5db */
pub const ADC_ALC_NGTHLD_m33db: u32 = 0x1d; /* Threshold = -33db   */
pub const ADC_ALC_NGTHLD_m31_5db: u32 = 0x1e; /* Threshold = -31.5db */
pub const ADC_ALC_NGTHLD_m30db: u32 = 0x1f; /* Threshold = -30db   */

/* Headphone dummy - Windows Specific flag, not needed for Linux */

/* HPMIX HIGAIN and VOLUME */
pub const DAC_HPMIX_HIGAIN_0db: u32 = 0x00; /* gain =  0db      */
pub const DAC_HPMIX_HIGAIN_m6db: u32 = 0x88; /* gain = -6db      */

pub const DAC_HPMIX_VOLUME_m12db: u32 = 0x00; /* volume = -12db   */
pub const DAC_HPMIX_VOLUME_m10_5db: u32 = 0x11; /* volume = -10.5db */
pub const DAC_HPMIX_VOLUME_m9db: u32 = 0x22; /* volume = -9db    */
pub const DAC_HPMIX_VOLUME_m7_5db: u32 = 0x33; /* volume = -7.5db  */
pub const DAC_HPMIX_VOLUME_m6db: u32 = 0x44; /* volume = -6db    */
pub const DAC_HPMIX_VOLUME_m4_5db: u32 = 0x88; /* volume = -4.5db  */
pub const DAC_HPMIX_VOLUME_m3db: u32 = 0x99; /* volume = -3db    */
pub const DAC_HPMIX_VOLUME_m1_5db: u32 = 0xaa; /* volume = -1.5db  */
pub const DAC_HPMIX_VOLUME_0db: u32 = 0xbb; /* volume =  0db    */

/* HPOUT VOLUME */
pub const DAC_HPOUT_VOLUME_0db: u32 = 0x00; /* volume =   0db   */
pub const DAC_HPOUT_VOLUME_m12db: u32 = 0x11; /* volume = -12db   */
pub const DAC_HPOUT_VOLUME_m24db: u32 = 0x22; /* volume = -24db   */
pub const DAC_HPOUT_VOLUME_m48db: u32 = 0x33; /* volume = -48db   */

/* LDAC/RDAC volume = 0db, -0.5db/setp, 0xc0 <-> -96db */

/* Automute */
pub const DAC_AUTOMUTE_NONE: u32 = 0x00; /* no automute  */
pub const DAC_AUTOMUTE_DIGITAL: u32 = 0x01; /* digital mute */
pub const DAC_AUTOMUTE_ANALOG: u32 = 0x02; /* analog mute  */

/* Mono - Windows specific, on Linux the information comes from DAI/topology */
pub const HEADPHONE_MONO: u32 = 0x01; /* on channel */
pub const HEADPHONE_STEREO: u32 = 0x00; /* stereo */

/* Speaker and headphone GPIO control */
pub const GPIO_CTL_IO_LEVEL_LOW: u32 = 0x00; /* low level enable */
pub const GPIO_CTL_IO_LEVEL_HIGH: u32 = 0x01; /* high level enable */

/* GPIO */
/* FIXME: for ES8396, no need to use */

/* Platform clocks */
/*
 * BCLK AND MCLK FREQ
 * BIT[7:4] MCLK FREQ
 * 0 - 19.2MHz
 * 1 - 24MHz
 * 2 - 12.288MHz
 * F - Default for 19.2MHz
 *
 * BIT[3:0] BCLK FREQ
 * 0 - 4.8MHz
 * 1 - 2.4MHz
 * 2 - 2.304MHz
 * 3 - 3.072MHz
 * 4 - 4.096MHz
 * F - Default for 4.8MHz
 */

// `struct device` is provided by external kernel bindings.
unsafe extern "C" {
    pub fn es83xx_dsm(
        dev: *mut device,
        arg: ::core::ffi::c_int,
        value: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    pub fn es83xx_dsm_dump(dev: *mut device) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
