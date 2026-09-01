// SPDX-License-Identifier: LGPL-2.0+
/*
 *  PCM Interface - misc routines
 *  Copyright (c) 1998 by Jaroslav Kysela <perex@perex.cz>
 */

// Dependencies originally supplied by:
// <linux/time.h>, <linux/export.h>, <sound/core.h>, <sound/pcm.h>, "pcm_local.h"

pub type snd_pcm_format_t = i32;
pub type size_t = usize;
pub type ssize_t = isize;

const EINVAL: i32 = 22;
const UINT_MAX: u32 = u32::MAX;

const SND_PCM_FORMAT_UNKNOWN: snd_pcm_format_t = -1;

const SNDRV_PCM_FORMAT_S8: usize = 0;
const SNDRV_PCM_FORMAT_U8: usize = 1;
const SNDRV_PCM_FORMAT_S16_LE: usize = 2;
const SNDRV_PCM_FORMAT_S16_BE: usize = 3;
const SNDRV_PCM_FORMAT_U16_LE: usize = 4;
const SNDRV_PCM_FORMAT_U16_BE: usize = 5;
const SNDRV_PCM_FORMAT_S24_LE: usize = 6;
const SNDRV_PCM_FORMAT_S24_BE: usize = 7;
const SNDRV_PCM_FORMAT_U24_LE: usize = 8;
const SNDRV_PCM_FORMAT_U24_BE: usize = 9;
const SNDRV_PCM_FORMAT_S32_LE: usize = 10;
const SNDRV_PCM_FORMAT_S32_BE: usize = 11;
const SNDRV_PCM_FORMAT_U32_LE: usize = 12;
const SNDRV_PCM_FORMAT_U32_BE: usize = 13;
const SNDRV_PCM_FORMAT_FLOAT_LE: usize = 14;
const SNDRV_PCM_FORMAT_FLOAT_BE: usize = 15;
const SNDRV_PCM_FORMAT_FLOAT64_LE: usize = 16;
const SNDRV_PCM_FORMAT_FLOAT64_BE: usize = 17;
const SNDRV_PCM_FORMAT_IEC958_SUBFRAME_LE: usize = 18;
const SNDRV_PCM_FORMAT_IEC958_SUBFRAME_BE: usize = 19;
const SNDRV_PCM_FORMAT_MU_LAW: usize = 20;
const SNDRV_PCM_FORMAT_A_LAW: usize = 21;
const SNDRV_PCM_FORMAT_IMA_ADPCM: usize = 22;
const SNDRV_PCM_FORMAT_MPEG: usize = 23;
const SNDRV_PCM_FORMAT_GSM: usize = 24;
const SNDRV_PCM_FORMAT_S20_LE: usize = 25;
const SNDRV_PCM_FORMAT_S20_BE: usize = 26;
const SNDRV_PCM_FORMAT_U20_LE: usize = 27;
const SNDRV_PCM_FORMAT_U20_BE: usize = 28;
const SNDRV_PCM_FORMAT_SPECIAL: usize = 31;
const SNDRV_PCM_FORMAT_S24_3LE: usize = 32;
const SNDRV_PCM_FORMAT_S24_3BE: usize = 33;
const SNDRV_PCM_FORMAT_U24_3LE: usize = 34;
const SNDRV_PCM_FORMAT_U24_3BE: usize = 35;
const SNDRV_PCM_FORMAT_S20_3LE: usize = 36;
const SNDRV_PCM_FORMAT_S20_3BE: usize = 37;
const SNDRV_PCM_FORMAT_U20_3LE: usize = 38;
const SNDRV_PCM_FORMAT_U20_3BE: usize = 39;
const SNDRV_PCM_FORMAT_S18_3LE: usize = 40;
const SNDRV_PCM_FORMAT_S18_3BE: usize = 41;
const SNDRV_PCM_FORMAT_U18_3LE: usize = 42;
const SNDRV_PCM_FORMAT_U18_3BE: usize = 43;
const SNDRV_PCM_FORMAT_G723_24: usize = 44;
const SNDRV_PCM_FORMAT_G723_24_1B: usize = 45;
const SNDRV_PCM_FORMAT_G723_40: usize = 46;
const SNDRV_PCM_FORMAT_G723_40_1B: usize = 47;
const SNDRV_PCM_FORMAT_DSD_U8: usize = 48;
const SNDRV_PCM_FORMAT_DSD_U16_LE: usize = 49;
const SNDRV_PCM_FORMAT_DSD_U32_LE: usize = 50;
const SNDRV_PCM_FORMAT_DSD_U16_BE: usize = 51;
const SNDRV_PCM_FORMAT_DSD_U32_BE: usize = 52;
const SNDRV_PCM_FORMAT_LAST: usize = SNDRV_PCM_FORMAT_DSD_U32_BE;

const SNDRV_PCM_RATE_CONTINUOUS: u32 = 1 << 30;
const SNDRV_PCM_RATE_KNOT: u32 = 1 << 31;

#[repr(C)]
pub struct snd_pcm_hardware {
    pub rates: u32,
    pub rate_min: u32,
    pub rate_max: u32,
}

#[repr(C)]
pub struct snd_pcm_known_rates_t {
    pub count: u32,
    pub list: *const u32,
}

unsafe extern "C" {
    static snd_pcm_known_rates: snd_pcm_known_rates_t;
    fn memset(s: *mut core::ffi::c_void, c: i32, n: size_t) -> *mut core::ffi::c_void;
    fn memcpy(
        dest: *mut core::ffi::c_void,
        src: *const core::ffi::c_void,
        n: size_t,
    ) -> *mut core::ffi::c_void;
}

/* NOTE: "signed" prefix must be given below since the default char is
 *       unsigned on some architectures!
 */
#[repr(C)]
#[derive(Copy, Clone)]
struct pcm_format_data {
    width: u8,          /* bit width */
    phys: u8,           /* physical bit width */
    le: i8,             /* 0 = big-endian, 1 = little-endian, -1 = others */
    signd: i8,          /* 0 = unsigned, 1 = signed, -1 = others */
    silence: [u8; 8],   /* silence data to fill */
}

const PCM_FORMAT_ZERO: pcm_format_data = pcm_format_data {
    width: 0,
    phys: 0,
    le: 0,
    signd: 0,
    silence: [0; 8],
};

const fn pcm_format(width: u8, phys: u8, le: i8, signd: i8, silence: [u8; 8]) -> pcm_format_data {
    pcm_format_data {
        width,
        phys,
        le,
        signd,
        silence,
    }
}

fn valid_format(format: snd_pcm_format_t) -> bool {
    format >= 0 && format <= SNDRV_PCM_FORMAT_LAST as snd_pcm_format_t
}

static pcm_formats: [pcm_format_data; SNDRV_PCM_FORMAT_LAST + 1] = {
    let mut formats = [PCM_FORMAT_ZERO; SNDRV_PCM_FORMAT_LAST + 1];
    formats[SNDRV_PCM_FORMAT_S8] = pcm_format(8, 8, -1, 1, [0; 8]);
    formats[SNDRV_PCM_FORMAT_U8] = pcm_format(8, 8, -1, 0, [0x80, 0, 0, 0, 0, 0, 0, 0]);
    formats[SNDRV_PCM_FORMAT_S16_LE] = pcm_format(16, 16, 1, 1, [0; 8]);
    formats[SNDRV_PCM_FORMAT_S16_BE] = pcm_format(16, 16, 0, 1, [0; 8]);
    formats[SNDRV_PCM_FORMAT_U16_LE] = pcm_format(16, 16, 1, 0, [0x00, 0x80, 0, 0, 0, 0, 0, 0]);
    formats[SNDRV_PCM_FORMAT_U16_BE] = pcm_format(16, 16, 0, 0, [0x80, 0x00, 0, 0, 0, 0, 0, 0]);
    formats[SNDRV_PCM_FORMAT_S24_LE] = pcm_format(24, 32, 1, 1, [0; 8]);
    formats[SNDRV_PCM_FORMAT_S24_BE] = pcm_format(24, 32, 0, 1, [0; 8]);
    formats[SNDRV_PCM_FORMAT_U24_LE] = pcm_format(24, 32, 1, 0, [0x00, 0x00, 0x80, 0, 0, 0, 0, 0]);
    formats[SNDRV_PCM_FORMAT_U24_BE] = pcm_format(24, 32, 0, 0, [0x00, 0x80, 0x00, 0x00, 0, 0, 0, 0]);
    formats[SNDRV_PCM_FORMAT_S32_LE] = pcm_format(32, 32, 1, 1, [0; 8]);
    formats[SNDRV_PCM_FORMAT_S32_BE] = pcm_format(32, 32, 0, 1, [0; 8]);
    formats[SNDRV_PCM_FORMAT_U32_LE] = pcm_format(32, 32, 1, 0, [0x00, 0x00, 0x00, 0x80, 0, 0, 0, 0]);
    formats[SNDRV_PCM_FORMAT_U32_BE] = pcm_format(32, 32, 0, 0, [0x80, 0x00, 0x00, 0x00, 0, 0, 0, 0]);
    formats[SNDRV_PCM_FORMAT_FLOAT_LE] = pcm_format(32, 32, 1, -1, [0; 8]);
    formats[SNDRV_PCM_FORMAT_FLOAT_BE] = pcm_format(32, 32, 0, -1, [0; 8]);
    formats[SNDRV_PCM_FORMAT_FLOAT64_LE] = pcm_format(64, 64, 1, -1, [0; 8]);
    formats[SNDRV_PCM_FORMAT_FLOAT64_BE] = pcm_format(64, 64, 0, -1, [0; 8]);
    formats[SNDRV_PCM_FORMAT_IEC958_SUBFRAME_LE] = pcm_format(32, 32, 1, -1, [0; 8]);
    formats[SNDRV_PCM_FORMAT_IEC958_SUBFRAME_BE] = pcm_format(32, 32, 0, -1, [0; 8]);
    formats[SNDRV_PCM_FORMAT_MU_LAW] = pcm_format(8, 8, -1, -1, [0x7f, 0, 0, 0, 0, 0, 0, 0]);
    formats[SNDRV_PCM_FORMAT_A_LAW] = pcm_format(8, 8, -1, -1, [0x55, 0, 0, 0, 0, 0, 0, 0]);
    formats[SNDRV_PCM_FORMAT_IMA_ADPCM] = pcm_format(4, 4, -1, -1, [0; 8]);
    formats[SNDRV_PCM_FORMAT_G723_24] = pcm_format(3, 3, -1, -1, [0; 8]);
    formats[SNDRV_PCM_FORMAT_G723_40] = pcm_format(5, 5, -1, -1, [0; 8]);
    formats[SNDRV_PCM_FORMAT_DSD_U8] = pcm_format(8, 8, 1, 0, [0x69, 0, 0, 0, 0, 0, 0, 0]);
    formats[SNDRV_PCM_FORMAT_DSD_U16_LE] = pcm_format(16, 16, 1, 0, [0x69, 0x69, 0, 0, 0, 0, 0, 0]);
    formats[SNDRV_PCM_FORMAT_DSD_U32_LE] = pcm_format(32, 32, 1, 0, [0x69, 0x69, 0x69, 0x69, 0, 0, 0, 0]);
    formats[SNDRV_PCM_FORMAT_DSD_U16_BE] = pcm_format(16, 16, 0, 0, [0x69, 0x69, 0, 0, 0, 0, 0, 0]);
    formats[SNDRV_PCM_FORMAT_DSD_U32_BE] = pcm_format(32, 32, 0, 0, [0x69, 0x69, 0x69, 0x69, 0, 0, 0, 0]);
    /* FIXME: the following two formats are not defined properly yet */
    formats[SNDRV_PCM_FORMAT_MPEG] = pcm_format(0, 0, -1, -1, [0; 8]);
    formats[SNDRV_PCM_FORMAT_GSM] = pcm_format(0, 0, -1, -1, [0; 8]);
    formats[SNDRV_PCM_FORMAT_S20_LE] = pcm_format(20, 32, 1, 1, [0; 8]);
    formats[SNDRV_PCM_FORMAT_S20_BE] = pcm_format(20, 32, 0, 1, [0; 8]);
    formats[SNDRV_PCM_FORMAT_U20_LE] = pcm_format(20, 32, 1, 0, [0x00, 0x00, 0x08, 0x00, 0, 0, 0, 0]);
    formats[SNDRV_PCM_FORMAT_U20_BE] = pcm_format(20, 32, 0, 0, [0x00, 0x08, 0x00, 0x00, 0, 0, 0, 0]);
    /* FIXME: the following format is not defined properly yet */
    formats[SNDRV_PCM_FORMAT_SPECIAL] = pcm_format(0, 0, -1, -1, [0; 8]);
    formats[SNDRV_PCM_FORMAT_S24_3LE] = pcm_format(24, 24, 1, 1, [0; 8]);
    formats[SNDRV_PCM_FORMAT_S24_3BE] = pcm_format(24, 24, 0, 1, [0; 8]);
    formats[SNDRV_PCM_FORMAT_U24_3LE] = pcm_format(24, 24, 1, 0, [0x00, 0x00, 0x80, 0, 0, 0, 0, 0]);
    formats[SNDRV_PCM_FORMAT_U24_3BE] = pcm_format(24, 24, 0, 0, [0x80, 0x00, 0x00, 0, 0, 0, 0, 0]);
    formats[SNDRV_PCM_FORMAT_S20_3LE] = pcm_format(20, 24, 1, 1, [0; 8]);
    formats[SNDRV_PCM_FORMAT_S20_3BE] = pcm_format(20, 24, 0, 1, [0; 8]);
    formats[SNDRV_PCM_FORMAT_U20_3LE] = pcm_format(20, 24, 1, 0, [0x00, 0x00, 0x08, 0, 0, 0, 0, 0]);
    formats[SNDRV_PCM_FORMAT_U20_3BE] = pcm_format(20, 24, 0, 0, [0x08, 0x00, 0x00, 0, 0, 0, 0, 0]);
    formats[SNDRV_PCM_FORMAT_S18_3LE] = pcm_format(18, 24, 1, 1, [0; 8]);
    formats[SNDRV_PCM_FORMAT_S18_3BE] = pcm_format(18, 24, 0, 1, [0; 8]);
    formats[SNDRV_PCM_FORMAT_U18_3LE] = pcm_format(18, 24, 1, 0, [0x00, 0x00, 0x02, 0, 0, 0, 0, 0]);
    formats[SNDRV_PCM_FORMAT_U18_3BE] = pcm_format(18, 24, 0, 0, [0x02, 0x00, 0x00, 0, 0, 0, 0, 0]);
    formats[SNDRV_PCM_FORMAT_G723_24_1B] = pcm_format(3, 8, -1, -1, [0; 8]);
    formats[SNDRV_PCM_FORMAT_G723_40_1B] = pcm_format(5, 8, -1, -1, [0; 8]);
    formats
};

/**
 * snd_pcm_format_signed - Check the PCM format is signed linear
 * @format: the format to check
 *
 * Return: 1 if the given PCM format is signed linear, 0 if unsigned
 * linear, and a negative error code for non-linear formats.
 */
#[unsafe(no_mangle)]
pub extern "C" fn snd_pcm_format_signed(format: snd_pcm_format_t) -> i32 {
    let val: i32;
    if !valid_format(format) {
        return -EINVAL;
    }
    val = pcm_formats[format as usize].signd as i32;
    if val < 0 {
        return -EINVAL;
    }
    val
}
// EXPORT_SYMBOL(snd_pcm_format_signed);

/**
 * snd_pcm_format_unsigned - Check the PCM format is unsigned linear
 * @format: the format to check
 *
 * Return: 1 if the given PCM format is unsigned linear, 0 if signed
 * linear, and a negative error code for non-linear formats.
 */
#[unsafe(no_mangle)]
pub extern "C" fn snd_pcm_format_unsigned(format: snd_pcm_format_t) -> i32 {
    let val: i32;

    val = snd_pcm_format_signed(format);
    if val < 0 {
        return val;
    }
    (val == 0) as i32
}
// EXPORT_SYMBOL(snd_pcm_format_unsigned);

/**
 * snd_pcm_format_linear - Check the PCM format is linear
 * @format: the format to check
 *
 * Return: 1 if the given PCM format is linear, 0 if not.
 */
#[unsafe(no_mangle)]
pub extern "C" fn snd_pcm_format_linear(format: snd_pcm_format_t) -> i32 {
    (snd_pcm_format_signed(format) >= 0) as i32
}
// EXPORT_SYMBOL(snd_pcm_format_linear);

/**
 * snd_pcm_format_little_endian - Check the PCM format is little-endian
 * @format: the format to check
 *
 * Return: 1 if the given PCM format is little-endian, 0 if
 * big-endian, or a negative error code if endian not specified.
 */
#[unsafe(no_mangle)]
pub extern "C" fn snd_pcm_format_little_endian(format: snd_pcm_format_t) -> i32 {
    let val: i32;
    if !valid_format(format) {
        return -EINVAL;
    }
    val = pcm_formats[format as usize].le as i32;
    if val < 0 {
        return -EINVAL;
    }
    val
}
// EXPORT_SYMBOL(snd_pcm_format_little_endian);

/**
 * snd_pcm_format_big_endian - Check the PCM format is big-endian
 * @format: the format to check
 *
 * Return: 1 if the given PCM format is big-endian, 0 if
 * little-endian, or a negative error code if endian not specified.
 */
#[unsafe(no_mangle)]
pub extern "C" fn snd_pcm_format_big_endian(format: snd_pcm_format_t) -> i32 {
    let val: i32;

    val = snd_pcm_format_little_endian(format);
    if val < 0 {
        return val;
    }
    (val == 0) as i32
}
// EXPORT_SYMBOL(snd_pcm_format_big_endian);

/**
 * snd_pcm_format_width - return the bit-width of the format
 * @format: the format to check
 *
 * Return: The bit-width of the format, or a negative error code
 * if unknown format.
 */
#[unsafe(no_mangle)]
pub extern "C" fn snd_pcm_format_width(format: snd_pcm_format_t) -> i32 {
    let val: i32;
    if !valid_format(format) {
        return -EINVAL;
    }
    val = pcm_formats[format as usize].width as i32;
    if val == 0 {
        return -EINVAL;
    }
    val
}
// EXPORT_SYMBOL(snd_pcm_format_width);

/**
 * snd_pcm_format_physical_width - return the physical bit-width of the format
 * @format: the format to check
 *
 * Return: The physical bit-width of the format, or a negative error code
 * if unknown format.
 */
#[unsafe(no_mangle)]
pub extern "C" fn snd_pcm_format_physical_width(format: snd_pcm_format_t) -> i32 {
    let val: i32;
    if !valid_format(format) {
        return -EINVAL;
    }
    val = pcm_formats[format as usize].phys as i32;
    if val == 0 {
        return -EINVAL;
    }
    val
}
// EXPORT_SYMBOL(snd_pcm_format_physical_width);

/**
 * snd_pcm_format_size - return the byte size of samples on the given format
 * @format: the format to check
 * @samples: sampling rate
 *
 * Return: The byte size of the given samples for the format, or a
 * negative error code if unknown format.
 */
#[unsafe(no_mangle)]
pub extern "C" fn snd_pcm_format_size(format: snd_pcm_format_t, samples: size_t) -> ssize_t {
    let phys_width = snd_pcm_format_physical_width(format);
    if phys_width < 0 {
        return -EINVAL as ssize_t;
    }
    (samples * phys_width as usize / 8) as ssize_t
}
// EXPORT_SYMBOL(snd_pcm_format_size);

/**
 * snd_pcm_format_silence_64 - return the silent data in 8 bytes array
 * @format: the format to check
 *
 * Return: The format pattern to fill or %NULL if error.
 */
#[unsafe(no_mangle)]
pub extern "C" fn snd_pcm_format_silence_64(format: snd_pcm_format_t) -> *const u8 {
    if !valid_format(format) {
        return core::ptr::null();
    }
    if pcm_formats[format as usize].phys == 0 {
        return core::ptr::null();
    }
    pcm_formats[format as usize].silence.as_ptr()
}
// EXPORT_SYMBOL(snd_pcm_format_silence_64);

/**
 * snd_pcm_format_set_silence - set the silence data on the buffer
 * @format: the PCM format
 * @data: the buffer pointer
 * @samples: the number of samples to set silence
 *
 * Sets the silence data on the buffer for the given samples.
 *
 * Return: Zero if successful, or a negative error code on failure.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_pcm_format_set_silence(
    format: snd_pcm_format_t,
    data: *mut core::ffi::c_void,
    mut samples: u32,
) -> i32 {
    let mut width: i32;
    let mut dst: *mut u8;
    let pat: *const u8;

    if !valid_format(format) {
        return -EINVAL;
    }
    if samples == 0 {
        return 0;
    }
    width = pcm_formats[format as usize].phys as i32; /* physical width */
    if width == 0 {
        return -EINVAL;
    }
    pat = pcm_formats[format as usize].silence.as_ptr();
    /* signed or 1 byte data */
    if pcm_formats[format as usize].signd == 1 || width <= 8 {
        let bytes: u32 = samples * width as u32 / 8;
        unsafe {
            memset(data, *pat as i32, bytes as size_t);
        }
        return 0;
    }
    /* non-zero samples, fill using a loop */
    width /= 8;
    dst = data as *mut u8;
    /*
     * Original C has an #if 0 generic memcpy loop here.  The active branch is
     * the optimized constant-width switch below.
     */
    /* a bit optimization for constant width */
    match width {
        2 => {
            while samples != 0 {
                samples -= 1;
                unsafe {
                    memcpy(dst as *mut core::ffi::c_void, pat as *const core::ffi::c_void, 2);
                    dst = dst.add(2);
                }
            }
        }
        3 => {
            while samples != 0 {
                samples -= 1;
                unsafe {
                    memcpy(dst as *mut core::ffi::c_void, pat as *const core::ffi::c_void, 3);
                    dst = dst.add(3);
                }
            }
        }
        4 => {
            while samples != 0 {
                samples -= 1;
                unsafe {
                    memcpy(dst as *mut core::ffi::c_void, pat as *const core::ffi::c_void, 4);
                    dst = dst.add(4);
                }
            }
        }
        8 => {
            while samples != 0 {
                samples -= 1;
                unsafe {
                    memcpy(dst as *mut core::ffi::c_void, pat as *const core::ffi::c_void, 8);
                    dst = dst.add(8);
                }
            }
        }
        _ => {}
    }
    0
}
// EXPORT_SYMBOL(snd_pcm_format_set_silence);

/**
 * snd_pcm_hw_limit_rates - determine rate_min/rate_max fields
 * @hw: the pcm hw instance
 *
 * Determines the rate_min and rate_max fields from the rates bits of
 * the given hw.
 *
 * Return: Zero if successful.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_pcm_hw_limit_rates(hw: *mut snd_pcm_hardware) -> i32 {
    let mut i: i32;
    let mut rmin: u32;
    let mut rmax: u32;

    rmin = UINT_MAX;
    rmax = 0;
    i = 0;
    while i < unsafe { snd_pcm_known_rates.count as i32 } {
        if unsafe { (*hw).rates } & (1u32 << i) != 0 {
            let rate = unsafe { *snd_pcm_known_rates.list.add(i as usize) };
            rmin = core::cmp::min(rmin, rate);
            rmax = core::cmp::max(rmax, rate);
        }
        i += 1;
    }
    if rmin > rmax {
        return -EINVAL;
    }
    unsafe {
        (*hw).rate_min = rmin;
        (*hw).rate_max = rmax;
    }
    0
}
// EXPORT_SYMBOL(snd_pcm_hw_limit_rates);

/**
 * snd_pcm_rate_to_rate_bit - converts sample rate to SNDRV_PCM_RATE_xxx bit
 * @rate: the sample rate to convert
 *
 * Return: The SNDRV_PCM_RATE_xxx flag that corresponds to the given rate, or
 * SNDRV_PCM_RATE_KNOT for an unknown rate.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_pcm_rate_to_rate_bit(rate: u32) -> u32 {
    let mut i: u32;

    i = 0;
    while i < unsafe { snd_pcm_known_rates.count } {
        if unsafe { *snd_pcm_known_rates.list.add(i as usize) } == rate {
            return 1u32 << i;
        }
        i += 1;
    }
    SNDRV_PCM_RATE_KNOT
}
// EXPORT_SYMBOL(snd_pcm_rate_to_rate_bit);

/**
 * snd_pcm_rate_bit_to_rate - converts SNDRV_PCM_RATE_xxx bit to sample rate
 * @rate_bit: the rate bit to convert
 *
 * Return: The sample rate that corresponds to the given SNDRV_PCM_RATE_xxx flag
 * or 0 for an unknown rate bit.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_pcm_rate_bit_to_rate(rate_bit: u32) -> u32 {
    let mut i: u32;

    i = 0;
    while i < unsafe { snd_pcm_known_rates.count } {
        if (1u32 << i) == rate_bit {
            return unsafe { *snd_pcm_known_rates.list.add(i as usize) };
        }
        i += 1;
    }
    0
}
// EXPORT_SYMBOL(snd_pcm_rate_bit_to_rate);

fn snd_pcm_rate_mask_sanitize(rates: u32) -> u32 {
    if rates & SNDRV_PCM_RATE_CONTINUOUS != 0 {
        SNDRV_PCM_RATE_CONTINUOUS
    } else if rates & SNDRV_PCM_RATE_KNOT != 0 {
        SNDRV_PCM_RATE_KNOT
    } else {
        rates
    }
}

/**
 * snd_pcm_rate_mask_intersect - computes the intersection between two rate masks
 * @rates_a: The first rate mask
 * @rates_b: The second rate mask
 *
 * This function computes the rates that are supported by both rate masks passed
 * to the function. It will take care of the special handling of
 * SNDRV_PCM_RATE_CONTINUOUS and SNDRV_PCM_RATE_KNOT.
 *
 * Return: A rate mask containing the rates that are supported by both rates_a
 * and rates_b.
 */
#[unsafe(no_mangle)]
pub extern "C" fn snd_pcm_rate_mask_intersect(mut rates_a: u32, mut rates_b: u32) -> u32 {
    rates_a = snd_pcm_rate_mask_sanitize(rates_a);
    rates_b = snd_pcm_rate_mask_sanitize(rates_b);

    if rates_a & SNDRV_PCM_RATE_CONTINUOUS != 0 {
        rates_b
    } else if rates_b & SNDRV_PCM_RATE_CONTINUOUS != 0 {
        rates_a
    } else if rates_a & SNDRV_PCM_RATE_KNOT != 0 {
        rates_b
    } else if rates_b & SNDRV_PCM_RATE_KNOT != 0 {
        rates_a
    } else {
        rates_a & rates_b
    }
}
// EXPORT_SYMBOL_GPL(snd_pcm_rate_mask_intersect);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
