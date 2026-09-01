/* SPDX-License-Identifier: GPL-2.0-only */
/****************************************************************************

   Copyright Echo Digital Audio Corporation (c) 1998 - 2004
   All rights reserved
   www.echoaudio.com

   This file is part of Echo Digital Audio's generic driver library.
   *************************************************************************

 Translation from C++ and adaptation for use in ALSA-Driver
 were made by Giuliano Pochini <pochini@shiny.it>

****************************************************************************/

/**** Echogals: Darla20, Gina20, Layla20, and Darla24 ****/
#[cfg(ECHOGALS_FAMILY)]
pub const NUM_ASIC_TESTS: u32 = 5;
#[cfg(ECHOGALS_FAMILY)]
pub const READ_DSP_TIMEOUT: i64 = 1000000; /* one second */

/**** Echo24: Gina24, Layla24, Mona, Mia, Mia-midi ****/
/* #[cfg(ECHO24_FAMILY)] defines DSP_56361: Some Echo24 cards use the 56361 DSP. */
#[cfg(ECHO24_FAMILY)]
pub const READ_DSP_TIMEOUT: i64 = 100000; /* .1 second */

/**** 3G: Gina3G, Layla3G ****/
/* #[cfg(ECHO3G_FAMILY)] defines DSP_56361. */
#[cfg(ECHO3G_FAMILY)]
pub const READ_DSP_TIMEOUT: i64 = 100000; /* .1 second */
#[cfg(ECHO3G_FAMILY)]
pub const MIN_MTC_1X_RATE: u32 = 32000;

/**** Indigo: Indigo, Indigo IO, Indigo DJ ****/
/* #[cfg(INDIGO_FAMILY)] defines DSP_56361. */
#[cfg(INDIGO_FAMILY)]
pub const READ_DSP_TIMEOUT: i64 = 100000; /* .1 second */

/*
 * Max inputs and outputs
 */
pub const DSP_MAXAUDIOINPUTS: usize = 16; /* Max audio input channels */
pub const DSP_MAXAUDIOOUTPUTS: usize = 16; /* Max audio output channels */
pub const DSP_MAXPIPES: usize = 32; /* Max total pipes (input + output) */

/*
 * These are the offsets for the memory-mapped DSP registers; the DSP base
 * address is treated as the start of a u32 array.
 */
pub const CHI32_CONTROL_REG: u32 = 4;
pub const CHI32_STATUS_REG: u32 = 5;
pub const CHI32_VECTOR_REG: u32 = 6;
pub const CHI32_DATA_REG: u32 = 7;

/*
 * Interesting bits within the DSP registers
 */
pub const CHI32_VECTOR_BUSY: u32 = 0x00000001;
pub const CHI32_STATUS_REG_HF3: u32 = 0x00000008;
pub const CHI32_STATUS_REG_HF4: u32 = 0x00000010;
pub const CHI32_STATUS_REG_HF5: u32 = 0x00000020;
pub const CHI32_STATUS_HOST_READ_FULL: u32 = 0x00000004;
pub const CHI32_STATUS_HOST_WRITE_EMPTY: u32 = 0x00000002;
pub const CHI32_STATUS_IRQ: u32 = 0x00000040;

/*
 * DSP commands sent via slave mode; these are sent to the DSP by write_dsp()
 */
pub const DSP_FNC_SET_COMMPAGE_ADDR: u32 = 0x02;
pub const DSP_FNC_LOAD_LAYLA_ASIC: u32 = 0xa0;
pub const DSP_FNC_LOAD_GINA24_ASIC: u32 = 0xa0;
pub const DSP_FNC_LOAD_MONA_PCI_CARD_ASIC: u32 = 0xa0;
pub const DSP_FNC_LOAD_LAYLA24_PCI_CARD_ASIC: u32 = 0xa0;
pub const DSP_FNC_LOAD_MONA_EXTERNAL_ASIC: u32 = 0xa1;
pub const DSP_FNC_LOAD_LAYLA24_EXTERNAL_ASIC: u32 = 0xa1;
pub const DSP_FNC_LOAD_3G_ASIC: u32 = 0xa0;

/*
 * Defines to handle the MIDI input state engine; these are used to properly
 * extract MIDI time code bytes and their timestamps from the MIDI input stream.
 */
pub const MIDI_IN_STATE_NORMAL: i32 = 0;
pub const MIDI_IN_STATE_TS_HIGH: i32 = 1;
pub const MIDI_IN_STATE_TS_LOW: i32 = 2;
pub const MIDI_IN_STATE_F1_DATA: i32 = 3;
pub const MIDI_IN_SKIP_DATA: i32 = -1;

/*----------------------------------------------------------------------------

Setting the sample rates on Layla24 is somewhat schizophrenic.

For standard rates, it works exactly like Mona and Gina24.  That is, for
8, 11.025, 16, 22.05, 32, 44.1, 48, 88.2, and 96 kHz, you just set the
appropriate bits in the control register and write the control register.

In order to support MIDI time code sync (and possibly SMPTE LTC sync in
the future), Layla24 also has "continuous sample rate mode".  In this mode,
Layla24 can generate any sample rate between 25 and 50 kHz inclusive, or
50 to 100 kHz inclusive for double speed mode.

To use continuous mode:

-Set the clock select bits in the control register to 0xe (see the #define
 below)

-Set double-speed mode if you want to use sample rates above 50 kHz

-Write the control register as you would normally

-Now, you need to set the frequency register. First, you need to determine the
 value for the frequency register.  This is given by the following formula:

frequency_reg = (LAYLA24_MAGIC_NUMBER / sample_rate) - 2

Note the #define below for the magic number

-Wait for the DSP handshake
-Write the frequency_reg value to the .SampleRate field of the comm page
-Send the vector command SET_LAYLA24_FREQUENCY_REG (see vmonkey.h)

Once you have set the control register up for continuous mode, you can just
write the frequency register to change the sample rate.  This could be
used for MIDI time code sync. For MTC sync, the control register is set for
continuous mode.  The driver then just keeps writing the
SET_LAYLA24_FREQUENCY_REG command.

-----------------------------------------------------------------------------*/
pub const LAYLA24_MAGIC_NUMBER: u32 = 677376000;
pub const LAYLA24_CONTINUOUS_CLOCK: u32 = 0x000e;

/*
 * DSP vector commands
 */
pub const DSP_VC_RESET: u32 = 0x80ff;

#[cfg(not(DSP_56361))]
pub const DSP_VC_ACK_INT: u32 = 0x8073;
#[cfg(not(DSP_56361))]
pub const DSP_VC_SET_VMIXER_GAIN: u32 = 0x0000; /* Not used, only for compile */
#[cfg(not(DSP_56361))]
pub const DSP_VC_START_TRANSFER: u32 = 0x0075; /* Handshke rqd. */
#[cfg(not(DSP_56361))]
pub const DSP_VC_METERS_ON: u32 = 0x0079;
#[cfg(not(DSP_56361))]
pub const DSP_VC_METERS_OFF: u32 = 0x007b;
#[cfg(not(DSP_56361))]
pub const DSP_VC_UPDATE_OUTVOL: u32 = 0x007d; /* Handshke rqd. */
#[cfg(not(DSP_56361))]
pub const DSP_VC_UPDATE_INGAIN: u32 = 0x007f; /* Handshke rqd. */
#[cfg(not(DSP_56361))]
pub const DSP_VC_ADD_AUDIO_BUFFER: u32 = 0x0081; /* Handshke rqd. */
#[cfg(not(DSP_56361))]
pub const DSP_VC_TEST_ASIC: u32 = 0x00eb;
#[cfg(not(DSP_56361))]
pub const DSP_VC_UPDATE_CLOCKS: u32 = 0x00ef; /* Handshke rqd. */
#[cfg(not(DSP_56361))]
pub const DSP_VC_SET_LAYLA_SAMPLE_RATE: u32 = 0x00f1; /* Handshke rqd. */
#[cfg(not(DSP_56361))]
pub const DSP_VC_SET_GD_AUDIO_STATE: u32 = 0x00f1; /* Handshke rqd. */
#[cfg(not(DSP_56361))]
pub const DSP_VC_WRITE_CONTROL_REG: u32 = 0x00f1; /* Handshke rqd. */
#[cfg(not(DSP_56361))]
pub const DSP_VC_MIDI_WRITE: u32 = 0x00f5; /* Handshke rqd. */
#[cfg(not(DSP_56361))]
pub const DSP_VC_STOP_TRANSFER: u32 = 0x00f7; /* Handshke rqd. */
#[cfg(not(DSP_56361))]
pub const DSP_VC_UPDATE_FLAGS: u32 = 0x00fd; /* Handshke rqd. */
#[cfg(not(DSP_56361))]
pub const DSP_VC_GO_COMATOSE: u32 = 0x00f9;

/* Vector commands for families that use either the 56301 or 56361 */
#[cfg(DSP_56361)]
pub const DSP_VC_ACK_INT: u32 = 0x80F5;
#[cfg(DSP_56361)]
pub const DSP_VC_SET_VMIXER_GAIN: u32 = 0x00DB; /* Handshke rqd. */
#[cfg(DSP_56361)]
pub const DSP_VC_START_TRANSFER: u32 = 0x00DD; /* Handshke rqd. */
#[cfg(DSP_56361)]
pub const DSP_VC_METERS_ON: u32 = 0x00EF;
#[cfg(DSP_56361)]
pub const DSP_VC_METERS_OFF: u32 = 0x00F1;
#[cfg(DSP_56361)]
pub const DSP_VC_UPDATE_OUTVOL: u32 = 0x00E3; /* Handshke rqd. */
#[cfg(DSP_56361)]
pub const DSP_VC_UPDATE_INGAIN: u32 = 0x00E5; /* Handshke rqd. */
#[cfg(DSP_56361)]
pub const DSP_VC_ADD_AUDIO_BUFFER: u32 = 0x00E1; /* Handshke rqd. */
#[cfg(DSP_56361)]
pub const DSP_VC_TEST_ASIC: u32 = 0x00ED;
#[cfg(DSP_56361)]
pub const DSP_VC_UPDATE_CLOCKS: u32 = 0x00E9; /* Handshke rqd. */
#[cfg(DSP_56361)]
pub const DSP_VC_SET_LAYLA24_FREQUENCY_REG: u32 = 0x00E9; /* Handshke rqd. */
#[cfg(DSP_56361)]
pub const DSP_VC_SET_LAYLA_SAMPLE_RATE: u32 = 0x00EB; /* Handshke rqd. */
#[cfg(DSP_56361)]
pub const DSP_VC_SET_GD_AUDIO_STATE: u32 = 0x00EB; /* Handshke rqd. */
#[cfg(DSP_56361)]
pub const DSP_VC_WRITE_CONTROL_REG: u32 = 0x00EB; /* Handshke rqd. */
#[cfg(DSP_56361)]
pub const DSP_VC_MIDI_WRITE: u32 = 0x00E7; /* Handshke rqd. */
#[cfg(DSP_56361)]
pub const DSP_VC_STOP_TRANSFER: u32 = 0x00DF; /* Handshke rqd. */
#[cfg(DSP_56361)]
pub const DSP_VC_UPDATE_FLAGS: u32 = 0x00FB; /* Handshke rqd. */
#[cfg(DSP_56361)]
pub const DSP_VC_GO_COMATOSE: u32 = 0x00d9;

/*
 * Timeouts
 */
pub const HANDSHAKE_TIMEOUT: u32 = 20000; /* send_vector command timeout (20ms) */
pub const VECTOR_BUSY_TIMEOUT: u32 = 100000; /* 100ms */
pub const MIDI_OUT_DELAY_USEC: u32 = 2000; /* How long to wait after MIDI fills up */

/*
 * Flags for .Flags field in the comm page
 */
pub const DSP_FLAG_MIDI_INPUT: u32 = 0x0001; /* Enable MIDI input */
pub const DSP_FLAG_SPDIF_NONAUDIO: u32 = 0x0002; /* Sets the "non-audio" bit
                                                 * in the S/PDIF out status
                                                 * bits.  Clear this flag for
                                                 * audio data;
                                                 * set it for AC3 or WMA or
                                                 * some such */
pub const DSP_FLAG_PROFESSIONAL_SPDIF: u32 = 0x0008; /* 1 Professional, 0 Consumer */

/*
 * Clock detect bits reported by the DSP for Gina20, Layla20, Darla24, and Mia
 */
pub const GLDM_CLOCK_DETECT_BIT_WORD: u32 = 0x0002;
pub const GLDM_CLOCK_DETECT_BIT_SUPER: u32 = 0x0004;
pub const GLDM_CLOCK_DETECT_BIT_SPDIF: u32 = 0x0008;
pub const GLDM_CLOCK_DETECT_BIT_ESYNC: u32 = 0x0010;

/*
 * Clock detect bits reported by the DSP for Gina24, Mona, and Layla24
 */
pub const GML_CLOCK_DETECT_BIT_WORD96: u32 = 0x0002;
pub const GML_CLOCK_DETECT_BIT_WORD48: u32 = 0x0004;
pub const GML_CLOCK_DETECT_BIT_SPDIF48: u32 = 0x0008;
pub const GML_CLOCK_DETECT_BIT_SPDIF96: u32 = 0x0010;
pub const GML_CLOCK_DETECT_BIT_WORD: u32 = GML_CLOCK_DETECT_BIT_WORD96 | GML_CLOCK_DETECT_BIT_WORD48;
pub const GML_CLOCK_DETECT_BIT_SPDIF: u32 = GML_CLOCK_DETECT_BIT_SPDIF48 | GML_CLOCK_DETECT_BIT_SPDIF96;
pub const GML_CLOCK_DETECT_BIT_ESYNC: u32 = 0x0020;
pub const GML_CLOCK_DETECT_BIT_ADAT: u32 = 0x0040;

/*
 * Layla clock numbers to send to DSP
 */
pub const LAYLA20_CLOCK_INTERNAL: u32 = 0;
pub const LAYLA20_CLOCK_SPDIF: u32 = 1;
pub const LAYLA20_CLOCK_WORD: u32 = 2;
pub const LAYLA20_CLOCK_SUPER: u32 = 3;

/*
 * Gina/Darla clock states
 */
pub const GD_CLOCK_NOCHANGE: u32 = 0;
pub const GD_CLOCK_44: u32 = 1;
pub const GD_CLOCK_48: u32 = 2;
pub const GD_CLOCK_SPDIFIN: u32 = 3;
pub const GD_CLOCK_UNDEF: u32 = 0xff;

/*
 * Gina/Darla S/PDIF status bits
 */
pub const GD_SPDIF_STATUS_NOCHANGE: u32 = 0;
pub const GD_SPDIF_STATUS_44: u32 = 1;
pub const GD_SPDIF_STATUS_48: u32 = 2;
pub const GD_SPDIF_STATUS_UNDEF: u32 = 0xff;

/*
 * Layla20 output clocks
 */
pub const LAYLA20_OUTPUT_CLOCK_SUPER: u32 = 0;
pub const LAYLA20_OUTPUT_CLOCK_WORD: u32 = 1;

/****************************************************************************

   Magic constants for the Darla24 hardware

 ****************************************************************************/
pub const GD24_96000: u32 = 0x0;
pub const GD24_48000: u32 = 0x1;
pub const GD24_44100: u32 = 0x2;
pub const GD24_32000: u32 = 0x3;
pub const GD24_22050: u32 = 0x4;
pub const GD24_16000: u32 = 0x5;
pub const GD24_11025: u32 = 0x6;
pub const GD24_8000: u32 = 0x7;
pub const GD24_88200: u32 = 0x8;
pub const GD24_EXT_SYNC: u32 = 0x9;

/*
 * Return values from the DSP when ASIC is loaded
 */
pub const ASIC_ALREADY_LOADED: u32 = 0x1;
pub const ASIC_NOT_LOADED: u32 = 0x0;

/*
 * DSP Audio formats
 *
 * These are the audio formats that the DSP can transfer
 * via input and output pipes.  LE means little-endian,
 * BE means big-endian.
 */
pub const DSP_AUDIOFORM_MS_8: u32 = 0; /* 8 bit mono */
pub const DSP_AUDIOFORM_MS_16LE: u32 = 1; /* 16 bit mono */
pub const DSP_AUDIOFORM_MS_24LE: u32 = 2; /* 24 bit mono */
pub const DSP_AUDIOFORM_MS_32LE: u32 = 3; /* 32 bit mono */
pub const DSP_AUDIOFORM_SS_8: u32 = 4; /* 8 bit stereo */
pub const DSP_AUDIOFORM_SS_16LE: u32 = 5; /* 16 bit stereo */
pub const DSP_AUDIOFORM_SS_24LE: u32 = 6; /* 24 bit stereo */
pub const DSP_AUDIOFORM_SS_32LE: u32 = 7; /* 32 bit stereo */
pub const DSP_AUDIOFORM_MM_32LE: u32 = 8; /* 32 bit mono->mono little-endian */
pub const DSP_AUDIOFORM_MM_32BE: u32 = 9; /* 32 bit mono->mono big-endian */
pub const DSP_AUDIOFORM_SS_32BE: u32 = 10; /* 32 bit stereo big endian */
pub const DSP_AUDIOFORM_INVALID: u32 = 0xFF; /* Invalid audio format */

/*
 * Super-interleave is defined as interleaving by 4 or more.  Darla20 and Gina20
 * do not support super interleave.
 */
pub const DSP_AUDIOFORM_SUPER_INTERLEAVE_16LE: u32 = 0x40;
pub const DSP_AUDIOFORM_SUPER_INTERLEAVE_24LE: u32 = 0xc0;
pub const DSP_AUDIOFORM_SUPER_INTERLEAVE_32LE: u32 = 0x80;

/*
 * Gina24, Mona, and Layla24 control register defines
 */
pub const GML_CONVERTER_ENABLE: u32 = 0x0010;
pub const GML_SPDIF_PRO_MODE: u32 = 0x0020; /* Professional S/PDIF == 1,
                                            consumer == 0 */
pub const GML_SPDIF_SAMPLE_RATE0: u32 = 0x0040;
pub const GML_SPDIF_SAMPLE_RATE1: u32 = 0x0080;
pub const GML_SPDIF_TWO_CHANNEL: u32 = 0x0100; /* 1 == two channels,
                                               0 == one channel */
pub const GML_SPDIF_NOT_AUDIO: u32 = 0x0200;
pub const GML_SPDIF_COPY_PERMIT: u32 = 0x0400;
pub const GML_SPDIF_24_BIT: u32 = 0x0800; /* 1 == 24 bit, 0 == 20 bit */
pub const GML_ADAT_MODE: u32 = 0x1000; /* 1 == ADAT mode, 0 == S/PDIF mode */
pub const GML_SPDIF_OPTICAL_MODE: u32 = 0x2000; /* 1 == optical mode, 0 == RCA mode */
pub const GML_SPDIF_CDROM_MODE: u32 = 0x3000; /* 1 == CDROM mode,
                                              * 0 == RCA or optical mode */
pub const GML_DOUBLE_SPEED_MODE: u32 = 0x4000; /* 1 == double speed,
                                               0 == single speed */

pub const GML_DIGITAL_IN_AUTO_MUTE: u32 = 0x800000;

pub const GML_96KHZ: u32 = 0x0 | GML_DOUBLE_SPEED_MODE;
pub const GML_88KHZ: u32 = 0x1 | GML_DOUBLE_SPEED_MODE;
pub const GML_48KHZ: u32 = 0x2;
pub const GML_44KHZ: u32 = 0x3;
pub const GML_32KHZ: u32 = 0x4;
pub const GML_22KHZ: u32 = 0x5;
pub const GML_16KHZ: u32 = 0x6;
pub const GML_11KHZ: u32 = 0x7;
pub const GML_8KHZ: u32 = 0x8;
pub const GML_SPDIF_CLOCK: u32 = 0x9;
pub const GML_ADAT_CLOCK: u32 = 0xA;
pub const GML_WORD_CLOCK: u32 = 0xB;
pub const GML_ESYNC_CLOCK: u32 = 0xC;
pub const GML_ESYNCx2_CLOCK: u32 = 0xD;

pub const GML_CLOCK_CLEAR_MASK: u32 = 0xffffbff0;
pub const GML_SPDIF_RATE_CLEAR_MASK: u32 = !(GML_SPDIF_SAMPLE_RATE0 | GML_SPDIF_SAMPLE_RATE1);
pub const GML_DIGITAL_MODE_CLEAR_MASK: u32 = 0xffffcfff;
pub const GML_SPDIF_FORMAT_CLEAR_MASK: u32 = 0xfffff01f;

/*
 * Mia sample rate and clock setting constants
 */
pub const MIA_32000: u32 = 0x0040;
pub const MIA_44100: u32 = 0x0042;
pub const MIA_48000: u32 = 0x0041;
pub const MIA_88200: u32 = 0x0142;
pub const MIA_96000: u32 = 0x0141;

pub const MIA_SPDIF: u32 = 0x00000044;
pub const MIA_SPDIF96: u32 = 0x00000144;

pub const MIA_MIDI_REV: u32 = 1; /* Must be Mia rev 1 for MIDI support */

/*
 * 3G register bits
 */
pub const E3G_CONVERTER_ENABLE: u32 = 0x0010;
pub const E3G_SPDIF_PRO_MODE: u32 = 0x0020; /* Professional S/PDIF == 1,
                                            consumer == 0 */
pub const E3G_SPDIF_SAMPLE_RATE0: u32 = 0x0040;
pub const E3G_SPDIF_SAMPLE_RATE1: u32 = 0x0080;
pub const E3G_SPDIF_TWO_CHANNEL: u32 = 0x0100; /* 1 == two channels,
                                               0 == one channel */
pub const E3G_SPDIF_NOT_AUDIO: u32 = 0x0200;
pub const E3G_SPDIF_COPY_PERMIT: u32 = 0x0400;
pub const E3G_SPDIF_24_BIT: u32 = 0x0800; /* 1 == 24 bit, 0 == 20 bit */
pub const E3G_DOUBLE_SPEED_MODE: u32 = 0x4000; /* 1 == double speed,
                                               0 == single speed */
pub const E3G_PHANTOM_POWER: u32 = 0x8000; /* 1 == phantom power on,
                                           0 == phantom power off */

pub const E3G_96KHZ: u32 = 0x0 | E3G_DOUBLE_SPEED_MODE;
pub const E3G_88KHZ: u32 = 0x1 | E3G_DOUBLE_SPEED_MODE;
pub const E3G_48KHZ: u32 = 0x2;
pub const E3G_44KHZ: u32 = 0x3;
pub const E3G_32KHZ: u32 = 0x4;
pub const E3G_22KHZ: u32 = 0x5;
pub const E3G_16KHZ: u32 = 0x6;
pub const E3G_11KHZ: u32 = 0x7;
pub const E3G_8KHZ: u32 = 0x8;
pub const E3G_SPDIF_CLOCK: u32 = 0x9;
pub const E3G_ADAT_CLOCK: u32 = 0xA;
pub const E3G_WORD_CLOCK: u32 = 0xB;
pub const E3G_CONTINUOUS_CLOCK: u32 = 0xE;

pub const E3G_ADAT_MODE: u32 = 0x1000;
pub const E3G_SPDIF_OPTICAL_MODE: u32 = 0x2000;

pub const E3G_CLOCK_CLEAR_MASK: u32 = 0xbfffbff0;
pub const E3G_DIGITAL_MODE_CLEAR_MASK: u32 = 0xffffcfff;
pub const E3G_SPDIF_FORMAT_CLEAR_MASK: u32 = 0xfffff01f;

/* Clock detect bits reported by the DSP */
pub const E3G_CLOCK_DETECT_BIT_WORD96: u32 = 0x0001;
pub const E3G_CLOCK_DETECT_BIT_WORD48: u32 = 0x0002;
pub const E3G_CLOCK_DETECT_BIT_SPDIF48: u32 = 0x0004;
pub const E3G_CLOCK_DETECT_BIT_ADAT: u32 = 0x0004;
pub const E3G_CLOCK_DETECT_BIT_SPDIF96: u32 = 0x0008;
pub const E3G_CLOCK_DETECT_BIT_WORD: u32 = E3G_CLOCK_DETECT_BIT_WORD96 | E3G_CLOCK_DETECT_BIT_WORD48;
pub const E3G_CLOCK_DETECT_BIT_SPDIF: u32 = E3G_CLOCK_DETECT_BIT_SPDIF48 | E3G_CLOCK_DETECT_BIT_SPDIF96;

/* Frequency control register */
pub const E3G_MAGIC_NUMBER: u32 = 677376000;
pub const E3G_FREQ_REG_DEFAULT: u32 = E3G_MAGIC_NUMBER / 48000 - 2;
pub const E3G_FREQ_REG_MAX: u32 = 0xffff;

/* 3G external box types */
pub const E3G_GINA3G_BOX_TYPE: u32 = 0x00;
pub const E3G_LAYLA3G_BOX_TYPE: u32 = 0x10;
pub const E3G_ASIC_NOT_LOADED: u32 = 0xffff;
pub const E3G_BOX_TYPE_MASK: u32 = 0xf0;

/* Indigo express control register values */
pub const INDIGO_EXPRESS_32000: u32 = 0x02;
pub const INDIGO_EXPRESS_44100: u32 = 0x01;
pub const INDIGO_EXPRESS_48000: u32 = 0x00;
pub const INDIGO_EXPRESS_DOUBLE_SPEED: u32 = 0x10;
pub const INDIGO_EXPRESS_QUAD_SPEED: u32 = 0x04;
pub const INDIGO_EXPRESS_CLOCK_MASK: u32 = 0x17;

/*
 * Gina20 & Layla20 have input gain controls for the analog inputs;
 * this is the magic number for the hardware that gives you 0 dB at -10.
 */
pub const GL20_INPUT_GAIN_MAGIC_NUMBER: u32 = 0xC8;

/*
 * Defines how much time must pass between DSP load attempts
 */
pub const DSP_LOAD_ATTEMPT_PERIOD: i64 = 1000000; /* One second */

/*
 * Size of arrays for the comm page.  MAX_PLAY_TAPS and MAX_REC_TAPS are
 * no longer used, but the sizes must still be right for the DSP to see
 * the comm page correctly.
 */
pub const MONITOR_ARRAY_SIZE: usize = 0x180;
pub const VMIXER_ARRAY_SIZE: usize = 0x40;
pub const MIDI_OUT_BUFFER_SIZE: usize = 32;
pub const MIDI_IN_BUFFER_SIZE: usize = 256;
pub const MAX_PLAY_TAPS: usize = 168;
pub const MAX_REC_TAPS: usize = 192;
pub const DSP_MIDI_OUT_FIFO_SIZE: usize = 64;

/* sg_entry is a single entry for the scatter-gather list.  The array of struct
sg_entry struct is read by the DSP, so all values must be little-endian. */
pub const MAX_SGLIST_ENTRIES: usize = 512;

#[repr(C)]
pub struct sg_entry {
    pub addr: __le32,
    pub size: __le32,
}

/****************************************************************************

  The comm page.  This structure is read and written by the DSP; the
  DSP code is a firm believer in the byte offsets written in the comments
  at the end of each line.  This structure should not be changed.

  Any reads from or writes to this structure should be in little-endian format.

 ****************************************************************************/
#[repr(C)]
pub struct comm_page {
    pub comm_size: __le32, /* size of this object              0x000   4 */
    pub flags: __le32, /* See Appendix A below            0x004   4 */
    pub unused: __le32, /* Unused entry                     0x008   4 */
    pub sample_rate: __le32, /* Card sample rate in Hz           0x00c   4 */
    pub handshake: __le32, /* DSP command handshake            0x010   4 */
    pub cmd_start: __le32, /* Chs. to start mask               0x014   4 */
    pub cmd_stop: __le32, /* Chs. to stop mask                0x018   4 */
    pub cmd_reset: __le32, /* Chs. to reset mask               0x01c   4 */
    pub audio_format: [__le16; DSP_MAXPIPES], /* Chs. audio format       0x020   32*2 */
    pub sglist_addr: [sg_entry; DSP_MAXPIPES],
    /* Chs. Physical sglist addrs   0x060   32*8 */
    pub position: [__le32; DSP_MAXPIPES],
    /* Positions for ea. ch.        0x160   32*4 */
    pub vu_meter: [i8; DSP_MAXPIPES],
    /* VU meters                    0x1e0   32*1 */
    pub peak_meter: [i8; DSP_MAXPIPES],
    /* Peak meters                  0x200   32*1 */
    pub line_out_level: [i8; DSP_MAXAUDIOOUTPUTS],
    /* Output gain                  0x220   16*1 */
    pub line_in_level: [i8; DSP_MAXAUDIOINPUTS],
    /* Input gain                   0x230   16*1 */
    pub monitors: [i8; MONITOR_ARRAY_SIZE],
    /* Monitor map                  0x240   0x180 */
    pub play_coeff: [__le32; MAX_PLAY_TAPS],
    /* Gina/Darla play filters - obsolete   0x3c0   168*4 */
    pub rec_coeff: [__le32; MAX_REC_TAPS],
    /* Gina/Darla record filters - obsolete 0x660   192*4 */
    pub midi_input: [__le16; MIDI_IN_BUFFER_SIZE],
    /* MIDI input data transfer buffer      0x960   256*2 */
    pub gd_clock_state: u8, /* Chg Gina/Darla clock state     0xb60   1 */
    pub gd_spdif_status: u8, /* Chg. Gina/Darla S/PDIF state   0xb61   1 */
    pub gd_resampler_state: u8, /* Should always be 3             0xb62   1 */
    pub filler2: u8, /*                                  0xb63   1 */
    pub nominal_level_mask: __le32, /* -10 level enable mask        0xb64   4 */
    pub input_clock: __le16, /* Chg. Input clock state          0xb68   2 */
    pub output_clock: __le16, /* Chg. Output clock state         0xb6a   2 */
    pub status_clocks: __le32, /* Current Input clock state       0xb6c   4 */
    pub ext_box_status: __le32, /* External box status             0xb70   4 */
    pub cmd_add_buffer: __le32, /* Pipes to add (obsolete)         0xb74   4 */
    pub midi_out_free_count: __le32,
    /* # of bytes free in MIDI output FIFO  0xb78   4 */
    pub unused2: __le32, /* Cyclic pipes                    0xb7c   4 */
    pub control_register: __le32,
    /* Mona, Gina24, Layla24, 3G ctrl reg   0xb80   4 */
    pub e3g_frq_register: __le32, /* 3G frequency register          0xb84   4 */
    pub filler: [u8; 24], /* filler                         0xb88   24*1 */
    pub vmixer: [i8; VMIXER_ARRAY_SIZE],
    /* Vmixer levels                0xba0   64*1 */
    pub midi_output: [u8; MIDI_OUT_BUFFER_SIZE],
    /* MIDI output data             0xbe0   32*1 */
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
