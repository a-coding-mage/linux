// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Driver for PowerMac AWACS onboard soundchips
 * Copyright (c) 2001 by Takashi Iwai <tiwai@suse.de>
 *   based on dmasound.c.
 */

/*******************************/
/* AWACs Audio Register Layout */
/*******************************/

#[repr(C)]
pub struct awacs_regs {
    pub control: u32, /* Audio control register */
    pub pad0: [u32; 3],
    pub codec_ctrl: u32, /* Codec control register */
    pub pad1: [u32; 3],
    pub codec_stat: u32, /* Codec status register */
    pub pad2: [u32; 3],
    pub clip_count: u32, /* Clipping count register */
    pub pad3: [u32; 3],
    pub byteswap: u32, /* Data is little-endian if 1 */
}

/*******************/
/* Audio Bit Masks */
/*******************/

/* Audio Control Reg Bit Masks */
/* ----- ------- --- --- ----- */
pub const MASK_ISFSEL: u32 = 0xf; /* Input SubFrame Select */
pub const MASK_OSFSEL: u32 = 0xf << 4; /* Output SubFrame Select */
pub const MASK_RATE: u32 = 0x7 << 8; /* Sound Rate */
pub const MASK_CNTLERR: u32 = 0x1 << 11; /* Error */
pub const MASK_PORTCHG: u32 = 0x1 << 12; /* Port Change */
pub const MASK_IEE: u32 = 0x1 << 13; /* Enable Interrupt on Error */
pub const MASK_IEPC: u32 = 0x1 << 14; /* Enable Interrupt on Port Change */
pub const MASK_SSFSEL: u32 = 0x3 << 15; /* Status SubFrame Select */

/* Audio Codec Control Reg Bit Masks */
/* ----- ----- ------- --- --- ----- */
pub const MASK_NEWECMD: u32 = 0x1 << 24; /* Lock: don't write to reg when 1 */
pub const MASK_EMODESEL: u32 = 0x3 << 22; /* Send info out on which frame? */
pub const MASK_EXMODEADDR: u32 = 0x3ff << 12; /* Extended Mode Address -- 10 bits */
pub const MASK_EXMODEDATA: u32 = 0xfff; /* Extended Mode Data -- 12 bits */

/* Audio Codec Control Address Values / Masks */
/* ----- ----- ------- ------- ------ - ----- */
pub const MASK_ADDR0: u32 = 0x0 << 12; /* Expanded Data Mode Address 0 */
pub const MASK_ADDR_MUX: u32 = MASK_ADDR0; /* Mux Control */
pub const MASK_ADDR_GAIN: u32 = MASK_ADDR0;

pub const MASK_ADDR1: u32 = 0x1 << 12; /* Expanded Data Mode Address 1 */
pub const MASK_ADDR_MUTE: u32 = MASK_ADDR1;
pub const MASK_ADDR_RATE: u32 = MASK_ADDR1;

pub const MASK_ADDR2: u32 = 0x2 << 12; /* Expanded Data Mode Address 2 */
pub const MASK_ADDR_VOLA: u32 = MASK_ADDR2; /* Volume Control A -- Headphones */
pub const MASK_ADDR_VOLHD: u32 = MASK_ADDR2;

pub const MASK_ADDR4: u32 = 0x4 << 12; /* Expanded Data Mode Address 4 */
pub const MASK_ADDR_VOLC: u32 = MASK_ADDR4; /* Volume Control C -- Speaker */
pub const MASK_ADDR_VOLSPK: u32 = MASK_ADDR4;

/* additional registers of screamer */
pub const MASK_ADDR5: u32 = 0x5 << 12; /* Expanded Data Mode Address 5 */
pub const MASK_ADDR6: u32 = 0x6 << 12; /* Expanded Data Mode Address 6 */
pub const MASK_ADDR7: u32 = 0x7 << 12; /* Expanded Data Mode Address 7 */

/* Address 0 Bit Masks & Macros */
/* ------- - --- ----- - ------ */
pub const MASK_GAINRIGHT: u32 = 0xf; /* Gain Right Mask */
pub const MASK_GAINLEFT: u32 = 0xf << 4; /* Gain Left Mask */
pub const MASK_GAINLINE: u32 = 0x1 << 8; /* Disable Mic preamp */
pub const MASK_GAINMIC: u32 = 0x0 << 8; /* Enable Mic preamp */
pub const MASK_MUX_CD: u32 = 0x1 << 9; /* Select CD in MUX */
pub const MASK_MUX_MIC: u32 = 0x1 << 10; /* Select Mic in MUX */
pub const MASK_MUX_AUDIN: u32 = 0x1 << 11; /* Select Audio In in MUX */
pub const MASK_MUX_LINE: u32 = MASK_MUX_AUDIN;
pub const SHIFT_GAINLINE: u32 = 8;
pub const SHIFT_MUX_CD: u32 = 9;
pub const SHIFT_MUX_MIC: u32 = 10;
pub const SHIFT_MUX_LINE: u32 = 11;

pub const fn GAINRIGHT(x: u32) -> u32 {
    x & MASK_GAINRIGHT
}

pub const fn GAINLEFT(x: u32) -> u32 {
    (x << 4) & MASK_GAINLEFT
}

/* Address 1 Bit Masks */
/* ------- - --- ----- */
pub const MASK_ADDR1RES1: u32 = 0x3; /* Reserved */
pub const MASK_RECALIBRATE: u32 = 0x1 << 2; /* Recalibrate */
pub const MASK_SAMPLERATE: u32 = 0x7 << 3; /* Sample Rate: */
pub const MASK_LOOPTHRU: u32 = 0x1 << 6; /* Loopthrough Enable */
pub const SHIFT_LOOPTHRU: u32 = 6;
pub const MASK_CMUTE: u32 = 0x1 << 7; /* Output C (Speaker) Mute when 1 */
pub const MASK_SPKMUTE: u32 = MASK_CMUTE;
pub const SHIFT_SPKMUTE: u32 = 7;
pub const MASK_ADDR1RES2: u32 = 0x1 << 8; /* Reserved */
pub const MASK_AMUTE: u32 = 0x1 << 9; /* Output A (Headphone) Mute when 1 */
pub const MASK_HDMUTE: u32 = MASK_AMUTE;
pub const SHIFT_HDMUTE: u32 = 9;
pub const MASK_PAROUT: u32 = 0x3 << 10; /* Parallel Out (???) */
pub const MASK_PAROUT0: u32 = 0x1 << 10; /* Parallel Out (???) */
pub const MASK_PAROUT1: u32 = 0x1 << 11; /* Parallel Out (enable speaker) */
pub const SHIFT_PAROUT: u32 = 10;
pub const SHIFT_PAROUT0: u32 = 10;
pub const SHIFT_PAROUT1: u32 = 11;

pub const SAMPLERATE_48000: u32 = 0x0 << 3; /* 48 or 44.1 kHz */
pub const SAMPLERATE_32000: u32 = 0x1 << 3; /* 32 or 29.4 kHz */
pub const SAMPLERATE_24000: u32 = 0x2 << 3; /* 24 or 22.05 kHz */
pub const SAMPLERATE_19200: u32 = 0x3 << 3; /* 19.2 or 17.64 kHz */
pub const SAMPLERATE_16000: u32 = 0x4 << 3; /* 16 or 14.7 kHz */
pub const SAMPLERATE_12000: u32 = 0x5 << 3; /* 12 or 11.025 kHz */
pub const SAMPLERATE_9600: u32 = 0x6 << 3; /* 9.6 or 8.82 kHz */
pub const SAMPLERATE_8000: u32 = 0x7 << 3; /* 8 or 7.35 kHz */

/* Address 2 & 4 Bit Masks & Macros */
/* ------- - - - --- ----- - ------ */
pub const MASK_OUTVOLRIGHT: u32 = 0xf; /* Output Right Volume */
pub const MASK_ADDR2RES1: u32 = 0x2 << 4; /* Reserved */
pub const MASK_ADDR4RES1: u32 = MASK_ADDR2RES1;
pub const MASK_OUTVOLLEFT: u32 = 0xf << 6; /* Output Left Volume */
pub const MASK_ADDR2RES2: u32 = 0x2 << 10; /* Reserved */
pub const MASK_ADDR4RES2: u32 = MASK_ADDR2RES2;

pub const fn VOLRIGHT(x: u32) -> u32 {
    (!x) & MASK_OUTVOLRIGHT
}

pub const fn VOLLEFT(x: u32) -> u32 {
    ((!x) << 6) & MASK_OUTVOLLEFT
}

/* address 6 */
pub const MASK_MIC_BOOST: u32 = 0x4; /* screamer mic boost */
pub const SHIFT_MIC_BOOST: u32 = 2;

/* Audio Codec Status Reg Bit Masks */
/* ----- ----- ------ --- --- ----- */
pub const MASK_EXTEND: u32 = 0x1 << 23; /* Extend */
pub const MASK_VALID: u32 = 0x1 << 22; /* Valid Data? */
pub const MASK_OFLEFT: u32 = 0x1 << 21; /* Overflow Left */
pub const MASK_OFRIGHT: u32 = 0x1 << 20; /* Overflow Right */
pub const MASK_ERRCODE: u32 = 0xf << 16; /* Error Code */
pub const MASK_REVISION: u32 = 0xf << 12; /* Revision Number */
pub const MASK_MFGID: u32 = 0xf << 8; /* Mfg. ID */
pub const MASK_CODSTATRES: u32 = 0xf << 4; /* bits 4 - 7 reserved */
pub const MASK_INSENSE: u32 = 0xf; /* port sense bits: */
pub const MASK_HDPCONN: u32 = 8; /* headphone plugged in */
pub const MASK_LOCONN: u32 = 4; /* line-out plugged in */
pub const MASK_LICONN: u32 = 2; /* line-in plugged in */
pub const MASK_MICCONN: u32 = 1; /* microphone plugged in */
pub const MASK_LICONN_IMAC: u32 = 8; /* line-in plugged in */
pub const MASK_HDPRCONN_IMAC: u32 = 4; /* headphone right plugged in */
pub const MASK_HDPLCONN_IMAC: u32 = 2; /* headphone left plugged in */
pub const MASK_LOCONN_IMAC: u32 = 1; /* line-out plugged in */

/* Clipping Count Reg Bit Masks */
/* -------- ----- --- --- ----- */
pub const MASK_CLIPLEFT: u32 = 0xff << 7; /* Clipping Count, Left Channel */
pub const MASK_CLIPRIGHT: u32 = 0xff; /* Clipping Count, Right Channel */

/* DBDMA ChannelStatus Bit Masks */
/* ----- ------------- --- ----- */
pub const MASK_CSERR: u32 = 0x1 << 7; /* Error */
pub const MASK_EOI: u32 = 0x1 << 6; /* End of Input --
                                      * only for Input Channel */
pub const MASK_CSUNUSED: u32 = 0x1f << 1; /* bits 1-5 not used */
pub const MASK_WAIT: u32 = 0x1; /* Wait */

/* Various Rates */
/* ------- ----- */
pub const RATE_48000: u32 = 0x0 << 8; /* 48 kHz */
pub const RATE_44100: u32 = 0x0 << 8; /* 44.1 kHz */
pub const RATE_32000: u32 = 0x1 << 8; /* 32 kHz */
pub const RATE_29400: u32 = 0x1 << 8; /* 29.4 kHz */
pub const RATE_24000: u32 = 0x2 << 8; /* 24 kHz */
pub const RATE_22050: u32 = 0x2 << 8; /* 22.05 kHz */
pub const RATE_19200: u32 = 0x3 << 8; /* 19.2 kHz */
pub const RATE_17640: u32 = 0x3 << 8; /* 17.64 kHz */
pub const RATE_16000: u32 = 0x4 << 8; /* 16 kHz */
pub const RATE_14700: u32 = 0x4 << 8; /* 14.7 kHz */
pub const RATE_12000: u32 = 0x5 << 8; /* 12 kHz */
pub const RATE_11025: u32 = 0x5 << 8; /* 11.025 kHz */
pub const RATE_9600: u32 = 0x6 << 8; /* 9.6 kHz */
pub const RATE_8820: u32 = 0x6 << 8; /* 8.82 kHz */
pub const RATE_8000: u32 = 0x7 << 8; /* 8 kHz */
pub const RATE_7350: u32 = 0x7 << 8; /* 7.35 kHz */

pub const RATE_LOW: u32 = 1; /* HIGH = 48kHz, etc;  LOW = 44.1kHz, etc. */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
