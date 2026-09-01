// SPDX-License-Identifier: LGPL-2.0+
/*
 *  Mu-Law conversion Plug-In Interface
 *  Copyright (c) 1999 by Jaroslav Kysela <perex@perex.cz>
 *                        Uros Bizjak <uros@kss-loka.si>
 *
 *  Based on reference implementation by Sun Microsystems, Inc.
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

type u16 = u16;
type snd_pcm_uframes_t = c_uint;
type snd_pcm_sframes_t = c_int;
type snd_pcm_format_t = c_int;

const SIGN_BIT: c_int = 0x80; /* Sign bit for a u-law byte. */
const QUANT_MASK: c_int = 0xf; /* Quantization field mask. */
const NSEGS: c_int = 8; /* Number of u-law segments. */
const SEG_SHIFT: c_int = 4; /* Left shift for segment number. */
const SEG_MASK: c_int = 0x70; /* Segment field mask. */

const BIAS: c_int = 0x84; /* Bias for linear code. */
const ENXIO: c_int = 6;
const EINVAL: c_int = 22;

extern "C" {
    static SNDRV_PCM_FORMAT_MU_LAW: snd_pcm_format_t;

    fn swab16(x: u16) -> u16;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn snd_BUG() -> c_int;
    fn snd_BUG_ON(condition: bool) -> c_int;
    fn snd_pcm_area_silence(
        dst_area: *mut snd_pcm_channel_area,
        dst_offset: snd_pcm_uframes_t,
        samples: snd_pcm_uframes_t,
        format: snd_pcm_format_t,
    );
    fn snd_pcm_format_big_endian(format: snd_pcm_format_t) -> c_int;
    fn snd_pcm_format_little_endian(format: snd_pcm_format_t) -> c_int;
    fn snd_pcm_format_signed(format: snd_pcm_format_t) -> c_int;
    fn snd_pcm_format_physical_width(format: snd_pcm_format_t) -> c_int;
    fn snd_pcm_format_width(format: snd_pcm_format_t) -> c_int;
    fn snd_pcm_format_linear(format: snd_pcm_format_t) -> c_int;
    fn snd_pcm_plugin_build(
        plug: *mut snd_pcm_substream,
        name: *const c_char,
        src_format: *mut snd_pcm_plugin_format,
        dst_format: *mut snd_pcm_plugin_format,
        extra: usize,
        r_plugin: *mut *mut snd_pcm_plugin,
    ) -> c_int;
}

#[repr(C)]
pub struct snd_pcm_substream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_channel_area {
    pub addr: *mut c_char,
    pub first: c_uint,
    pub step: c_uint,
}

#[repr(C)]
pub struct snd_pcm_plugin_channel {
    pub area: snd_pcm_channel_area,
    pub frames: snd_pcm_uframes_t,
    pub enabled: c_int,
    pub wanted: c_int,
}

#[repr(C)]
pub struct snd_pcm_plugin_format {
    pub format: snd_pcm_format_t,
    pub rate: c_uint,
    pub channels: c_uint,
}

type TransferFn = Option<
    unsafe extern "C" fn(
        plugin: *mut snd_pcm_plugin,
        src_channels: *const snd_pcm_plugin_channel,
        dst_channels: *mut snd_pcm_plugin_channel,
        frames: snd_pcm_uframes_t,
    ) -> snd_pcm_sframes_t,
>;

#[repr(C)]
pub struct snd_pcm_plugin {
    pub src_format: snd_pcm_plugin_format,
    pub dst_format: snd_pcm_plugin_format,
    pub transfer: TransferFn,
    pub extra_data: *mut c_void,
}

type mulaw_f = Option<
    unsafe extern "C" fn(
        plugin: *mut snd_pcm_plugin,
        src_channels: *const snd_pcm_plugin_channel,
        dst_channels: *mut snd_pcm_plugin_channel,
        frames: snd_pcm_uframes_t,
    ),
>;

#[repr(C)]
struct mulaw_priv {
    func: mulaw_f,
    cvt_endian: c_int,     /* need endian conversion? */
    native_ofs: c_uint,    /* byte offset in native format */
    copy_ofs: c_uint,      /* byte offset in s16 format */
    native_bytes: c_uint,  /* byte size of the native format */
    copy_bytes: c_uint,    /* bytes to copy per conversion */
    flip: u16,             /* MSB flip for signedness, done after endian conversion */
}

#[inline]
unsafe fn val_seg(mut val: c_int) -> c_int {
    let mut r: c_int = 0;
    val >>= 7;
    if val & 0xf0 != 0 {
        val >>= 4;
        r += 4;
    }
    if val & 0x0c != 0 {
        val >>= 2;
        r += 2;
    }
    if val & 0x02 != 0 {
        r += 1;
    }
    r
}

/*
 * linear2ulaw() - Convert a linear PCM value to u-law
 *
 * In order to simplify the encoding process, the original linear magnitude
 * is biased by adding 33 which shifts the encoding range from (0 - 8158) to
 * (33 - 8191). The result can be seen in the following encoding table:
 *
 *	Biased Linear Input Code	Compressed Code
 *	------------------------	---------------
 *	00000001wxyza			000wxyz
 *	0000001wxyzab			001wxyz
 *	000001wxyzabc			010wxyz
 *	00001wxyzabcd			011wxyz
 *	0001wxyzabcde			100wxyz
 *	001wxyzabcdef			101wxyz
 *	01wxyzabcdefg			110wxyz
 *	1wxyzabcdefgh			111wxyz
 *
 * Each biased linear code has a leading 1 which identifies the segment
 * number. The value of the segment number is equal to 7 minus the number
 * of leading 0's. The quantization interval is directly available as the
 * four bits wxyz.  * The trailing bits (a - h) are ignored.
 *
 * Ordinarily the complement of the resulting code word is used for
 * transmission, and so the code word is complemented before it is returned.
 *
 * For further information see John C. Bellamy's Digital Telephony, 1982,
 * John Wiley & Sons, pps 98-111 and 472-476.
 */
unsafe fn linear2ulaw(mut pcm_val: c_int) -> u8 {
    /* 2's complement (16-bit range) */
    let mask: c_int;
    let seg: c_int;
    let uval: u8;

    /* Get the sign and the magnitude of the value. */
    if pcm_val < 0 {
        pcm_val = BIAS - pcm_val;
        mask = 0x7F;
    } else {
        pcm_val += BIAS;
        mask = 0xFF;
    }
    if pcm_val > 0x7FFF {
        pcm_val = 0x7FFF;
    }

    /* Convert the scaled magnitude to segment number. */
    seg = val_seg(pcm_val);

    /*
     * Combine the sign, segment, quantization bits;
     * and complement the code word.
     */
    uval = ((seg << 4) | ((pcm_val >> (seg + 3)) & 0xF)) as u8;
    uval ^ mask as u8
}

/*
 * ulaw2linear() - Convert a u-law value to 16-bit linear PCM
 *
 * First, a biased linear code is derived from the code word. An unbiased
 * output can then be obtained by subtracting 33 from the biased code.
 *
 * Note that this function expects to be passed the complement of the
 * original code word. This is in keeping with ISDN conventions.
 */
unsafe fn ulaw2linear(mut u_val: u8) -> c_int {
    let mut t: c_int;

    /* Complement to obtain normal u-law value. */
    u_val = !u_val;

    /*
     * Extract and bias the quantization bits. Then
     * shift up by the segment number and subtract out the bias.
     */
    t = (((u_val as c_int) & QUANT_MASK) << 3) + BIAS;
    t <<= (((u_val as c_uint) & SEG_MASK as c_uint) >> SEG_SHIFT) as c_int;

    if (u_val as c_int) & SIGN_BIT != 0 {
        BIAS - t
    } else {
        t - BIAS
    }
}

/*
 *  Basic Mu-Law plugin
 */

#[inline]
unsafe fn cvt_s16_to_native(data: *mut mulaw_priv, dst: *mut u8, mut sample: u16) {
    sample ^= (*data).flip;
    if (*data).cvt_endian != 0 {
        sample = swab16(sample);
    }
    if (*data).native_bytes > (*data).copy_bytes {
        memset(dst as *mut c_void, 0, (*data).native_bytes as usize);
    }
    memcpy(
        dst.add((*data).native_ofs as usize) as *mut c_void,
        (&sample as *const u16 as *const c_char).add((*data).copy_ofs as usize) as *const c_void,
        (*data).copy_bytes as usize,
    );
}

unsafe extern "C" fn mulaw_decode(
    plugin: *mut snd_pcm_plugin,
    src_channels: *const snd_pcm_plugin_channel,
    dst_channels: *mut snd_pcm_plugin_channel,
    frames: snd_pcm_uframes_t,
) {
    let data: *mut mulaw_priv = (*plugin).extra_data as *mut mulaw_priv;
    let mut channel: c_int;
    let nchannels: c_int = (*plugin).src_format.channels as c_int;
    channel = 0;
    while channel < nchannels {
        let mut src: *mut c_char;
        let mut dst: *mut c_char;
        let src_step: c_int;
        let dst_step: c_int;
        let mut frames1: snd_pcm_uframes_t;
        if (*src_channels.add(channel as usize)).enabled == 0 {
            if (*dst_channels.add(channel as usize)).wanted != 0 {
                snd_pcm_area_silence(
                    &mut (*dst_channels.add(channel as usize)).area,
                    0,
                    frames,
                    (*plugin).dst_format.format,
                );
            }
            (*dst_channels.add(channel as usize)).enabled = 0;
            channel += 1;
            continue;
        }
        (*dst_channels.add(channel as usize)).enabled = 1;
        src = (*src_channels.add(channel as usize)).area.addr.add(
            ((*src_channels.add(channel as usize)).area.first / 8) as usize,
        );
        dst = (*dst_channels.add(channel as usize)).area.addr.add(
            ((*dst_channels.add(channel as usize)).area.first / 8) as usize,
        );
        src_step = ((*src_channels.add(channel as usize)).area.step / 8) as c_int;
        dst_step = ((*dst_channels.add(channel as usize)).area.step / 8) as c_int;
        frames1 = frames;
        while frames1 > 0 {
            frames1 -= 1;
            let sample: i16 = ulaw2linear(*(src as *mut u8)) as i16;
            cvt_s16_to_native(data, dst as *mut u8, sample as u16);
            src = src.offset(src_step as isize);
            dst = dst.offset(dst_step as isize);
        }
        channel += 1;
    }
}

#[inline]
unsafe fn cvt_native_to_s16(data: *mut mulaw_priv, src: *mut u8) -> i16 {
    let mut sample: u16 = 0;
    memcpy(
        (&mut sample as *mut u16 as *mut c_char).add((*data).copy_ofs as usize) as *mut c_void,
        src.add((*data).native_ofs as usize) as *const c_void,
        (*data).copy_bytes as usize,
    );
    if (*data).cvt_endian != 0 {
        sample = swab16(sample);
    }
    sample ^= (*data).flip;
    sample as i16
}

unsafe extern "C" fn mulaw_encode(
    plugin: *mut snd_pcm_plugin,
    src_channels: *const snd_pcm_plugin_channel,
    dst_channels: *mut snd_pcm_plugin_channel,
    frames: snd_pcm_uframes_t,
) {
    let data: *mut mulaw_priv = (*plugin).extra_data as *mut mulaw_priv;
    let mut channel: c_int;
    let nchannels: c_int = (*plugin).src_format.channels as c_int;
    channel = 0;
    while channel < nchannels {
        let mut src: *mut c_char;
        let mut dst: *mut c_char;
        let src_step: c_int;
        let dst_step: c_int;
        let mut frames1: snd_pcm_uframes_t;
        if (*src_channels.add(channel as usize)).enabled == 0 {
            if (*dst_channels.add(channel as usize)).wanted != 0 {
                snd_pcm_area_silence(
                    &mut (*dst_channels.add(channel as usize)).area,
                    0,
                    frames,
                    (*plugin).dst_format.format,
                );
            }
            (*dst_channels.add(channel as usize)).enabled = 0;
            channel += 1;
            continue;
        }
        (*dst_channels.add(channel as usize)).enabled = 1;
        src = (*src_channels.add(channel as usize)).area.addr.add(
            ((*src_channels.add(channel as usize)).area.first / 8) as usize,
        );
        dst = (*dst_channels.add(channel as usize)).area.addr.add(
            ((*dst_channels.add(channel as usize)).area.first / 8) as usize,
        );
        src_step = ((*src_channels.add(channel as usize)).area.step / 8) as c_int;
        dst_step = ((*dst_channels.add(channel as usize)).area.step / 8) as c_int;
        frames1 = frames;
        while frames1 > 0 {
            frames1 -= 1;
            let sample: i16 = cvt_native_to_s16(data, src as *mut u8);
            *(dst as *mut u8) = linear2ulaw(sample as c_int);
            src = src.offset(src_step as isize);
            dst = dst.offset(dst_step as isize);
        }
        channel += 1;
    }
}

unsafe extern "C" fn mulaw_transfer(
    plugin: *mut snd_pcm_plugin,
    src_channels: *const snd_pcm_plugin_channel,
    dst_channels: *mut snd_pcm_plugin_channel,
    mut frames: snd_pcm_uframes_t,
) -> snd_pcm_sframes_t {
    let data: *mut mulaw_priv;

    if snd_BUG_ON(plugin.is_null() || src_channels.is_null() || dst_channels.is_null()) != 0 {
        return -ENXIO;
    }
    if frames == 0 {
        return 0;
    }
    /* CONFIG_SND_DEBUG: validate that channel area offsets and steps are byte-aligned. */
    #[cfg(CONFIG_SND_DEBUG)]
    {
        let mut channel: c_uint = 0;
        while channel < (*plugin).src_format.channels {
            if snd_BUG_ON(
                (*src_channels.add(channel as usize)).area.first % 8 != 0
                    || (*src_channels.add(channel as usize)).area.step % 8 != 0,
            ) != 0
            {
                return -ENXIO;
            }
            if snd_BUG_ON(
                (*dst_channels.add(channel as usize)).area.first % 8 != 0
                    || (*dst_channels.add(channel as usize)).area.step % 8 != 0,
            ) != 0
            {
                return -ENXIO;
            }
            channel += 1;
        }
    }
    if frames > (*dst_channels.add(0)).frames {
        frames = (*dst_channels.add(0)).frames;
    }
    data = (*plugin).extra_data as *mut mulaw_priv;
    if let Some(func) = (*data).func {
        func(plugin, src_channels, dst_channels, frames);
    }
    frames as snd_pcm_sframes_t
}

unsafe fn init_data(data: *mut mulaw_priv, format: snd_pcm_format_t) {
    /* SNDRV_LITTLE_ENDIAN selects whether big-endian or little-endian formats need conversion. */
    #[cfg(SNDRV_LITTLE_ENDIAN)]
    {
        (*data).cvt_endian = (snd_pcm_format_big_endian(format) > 0) as c_int;
    }
    #[cfg(not(SNDRV_LITTLE_ENDIAN))]
    {
        (*data).cvt_endian = (snd_pcm_format_little_endian(format) > 0) as c_int;
    }
    if snd_pcm_format_signed(format) == 0 {
        (*data).flip = 0x8000;
    }
    (*data).native_bytes = (snd_pcm_format_physical_width(format) / 8) as c_uint;
    (*data).copy_bytes = if (*data).native_bytes < 2 { 1 } else { 2 };
    if snd_pcm_format_little_endian(format) != 0 {
        (*data).native_ofs = (*data).native_bytes - (*data).copy_bytes;
        (*data).copy_ofs = 2 - (*data).copy_bytes;
    } else {
        /* S24 in 4bytes need an 1 byte offset */
        (*data).native_ofs =
            (*data).native_bytes - (snd_pcm_format_width(format) / 8) as c_uint;
    }
}

#[no_mangle]
pub unsafe extern "C" fn snd_pcm_plugin_build_mulaw(
    plug: *mut snd_pcm_substream,
    src_format: *mut snd_pcm_plugin_format,
    dst_format: *mut snd_pcm_plugin_format,
    r_plugin: *mut *mut snd_pcm_plugin,
) -> c_int {
    let mut err: c_int;
    let data: *mut mulaw_priv;
    let mut plugin: *mut snd_pcm_plugin = ptr::null_mut();
    let format: *mut snd_pcm_plugin_format;
    let func: mulaw_f;

    if snd_BUG_ON(r_plugin.is_null()) != 0 {
        return -ENXIO;
    }
    *r_plugin = ptr::null_mut();

    if snd_BUG_ON((*src_format).rate != (*dst_format).rate) != 0 {
        return -ENXIO;
    }
    if snd_BUG_ON((*src_format).channels != (*dst_format).channels) != 0 {
        return -ENXIO;
    }

    if (*dst_format).format == SNDRV_PCM_FORMAT_MU_LAW {
        format = src_format;
        func = Some(mulaw_encode);
    } else if (*src_format).format == SNDRV_PCM_FORMAT_MU_LAW {
        format = dst_format;
        func = Some(mulaw_decode);
    } else {
        snd_BUG();
        return -EINVAL;
    }
    if snd_pcm_format_linear((*format).format) == 0 {
        return -EINVAL;
    }

    err = snd_pcm_plugin_build(
        plug,
        b"Mu-Law<->linear conversion\0".as_ptr() as *const c_char,
        src_format,
        dst_format,
        core::mem::size_of::<mulaw_priv>(),
        &mut plugin,
    );
    if err < 0 {
        return err;
    }
    data = (*plugin).extra_data as *mut mulaw_priv;
    (*data).func = func;
    init_data(data, (*format).format);
    (*plugin).transfer = Some(mulaw_transfer);
    *r_plugin = plugin;
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
