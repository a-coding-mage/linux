/* SPDX-License-Identifier: GPL-2.0-or-later */

/*
 *  Advanced Linux Sound Architecture - ALSA - Driver
 *  Copyright (c) 1994-2000 by Jaroslav Kysela <perex@perex.cz>
 */

/***************************************************************************
 *                                                                          *
 *        Digital audio interface					    *
 *                                                                          *
 ****************************************************************************/
/* IEC958 subframe format */
pub const IEC958_SUBFRAME_PREAMBLE_MASK: u32 = 0xf;
pub const IEC958_SUBFRAME_AUXILIARY_MASK: u32 = 0xf << 4;
pub const IEC958_SUBFRAME_SAMPLE_24_MASK: u32 = 0xffffff << 4;
pub const IEC958_SUBFRAME_SAMPLE_20_MASK: u32 = 0xfffff << 8;
pub const IEC958_SUBFRAME_VALIDITY: u32 = 0x1 << 28;
pub const IEC958_SUBFRAME_USER_DATA: u32 = 0x1 << 29;
pub const IEC958_SUBFRAME_CHANNEL_STATUS: u32 = 0x1 << 30;
pub const IEC958_SUBFRAME_PARITY: u32 = 0x1 << 31;

/* AES/IEC958 channel status bits */
pub const IEC958_AES0_PROFESSIONAL: u32 = 1 << 0;  /* 0 = consumer, 1 = professional */
pub const IEC958_AES0_NONAUDIO: u32 = 1 << 1;  /* 0 = audio, 1 = non-audio */
pub const IEC958_AES0_PRO_EMPHASIS: u32 = 7 << 2;  /* mask - emphasis */
pub const IEC958_AES0_PRO_EMPHASIS_NOTID: u32 = 0 << 2;  /* emphasis not indicated */
pub const IEC958_AES0_PRO_EMPHASIS_NONE: u32 = 1 << 2;  /* none emphasis */
pub const IEC958_AES0_PRO_EMPHASIS_5015: u32 = 3 << 2;  /* 50/15us emphasis */
pub const IEC958_AES0_PRO_EMPHASIS_CCITT: u32 = 7 << 2;  /* CCITT J.17 emphasis */
pub const IEC958_AES0_PRO_FREQ_UNLOCKED: u32 = 1 << 5;  /* source sample frequency: 0 = locked, 1 = unlocked */
pub const IEC958_AES0_PRO_FS: u32 = 3 << 6;  /* mask - sample frequency */
pub const IEC958_AES0_PRO_FS_NOTID: u32 = 0 << 6;  /* fs not indicated */
pub const IEC958_AES0_PRO_FS_44100: u32 = 1 << 6;  /* 44.1kHz */
pub const IEC958_AES0_PRO_FS_48000: u32 = 2 << 6;  /* 48kHz */
pub const IEC958_AES0_PRO_FS_32000: u32 = 3 << 6;  /* 32kHz */
pub const IEC958_AES0_CON_NOT_COPYRIGHT: u32 = 1 << 2;  /* 0 = copyright, 1 = not copyright */
pub const IEC958_AES0_CON_EMPHASIS: u32 = 7 << 3;  /* mask - emphasis */
pub const IEC958_AES0_CON_EMPHASIS_NONE: u32 = 0 << 3;  /* none emphasis */
pub const IEC958_AES0_CON_EMPHASIS_5015: u32 = 1 << 3;  /* 50/15us emphasis */
pub const IEC958_AES0_CON_MODE: u32 = 3 << 6;  /* mask - mode */
pub const IEC958_AES1_PRO_MODE: u32 = 15 << 0;  /* mask - channel mode */
pub const IEC958_AES1_PRO_MODE_NOTID: u32 = 0 << 0;  /* not indicated */
pub const IEC958_AES1_PRO_MODE_STEREOPHONIC: u32 = 2 << 0;  /* stereophonic - ch A is left */
pub const IEC958_AES1_PRO_MODE_SINGLE: u32 = 4 << 0;  /* single channel */
pub const IEC958_AES1_PRO_MODE_TWO: u32 = 8 << 0;  /* two channels */
pub const IEC958_AES1_PRO_MODE_PRIMARY: u32 = 12 << 0;  /* primary/secondary */
pub const IEC958_AES1_PRO_MODE_BYTE3: u32 = 15 << 0;  /* vector to byte 3 */
pub const IEC958_AES1_PRO_USERBITS: u32 = 15 << 4;  /* mask - user bits */
pub const IEC958_AES1_PRO_USERBITS_NOTID: u32 = 0 << 4;  /* not indicated */
pub const IEC958_AES1_PRO_USERBITS_192: u32 = 8 << 4;  /* 192-bit structure */
pub const IEC958_AES1_PRO_USERBITS_UDEF: u32 = 12 << 4;  /* user defined application */
pub const IEC958_AES1_CON_CATEGORY: u32 = 0x7f;
pub const IEC958_AES1_CON_GENERAL: u32 = 0x00;
pub const IEC958_AES1_CON_LASEROPT_MASK: u32 = 0x07;
pub const IEC958_AES1_CON_LASEROPT_ID: u32 = 0x01;
pub const IEC958_AES1_CON_IEC908_CD: u32 = IEC958_AES1_CON_LASEROPT_ID | 0x00;
pub const IEC958_AES1_CON_NON_IEC908_CD: u32 = IEC958_AES1_CON_LASEROPT_ID | 0x08;
pub const IEC958_AES1_CON_MINI_DISC: u32 = IEC958_AES1_CON_LASEROPT_ID | 0x48;
pub const IEC958_AES1_CON_DVD: u32 = IEC958_AES1_CON_LASEROPT_ID | 0x18;
pub const IEC958_AES1_CON_LASTEROPT_OTHER: u32 = IEC958_AES1_CON_LASEROPT_ID | 0x78;
pub const IEC958_AES1_CON_DIGDIGCONV_MASK: u32 = 0x07;
pub const IEC958_AES1_CON_DIGDIGCONV_ID: u32 = 0x02;
pub const IEC958_AES1_CON_PCM_CODER: u32 = IEC958_AES1_CON_DIGDIGCONV_ID | 0x00;
pub const IEC958_AES1_CON_MIXER: u32 = IEC958_AES1_CON_DIGDIGCONV_ID | 0x10;
pub const IEC958_AES1_CON_RATE_CONVERTER: u32 = IEC958_AES1_CON_DIGDIGCONV_ID | 0x18;
pub const IEC958_AES1_CON_SAMPLER: u32 = IEC958_AES1_CON_DIGDIGCONV_ID | 0x20;
pub const IEC958_AES1_CON_DSP: u32 = IEC958_AES1_CON_DIGDIGCONV_ID | 0x28;
pub const IEC958_AES1_CON_DIGDIGCONV_OTHER: u32 = IEC958_AES1_CON_DIGDIGCONV_ID | 0x78;
pub const IEC958_AES1_CON_MAGNETIC_MASK: u32 = 0x07;
pub const IEC958_AES1_CON_MAGNETIC_ID: u32 = 0x03;
pub const IEC958_AES1_CON_DAT: u32 = IEC958_AES1_CON_MAGNETIC_ID | 0x00;
pub const IEC958_AES1_CON_VCR: u32 = IEC958_AES1_CON_MAGNETIC_ID | 0x08;
pub const IEC958_AES1_CON_DCC: u32 = IEC958_AES1_CON_MAGNETIC_ID | 0x40;
pub const IEC958_AES1_CON_MAGNETIC_DISC: u32 = IEC958_AES1_CON_MAGNETIC_ID | 0x18;
pub const IEC958_AES1_CON_MAGNETIC_OTHER: u32 = IEC958_AES1_CON_MAGNETIC_ID | 0x78;
pub const IEC958_AES1_CON_BROADCAST1_MASK: u32 = 0x07;
pub const IEC958_AES1_CON_BROADCAST1_ID: u32 = 0x04;
pub const IEC958_AES1_CON_DAB_JAPAN: u32 = IEC958_AES1_CON_BROADCAST1_ID | 0x00;
pub const IEC958_AES1_CON_DAB_EUROPE: u32 = IEC958_AES1_CON_BROADCAST1_ID | 0x08;
pub const IEC958_AES1_CON_DAB_USA: u32 = IEC958_AES1_CON_BROADCAST1_ID | 0x60;
pub const IEC958_AES1_CON_SOFTWARE: u32 = IEC958_AES1_CON_BROADCAST1_ID | 0x40;
pub const IEC958_AES1_CON_IEC62105: u32 = IEC958_AES1_CON_BROADCAST1_ID | 0x20;
pub const IEC958_AES1_CON_BROADCAST1_OTHER: u32 = IEC958_AES1_CON_BROADCAST1_ID | 0x78;
pub const IEC958_AES1_CON_BROADCAST2_MASK: u32 = 0x0f;
pub const IEC958_AES1_CON_BROADCAST2_ID: u32 = 0x0e;
pub const IEC958_AES1_CON_MUSICAL_MASK: u32 = 0x07;
pub const IEC958_AES1_CON_MUSICAL_ID: u32 = 0x05;
pub const IEC958_AES1_CON_SYNTHESIZER: u32 = IEC958_AES1_CON_MUSICAL_ID | 0x00;
pub const IEC958_AES1_CON_MICROPHONE: u32 = IEC958_AES1_CON_MUSICAL_ID | 0x08;
pub const IEC958_AES1_CON_MUSICAL_OTHER: u32 = IEC958_AES1_CON_MUSICAL_ID | 0x78;
pub const IEC958_AES1_CON_ADC_MASK: u32 = 0x1f;
pub const IEC958_AES1_CON_ADC_ID: u32 = 0x06;
pub const IEC958_AES1_CON_ADC: u32 = IEC958_AES1_CON_ADC_ID | 0x00;
pub const IEC958_AES1_CON_ADC_OTHER: u32 = IEC958_AES1_CON_ADC_ID | 0x60;
pub const IEC958_AES1_CON_ADC_COPYRIGHT_MASK: u32 = 0x1f;
pub const IEC958_AES1_CON_ADC_COPYRIGHT_ID: u32 = 0x16;
pub const IEC958_AES1_CON_ADC_COPYRIGHT: u32 = IEC958_AES1_CON_ADC_COPYRIGHT_ID | 0x00;
pub const IEC958_AES1_CON_ADC_COPYRIGHT_OTHER: u32 = IEC958_AES1_CON_ADC_COPYRIGHT_ID | 0x60;
pub const IEC958_AES1_CON_SOLIDMEM_MASK: u32 = 0x0f;
pub const IEC958_AES1_CON_SOLIDMEM_ID: u32 = 0x08;
pub const IEC958_AES1_CON_SOLIDMEM_DIGITAL_RECORDER_PLAYER: u32 = IEC958_AES1_CON_SOLIDMEM_ID | 0x00;
pub const IEC958_AES1_CON_SOLIDMEM_OTHER: u32 = IEC958_AES1_CON_SOLIDMEM_ID | 0x70;
pub const IEC958_AES1_CON_EXPERIMENTAL: u32 = 0x40;
pub const IEC958_AES1_CON_ORIGINAL: u32 = 1 << 7;  /* this bits depends on the category code */
pub const IEC958_AES2_PRO_SBITS: u32 = 7 << 0;  /* mask - sample bits */
pub const IEC958_AES2_PRO_SBITS_20: u32 = 2 << 0;  /* 20-bit - coordination */
pub const IEC958_AES2_PRO_SBITS_24: u32 = 4 << 0;  /* 24-bit - main audio */
pub const IEC958_AES2_PRO_SBITS_UDEF: u32 = 6 << 0;  /* user defined application */
pub const IEC958_AES2_PRO_WORDLEN: u32 = 7 << 3;  /* mask - source word length */
pub const IEC958_AES2_PRO_WORDLEN_NOTID: u32 = 0 << 3;  /* not indicated */
pub const IEC958_AES2_PRO_WORDLEN_22_18: u32 = 2 << 3;  /* 22-bit or 18-bit */
pub const IEC958_AES2_PRO_WORDLEN_23_19: u32 = 4 << 3;  /* 23-bit or 19-bit */
pub const IEC958_AES2_PRO_WORDLEN_24_20: u32 = 5 << 3;  /* 24-bit or 20-bit */
pub const IEC958_AES2_PRO_WORDLEN_20_16: u32 = 6 << 3;  /* 20-bit or 16-bit */
pub const IEC958_AES2_CON_SOURCE: u32 = 15 << 0;  /* mask - source number */
pub const IEC958_AES2_CON_SOURCE_UNSPEC: u32 = 0 << 0;  /* unspecified */
pub const IEC958_AES2_CON_CHANNEL: u32 = 15 << 4;  /* mask - channel number */
pub const IEC958_AES2_CON_CHANNEL_UNSPEC: u32 = 0 << 4;  /* unspecified */
pub const IEC958_AES3_CON_FS: u32 = (1 << 7) | (15 << 0);  /* mask - sample frequency */
pub const IEC958_AES3_CON_FS_44100: u32 = 0 << 0;  /* 44.1kHz */
pub const IEC958_AES3_CON_FS_NOTID: u32 = 1 << 0;  /* non indicated */
pub const IEC958_AES3_CON_FS_48000: u32 = 2 << 0;  /* 48kHz */
pub const IEC958_AES3_CON_FS_32000: u32 = 3 << 0;  /* 32kHz */
pub const IEC958_AES3_CON_FS_22050: u32 = 4 << 0;  /* 22.05kHz */
pub const IEC958_AES3_CON_FS_384000: u32 = 5 << 0;  /* 384kHz */
pub const IEC958_AES3_CON_FS_24000: u32 = 6 << 0;  /* 24kHz */
pub const IEC958_AES3_CON_FS_88200: u32 = 8 << 0;  /* 88.2kHz */
pub const IEC958_AES3_CON_FS_768000: u32 = 9 << 0;  /* 768kHz */
pub const IEC958_AES3_CON_FS_96000: u32 = 10 << 0;  /* 96kHz */
pub const IEC958_AES3_CON_FS_176400: u32 = 12 << 0;  /* 176.4kHz */
pub const IEC958_AES3_CON_FS_352400: u32 = 13 << 0;  /* 352.4kHz */
pub const IEC958_AES3_CON_FS_192000: u32 = 14 << 0;  /* 192kHz */
pub const IEC958_AES3_CON_FS_128000: u32 = (1 << 7) | (11 << 0);  /* 128kHz */
pub const IEC958_AES3_CON_FS_705600: u32 = (1 << 7) | (13 << 0);  /* 705.6kHz */
pub const IEC958_AES3_CON_CLOCK: u32 = 3 << 4;  /* mask - clock accuracy */
pub const IEC958_AES3_CON_CLOCK_1000PPM: u32 = 0 << 4;  /* 1000 ppm */
pub const IEC958_AES3_CON_CLOCK_50PPM: u32 = 1 << 4;  /* 50 ppm */
pub const IEC958_AES3_CON_CLOCK_VARIABLE: u32 = 2 << 4;  /* variable pitch */
pub const IEC958_AES4_CON_MAX_WORDLEN_24: u32 = 1 << 0;  /* 0 = 20-bit, 1 = 24-bit */
pub const IEC958_AES4_CON_WORDLEN: u32 = 7 << 1;  /* mask - sample word length */
pub const IEC958_AES4_CON_WORDLEN_NOTID: u32 = 0 << 1;  /* not indicated */
pub const IEC958_AES4_CON_WORDLEN_20_16: u32 = 1 << 1;  /* 20-bit or 16-bit */
pub const IEC958_AES4_CON_WORDLEN_22_18: u32 = 2 << 1;  /* 22-bit or 18-bit */
pub const IEC958_AES4_CON_WORDLEN_23_19: u32 = 4 << 1;  /* 23-bit or 19-bit */
pub const IEC958_AES4_CON_WORDLEN_24_20: u32 = 5 << 1;  /* 24-bit or 20-bit */
pub const IEC958_AES4_CON_WORDLEN_21_17: u32 = 6 << 1;  /* 21-bit or 17-bit */
pub const IEC958_AES4_CON_ORIGFS: u32 = 15 << 4;  /* mask - original sample frequency */
pub const IEC958_AES4_CON_ORIGFS_NOTID: u32 = 0 << 4;  /* not indicated */
pub const IEC958_AES4_CON_ORIGFS_192000: u32 = 1 << 4;  /* 192kHz */
pub const IEC958_AES4_CON_ORIGFS_12000: u32 = 2 << 4;  /* 12kHz */
pub const IEC958_AES4_CON_ORIGFS_176400: u32 = 3 << 4;  /* 176.4kHz */
pub const IEC958_AES4_CON_ORIGFS_96000: u32 = 5 << 4;  /* 96kHz */
pub const IEC958_AES4_CON_ORIGFS_8000: u32 = 6 << 4;  /* 8kHz */
pub const IEC958_AES4_CON_ORIGFS_88200: u32 = 7 << 4;  /* 88.2kHz */
pub const IEC958_AES4_CON_ORIGFS_16000: u32 = 8 << 4;  /* 16kHz */
pub const IEC958_AES4_CON_ORIGFS_24000: u32 = 9 << 4;  /* 24kHz */
pub const IEC958_AES4_CON_ORIGFS_11025: u32 = 10 << 4;  /* 11.025kHz */
pub const IEC958_AES4_CON_ORIGFS_22050: u32 = 11 << 4;  /* 22.05kHz */
pub const IEC958_AES4_CON_ORIGFS_32000: u32 = 12 << 4;  /* 32kHz */
pub const IEC958_AES4_CON_ORIGFS_48000: u32 = 13 << 4;  /* 48kHz */
pub const IEC958_AES4_CON_ORIGFS_44100: u32 = 15 << 4;  /* 44.1kHz */
pub const IEC958_AES5_CON_CGMSA: u32 = 3 << 0;  /* mask - CGMS-A */
pub const IEC958_AES5_CON_CGMSA_COPYFREELY: u32 = 0 << 0;  /* copying is permitted without restriction */
pub const IEC958_AES5_CON_CGMSA_COPYONCE: u32 = 1 << 0;  /* one generation of copies may be made */
pub const IEC958_AES5_CON_CGMSA_COPYNOMORE: u32 = 2 << 0;  /* condition not be used */
pub const IEC958_AES5_CON_CGMSA_COPYNEVER: u32 = 3 << 0;  /* no copying is permitted */

/***************************************************************************
 *                                                                          *
 *        CEA-861 Audio InfoFrame. Used in HDMI and DisplayPort		    *
 *                                                                          *
 ****************************************************************************/
pub const CEA861_AUDIO_INFOFRAME_DB1CC: u32 = 7 << 0;  /* mask - channel count */
pub const CEA861_AUDIO_INFOFRAME_DB1CT: u32 = 0xf << 4;  /* mask - coding type */
pub const CEA861_AUDIO_INFOFRAME_DB1CT_FROM_STREAM: u32 = 0 << 4;  /* refer to stream */
pub const CEA861_AUDIO_INFOFRAME_DB1CT_IEC60958: u32 = 1 << 4;  /* IEC-60958 L-PCM */
pub const CEA861_AUDIO_INFOFRAME_DB1CT_AC3: u32 = 2 << 4;  /* AC-3 */
pub const CEA861_AUDIO_INFOFRAME_DB1CT_MPEG1: u32 = 3 << 4;  /* MPEG1 Layers 1 & 2 */
pub const CEA861_AUDIO_INFOFRAME_DB1CT_MP3: u32 = 4 << 4;  /* MPEG1 Layer 3 */
pub const CEA861_AUDIO_INFOFRAME_DB1CT_MPEG2_MULTICH: u32 = 5 << 4;  /* MPEG2 Multichannel */
pub const CEA861_AUDIO_INFOFRAME_DB1CT_AAC: u32 = 6 << 4;  /* AAC */
pub const CEA861_AUDIO_INFOFRAME_DB1CT_DTS: u32 = 7 << 4;  /* DTS */
pub const CEA861_AUDIO_INFOFRAME_DB1CT_ATRAC: u32 = 8 << 4;  /* ATRAC */
pub const CEA861_AUDIO_INFOFRAME_DB1CT_ONEBIT: u32 = 9 << 4;  /* One Bit Audio */
pub const CEA861_AUDIO_INFOFRAME_DB1CT_DOLBY_DIG_PLUS: u32 = 10 << 4;  /* Dolby Digital + */
pub const CEA861_AUDIO_INFOFRAME_DB1CT_DTS_HD: u32 = 11 << 4;  /* DTS-HD */
pub const CEA861_AUDIO_INFOFRAME_DB1CT_MAT: u32 = 12 << 4;  /* MAT (MLP) */
pub const CEA861_AUDIO_INFOFRAME_DB1CT_DST: u32 = 13 << 4;  /* DST */
pub const CEA861_AUDIO_INFOFRAME_DB1CT_WMA_PRO: u32 = 14 << 4;  /* WMA Pro */
pub const CEA861_AUDIO_INFOFRAME_DB2SF: u32 = 7 << 2;  /* mask - sample frequency */
pub const CEA861_AUDIO_INFOFRAME_DB2SF_FROM_STREAM: u32 = 0 << 2;  /* refer to stream */
pub const CEA861_AUDIO_INFOFRAME_DB2SF_32000: u32 = 1 << 2;  /* 32kHz */
pub const CEA861_AUDIO_INFOFRAME_DB2SF_44100: u32 = 2 << 2;  /* 44.1kHz */
pub const CEA861_AUDIO_INFOFRAME_DB2SF_48000: u32 = 3 << 2;  /* 48kHz */
pub const CEA861_AUDIO_INFOFRAME_DB2SF_88200: u32 = 4 << 2;  /* 88.2kHz */
pub const CEA861_AUDIO_INFOFRAME_DB2SF_96000: u32 = 5 << 2;  /* 96kHz */
pub const CEA861_AUDIO_INFOFRAME_DB2SF_176400: u32 = 6 << 2;  /* 176.4kHz */
pub const CEA861_AUDIO_INFOFRAME_DB2SF_192000: u32 = 7 << 2;  /* 192kHz */
pub const CEA861_AUDIO_INFOFRAME_DB2SS: u32 = 3 << 0;  /* mask - sample size */
pub const CEA861_AUDIO_INFOFRAME_DB2SS_FROM_STREAM: u32 = 0 << 0;  /* refer to stream */
pub const CEA861_AUDIO_INFOFRAME_DB2SS_16BIT: u32 = 1 << 0;  /* 16 bits */
pub const CEA861_AUDIO_INFOFRAME_DB2SS_20BIT: u32 = 2 << 0;  /* 20 bits */
pub const CEA861_AUDIO_INFOFRAME_DB2SS_24BIT: u32 = 3 << 0;  /* 24 bits */
pub const CEA861_AUDIO_INFOFRAME_DB5_DM_INH: u32 = 1 << 7;  /* mask - inhibit downmixing */
pub const CEA861_AUDIO_INFOFRAME_DB5_DM_INH_PERMITTED: u32 = 0 << 7;  /* stereo downmix permitted */
pub const CEA861_AUDIO_INFOFRAME_DB5_DM_INH_PROHIBITED: u32 = 1 << 7;  /* stereo downmis prohibited */
pub const CEA861_AUDIO_INFOFRAME_DB5_LSV: u32 = 0xf << 3;  /* mask - level-shift values */

/****************************************************************************
 *                                                                           *
 *                            MIDI v1.0 interface                            *
 *                                                                           *
 *****************************************************************************/

pub const MIDI_CHANNELS: u32 = 16;
pub const MIDI_GM_DRUM_CHANNEL: i32 = 10 - 1;

/*
 *  MIDI commands
 */

pub const MIDI_CMD_NOTE_OFF: u32 = 0x80;
pub const MIDI_CMD_NOTE_ON: u32 = 0x90;
pub const MIDI_CMD_NOTE_PRESSURE: u32 = 0xa0;
pub const MIDI_CMD_CONTROL: u32 = 0xb0;
pub const MIDI_CMD_PGM_CHANGE: u32 = 0xc0;
pub const MIDI_CMD_CHANNEL_PRESSURE: u32 = 0xd0;
pub const MIDI_CMD_BENDER: u32 = 0xe0;

pub const MIDI_CMD_COMMON_SYSEX: u32 = 0xf0;
pub const MIDI_CMD_COMMON_MTC_QUARTER: u32 = 0xf1;
pub const MIDI_CMD_COMMON_SONG_POS: u32 = 0xf2;
pub const MIDI_CMD_COMMON_SONG_SELECT: u32 = 0xf3;
pub const MIDI_CMD_COMMON_TUNE_REQUEST: u32 = 0xf6;
pub const MIDI_CMD_COMMON_SYSEX_END: u32 = 0xf7;
pub const MIDI_CMD_COMMON_CLOCK: u32 = 0xf8;
pub const MIDI_CMD_COMMON_START: u32 = 0xfa;
pub const MIDI_CMD_COMMON_CONTINUE: u32 = 0xfb;
pub const MIDI_CMD_COMMON_STOP: u32 = 0xfc;
pub const MIDI_CMD_COMMON_SENSING: u32 = 0xfe;
pub const MIDI_CMD_COMMON_RESET: u32 = 0xff;

/*
 *  MIDI controllers
 */

pub const MIDI_CTL_MSB_BANK: u32 = 0x00;
pub const MIDI_CTL_MSB_MODWHEEL: u32 = 0x01;
pub const MIDI_CTL_MSB_BREATH: u32 = 0x02;
pub const MIDI_CTL_MSB_FOOT: u32 = 0x04;
pub const MIDI_CTL_MSB_PORTAMENTO_TIME: u32 = 0x05;
pub const MIDI_CTL_MSB_DATA_ENTRY: u32 = 0x06;
pub const MIDI_CTL_MSB_MAIN_VOLUME: u32 = 0x07;
pub const MIDI_CTL_MSB_BALANCE: u32 = 0x08;
pub const MIDI_CTL_MSB_PAN: u32 = 0x0a;
pub const MIDI_CTL_MSB_EXPRESSION: u32 = 0x0b;
pub const MIDI_CTL_MSB_EFFECT1: u32 = 0x0c;
pub const MIDI_CTL_MSB_EFFECT2: u32 = 0x0d;
pub const MIDI_CTL_MSB_GENERAL_PURPOSE1: u32 = 0x10;
pub const MIDI_CTL_MSB_GENERAL_PURPOSE2: u32 = 0x11;
pub const MIDI_CTL_MSB_GENERAL_PURPOSE3: u32 = 0x12;
pub const MIDI_CTL_MSB_GENERAL_PURPOSE4: u32 = 0x13;
pub const MIDI_CTL_LSB_BANK: u32 = 0x20;
pub const MIDI_CTL_LSB_MODWHEEL: u32 = 0x21;
pub const MIDI_CTL_LSB_BREATH: u32 = 0x22;
pub const MIDI_CTL_LSB_FOOT: u32 = 0x24;
pub const MIDI_CTL_LSB_PORTAMENTO_TIME: u32 = 0x25;
pub const MIDI_CTL_LSB_DATA_ENTRY: u32 = 0x26;
pub const MIDI_CTL_LSB_MAIN_VOLUME: u32 = 0x27;
pub const MIDI_CTL_LSB_BALANCE: u32 = 0x28;
pub const MIDI_CTL_LSB_PAN: u32 = 0x2a;
pub const MIDI_CTL_LSB_EXPRESSION: u32 = 0x2b;
pub const MIDI_CTL_LSB_EFFECT1: u32 = 0x2c;
pub const MIDI_CTL_LSB_EFFECT2: u32 = 0x2d;
pub const MIDI_CTL_LSB_GENERAL_PURPOSE1: u32 = 0x30;
pub const MIDI_CTL_LSB_GENERAL_PURPOSE2: u32 = 0x31;
pub const MIDI_CTL_LSB_GENERAL_PURPOSE3: u32 = 0x32;
pub const MIDI_CTL_LSB_GENERAL_PURPOSE4: u32 = 0x33;
pub const MIDI_CTL_SUSTAIN: u32 = 0x40;
pub const MIDI_CTL_PORTAMENTO: u32 = 0x41;
pub const MIDI_CTL_SOSTENUTO: u32 = 0x42;
pub const MIDI_CTL_SOFT_PEDAL: u32 = 0x43;
pub const MIDI_CTL_LEGATO_FOOTSWITCH: u32 = 0x44;
pub const MIDI_CTL_HOLD2: u32 = 0x45;
pub const MIDI_CTL_SC1_SOUND_VARIATION: u32 = 0x46;
pub const MIDI_CTL_SC2_TIMBRE: u32 = 0x47;
pub const MIDI_CTL_SC3_RELEASE_TIME: u32 = 0x48;
pub const MIDI_CTL_SC4_ATTACK_TIME: u32 = 0x49;
pub const MIDI_CTL_SC5_BRIGHTNESS: u32 = 0x4a;
pub const MIDI_CTL_SC6: u32 = 0x4b;
pub const MIDI_CTL_SC7: u32 = 0x4c;
pub const MIDI_CTL_SC8: u32 = 0x4d;
pub const MIDI_CTL_SC9: u32 = 0x4e;
pub const MIDI_CTL_SC10: u32 = 0x4f;
pub const MIDI_CTL_GENERAL_PURPOSE5: u32 = 0x50;
pub const MIDI_CTL_GENERAL_PURPOSE6: u32 = 0x51;
pub const MIDI_CTL_GENERAL_PURPOSE7: u32 = 0x52;
pub const MIDI_CTL_GENERAL_PURPOSE8: u32 = 0x53;
pub const MIDI_CTL_PORTAMENTO_CONTROL: u32 = 0x54;
pub const MIDI_CTL_E1_REVERB_DEPTH: u32 = 0x5b;
pub const MIDI_CTL_E2_TREMOLO_DEPTH: u32 = 0x5c;
pub const MIDI_CTL_E3_CHORUS_DEPTH: u32 = 0x5d;
pub const MIDI_CTL_E4_DETUNE_DEPTH: u32 = 0x5e;
pub const MIDI_CTL_E5_PHASER_DEPTH: u32 = 0x5f;
pub const MIDI_CTL_DATA_INCREMENT: u32 = 0x60;
pub const MIDI_CTL_DATA_DECREMENT: u32 = 0x61;
pub const MIDI_CTL_NONREG_PARM_NUM_LSB: u32 = 0x62;
pub const MIDI_CTL_NONREG_PARM_NUM_MSB: u32 = 0x63;
pub const MIDI_CTL_REGIST_PARM_NUM_LSB: u32 = 0x64;
pub const MIDI_CTL_REGIST_PARM_NUM_MSB: u32 = 0x65;
pub const MIDI_CTL_ALL_SOUNDS_OFF: u32 = 0x78;
pub const MIDI_CTL_RESET_CONTROLLERS: u32 = 0x79;
pub const MIDI_CTL_LOCAL_CONTROL_SWITCH: u32 = 0x7a;
pub const MIDI_CTL_ALL_NOTES_OFF: u32 = 0x7b;
pub const MIDI_CTL_OMNI_OFF: u32 = 0x7c;
pub const MIDI_CTL_OMNI_ON: u32 = 0x7d;
pub const MIDI_CTL_MONO1: u32 = 0x7e;
pub const MIDI_CTL_MONO2: u32 = 0x7f;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
