// SPDX-License-Identifier: LGPL-2.0+
/*
 *  Linear conversion Plug-In
 *  Copyright (c) 1999 by Jaroslav Kysela <perex@perex.cz>,
 *			  Abramo Bagnara <abramo@alsa-project.org>
 */

/*
 * Dependencies from the original C source:
 * <linux/time.h>, <sound/core.h>, <sound/pcm.h>, and "pcm_plugin.h".
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem;
use core::ptr;

pub type snd_pcm_uframes_t = u64;
pub type snd_pcm_sframes_t = i64;
pub type snd_pcm_format_t = c_int;

const ENXIO: c_int = 6;

#[repr(C)]
pub struct snd_pcm_substream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_plugin_format {
    pub format: snd_pcm_format_t,
    pub rate: c_uint,
    pub channels: c_uint,
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
pub struct snd_pcm_plugin {
    pub src_format: snd_pcm_plugin_format,
    pub dst_format: snd_pcm_plugin_format,
    pub transfer: Option<
        unsafe extern "C" fn(
            *mut snd_pcm_plugin,
            *const snd_pcm_plugin_channel,
            *mut snd_pcm_plugin_channel,
            snd_pcm_uframes_t,
        ) -> snd_pcm_sframes_t,
    >,
    pub extra_data: *mut c_void,
}

unsafe extern "C" {
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn swab32(val: u32) -> u32;
    fn cpu_to_le32(val: u32) -> u32;
    fn cpu_to_be32(val: u32) -> u32;
    fn snd_BUG_ON(cond: bool) -> c_int;
    fn snd_pcm_area_silence(
        dst_area: *mut snd_pcm_channel_area,
        dst_offset: c_uint,
        samples: snd_pcm_uframes_t,
        format: snd_pcm_format_t,
    );
    fn snd_pcm_format_width(format: snd_pcm_format_t) -> c_int;
    fn snd_pcm_format_physical_width(format: snd_pcm_format_t) -> c_int;
    fn snd_pcm_format_little_endian(format: snd_pcm_format_t) -> c_int;
    fn snd_pcm_format_signed(format: snd_pcm_format_t) -> c_int;
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

/*
 *  Basic linear conversion plugin
 */

#[repr(C)]
struct linear_priv {
    cvt_endian: c_int,  /* need endian conversion? */
    src_ofs: c_uint,    /* byte offset in source format */
    dst_ofs: c_uint,    /* byte soffset in destination format */
    copy_ofs: c_uint,   /* byte offset in temporary u32 data */
    dst_bytes: c_uint,  /* byte size of destination format */
    copy_bytes: c_uint, /* bytes to copy per conversion */
    flip: c_uint,       /* MSB flip for signeness, done after endian conv */
}

#[inline]
unsafe fn do_convert(data: *mut linear_priv, dst: *mut u8, src: *mut u8) {
    let mut tmp: c_uint = 0;
    let p = &mut tmp as *mut c_uint as *mut u8;

    unsafe {
        memcpy(
            p.add((*data).copy_ofs as usize) as *mut c_void,
            src.add((*data).src_ofs as usize) as *const c_void,
            (*data).copy_bytes as usize,
        );
        if (*data).cvt_endian != 0 {
            tmp = swab32(tmp);
        }
        tmp ^= (*data).flip;
        memcpy(
            dst as *mut c_void,
            p.add((*data).dst_ofs as usize) as *const c_void,
            (*data).dst_bytes as usize,
        );
    }
}

unsafe extern "C" fn convert(
    plugin: *mut snd_pcm_plugin,
    src_channels: *const snd_pcm_plugin_channel,
    dst_channels: *mut snd_pcm_plugin_channel,
    frames: snd_pcm_uframes_t,
) {
    let data = unsafe { (*plugin).extra_data as *mut linear_priv };
    let mut channel: c_int;
    let nchannels: c_int = unsafe { (*plugin).src_format.channels as c_int };

    channel = 0;
    while channel < nchannels {
        let mut src: *mut c_char;
        let mut dst: *mut c_char;
        let src_step: c_int;
        let dst_step: c_int;
        let mut frames1: snd_pcm_uframes_t;

        unsafe {
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
            src = (*src_channels.add(channel as usize))
                .area
                .addr
                .add(((*src_channels.add(channel as usize)).area.first / 8) as usize);
            dst = (*dst_channels.add(channel as usize))
                .area
                .addr
                .add(((*dst_channels.add(channel as usize)).area.first / 8) as usize);
            src_step = ((*src_channels.add(channel as usize)).area.step / 8) as c_int;
            dst_step = ((*dst_channels.add(channel as usize)).area.step / 8) as c_int;
            frames1 = frames;
            while {
                let old = frames1;
                frames1 = frames1.wrapping_sub(1);
                old > 0
            } {
                do_convert(data, dst as *mut u8, src as *mut u8);
                src = src.add(src_step as usize);
                dst = dst.add(dst_step as usize);
            }
        }
        channel += 1;
    }
}

unsafe extern "C" fn linear_transfer(
    plugin: *mut snd_pcm_plugin,
    src_channels: *const snd_pcm_plugin_channel,
    dst_channels: *mut snd_pcm_plugin_channel,
    mut frames: snd_pcm_uframes_t,
) -> snd_pcm_sframes_t {
    unsafe {
        if snd_BUG_ON(plugin.is_null() || src_channels.is_null() || dst_channels.is_null()) != 0 {
            return -(ENXIO as snd_pcm_sframes_t);
        }
        if frames == 0 {
            return 0;
        }

        /*
         * Original code checks this under CONFIG_SND_DEBUG:
         * each channel's area.first and area.step must be byte-aligned.
         */
        #[cfg(CONFIG_SND_DEBUG)]
        {
            let mut channel: c_uint = 0;
            while channel < (*plugin).src_format.channels {
                if snd_BUG_ON(
                    (*src_channels.add(channel as usize)).area.first % 8 != 0
                        || (*src_channels.add(channel as usize)).area.step % 8 != 0,
                ) != 0
                {
                    return -(ENXIO as snd_pcm_sframes_t);
                }
                if snd_BUG_ON(
                    (*dst_channels.add(channel as usize)).area.first % 8 != 0
                        || (*dst_channels.add(channel as usize)).area.step % 8 != 0,
                ) != 0
                {
                    return -(ENXIO as snd_pcm_sframes_t);
                }
                channel += 1;
            }
        }

        if frames > (*dst_channels.add(0)).frames {
            frames = (*dst_channels.add(0)).frames;
        }
        convert(plugin, src_channels, dst_channels, frames);
        frames as snd_pcm_sframes_t
    }
}

unsafe fn init_data(
    data: *mut linear_priv,
    src_format: snd_pcm_format_t,
    dst_format: snd_pcm_format_t,
) {
    let src_le: c_int;
    let dst_le: c_int;
    let src_bytes: c_int;
    let dst_bytes: c_int;

    unsafe {
        src_bytes = snd_pcm_format_width(src_format) / 8;
        dst_bytes = snd_pcm_format_width(dst_format) / 8;
        src_le = (snd_pcm_format_little_endian(src_format) > 0) as c_int;
        dst_le = (snd_pcm_format_little_endian(dst_format) > 0) as c_int;

        (*data).dst_bytes = dst_bytes as c_uint;
        (*data).cvt_endian = (src_le != dst_le) as c_int;
        (*data).copy_bytes = if src_bytes < dst_bytes {
            src_bytes
        } else {
            dst_bytes
        } as c_uint;
        if src_le != 0 {
            (*data).copy_ofs = 4u32.wrapping_sub((*data).copy_bytes);
            (*data).src_ofs = (src_bytes as c_uint).wrapping_sub((*data).copy_bytes);
        } else {
            (*data).src_ofs =
                (snd_pcm_format_physical_width(src_format) / 8 - src_bytes) as c_uint;
        }
        if dst_le != 0 {
            (*data).dst_ofs = 4u32.wrapping_sub((*data).dst_bytes);
        } else {
            (*data).dst_ofs =
                (snd_pcm_format_physical_width(dst_format) / 8 - dst_bytes) as c_uint;
        }
        if snd_pcm_format_signed(src_format) != snd_pcm_format_signed(dst_format) {
            if dst_le != 0 {
                (*data).flip = cpu_to_le32(0x80000000);
            } else {
                (*data).flip = cpu_to_be32(0x80000000);
            }
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_pcm_plugin_build_linear(
    plug: *mut snd_pcm_substream,
    src_format: *mut snd_pcm_plugin_format,
    dst_format: *mut snd_pcm_plugin_format,
    r_plugin: *mut *mut snd_pcm_plugin,
) -> c_int {
    let err: c_int;
    let data: *mut linear_priv;
    let mut plugin: *mut snd_pcm_plugin = ptr::null_mut();

    unsafe {
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
        if snd_BUG_ON(
            snd_pcm_format_linear((*src_format).format) == 0
                || snd_pcm_format_linear((*dst_format).format) == 0,
        ) != 0
        {
            return -ENXIO;
        }

        err = snd_pcm_plugin_build(
            plug,
            c"linear format conversion".as_ptr(),
            src_format,
            dst_format,
            mem::size_of::<linear_priv>(),
            &mut plugin,
        );
        if err < 0 {
            return err;
        }
        data = (*plugin).extra_data as *mut linear_priv;
        init_data(data, (*src_format).format, (*dst_format).format);
        (*plugin).transfer = Some(linear_transfer);
        *r_plugin = plugin;
        0
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
