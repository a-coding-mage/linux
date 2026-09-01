// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   AD1843 low level driver
 *
 *   Copyright 2003 Vivien Chappelier <vivien.chappelier@linux-mips.org>
 *   Copyright 2008 Thomas Bogendoerfer <tsbogend@alpha.franken.de>
 *
 *   inspired from vwsnd.c (SGI VW audio driver)
 *     Copyright 1999 Silicon Graphics, Inc.  All rights reserved.
 */

/*
 * Original C dependencies:
 * <linux/init.h>, <linux/sched.h>, <linux/errno.h>,
 * <sound/core.h>, <sound/pcm.h>, <sound/ad1843.h>
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

pub type snd_pcm_format_t = c_int;

extern "C" {
    static mut jiffies: c_ulong;

    fn printk(fmt: *const c_char, ...) -> c_int;
    fn msecs_to_jiffies(m: c_uint) -> c_ulong;
    fn schedule_timeout_interruptible(timeout: c_long) -> c_long;
    fn time_after(a: c_ulong, b: c_ulong) -> bool;
}

pub type c_long = i64;

extern "C" {
    static AD1843_GAIN_SIZE: usize;
    static AD1843_GAIN_RECLEV: c_int;
    static AD1843_GAIN_LINE: c_int;
    static AD1843_GAIN_LINE_2: c_int;
    static AD1843_GAIN_MIC: c_int;
    static AD1843_GAIN_PCM_0: c_int;
    static AD1843_GAIN_PCM_1: c_int;

    static SNDRV_PCM_FORMAT_S8: snd_pcm_format_t;
    static SNDRV_PCM_FORMAT_U8: snd_pcm_format_t;
    static SNDRV_PCM_FORMAT_S16_LE: snd_pcm_format_t;
    static SNDRV_PCM_FORMAT_MU_LAW: snd_pcm_format_t;
    static SNDRV_PCM_FORMAT_A_LAW: snd_pcm_format_t;
}

const EINVAL: c_int = 22;
const EIO: c_int = 5;
const KERN_ERR: &[u8] = b"\x013";

#[repr(C)]
pub struct snd_ad1843 {
    pub chip: *mut c_void,
    pub read: Option<unsafe extern "C" fn(*mut c_void, c_int) -> c_int>,
    pub write: Option<unsafe extern "C" fn(*mut c_void, c_int, c_int)>,
}

/*
 * AD1843 bitfield definitions.  All are named as in the AD1843 data
 * sheet, with ad1843_ prepended and individual bit numbers removed.
 *
 * E.g., bits LSS0 through LSS2 become ad1843_LSS.
 *
 * Only the bitfields we need are defined.
 */

#[repr(C)]
struct ad1843_bitfield {
    reg: c_char,
    lo_bit: c_char,
    nbits: c_char,
}

static ad1843_PDNO: ad1843_bitfield = ad1843_bitfield { reg: 0, lo_bit: 14, nbits: 1 }; /* Converter Power-Down Flag */
static ad1843_INIT: ad1843_bitfield = ad1843_bitfield { reg: 0, lo_bit: 15, nbits: 1 }; /* Clock Initialization Flag */
static ad1843_RIG: ad1843_bitfield = ad1843_bitfield { reg: 2, lo_bit: 0, nbits: 4 }; /* Right ADC Input Gain */
static ad1843_RMGE: ad1843_bitfield = ad1843_bitfield { reg: 2, lo_bit: 4, nbits: 1 }; /* Right ADC Mic Gain Enable */
static ad1843_RSS: ad1843_bitfield = ad1843_bitfield { reg: 2, lo_bit: 5, nbits: 3 }; /* Right ADC Source Select */
static ad1843_LIG: ad1843_bitfield = ad1843_bitfield { reg: 2, lo_bit: 8, nbits: 4 }; /* Left ADC Input Gain */
static ad1843_LMGE: ad1843_bitfield = ad1843_bitfield { reg: 2, lo_bit: 12, nbits: 1 }; /* Left ADC Mic Gain Enable */
static ad1843_LSS: ad1843_bitfield = ad1843_bitfield { reg: 2, lo_bit: 13, nbits: 3 }; /* Left ADC Source Select */
static ad1843_RD2M: ad1843_bitfield = ad1843_bitfield { reg: 3, lo_bit: 0, nbits: 5 }; /* Right DAC 2 Mix Gain/Atten */
static ad1843_RD2MM: ad1843_bitfield = ad1843_bitfield { reg: 3, lo_bit: 7, nbits: 1 }; /* Right DAC 2 Mix Mute */
static ad1843_LD2M: ad1843_bitfield = ad1843_bitfield { reg: 3, lo_bit: 8, nbits: 5 }; /* Left DAC 2 Mix Gain/Atten */
static ad1843_LD2MM: ad1843_bitfield = ad1843_bitfield { reg: 3, lo_bit: 15, nbits: 1 }; /* Left DAC 2 Mix Mute */
static ad1843_RX1M: ad1843_bitfield = ad1843_bitfield { reg: 4, lo_bit: 0, nbits: 5 }; /* Right Aux 1 Mix Gain/Atten */
static ad1843_RX1MM: ad1843_bitfield = ad1843_bitfield { reg: 4, lo_bit: 7, nbits: 1 }; /* Right Aux 1 Mix Mute */
static ad1843_LX1M: ad1843_bitfield = ad1843_bitfield { reg: 4, lo_bit: 8, nbits: 5 }; /* Left Aux 1 Mix Gain/Atten */
static ad1843_LX1MM: ad1843_bitfield = ad1843_bitfield { reg: 4, lo_bit: 15, nbits: 1 }; /* Left Aux 1 Mix Mute */
static ad1843_RX2M: ad1843_bitfield = ad1843_bitfield { reg: 5, lo_bit: 0, nbits: 5 }; /* Right Aux 2 Mix Gain/Atten */
static ad1843_RX2MM: ad1843_bitfield = ad1843_bitfield { reg: 5, lo_bit: 7, nbits: 1 }; /* Right Aux 2 Mix Mute */
static ad1843_LX2M: ad1843_bitfield = ad1843_bitfield { reg: 5, lo_bit: 8, nbits: 5 }; /* Left Aux 2 Mix Gain/Atten */
static ad1843_LX2MM: ad1843_bitfield = ad1843_bitfield { reg: 5, lo_bit: 15, nbits: 1 }; /* Left Aux 2 Mix Mute */
static ad1843_RMCM: ad1843_bitfield = ad1843_bitfield { reg: 7, lo_bit: 0, nbits: 5 }; /* Right Mic Mix Gain/Atten */
static ad1843_RMCMM: ad1843_bitfield = ad1843_bitfield { reg: 7, lo_bit: 7, nbits: 1 }; /* Right Mic Mix Mute */
static ad1843_LMCM: ad1843_bitfield = ad1843_bitfield { reg: 7, lo_bit: 8, nbits: 5 }; /* Left Mic Mix Gain/Atten */
static ad1843_LMCMM: ad1843_bitfield = ad1843_bitfield { reg: 7, lo_bit: 15, nbits: 1 }; /* Left Mic Mix Mute */
static ad1843_HPOS: ad1843_bitfield = ad1843_bitfield { reg: 8, lo_bit: 4, nbits: 1 }; /* Headphone Output Voltage Swing */
static ad1843_HPOM: ad1843_bitfield = ad1843_bitfield { reg: 8, lo_bit: 5, nbits: 1 }; /* Headphone Output Mute */
static ad1843_MPOM: ad1843_bitfield = ad1843_bitfield { reg: 8, lo_bit: 6, nbits: 1 }; /* Mono Output Mute */
static ad1843_RDA1G: ad1843_bitfield = ad1843_bitfield { reg: 9, lo_bit: 0, nbits: 6 }; /* Right DAC1 Analog/Digital Gain */
static ad1843_RDA1GM: ad1843_bitfield = ad1843_bitfield { reg: 9, lo_bit: 7, nbits: 1 }; /* Right DAC1 Analog Mute */
static ad1843_LDA1G: ad1843_bitfield = ad1843_bitfield { reg: 9, lo_bit: 8, nbits: 6 }; /* Left DAC1 Analog/Digital Gain */
static ad1843_LDA1GM: ad1843_bitfield = ad1843_bitfield { reg: 9, lo_bit: 15, nbits: 1 }; /* Left DAC1 Analog Mute */
static ad1843_RDA2G: ad1843_bitfield = ad1843_bitfield { reg: 10, lo_bit: 0, nbits: 6 }; /* Right DAC2 Analog/Digital Gain */
static ad1843_RDA2GM: ad1843_bitfield = ad1843_bitfield { reg: 10, lo_bit: 7, nbits: 1 }; /* Right DAC2 Analog Mute */
static ad1843_LDA2G: ad1843_bitfield = ad1843_bitfield { reg: 10, lo_bit: 8, nbits: 6 }; /* Left DAC2 Analog/Digital Gain */
static ad1843_LDA2GM: ad1843_bitfield = ad1843_bitfield { reg: 10, lo_bit: 15, nbits: 1 }; /* Left DAC2 Analog Mute */
static ad1843_RDA1AM: ad1843_bitfield = ad1843_bitfield { reg: 11, lo_bit: 7, nbits: 1 }; /* Right DAC1 Digital Mute */
static ad1843_LDA1AM: ad1843_bitfield = ad1843_bitfield { reg: 11, lo_bit: 15, nbits: 1 }; /* Left DAC1 Digital Mute */
static ad1843_RDA2AM: ad1843_bitfield = ad1843_bitfield { reg: 12, lo_bit: 7, nbits: 1 }; /* Right DAC2 Digital Mute */
static ad1843_LDA2AM: ad1843_bitfield = ad1843_bitfield { reg: 12, lo_bit: 15, nbits: 1 }; /* Left DAC2 Digital Mute */
static ad1843_ADLC: ad1843_bitfield = ad1843_bitfield { reg: 15, lo_bit: 0, nbits: 2 }; /* ADC Left Sample Rate Source */
static ad1843_ADRC: ad1843_bitfield = ad1843_bitfield { reg: 15, lo_bit: 2, nbits: 2 }; /* ADC Right Sample Rate Source */
static ad1843_DA1C: ad1843_bitfield = ad1843_bitfield { reg: 15, lo_bit: 8, nbits: 2 }; /* DAC1 Sample Rate Source */
static ad1843_DA2C: ad1843_bitfield = ad1843_bitfield { reg: 15, lo_bit: 10, nbits: 2 }; /* DAC2 Sample Rate Source */
static ad1843_C1C: ad1843_bitfield = ad1843_bitfield { reg: 17, lo_bit: 0, nbits: 16 }; /* Clock 1 Sample Rate Select */
static ad1843_C2C: ad1843_bitfield = ad1843_bitfield { reg: 20, lo_bit: 0, nbits: 16 }; /* Clock 2 Sample Rate Select */
static ad1843_C3C: ad1843_bitfield = ad1843_bitfield { reg: 23, lo_bit: 0, nbits: 16 }; /* Clock 3 Sample Rate Select */
static ad1843_DAADL: ad1843_bitfield = ad1843_bitfield { reg: 25, lo_bit: 4, nbits: 2 }; /* Digital ADC Left Source Select */
static ad1843_DAADR: ad1843_bitfield = ad1843_bitfield { reg: 25, lo_bit: 6, nbits: 2 }; /* Digital ADC Right Source Select */
static ad1843_DAMIX: ad1843_bitfield = ad1843_bitfield { reg: 25, lo_bit: 14, nbits: 1 }; /* DAC Digital Mix Enable */
static ad1843_DRSFLT: ad1843_bitfield = ad1843_bitfield { reg: 25, lo_bit: 15, nbits: 1 }; /* Digital Reampler Filter Mode */
static ad1843_ADLF: ad1843_bitfield = ad1843_bitfield { reg: 26, lo_bit: 0, nbits: 2 }; /* ADC Left Channel Data Format */
static ad1843_ADRF: ad1843_bitfield = ad1843_bitfield { reg: 26, lo_bit: 2, nbits: 2 }; /* ADC Right Channel Data Format */
static ad1843_ADTLK: ad1843_bitfield = ad1843_bitfield { reg: 26, lo_bit: 4, nbits: 1 }; /* ADC Transmit Lock Mode Select */
static ad1843_SCF: ad1843_bitfield = ad1843_bitfield { reg: 26, lo_bit: 7, nbits: 1 }; /* SCLK Frequency Select */
static ad1843_DA1F: ad1843_bitfield = ad1843_bitfield { reg: 26, lo_bit: 8, nbits: 2 }; /* DAC1 Data Format Select */
static ad1843_DA2F: ad1843_bitfield = ad1843_bitfield { reg: 26, lo_bit: 10, nbits: 2 }; /* DAC2 Data Format Select */
static ad1843_DA1SM: ad1843_bitfield = ad1843_bitfield { reg: 26, lo_bit: 14, nbits: 1 }; /* DAC1 Stereo/Mono Mode Select */
static ad1843_DA2SM: ad1843_bitfield = ad1843_bitfield { reg: 26, lo_bit: 15, nbits: 1 }; /* DAC2 Stereo/Mono Mode Select */
static ad1843_ADLEN: ad1843_bitfield = ad1843_bitfield { reg: 27, lo_bit: 0, nbits: 1 }; /* ADC Left Channel Enable */
static ad1843_ADREN: ad1843_bitfield = ad1843_bitfield { reg: 27, lo_bit: 1, nbits: 1 }; /* ADC Right Channel Enable */
static ad1843_AAMEN: ad1843_bitfield = ad1843_bitfield { reg: 27, lo_bit: 4, nbits: 1 }; /* Analog to Analog Mix Enable */
static ad1843_ANAEN: ad1843_bitfield = ad1843_bitfield { reg: 27, lo_bit: 7, nbits: 1 }; /* Analog Channel Enable */
static ad1843_DA1EN: ad1843_bitfield = ad1843_bitfield { reg: 27, lo_bit: 8, nbits: 1 }; /* DAC1 Enable */
static ad1843_DA2EN: ad1843_bitfield = ad1843_bitfield { reg: 27, lo_bit: 9, nbits: 1 }; /* DAC2 Enable */
static ad1843_DDMEN: ad1843_bitfield = ad1843_bitfield { reg: 27, lo_bit: 12, nbits: 1 }; /* DAC2 to DAC1 Mix  Enable */
static ad1843_C1EN: ad1843_bitfield = ad1843_bitfield { reg: 28, lo_bit: 11, nbits: 1 }; /* Clock Generator 1 Enable */
static ad1843_C2EN: ad1843_bitfield = ad1843_bitfield { reg: 28, lo_bit: 12, nbits: 1 }; /* Clock Generator 2 Enable */
static ad1843_C3EN: ad1843_bitfield = ad1843_bitfield { reg: 28, lo_bit: 13, nbits: 1 }; /* Clock Generator 3 Enable */
static ad1843_PDNI: ad1843_bitfield = ad1843_bitfield { reg: 28, lo_bit: 15, nbits: 1 }; /* Converter Power Down */

/*
 * The various registers of the AD1843 use three different formats for
 * specifying gain.  The ad1843_gain structure parameterizes the
 * formats.
 */

#[repr(C)]
struct ad1843_gain {
    negative: c_int, /* nonzero if gain is negative. */
    lfield: *const ad1843_bitfield,
    rfield: *const ad1843_bitfield,
    lmute: *const ad1843_bitfield,
    rmute: *const ad1843_bitfield,
}

unsafe impl Sync for ad1843_gain {}

static ad1843_gain_RECLEV: ad1843_gain = ad1843_gain {
    negative: 0,
    lfield: &ad1843_LIG,
    rfield: &ad1843_RIG,
    lmute: core::ptr::null(),
    rmute: core::ptr::null(),
};
static ad1843_gain_LINE: ad1843_gain = ad1843_gain {
    negative: 1,
    lfield: &ad1843_LX1M,
    rfield: &ad1843_RX1M,
    lmute: &ad1843_LX1MM,
    rmute: &ad1843_RX1MM,
};
static ad1843_gain_LINE_2: ad1843_gain = ad1843_gain {
    negative: 1,
    lfield: &ad1843_LDA2G,
    rfield: &ad1843_RDA2G,
    lmute: &ad1843_LDA2GM,
    rmute: &ad1843_RDA2GM,
};
static ad1843_gain_MIC: ad1843_gain = ad1843_gain {
    negative: 1,
    lfield: &ad1843_LMCM,
    rfield: &ad1843_RMCM,
    lmute: &ad1843_LMCMM,
    rmute: &ad1843_RMCMM,
};
static ad1843_gain_PCM_0: ad1843_gain = ad1843_gain {
    negative: 1,
    lfield: &ad1843_LDA1G,
    rfield: &ad1843_RDA1G,
    lmute: &ad1843_LDA1GM,
    rmute: &ad1843_RDA1GM,
};
static ad1843_gain_PCM_1: ad1843_gain = ad1843_gain {
    negative: 1,
    lfield: &ad1843_LD2M,
    rfield: &ad1843_RD2M,
    lmute: &ad1843_LD2MM,
    rmute: &ad1843_RD2MM,
};

static ad1843_gain: [*const ad1843_gain; 6] = [
    &ad1843_gain_RECLEV,
    &ad1843_gain_LINE,
    &ad1843_gain_LINE_2,
    &ad1843_gain_MIC,
    &ad1843_gain_PCM_0,
    &ad1843_gain_PCM_1,
];

unsafe impl Sync for ad1843_bitfield {}
unsafe impl Sync for [*const ad1843_gain; 6] {}

/* read the current value of an AD1843 bitfield. */

unsafe fn ad1843_read_bits(ad1843: *mut snd_ad1843, field: *const ad1843_bitfield) -> c_int {
    let w: c_int;

    w = ((*ad1843).read.unwrap())((*ad1843).chip, (*field).reg as c_int);
    (w >> ((*field).lo_bit as c_int)) & ((1 << ((*field).nbits as c_int)) - 1)
}

/*
 * write a new value to an AD1843 bitfield and return the old value.
 */

unsafe fn ad1843_write_bits(
    ad1843: *mut snd_ad1843,
    field: *const ad1843_bitfield,
    newval: c_int,
) -> c_int {
    let mut w: c_int;
    let mask: c_int;
    let oldval: c_int;
    let newbits: c_int;

    w = ((*ad1843).read.unwrap())((*ad1843).chip, (*field).reg as c_int);
    mask = ((1 << ((*field).nbits as c_int)) - 1) << ((*field).lo_bit as c_int);
    oldval = (w & mask) >> ((*field).lo_bit as c_int);
    newbits = (newval << ((*field).lo_bit as c_int)) & mask;
    w = (w & !mask) | newbits;
    ((*ad1843).write.unwrap())((*ad1843).chip, (*field).reg as c_int, w);

    oldval
}

/*
 * ad1843_read_multi reads multiple bitfields from the same AD1843
 * register.  It uses a single read cycle to do it.  (Reading the
 * ad1843 requires 256 bit times at 12.288 MHz, or nearly 20
 * microseconds.)
 *
 * Called like this.
 *
 *  ad1843_read_multi(ad1843, nfields,
 *		      &ad1843_FIELD1, &val1,
 *		      &ad1843_FIELD2, &val2, ...);
 */

unsafe fn ad1843_read_multi(ad1843: *mut snd_ad1843, fields: &[(*const ad1843_bitfield, *mut c_int)]) {
    let mut w: c_int = 0;
    let mut reg: c_int = -1;

    for &(fp, value) in fields {
        if reg == -1 {
            reg = (*fp).reg as c_int;
            w = ((*ad1843).read.unwrap())((*ad1843).chip, reg);
        }

        let mask = (1 << ((*fp).nbits as c_int)) - 1;
        *value = (w >> ((*fp).lo_bit as c_int)) & mask;
    }
}

/*
 * ad1843_write_multi stores multiple bitfields into the same AD1843
 * register.  It uses one read and one write cycle to do it.
 *
 * Called like this.
 *
 *  ad1843_write_multi(ad1843, nfields,
 *		       &ad1843_FIELD1, val1,
 *		       &ad1843_FIELF2, val2, ...);
 */

unsafe fn ad1843_write_multi(ad1843: *mut snd_ad1843, fields: &[(*const ad1843_bitfield, c_int)]) {
    let mut reg: c_int;
    let mut w: c_int;
    let m: c_int;
    let mut mask: c_int;
    let mut bits: c_int;

    mask = 0;
    bits = 0;
    reg = -1;

    for &(fp, value) in fields {
        if reg == -1 {
            reg = (*fp).reg as c_int;
        } else {
            debug_assert!(reg == (*fp).reg as c_int);
        }
        m = ((1 << ((*fp).nbits as c_int)) - 1) << ((*fp).lo_bit as c_int);
        mask |= m;
        bits |= (value << ((*fp).lo_bit as c_int)) & m;
    }

    if (!mask & 0xFFFF) != 0 {
        w = ((*ad1843).read.unwrap())((*ad1843).chip, reg);
    } else {
        w = 0;
    }
    w = (w & !mask) | bits;
    ((*ad1843).write.unwrap())((*ad1843).chip, reg, w);
}

#[no_mangle]
pub unsafe extern "C" fn ad1843_get_gain_max(_ad1843: *mut snd_ad1843, id: c_int) -> c_int {
    let gp = ad1843_gain[id as usize];
    let mut ret: c_int;

    ret = 1 << ((*(*gp).lfield).nbits as c_int);
    if (*gp).lmute.is_null() {
        ret -= 1;
    }
    ret
}

/*
 * ad1843_get_gain reads the specified register and extracts the gain value
 * using the supplied gain type.
 */

#[no_mangle]
pub unsafe extern "C" fn ad1843_get_gain(ad1843: *mut snd_ad1843, id: c_int) -> c_int {
    let mut lg: c_int = 0;
    let mut rg: c_int = 0;
    let mut lm: c_int = 0;
    let mut rm: c_int = 0;
    let gp = ad1843_gain[id as usize];
    let mask: u16 = ((1 << ((*(*gp).lfield).nbits as c_int)) - 1) as u16;

    ad1843_read_multi(ad1843, &[((*gp).lfield, &mut lg), ((*gp).rfield, &mut rg)]);
    if (*gp).negative != 0 {
        lg = mask as c_int - lg;
        rg = mask as c_int - rg;
    }
    if !(*gp).lmute.is_null() {
        ad1843_read_multi(ad1843, &[((*gp).lmute, &mut lm), ((*gp).rmute, &mut rm)]);
        if lm != 0 {
            lg = 0;
        }
        if rm != 0 {
            rg = 0;
        }
    }
    (lg << 0) | (rg << 8)
}

/*
 * Set an audio channel's gain.
 *
 * Returns the new gain, which may be lower than the old gain.
 */

#[no_mangle]
pub unsafe extern "C" fn ad1843_set_gain(
    ad1843: *mut snd_ad1843,
    id: c_int,
    newval: c_int,
) -> c_int {
    let gp = ad1843_gain[id as usize];
    let mask: u16 = ((1 << ((*(*gp).lfield).nbits as c_int)) - 1) as u16;

    let mut lg: c_int = (newval >> 0) & mask as c_int;
    let mut rg: c_int = (newval >> 8) & mask as c_int;
    let lm: c_int = if lg == 0 { 1 } else { 0 };
    let rm: c_int = if rg == 0 { 1 } else { 0 };

    if (*gp).negative != 0 {
        lg = mask as c_int - lg;
        rg = mask as c_int - rg;
    }
    if !(*gp).lmute.is_null() {
        ad1843_write_multi(ad1843, &[((*gp).lmute, lm), ((*gp).rmute, rm)]);
    }
    ad1843_write_multi(ad1843, &[((*gp).lfield, lg), ((*gp).rfield, rg)]);
    ad1843_get_gain(ad1843, id)
}

/* Returns the current recording source */

#[no_mangle]
pub unsafe extern "C" fn ad1843_get_recsrc(ad1843: *mut snd_ad1843) -> c_int {
    let mut val = ad1843_read_bits(ad1843, &ad1843_LSS);

    if val < 0 || val > 2 {
        val = 2;
        ad1843_write_multi(ad1843, &[(&ad1843_LSS, val), (&ad1843_RSS, val)]);
    }
    val
}

/*
 * Set recording source.
 *
 * Returns newsrc on success, -errno on failure.
 */

#[no_mangle]
pub unsafe extern "C" fn ad1843_set_recsrc(ad1843: *mut snd_ad1843, newsrc: c_int) -> c_int {
    if newsrc < 0 || newsrc > 2 {
        return -EINVAL;
    }

    ad1843_write_multi(ad1843, &[(&ad1843_LSS, newsrc), (&ad1843_RSS, newsrc)]);
    newsrc
}

/* Setup ad1843 for D/A conversion. */

#[no_mangle]
pub unsafe extern "C" fn ad1843_setup_dac(
    ad1843: *mut snd_ad1843,
    id: c_uint,
    framerate: c_uint,
    fmt: snd_pcm_format_t,
    channels: c_uint,
) {
    let mut ad_fmt: c_int = 0;
    let mut ad_mode: c_int = 0;

    if fmt == SNDRV_PCM_FORMAT_S8 {
        ad_fmt = 0;
    } else if fmt == SNDRV_PCM_FORMAT_U8 {
        ad_fmt = 0;
    } else if fmt == SNDRV_PCM_FORMAT_S16_LE {
        ad_fmt = 1;
    } else if fmt == SNDRV_PCM_FORMAT_MU_LAW {
        ad_fmt = 2;
    } else if fmt == SNDRV_PCM_FORMAT_A_LAW {
        ad_fmt = 3;
    }

    match channels {
        2 => ad_mode = 0,
        1 => ad_mode = 1,
        _ => {}
    }

    if id != 0 {
        ad1843_write_bits(ad1843, &ad1843_C2C, framerate as c_int);
        ad1843_write_multi(ad1843, &[(&ad1843_DA2SM, ad_mode), (&ad1843_DA2F, ad_fmt)]);
    } else {
        ad1843_write_bits(ad1843, &ad1843_C1C, framerate as c_int);
        ad1843_write_multi(ad1843, &[(&ad1843_DA1SM, ad_mode), (&ad1843_DA1F, ad_fmt)]);
    }
}

#[no_mangle]
pub unsafe extern "C" fn ad1843_shutdown_dac(ad1843: *mut snd_ad1843, id: c_uint) {
    if id != 0 {
        ad1843_write_bits(ad1843, &ad1843_DA2F, 1);
    } else {
        ad1843_write_bits(ad1843, &ad1843_DA1F, 1);
    }
}

#[no_mangle]
pub unsafe extern "C" fn ad1843_setup_adc(
    ad1843: *mut snd_ad1843,
    framerate: c_uint,
    fmt: snd_pcm_format_t,
    _channels: c_uint,
) {
    let mut da_fmt: c_int = 0;

    if fmt == SNDRV_PCM_FORMAT_S8 {
        da_fmt = 0;
    } else if fmt == SNDRV_PCM_FORMAT_U8 {
        da_fmt = 0;
    } else if fmt == SNDRV_PCM_FORMAT_S16_LE {
        da_fmt = 1;
    } else if fmt == SNDRV_PCM_FORMAT_MU_LAW {
        da_fmt = 2;
    } else if fmt == SNDRV_PCM_FORMAT_A_LAW {
        da_fmt = 3;
    }

    ad1843_write_bits(ad1843, &ad1843_C3C, framerate as c_int);
    ad1843_write_multi(ad1843, &[(&ad1843_ADLF, da_fmt), (&ad1843_ADRF, da_fmt)]);
}

#[no_mangle]
pub unsafe extern "C" fn ad1843_shutdown_adc(_ad1843: *mut snd_ad1843) {
    /* nothing to do */
}

/*
 * Fully initialize the ad1843.  As described in the AD1843 data
 * sheet, section "START-UP SEQUENCE".  The numbered comments are
 * subsection headings from the data sheet.  See the data sheet, pages
 * 52-54, for more info.
 *
 * return 0 on success, -errno on failure.  */

#[no_mangle]
pub unsafe extern "C" fn ad1843_init(ad1843: *mut snd_ad1843) -> c_int {
    let later: c_ulong;

    if ad1843_read_bits(ad1843, &ad1843_INIT) != 0 {
        printk(
            concat!("\x013", "ad1843: AD1843 won't initialize\n\0").as_ptr() as *const c_char,
        );
        return -EIO;
    }

    ad1843_write_bits(ad1843, &ad1843_SCF, 1);

    /* 4. Put the conversion resources into standby. */
    ad1843_write_bits(ad1843, &ad1843_PDNI, 0);
    later = jiffies.wrapping_add(msecs_to_jiffies(500));

    while ad1843_read_bits(ad1843, &ad1843_PDNO) != 0 {
        if time_after(jiffies, later) {
            printk(concat!("\x013", "ad1843: AD1843 won't power up\n\0").as_ptr() as *const c_char);
            return -EIO;
        }
        schedule_timeout_interruptible(5);
    }

    /* 5. Power up the clock generators and enable clock output pins. */
    ad1843_write_multi(
        ad1843,
        &[(&ad1843_C1EN, 1), (&ad1843_C2EN, 1), (&ad1843_C3EN, 1)],
    );

    /* 6. Configure conversion resources while they are in standby. */

    /* DAC1/2 use clock 1/2 as source, ADC uses clock 3.  Always. */
    ad1843_write_multi(
        ad1843,
        &[
            (&ad1843_DA1C, 1),
            (&ad1843_DA2C, 2),
            (&ad1843_ADLC, 3),
            (&ad1843_ADRC, 3),
        ],
    );

    /* 7. Enable conversion resources. */
    ad1843_write_bits(ad1843, &ad1843_ADTLK, 1);
    ad1843_write_multi(
        ad1843,
        &[
            (&ad1843_ANAEN, 1),
            (&ad1843_AAMEN, 1),
            (&ad1843_DA1EN, 1),
            (&ad1843_DA2EN, 1),
            (&ad1843_DDMEN, 1),
            (&ad1843_ADLEN, 1),
            (&ad1843_ADREN, 1),
        ],
    );

    /* 8. Configure conversion resources while they are enabled. */

    /* set gain to 0 for all channels */
    ad1843_set_gain(ad1843, AD1843_GAIN_RECLEV, 0);
    ad1843_set_gain(ad1843, AD1843_GAIN_LINE, 0);
    ad1843_set_gain(ad1843, AD1843_GAIN_LINE_2, 0);
    ad1843_set_gain(ad1843, AD1843_GAIN_MIC, 0);
    ad1843_set_gain(ad1843, AD1843_GAIN_PCM_0, 0);
    ad1843_set_gain(ad1843, AD1843_GAIN_PCM_1, 0);

    /* Unmute all channels. */
    /* DAC1 */
    ad1843_write_multi(ad1843, &[(&ad1843_LDA1GM, 0), (&ad1843_RDA1GM, 0)]);
    /* DAC2 */
    ad1843_write_multi(ad1843, &[(&ad1843_LDA2GM, 0), (&ad1843_RDA2GM, 0)]);

    /* Set default recording source to Line In and set
     * mic gain to +20 dB.
     */
    ad1843_set_recsrc(ad1843, 2);
    ad1843_write_multi(ad1843, &[(&ad1843_LMGE, 1), (&ad1843_RMGE, 1)]);

    /* Set Speaker Out level to +/- 4V and unmute it. */
    ad1843_write_multi(
        ad1843,
        &[(&ad1843_HPOS, 1), (&ad1843_HPOM, 0), (&ad1843_MPOM, 0)],
    );

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
