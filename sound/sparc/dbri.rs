// SPDX-License-Identifier: GPL-2.0-only
/*
 * Driver for DBRI sound chip found on Sparcs.
 * Copyright (C) 2004, 2005 Martin Habets (mhabets@users.sourceforge.net)
 *
 * Converted to ring buffered version by Krzysztof Helt (krzysztof.h1@wp.pl)
 *
 * Based entirely upon drivers/sbus/audio/dbri.c which is:
 * Copyright (C) 1997 Rudolf Koenig (rfkoenig@immd4.informatik.uni-erlangen.de)
 * Copyright (C) 1998, 1999 Brent Baccala (baccala@freesoft.org)
 *
 * This is the low level driver for the DBRI & MMCODEC duo used for ISDN & AUDIO
 * on Sun SPARCStation 10, 20, LX and Voyager models.
 *
 * - DBRI: AT&T T5900FX Dual Basic Rates ISDN Interface. It is a 32 channel
 *   data time multiplexer with ISDN support (aka T7259)
 *   Interfaces: SBus,ISDN NT & TE, CHI, 4 bits parallel.
 *   CHI: (spelled ki) Concentration Highway Interface (AT&T or Intel bus ?).
 *   Documentation:
 *   - "STP 4000SBus Dual Basic Rate ISDN (DBRI) Transceiver" from
 *     Sparc Technology Business (courtesy of Sun Support)
 *   - Data sheet of the T7903, a newer but very similar ISA bus equivalent
 *     available from the Lucent (formerly AT&T microelectronics) home
 *     page.
 *   - https://www.freesoft.org/Linux/DBRI/
 * - MMCODEC: Crystal Semiconductor CS4215 16 bit Multimedia Audio Codec
 *   Interfaces: CHI, Audio In & Out, 2 bits parallel
 *   Documentation: from the Crystal Semiconductor home page.
 *
 * The DBRI is a 32 pipe machine, each pipe can transfer some bits between
 * memory and a serial device (long pipes, no. 0-15) or between two serial
 * devices (short pipes, no. 16-31), or simply send a fixed data to a serial
 * device (short pipes).
 * A timeslot defines the bit-offset and no. of bits read from a serial device.
 * The timeslots are linked to 6 circular lists, one for each direction for
 * each serial device (NT,TE,CHI). A timeslot is associated to 1 or 2 pipes
 * (the second one is a monitor/tee pipe, valid only for serial input).
 *
 * The mmcodec is connected via the CHI bus and needs the data & some
 * parameters (volume, output selection) time multiplexed in 8 byte
 * chunks. It also has a control mode, which serves for audio format setting.
 *
 * Looking at the CS4215 data sheet it is easy to set up 2 or 4 codecs on
 * the same CHI bus, so I thought perhaps it is possible to use the on-board
 * & the speakerbox codec simultaneously, giving 2 (not very independent :-)
 * audio devices. But the SUN HW group decided against it, at least on my
 * LX the speakerbox connector has at least 1 pin missing and 1 wrongly
 * connected.
 *
 * I've tried to stick to the following function naming conventions:
 * snd_*	ALSA stuff
 * cs4215_*	CS4215 codec specific stuff
 * dbri_*	DBRI high-level stuff
 * other	DBRI low-level stuff
 */

// For module_param, MODULE_AUTHOR, etc. - these are kernel macros
// that would need to be handled by build system integration

const D_INT: u32 = 1 << 0;
const D_GEN: u32 = 1 << 1;
const D_CMD: u32 = 1 << 2;
const D_MM: u32 = 1 << 3;
const D_USR: u32 = 1 << 4;
const D_DESC: u32 = 1 << 5;

static mut DBRI_DEBUG: i32 = 0;

#[cfg(feature = "DBRI_DEBUG")]
static CMDS: &[&str] = &[
    "WAIT", "PAUSE", "JUMP", "IIQ", "REX", "SDP", "CDP", "DTS",
    "SSP", "CHI", "NT", "TE", "CDEC", "TEST", "CDM", "RESRV"
];

macro_rules! dprintk {
    ($flag:expr, $($arg:tt)*) => {
        #[cfg(feature = "DBRI_DEBUG")]
        {
            unsafe {
                if DBRI_DEBUG & $flag != 0 {
                    println!($($arg)*);
                }
            }
        }
    };
}

const fn DBRI_CMD(cmd: u32, intr: u32, value: u32) -> i32 {
    ((cmd << 28) | (intr << 27) | value) as i32
}

struct Cs4215 {
    data: [u8; 4],
    ctrl: [u8; 4],
    onboard: u8,
    offset: u8,
    status: u32,
    version: u32,
    precision: u8,
    channels: u8,
}

const CS4215_CLB: u8 = 1 << 2;
const CS4215_OLB: u8 = 1 << 3;
const CS4215_MLB: u8 = 1 << 4;
const CS4215_RSRVD_1: u8 = 1 << 5;

const CS4215_DFR_LINEAR16: u8 = 0;
const CS4215_DFR_ULAW: u8 = 1;
const CS4215_DFR_ALAW: u8 = 2;
const CS4215_DFR_LINEAR8: u8 = 3;
const CS4215_DFR_STEREO: u8 = 1 << 2;

struct Cs4215Freq {
    freq: u16,
    xtal: u8,
    csval: u8,
}

static CS4215_FREQ: &[Cs4215Freq] = &[
    Cs4215Freq { freq: 8000, xtal: 1 << 4, csval: 0 << 3 },
    Cs4215Freq { freq: 16000, xtal: 1 << 4, csval: 1 << 3 },
    Cs4215Freq { freq: 27429, xtal: 1 << 4, csval: 2 << 3 },
    Cs4215Freq { freq: 32000, xtal: 1 << 4, csval: 3 << 3 },
    Cs4215Freq { freq: 48000, xtal: 1 << 4, csval: 6 << 3 },
    Cs4215Freq { freq: 9600, xtal: 1 << 4, csval: 7 << 3 },
    Cs4215Freq { freq: 5512, xtal: 2 << 4, csval: 0 << 3 },
    Cs4215Freq { freq: 11025, xtal: 2 << 4, csval: 1 << 3 },
    Cs4215Freq { freq: 18900, xtal: 2 << 4, csval: 2 << 3 },
    Cs4215Freq { freq: 22050, xtal: 2 << 4, csval: 3 << 3 },
    Cs4215Freq { freq: 37800, xtal: 2 << 4, csval: 4 << 3 },
    Cs4215Freq { freq: 44100, xtal: 2 << 4, csval: 5 << 3 },
    Cs4215Freq { freq: 33075, xtal: 2 << 4, csval: 6 << 3 },
    Cs4215Freq { freq: 6615, xtal: 2 << 4, csval: 7 << 3 },
    Cs4215Freq { freq: 0, xtal: 0, csval: 0 },
];

const CS4215_HPF: u8 = 1 << 7;
const CS4215_12_MASK: u16 = 0xfcbf;

const CS4215_XEN: u8 = 1 << 0;
const CS4215_XCLK: u8 = 1 << 1;
const CS4215_BSEL_64: u8 = 0 << 2;
const CS4215_BSEL_128: u8 = 1 << 2;
const CS4215_BSEL_256: u8 = 2 << 2;
const CS4215_MCK_MAST: u8 = 0 << 4;
const CS4215_MCK_XTL1: u8 = 1 << 4;
const CS4215_MCK_XTL2: u8 = 2 << 4;
const CS4215_MCK_CLK1: u8 = 3 << 4;
const CS4215_MCK_CLK2: u8 = 4 << 4;

const CS4215_DAD: u8 = 1 << 0;
const CS4215_ENL: u8 = 1 << 1;

const CS4215_VERSION_MASK: u8 = 0xf;

const fn CS4215_LO(v: u8) -> u8 { v }
const CS4215_LE: u8 = 1 << 6;
const CS4215_HE: u8 = 1 << 7;

const fn CS4215_RO(v: u8) -> u8 { v }
const CS4215_SE: u8 = 1 << 6;
const CS4215_ADI: u8 = 1 << 7;

const fn CS4215_LG(v: u8) -> u8 { v }
const CS4215_IS: u8 = 1 << 4;
const CS4215_OVR: u8 = 1 << 5;
const CS4215_PIO0: u8 = 1 << 6;
const CS4215_PIO1: u8 = 1 << 7;

const fn CS4215_RG(v: u8) -> u8 { v }
const fn CS4215_MA(v: u8) -> u8 { v << 4 }

const REG0: usize = 0x00;
const REG1: usize = 0x04;
const REG2: usize = 0x08;
const REG3: usize = 0x0c;
const REG8: usize = 0x20;
const REG9: usize = 0x24;

const DBRI_NO_CMDS: usize = 64;
const DBRI_INT_BLK: usize = 64;
const DBRI_NO_DESCS: usize = 64;
const DBRI_NO_PIPES: usize = 32;
const DBRI_MAX_PIPE: i32 = (DBRI_NO_PIPES - 1) as i32;

const DBRI_REC: usize = 0;
const DBRI_PLAY: usize = 1;
const DBRI_NO_STREAMS: usize = 2;

#[repr(C)]
struct DbriMem {
    word1: u32,
    ba: u32,
    nda: u32,
    word4: u32,
}

#[repr(C)]
struct DbriDma {
    cmd: [i32; DBRI_NO_CMDS],
    intr: [i32; DBRI_INT_BLK],
    desc: [DbriMem; DBRI_NO_DESCS],
}

const fn dbri_dma_off_intr(elem: usize) -> u32 {
    (elem as u32) * 4
}

const fn dbri_dma_off_desc(elem: usize) -> u32 {
    (DBRI_NO_CMDS as u32 * 4 + DBRI_INT_BLK as u32 * 4 + elem as u32 * 16) as u32
}

enum PipeDirection { PipeInput, PipeOutput }

struct DbriPipe {
    sdp: u32,
    nextpipe: i32,
    length: i32,
    first_desc: i32,
    desc: i32,
    recv_fixed_ptr: *mut u32,
}

struct DbriStreaminfo {
    substream: *mut core::ffi::c_void,
    dvma_buffer: u32,
    size: i32,
    offset: usize,
    pipe: i32,
    left_gain: i32,
    right_gain: i32,
}

struct SndDbri {
    regs_size: i32,
    irq: i32,
    op: *mut core::ffi::c_void,
    lock: u32,

    dma: *mut DbriDma,
    dma_dvma: u32,

    regs: *mut u8,
    dbri_irqp: i32,

    pipes: [DbriPipe; DBRI_NO_PIPES],
    next_desc: [i32; DBRI_NO_DESCS],
    cmdlock: u32,
    cmdptr: *mut i32,

    chi_bpf: i32,

    mm: Cs4215,
    stream_info: [DbriStreaminfo; DBRI_NO_STREAMS],
}

const DBRI_MAX_VOLUME: i32 = 63;
const DBRI_MAX_GAIN: i32 = 15;

const D_P: u32 = 1 << 15;
const D_G: u32 = 1 << 14;
const D_S: u32 = 1 << 13;
const D_E: u32 = 1 << 12;
const D_X: u32 = 1 << 7;
const D_T: u32 = 1 << 6;
const D_N: u32 = 1 << 5;
const D_C: u32 = 1 << 4;
const D_F: u32 = 1 << 3;
const D_D: u32 = 1 << 2;
const D_H: u32 = 1 << 1;
const D_R: u32 = 1 << 0;

const D_LITTLE_END: u32 = 1 << 8;
const D_BIG_END: u32 = 0 << 8;
const D_MRR: u32 = 1 << 4;
const D_MLE: u32 = 1 << 3;
const D_LBG: u32 = 1 << 2;
const D_MBE: u32 = 1 << 1;
const D_IR: u32 = 1 << 0;

const D_ENPIO3: u32 = 1 << 7;
const D_ENPIO2: u32 = 1 << 6;
const D_ENPIO1: u32 = 1 << 5;
const D_ENPIO0: u32 = 1 << 4;
const D_ENPIO: u32 = 0xf0;
const D_PIO3: u32 = 1 << 3;
const D_PIO2: u32 = 1 << 2;
const D_PIO1: u32 = 1 << 1;
const D_PIO0: u32 = 1 << 0;

const D_WAIT: u32 = 0x0;
const D_PAUSE: u32 = 0x1;
const D_JUMP: u32 = 0x2;
const D_IIQ: u32 = 0x3;
const D_REX: u32 = 0x4;
const D_SDP: u32 = 0x5;
const D_CDP: u32 = 0x6;
const D_DTS: u32 = 0x7;
const D_SSP: u32 = 0x8;
const D_CHI: u32 = 0x9;
const D_NT: u32 = 0xa;
const D_TE: u32 = 0xb;
const D_CDEC: u32 = 0xc;
const D_TEST: u32 = 0xd;
const D_CDM: u32 = 0xe;

const fn D_PIPE(v: u32) -> u32 { v << 0 }

const D_SDP_2SAME: u32 = 1 << 18;
const D_SDP_CHANGE: u32 = 2 << 18;
const D_SDP_EVERY: u32 = 3 << 18;
const D_SDP_EOL: u32 = 1 << 17;
const D_SDP_IDLE: u32 = 1 << 16;

const D_SDP_MEM: u32 = 0 << 13;
const D_SDP_HDLC: u32 = 2 << 13;
const D_SDP_HDLC_D: u32 = 3 << 13;
const D_SDP_SER: u32 = 4 << 13;
const D_SDP_FIXED: u32 = 6 << 13;

const fn D_SDP_MODE(v: u32) -> u32 { v & (7 << 13) }

const D_SDP_TO_SER: u32 = 1 << 12;
const D_SDP_FROM_SER: u32 = 0 << 12;
const D_SDP_MSB: u32 = 1 << 11;
const D_SDP_LSB: u32 = 0 << 11;
const D_SDP_P: u32 = 1 << 10;
const D_SDP_A: u32 = 1 << 8;
const D_SDP_C: u32 = 1 << 7;

const D_DTS_VI: u32 = 1 << 17;
const D_DTS_VO: u32 = 1 << 16;
const D_DTS_INS: u32 = 1 << 15;
const D_DTS_DEL: u32 = 0 << 15;

const fn D_DTS_PRVIN(v: i32) -> u32 { (v as u32) << 10 }
const fn D_DTS_PRVOUT(v: i32) -> u32 { (v as u32) << 5 }

const fn D_TS_LEN(v: i32) -> u32 { (v as u32) << 24 }
const fn D_TS_CYCLE(v: i32) -> u32 { (v as u32) << 14 }
const D_TS_DI: u32 = 1 << 13;
const D_TS_1CHANNEL: u32 = 0 << 10;
const D_TS_MONITOR: u32 = 2 << 10;
const D_TS_NONCONTIG: u32 = 3 << 10;
const D_TS_ANCHOR: u32 = 7 << 10;

const fn D_TS_MON(v: i32) -> u32 { (v as u32) << 5 }
const fn D_TS_NEXT(v: i32) -> u32 { v as u32 }

const fn D_CHI_CHICM(v: i32) -> u32 { (v as u32) << 16 }
const D_CHI_IR: u32 = 1 << 15;
const D_CHI_EN: u32 = 1 << 14;
const D_CHI_OD: u32 = 1 << 13;
const D_CHI_FE: u32 = 1 << 12;
const D_CHI_FD: u32 = 1 << 11;

const fn D_CHI_BPF(v: i32) -> u32 { v as u32 }

const D_NT_FBIT: u32 = 1 << 17;
const D_NT_NBF: u32 = 1 << 16;
const D_NT_IRM_IMM: u32 = 1 << 15;
const D_NT_IRM_EN: u32 = 1 << 14;
const D_NT_ISNT: u32 = 1 << 13;
const D_NT_FT: u32 = 1 << 12;
const D_NT_EZ: u32 = 1 << 11;
const D_NT_IFA: u32 = 1 << 10;
const D_NT_ACT: u32 = 1 << 9;
const D_NT_MFE: u32 = 1 << 8;

const fn D_NT_RLB(v: i32) -> u32 { (v as u32) << 5 }
const fn D_NT_LLB(v: i32) -> u32 { (v as u32) << 2 }

const D_NT_FACT: u32 = 1 << 1;
const D_NT_ABV: u32 = 1 << 0;

const fn D_CDEC_CK(v: i32) -> u32 { (v as u32) << 24 }
const fn D_CDEC_FED(v: i32) -> u32 { (v as u32) << 12 }
const fn D_CDEC_RED(v: i32) -> u32 { v as u32 }

const fn D_TEST_RAM(v: i32) -> u32 { (v as u32) << 16 }
const fn D_TEST_SIZE(v: i32) -> u32 { (v as u32) << 11 }

const D_TEST_ROMONOFF: u32 = 0x5;
const D_TEST_PROC: u32 = 0x6;
const D_TEST_SER: u32 = 0x7;
const D_TEST_RAMREAD: u32 = 0x8;
const D_TEST_RAMWRITE: u32 = 0x9;
const D_TEST_RAMBIST: u32 = 0xa;
const D_TEST_MCBIST: u32 = 0xb;
const D_TEST_DUMP: u32 = 0xe;

const D_CDM_THI: u32 = 1 << 8;
const D_CDM_RHI: u32 = 1 << 7;
const D_CDM_RCE: u32 = 1 << 6;
const D_CDM_XCE: u32 = 1 << 2;
const D_CDM_XEN: u32 = 1 << 1;
const D_CDM_REN: u32 = 1 << 0;

const D_INTR_BRDY: u32 = 1;
const D_INTR_MINT: u32 = 2;
const D_INTR_IBEG: u32 = 3;
const D_INTR_IEND: u32 = 4;
const D_INTR_EOL: u32 = 5;
const D_INTR_CMDI: u32 = 6;
const D_INTR_XCMP: u32 = 8;
const D_INTR_SBRI: u32 = 9;
const D_INTR_FXDT: u32 = 10;
const D_INTR_CHIL: u32 = 11;
const D_INTR_COLL: u32 = 11;
const D_INTR_DBYT: u32 = 12;
const D_INTR_RBYT: u32 = 13;
const D_INTR_LINT: u32 = 14;
const D_INTR_UNDR: u32 = 15;

const D_INTR_TE: u32 = 32;
const D_INTR_NT: u32 = 34;
const D_INTR_CHI: u32 = 36;
const D_INTR_CMD: u32 = 38;

const fn D_INTR_GETCHAN(v: i32) -> u32 { ((v as u32) >> 24) & 0x3f }
const fn D_INTR_GETCODE(v: i32) -> u32 { ((v as u32) >> 20) & 0xf }
const fn D_INTR_GETCMD(v: i32) -> u32 { ((v as u32) >> 16) & 0xf }
const fn D_INTR_GETVAL(v: i32) -> i32 { v & 0xffff }
const fn D_INTR_GETRVAL(v: i32) -> u32 { (v as u32) & 0xfffff }

const D_P_0: i32 = 0;
const D_P_1: i32 = 1;
const D_P_2: i32 = 2;
const D_P_3: i32 = 3;
const D_P_4: i32 = 4;
const D_P_5: i32 = 5;
const D_P_6: i32 = 6;
const D_P_7: i32 = 7;
const D_P_8: i32 = 8;
const D_P_9: i32 = 9;
const D_P_10: i32 = 10;
const D_P_11: i32 = 11;
const D_P_12: i32 = 12;
const D_P_13: i32 = 13;
const D_P_14: i32 = 14;
const D_P_15: i32 = 15;
const D_P_16: i32 = 16;
const D_P_17: i32 = 17;
const D_P_18: i32 = 18;
const D_P_19: i32 = 19;
const D_P_20: i32 = 20;
const D_P_21: i32 = 21;
const D_P_22: i32 = 22;
const D_P_23: i32 = 23;
const D_P_24: i32 = 24;
const D_P_25: i32 = 25;
const D_P_26: i32 = 26;
const D_P_27: i32 = 27;
const D_P_28: i32 = 28;
const D_P_29: i32 = 29;
const D_P_30: i32 = 30;
const D_P_31: i32 = 31;

const DBRI_TD_F: u32 = 1 << 31;
const DBRI_TD_D: u32 = 1 << 30;

const fn DBRI_TD_CNT(v: i32) -> u32 { (v as u32) << 16 }

const DBRI_TD_B: u32 = 1 << 15;
const DBRI_TD_M: u32 = 1 << 14;
const DBRI_TD_I: u32 = 1 << 13;

const fn DBRI_TD_FCNT(v: u32) -> u32 { v }

const DBRI_TD_UNR: u32 = 1 << 3;
const DBRI_TD_ABT: u32 = 1 << 2;
const DBRI_TD_TBC: u32 = 1 << 0;

const fn DBRI_TD_STATUS(v: u32) -> u32 { v & 0xff }

const DBRI_TD_MAXCNT: i32 = (1 << 13) - 4;

const DBRI_RD_F: u32 = 1 << 31;
const DBRI_RD_C: u32 = 1 << 30;
const DBRI_RD_B: u32 = 1 << 15;
const DBRI_RD_M: u32 = 1 << 14;

const fn DBRI_RD_BCNT(v: i32) -> u32 { v as u32 }

const DBRI_RD_CRC: u32 = 1 << 7;
const DBRI_RD_BBC: u32 = 1 << 6;
const DBRI_RD_ABT: u32 = 1 << 5;
const DBRI_RD_OVRN: u32 = 1 << 3;

const fn DBRI_RD_STATUS(v: u32) -> u32 { v & 0xff }
const fn DBRI_RD_CNT(v: u32) -> u32 { (v >> 16) & 0x1fff }

fn reverse_bytes(mut b: u32, len: i32) -> u32 {
    match len {
        32 => {
            b = ((b & 0xffff0000) >> 16) | ((b & 0x0000ffff) << 16);
            b = ((b & 0xff00ff00) >> 8) | ((b & 0x00ff00ff) << 8);
            b = ((b & 0xf0f0f0f0) >> 4) | ((b & 0x0f0f0f0f) << 4);
            b = ((b & 0xcccccccc) >> 2) | ((b & 0x33333333) << 2);
            b = ((b & 0xaaaaaaaa) >> 1) | ((b & 0x55555555) << 1);
            b
        }
        16 => {
            b = ((b & 0xff00ff00) >> 8) | ((b & 0x00ff00ff) << 8);
            b = ((b & 0xf0f0f0f0) >> 4) | ((b & 0x0f0f0f0f) << 4);
            b = ((b & 0xcccccccc) >> 2) | ((b & 0x33333333) << 2);
            b = ((b & 0xaaaaaaaa) >> 1) | ((b & 0x55555555) << 1);
            b
        }
        8 => {
            b = ((b & 0xf0f0f0f0) >> 4) | ((b & 0x0f0f0f0f) << 4);
            b = ((b & 0xcccccccc) >> 2) | ((b & 0x33333333) << 2);
            b = ((b & 0xaaaaaaaa) >> 1) | ((b & 0x55555555) << 1);
            b
        }
        4 => {
            b = ((b & 0xcccccccc) >> 2) | ((b & 0x33333333) << 2);
            b = ((b & 0xaaaaaaaa) >> 1) | ((b & 0x55555555) << 1);
            b
        }
        2 => {
            b = ((b & 0xaaaaaaaa) >> 1) | ((b & 0x55555555) << 1);
            b
        }
        1 | 0 => b,
        _ => {
            eprintln!("DBRI reverse_bytes: unsupported length");
            b
        }
    }
}

const MAXLOOPS: i32 = 20;

unsafe fn dbri_cmdwait(_dbri: *mut SndDbri) {
    // Placeholder for kernel-specific implementation
    // Would need spin locks and hardware access
}

unsafe fn dbri_cmdlock(_dbri: *mut SndDbri, _len: i32) -> *mut i32 {
    // Placeholder for kernel-specific implementation
    core::ptr::null_mut()
}

unsafe fn dbri_cmdsend(_dbri: *mut SndDbri, _cmd: *mut i32, _len: i32) {
    // Placeholder for kernel-specific implementation
}

unsafe fn dbri_reset(_dbri: *mut SndDbri) {
    // Placeholder for kernel-specific implementation
}

unsafe fn dbri_initialize(_dbri: *mut SndDbri) {
    // Placeholder for kernel-specific implementation
}

fn pipe_active(_dbri: *const SndDbri, pipe: i32) -> bool {
    pipe >= 0 && pipe <= DBRI_MAX_PIPE
}

unsafe fn reset_pipe(_dbri: *mut SndDbri, _pipe: i32) {
    // Placeholder for kernel-specific implementation
}

unsafe fn setup_pipe(_dbri: *mut SndDbri, _pipe: i32, _sdp: u32) {
    // Placeholder for kernel-specific implementation
}

unsafe fn link_time_slot(
    _dbri: *mut SndDbri,
    _pipe: i32,
    _prevpipe: i32,
    _nextpipe: i32,
    _length: i32,
    _cycle: i32,
) {
    // Placeholder for kernel-specific implementation
}

unsafe fn xmit_fixed(_dbri: *mut SndDbri, _pipe: i32, _data: u32) {
    // Placeholder for kernel-specific implementation
}

unsafe fn recv_fixed(_dbri: *mut SndDbri, _pipe: i32, _ptr: *mut u32) {
    // Placeholder for kernel-specific implementation
}

unsafe fn setup_descs(_dbri: *mut SndDbri, _streamno: usize, _period: u32) -> i32 {
    // Placeholder for kernel-specific implementation
    0
}

enum MasterOrSlave { ChiMaster, ChiSlave }

unsafe fn reset_chi(
    _dbri: *mut SndDbri,
    _master_or_slave: MasterOrSlave,
    _bits_per_frame: i32,
) {
    // Placeholder for kernel-specific implementation
}

unsafe fn cs4215_setup_pipes(_dbri: *mut SndDbri) {
    // Placeholder for kernel-specific implementation
}

fn cs4215_init_data(mm: *mut Cs4215) -> i32 {
    unsafe {
        (*mm).data[0] = CS4215_LO(0x20) | CS4215_HE | CS4215_LE;
        (*mm).data[1] = CS4215_RO(0x20) | CS4215_SE;
        (*mm).data[2] = CS4215_LG(0x8) | CS4215_IS | CS4215_PIO0 | CS4215_PIO1;
        (*mm).data[3] = CS4215_RG(0x8) | CS4215_MA(0xf);

        (*mm).ctrl[0] = CS4215_RSRVD_1 | CS4215_MLB;
        (*mm).ctrl[1] = CS4215_DFR_ULAW | CS4215_FREQ[0].csval;
        (*mm).ctrl[2] = CS4215_XCLK | CS4215_BSEL_128 | CS4215_FREQ[0].xtal;
        (*mm).ctrl[3] = 0;

        (*mm).status = 0;
        (*mm).version = 0xff;
        (*mm).precision = 8;
        (*mm).channels = 1;
    }
    0
}

unsafe fn cs4215_setdata(_dbri: *mut SndDbri, _muted: i32) {
    // Placeholder for kernel-specific implementation
}

unsafe fn cs4215_open(_dbri: *mut SndDbri) {
    // Placeholder for kernel-specific implementation
}

unsafe fn cs4215_setctrl(_dbri: *mut SndDbri) -> i32 {
    // Placeholder for kernel-specific implementation
    0
}

unsafe fn cs4215_prepare(
    _dbri: *mut SndDbri,
    _rate: u32,
    _format: u32,
    _channels: u32,
) -> i32 {
    // Placeholder for kernel-specific implementation
    0
}

unsafe fn cs4215_init(_dbri: *mut SndDbri) -> i32 {
    // Placeholder for kernel-specific implementation
    0
}

unsafe fn xmit_descs(_dbri: *mut SndDbri) {
    // Placeholder for kernel-specific implementation
}

unsafe fn transmission_complete_intr(_dbri: *mut SndDbri, _pipe: i32) {
    // Placeholder for kernel-specific implementation
}

unsafe fn reception_complete_intr(_dbri: *mut SndDbri, _pipe: i32) {
    // Placeholder for kernel-specific implementation
}

unsafe fn dbri_process_one_interrupt(_dbri: *mut SndDbri, _x: i32) {
    // Placeholder for kernel-specific implementation
}

unsafe fn dbri_process_interrupt_buffer(_dbri: *mut SndDbri) {
    // Placeholder for kernel-specific implementation
}

unsafe extern "C" fn snd_dbri_interrupt(_irq: i32, _dev_id: *mut core::ffi::c_void) -> i32 {
    // Placeholder for kernel-specific implementation
    0
}

struct SndPcmHardware {
    info: u32,
    formats: u64,
    rates: u32,
    rate_min: u32,
    rate_max: u32,
    channels_min: u32,
    channels_max: u32,
    buffer_bytes_max: usize,
    period_bytes_min: usize,
    period_bytes_max: usize,
    periods_min: usize,
    periods_max: usize,
}

static SND_DBRI_PCM_HW: SndPcmHardware = SndPcmHardware {
    info: 0,
    formats: 0,
    rates: 0,
    rate_min: 5512,
    rate_max: 48000,
    channels_min: 1,
    channels_max: 2,
    buffer_bytes_max: 64 * 1024,
    period_bytes_min: 1,
    period_bytes_max: (DBRI_TD_MAXCNT + 1) as usize,
    periods_min: 1,
    periods_max: 1024,
};

unsafe fn snd_dbri_open(_substream: *mut core::ffi::c_void) -> i32 {
    // Placeholder for kernel-specific implementation
    0
}

unsafe fn snd_dbri_close(_substream: *mut core::ffi::c_void) -> i32 {
    // Placeholder for kernel-specific implementation
    0
}

unsafe fn snd_dbri_hw_params(_substream: *mut core::ffi::c_void) -> i32 {
    // Placeholder for kernel-specific implementation
    0
}

unsafe fn snd_dbri_hw_free(_substream: *mut core::ffi::c_void) -> i32 {
    // Placeholder for kernel-specific implementation
    0
}

unsafe fn snd_dbri_prepare(_substream: *mut core::ffi::c_void) -> i32 {
    // Placeholder for kernel-specific implementation
    0
}

unsafe fn snd_dbri_trigger(_substream: *mut core::ffi::c_void, _cmd: i32) -> i32 {
    // Placeholder for kernel-specific implementation
    0
}

unsafe fn snd_dbri_pointer(_substream: *mut core::ffi::c_void) -> usize {
    // Placeholder for kernel-specific implementation
    0
}

unsafe fn snd_dbri_pcm(_card: *mut core::ffi::c_void) -> i32 {
    // Placeholder for kernel-specific implementation
    0
}

unsafe fn snd_cs4215_info_volume(
    _kcontrol: *mut core::ffi::c_void,
    _uinfo: *mut core::ffi::c_void,
) -> i32 {
    // Placeholder for kernel-specific implementation
    0
}

unsafe fn snd_cs4215_get_volume(
    _kcontrol: *mut core::ffi::c_void,
    _ucontrol: *mut core::ffi::c_void,
) -> i32 {
    // Placeholder for kernel-specific implementation
    0
}

unsafe fn snd_cs4215_put_volume(
    _kcontrol: *mut core::ffi::c_void,
    _ucontrol: *mut core::ffi::c_void,
) -> i32 {
    // Placeholder for kernel-specific implementation
    0
}

unsafe fn snd_cs4215_info_single(
    _kcontrol: *mut core::ffi::c_void,
    _uinfo: *mut core::ffi::c_void,
) -> i32 {
    // Placeholder for kernel-specific implementation
    0
}

unsafe fn snd_cs4215_get_single(
    _kcontrol: *mut core::ffi::c_void,
    _ucontrol: *mut core::ffi::c_void,
) -> i32 {
    // Placeholder for kernel-specific implementation
    0
}

unsafe fn snd_cs4215_put_single(
    _kcontrol: *mut core::ffi::c_void,
    _ucontrol: *mut core::ffi::c_void,
) -> i32 {
    // Placeholder for kernel-specific implementation
    0
}

unsafe fn snd_dbri_mixer(_card: *mut core::ffi::c_void) -> i32 {
    // Placeholder for kernel-specific implementation
    0
}

unsafe fn dbri_regs_read(
    _entry: *mut core::ffi::c_void,
    _buffer: *mut core::ffi::c_void,
) {
    // Placeholder for kernel-specific implementation
}

#[cfg(feature = "DBRI_DEBUG")]
unsafe fn dbri_debug_read(
    _entry: *mut core::ffi::c_void,
    _buffer: *mut core::ffi::c_void,
) {
    // Placeholder for kernel-specific implementation
}

unsafe fn snd_dbri_proc(_card: *mut core::ffi::c_void) {
    // Placeholder for kernel-specific implementation
}

unsafe fn snd_dbri_free(_dbri: *mut SndDbri) {
    // Placeholder for kernel-specific implementation
}

unsafe fn snd_dbri_create(
    _card: *mut core::ffi::c_void,
    _op: *mut core::ffi::c_void,
    _irq: i32,
    _dev: i32,
) -> i32 {
    // Placeholder for kernel-specific implementation
    0
}

unsafe fn dbri_probe(_op: *mut core::ffi::c_void) -> i32 {
    // Placeholder for kernel-specific implementation
    0
}

unsafe fn dbri_remove(_op: *mut core::ffi::c_void) {
    // Placeholder for kernel-specific implementation
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
