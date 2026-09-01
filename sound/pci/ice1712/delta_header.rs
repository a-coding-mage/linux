/* SPDX-License-Identifier: GPL-2.0-or-later */

/*
 *   ALSA driver for ICEnsemble ICE1712 (Envy24)
 *
 *   Lowlevel functions for M-Audio Delta 1010, 44, 66, Dio2496, Audiophile
 *                          Digigram VX442
 *
 *	Copyright (c) 2000 Jaroslav Kysela <perex@perex.cz>
 */

pub const DELTA_DEVICE_DESC: &str =
    "{MidiMan M Audio,Delta 1010},\
     {MidiMan M Audio,Delta 1010LT},\
     {MidiMan M Audio,Delta DiO 2496},\
     {MidiMan M Audio,Delta 66},\
     {MidiMan M Audio,Delta 44},\
     {MidiMan M Audio,Delta 410},\
     {MidiMan M Audio,Audiophile 24/96},\
     {Digigram,VX442},\
     {Lionstracs,Mediastation},\
     {Edirol,DA2496},";

pub const ICE1712_SUBDEVICE_DELTA1010: u32 = 0x121430d6;
pub const ICE1712_SUBDEVICE_DELTA1010E: u32 = 0xff1430d6;
pub const ICE1712_SUBDEVICE_DELTADIO2496: u32 = 0x121431d6;
pub const ICE1712_SUBDEVICE_DELTA66: u32 = 0x121432d6;
pub const ICE1712_SUBDEVICE_DELTA66E: u32 = 0xff1432d6;
pub const ICE1712_SUBDEVICE_DELTA44: u32 = 0x121433d6;
pub const ICE1712_SUBDEVICE_AUDIOPHILE: u32 = 0x121434d6;
pub const ICE1712_SUBDEVICE_DELTA410: u32 = 0x121438d6;
pub const ICE1712_SUBDEVICE_DELTA1010LT: u32 = 0x12143bd6;
pub const ICE1712_SUBDEVICE_VX442: u32 = 0x12143cd6;
pub const ICE1712_SUBDEVICE_MEDIASTATION: u32 = 0x694c0100;
pub const ICE1712_SUBDEVICE_EDIROLDA2496: u32 = 0xce164010;

/* entry point */
unsafe extern "C" {
    pub static mut snd_ice1712_delta_cards: [snd_ice1712_card_info; 0];
}

/*
 *  MidiMan M-Audio Delta GPIO definitions
 */

/* MidiMan M-Audio Delta shared pins */
pub const ICE1712_DELTA_DFS: u8 = 0x01; /* fast/slow sample rate mode */
                                            /* (>48kHz must be 1) */
pub const ICE1712_DELTA_SPDIF_IN_STAT: u8 = 0x02;
                                            /* S/PDIF input status */
                                            /* 0 = valid signal is present */
                                            /* all except Delta44 */
                                            /* look to CS8414 datasheet */
pub const ICE1712_DELTA_SPDIF_OUT_STAT_CLOCK: u8 = 0x04;
                                            /* S/PDIF output status clock */
                                            /* (writing on rising edge - 0->1) */
                                            /* all except Delta44 */
                                            /* look to CS8404A datasheet */
pub const ICE1712_DELTA_SPDIF_OUT_STAT_DATA: u8 = 0x08;
                                            /* S/PDIF output status data */
                                            /* all except Delta44 */
                                            /* look to CS8404A datasheet */
/* MidiMan M-Audio DeltaDiO */
/* 0x01 = DFS */
/* 0x02 = SPDIF_IN_STAT */
/* 0x04 = SPDIF_OUT_STAT_CLOCK */
/* 0x08 = SPDIF_OUT_STAT_DATA */
pub const ICE1712_DELTA_SPDIF_INPUT_SELECT: u8 = 0x10;
                                            /* coaxial (0), optical (1) */
                                            /* S/PDIF input select*/

/* MidiMan M-Audio Delta1010 */
/* 0x01 = DFS */
/* 0x02 = SPDIF_IN_STAT */
/* 0x04 = SPDIF_OUT_STAT_CLOCK */
/* 0x08 = SPDIF_OUT_STAT_DATA */
pub const ICE1712_DELTA_WORD_CLOCK_SELECT: u8 = 0x10;
                                            /* 1 - clock are taken from S/PDIF input */
                                            /* 0 - clock are taken from Word Clock input */
                                            /* affected SPMCLKIN pin of Envy24 */
pub const ICE1712_DELTA_WORD_CLOCK_STATUS: u8 = 0x20;
                                            /* 0 = valid word clock signal is present */

/* MidiMan M-Audio Delta66 */
/* 0x01 = DFS */
/* 0x02 = SPDIF_IN_STAT */
/* 0x04 = SPDIF_OUT_STAT_CLOCK */
/* 0x08 = SPDIF_OUT_STAT_DATA */
pub const ICE1712_DELTA_CODEC_SERIAL_DATA: u8 = 0x10;
                                            /* AKM4524 serial data */
pub const ICE1712_DELTA_CODEC_SERIAL_CLOCK: u8 = 0x20;
                                            /* AKM4524 serial clock */
                                            /* (writing on rising edge - 0->1 */
pub const ICE1712_DELTA_CODEC_CHIP_A: u8 = 0x40;
pub const ICE1712_DELTA_CODEC_CHIP_B: u8 = 0x80;
                                            /* 1 - select chip A or B */

/* MidiMan M-Audio Delta44 */
/* 0x01 = DFS */
/* 0x10 = CODEC_SERIAL_DATA */
/* 0x20 = CODEC_SERIAL_CLOCK */
/* 0x40 = CODEC_CHIP_A */
/* 0x80 = CODEC_CHIP_B */

/* MidiMan M-Audio Audiophile/Delta410 definitions */
/* thanks to Kristof Pelckmans <Kristof.Pelckmans@antwerpen.be> for Delta410 info */
/* 0x01 = DFS */
pub const ICE1712_DELTA_AP_CCLK: u8 = 0x02; /* SPI clock */
                                            /* (clocking on rising edge - 0->1) */
pub const ICE1712_DELTA_AP_DIN: u8 = 0x04; /* data input */
pub const ICE1712_DELTA_AP_DOUT: u8 = 0x08; /* data output */
pub const ICE1712_DELTA_AP_CS_DIGITAL: u8 = 0x10; /* CS8427 chip select */
                                            /* low signal = select */
pub const ICE1712_DELTA_AP_CS_CODEC: u8 = 0x20; /* AK4528 (audiophile), AK4529 (Delta410) chip select */
                                            /* low signal = select */

/* MidiMan M-Audio Delta1010LT definitions */
/* thanks to Anders Johansson <ajh@watri.uwa.edu.au> */
/* 0x01 = DFS */
pub const ICE1712_DELTA_1010LT_CCLK: u8 = 0x02; /* SPI clock (AK4524 + CS8427) */
pub const ICE1712_DELTA_1010LT_DIN: u8 = 0x04; /* data input (CS8427) */
pub const ICE1712_DELTA_1010LT_DOUT: u8 = 0x08; /* data output (AK4524 + CS8427) */
pub const ICE1712_DELTA_1010LT_CS: u8 = 0x70; /* mask for CS address */
pub const ICE1712_DELTA_1010LT_CS_CHIP_A: u8 = 0x00; /* AK4524 #0 */
pub const ICE1712_DELTA_1010LT_CS_CHIP_B: u8 = 0x10; /* AK4524 #1 */
pub const ICE1712_DELTA_1010LT_CS_CHIP_C: u8 = 0x20; /* AK4524 #2 */
pub const ICE1712_DELTA_1010LT_CS_CHIP_D: u8 = 0x30; /* AK4524 #3 */
pub const ICE1712_DELTA_1010LT_CS_CS8427: u8 = 0x40; /* CS8427 */
pub const ICE1712_DELTA_1010LT_CS_NONE: u8 = 0x50; /* nothing */
pub const ICE1712_DELTA_1010LT_WORDCLOCK: u8 = 0x80; /* sample clock source: 0 = Word Clock Input, 1 = S/PDIF Input ??? */

/* M-Audio Delta 66 rev. E definitions.
 * Newer revisions of Delta 66 have CS8427 over SPI for
 * S/PDIF transceiver instead of CS8404/CS8414. */
/* 0x01 = DFS */
pub const ICE1712_DELTA_66E_CCLK: u8 = 0x02; /* SPI clock */
pub const ICE1712_DELTA_66E_DIN: u8 = 0x04; /* data input */
pub const ICE1712_DELTA_66E_DOUT: u8 = 0x08; /* data output */
pub const ICE1712_DELTA_66E_CS_CS8427: u8 = 0x10; /* chip select, low = CS8427 */
pub const ICE1712_DELTA_66E_CS_CHIP_A: u8 = 0x20; /* AK4524 #0 */
pub const ICE1712_DELTA_66E_CS_CHIP_B: u8 = 0x40; /* AK4524 #1 */

/* Digigram VX442 definitions */
pub const ICE1712_VX442_CCLK: u8 = 0x02; /* SPI clock */
pub const ICE1712_VX442_DIN: u8 = 0x04; /* data input */
pub const ICE1712_VX442_DOUT: u8 = 0x08; /* data output */
pub const ICE1712_VX442_CS_DIGITAL: u8 = 0x10; /* chip select, low = CS8427 */
pub const ICE1712_VX442_CODEC_CHIP_A: u8 = 0x20; /* select chip A */
pub const ICE1712_VX442_CODEC_CHIP_B: u8 = 0x40; /* select chip B */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
