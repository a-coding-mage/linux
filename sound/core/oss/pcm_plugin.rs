// SPDX-License-Identifier: LGPL-2.0+
/*
 *  PCM Plug-In shared (kernel/library) code
 *  Copyright (c) 1999 by Jaroslav Kysela <perex@perex.cz>
 *  Copyright (c) 2000 by Abramo Bagnara <abramo@alsa-project.org>
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

pub type size_t = usize;
pub type ssize_t = isize;
pub type u64 = u64;
pub type snd_pcm_uframes_t = usize;
pub type snd_pcm_sframes_t = isize;
pub type snd_pcm_format_t = c_int;
pub type snd_pcm_access_t = c_int;

// #if 0
// #define PLUGIN_DEBUG
// #endif

pub const ENOMEM: c_int = 12;
pub const ENXIO: c_int = 6;
pub const EINVAL: c_int = 22;
pub const GFP_KERNEL: c_int = 0;

pub const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
pub const SNDRV_PCM_STREAM_CAPTURE: c_int = 1;
pub const SNDRV_PCM_ACCESS_RW_INTERLEAVED: snd_pcm_access_t = 3;
pub const SNDRV_PCM_ACCESS_RW_NONINTERLEAVED: snd_pcm_access_t = 4;

extern "C" {
    pub static SNDRV_PCM_FORMAT_MU_LAW: snd_pcm_format_t;
    pub static SNDRV_PCM_FORMAT_S16: snd_pcm_format_t;
    pub static SNDRV_PCM_FORMAT_S16_LE: snd_pcm_format_t;
    pub static SNDRV_PCM_FORMAT_S16_BE: snd_pcm_format_t;
    pub static SNDRV_PCM_FORMAT_U16_LE: snd_pcm_format_t;
    pub static SNDRV_PCM_FORMAT_U16_BE: snd_pcm_format_t;
    pub static SNDRV_PCM_FORMAT_S24_3LE: snd_pcm_format_t;
    pub static SNDRV_PCM_FORMAT_S24_3BE: snd_pcm_format_t;
    pub static SNDRV_PCM_FORMAT_U24_3LE: snd_pcm_format_t;
    pub static SNDRV_PCM_FORMAT_U24_3BE: snd_pcm_format_t;
    pub static SNDRV_PCM_FORMAT_S24_LE: snd_pcm_format_t;
    pub static SNDRV_PCM_FORMAT_S24_BE: snd_pcm_format_t;
    pub static SNDRV_PCM_FORMAT_U24_LE: snd_pcm_format_t;
    pub static SNDRV_PCM_FORMAT_U24_BE: snd_pcm_format_t;
    pub static SNDRV_PCM_FORMAT_S32_LE: snd_pcm_format_t;
    pub static SNDRV_PCM_FORMAT_S32_BE: snd_pcm_format_t;
    pub static SNDRV_PCM_FORMAT_U32_LE: snd_pcm_format_t;
    pub static SNDRV_PCM_FORMAT_U32_BE: snd_pcm_format_t;
    pub static SNDRV_PCM_FORMAT_S8: snd_pcm_format_t;
    pub static SNDRV_PCM_FORMAT_U8: snd_pcm_format_t;

    pub static SNDRV_PCM_FMTBIT_U8: u64;
    pub static SNDRV_PCM_FMTBIT_S8: u64;
    pub static SNDRV_PCM_FMTBIT_U16_LE: u64;
    pub static SNDRV_PCM_FMTBIT_S16_LE: u64;
    pub static SNDRV_PCM_FMTBIT_U16_BE: u64;
    pub static SNDRV_PCM_FMTBIT_S16_BE: u64;
    pub static SNDRV_PCM_FMTBIT_U24_LE: u64;
    pub static SNDRV_PCM_FMTBIT_S24_LE: u64;
    pub static SNDRV_PCM_FMTBIT_U24_BE: u64;
    pub static SNDRV_PCM_FMTBIT_S24_BE: u64;
    pub static SNDRV_PCM_FMTBIT_U24_3LE: u64;
    pub static SNDRV_PCM_FMTBIT_S24_3LE: u64;
    pub static SNDRV_PCM_FMTBIT_U24_3BE: u64;
    pub static SNDRV_PCM_FMTBIT_S24_3BE: u64;
    pub static SNDRV_PCM_FMTBIT_U32_LE: u64;
    pub static SNDRV_PCM_FMTBIT_S32_LE: u64;
    pub static SNDRV_PCM_FMTBIT_U32_BE: u64;
    pub static SNDRV_PCM_FMTBIT_S32_BE: u64;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_mask {
    pub bits: [u32; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
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
pub struct snd_pcm_runtime_oss {
    pub plugin_first: *mut snd_pcm_plugin,
    pub plugin_last: *mut snd_pcm_plugin,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    pub oss: snd_pcm_runtime_oss,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_plugin {
    pub name: *const c_char,
    pub plug: *mut snd_pcm_substream,
    pub stream: c_int,
    pub access: snd_pcm_access_t,
    pub src_format: snd_pcm_plugin_format,
    pub src_width: ssize_t,
    pub dst_format: snd_pcm_plugin_format,
    pub dst_width: ssize_t,
    pub buf_channels: *mut snd_pcm_plugin_channel,
    pub client_channels: Option<
        unsafe extern "C" fn(
            *mut snd_pcm_plugin,
            snd_pcm_uframes_t,
            *mut *mut snd_pcm_plugin_channel,
        ) -> snd_pcm_sframes_t,
    >,
    pub private_free: Option<unsafe extern "C" fn(*mut snd_pcm_plugin)>,
    pub buf: *mut c_char,
    pub buf_frames: snd_pcm_uframes_t,
    pub next: *mut snd_pcm_plugin,
    pub prev: *mut snd_pcm_plugin,
    pub dst_frames:
        Option<unsafe extern "C" fn(*mut snd_pcm_plugin, snd_pcm_uframes_t) -> snd_pcm_sframes_t>,
    pub src_frames:
        Option<unsafe extern "C" fn(*mut snd_pcm_plugin, snd_pcm_uframes_t) -> snd_pcm_sframes_t>,
    pub transfer: Option<
        unsafe extern "C" fn(
            *mut snd_pcm_plugin,
            *mut snd_pcm_plugin_channel,
            *mut snd_pcm_plugin_channel,
            snd_pcm_uframes_t,
        ) -> snd_pcm_sframes_t,
    >,
}

extern "C" {
    fn snd_pcm_format_physical_width(format: snd_pcm_format_t) -> c_int;
    fn snd_pcm_format_width(format: snd_pcm_format_t) -> c_uint;
    fn snd_pcm_format_unsigned(format: snd_pcm_format_t) -> c_int;
    fn snd_pcm_format_big_endian(format: snd_pcm_format_t) -> c_int;
    fn snd_pcm_format_linear(format: snd_pcm_format_t) -> c_int;
    fn snd_pcm_format_set_silence(format: snd_pcm_format_t, data: *mut c_void, samples: size_t)
        -> c_int;
    fn snd_pcm_format_silence_64(format: snd_pcm_format_t) -> *const c_uchar;
    fn snd_mask_set(mask: *mut snd_mask, val: snd_pcm_format_t);
    fn snd_mask_test(mask: *const snd_mask, val: snd_pcm_format_t) -> c_int;
    fn snd_pcm_plug_stream(plug: *mut snd_pcm_substream) -> c_int;
    fn snd_BUG() -> c_int;
    fn snd_BUG_ON(cond: bool) -> c_int;
    fn array3_size(a: size_t, b: size_t, c: size_t) -> size_t;
    fn kvfree(p: *mut c_void);
    fn kvzalloc(size: size_t, flags: c_int) -> *mut c_char;
    fn kfree(p: *mut c_void);
    fn kzalloc_flex_plugin(extra: size_t) -> *mut snd_pcm_plugin;
    fn kzalloc_plugin_channels(channels: c_uint) -> *mut snd_pcm_plugin_channel;
    fn params_format(params: *mut snd_pcm_hw_params) -> snd_pcm_format_t;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_access(params: *mut snd_pcm_hw_params) -> snd_pcm_access_t;
    fn snd_pcm_plugin_build_mulaw(
        plug: *mut snd_pcm_substream,
        src_format: *mut snd_pcm_plugin_format,
        dst_format: *mut snd_pcm_plugin_format,
        ret: *mut *mut snd_pcm_plugin,
    ) -> c_int;
    fn snd_pcm_plugin_build_route(
        plug: *mut snd_pcm_substream,
        src_format: *mut snd_pcm_plugin_format,
        dst_format: *mut snd_pcm_plugin_format,
        ret: *mut *mut snd_pcm_plugin,
    ) -> c_int;
    fn snd_pcm_plugin_build_linear(
        plug: *mut snd_pcm_substream,
        src_format: *mut snd_pcm_plugin_format,
        dst_format: *mut snd_pcm_plugin_format,
        ret: *mut *mut snd_pcm_plugin,
    ) -> c_int;
    fn snd_pcm_plugin_build_rate(
        plug: *mut snd_pcm_substream,
        src_format: *mut snd_pcm_plugin_format,
        dst_format: *mut snd_pcm_plugin_format,
        ret: *mut *mut snd_pcm_plugin,
    ) -> c_int;
    fn snd_pcm_plugin_build_copy(
        plug: *mut snd_pcm_substream,
        src_format: *mut snd_pcm_plugin_format,
        dst_format: *mut snd_pcm_plugin_format,
        ret: *mut *mut snd_pcm_plugin,
    ) -> c_int;
    fn snd_pcm_plugin_append(plugin: *mut snd_pcm_plugin) -> c_int;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn pdprintf(fmt: *const c_char, ...);
}

pub type c_uchar = u8;

#[inline]
unsafe fn snd_pcm_plug_first(plug: *mut snd_pcm_substream) -> *mut snd_pcm_plugin {
    (*(*plug).runtime).oss.plugin_first
}

#[inline]
unsafe fn snd_pcm_plug_last(plug: *mut snd_pcm_substream) -> *mut snd_pcm_plugin {
    (*(*plug).runtime).oss.plugin_last
}

#[inline]
fn lower_32_bits(x: u64) -> u32 {
    x as u32
}

#[inline]
fn upper_32_bits(x: u64) -> u32 {
    (x >> 32) as u32
}

/*
 *  because some cards might have rates "very close", we ignore
 *  all "resampling" requests within +-5%
 */
unsafe fn rate_match(src_rate: c_uint, dst_rate: c_uint) -> c_int {
    let low: c_uint = src_rate.wrapping_mul(95) / 100;
    let high: c_uint = src_rate.wrapping_mul(105) / 100;
    (dst_rate >= low && dst_rate <= high) as c_int
}

unsafe fn snd_pcm_plugin_alloc(
    plugin: *mut snd_pcm_plugin,
    frames: snd_pcm_uframes_t,
) -> c_int {
    let format: *mut snd_pcm_plugin_format;
    let width: ssize_t;
    let mut size: size_t;
    let mut channel: c_uint;
    let mut c: *mut snd_pcm_plugin_channel;

    if (*plugin).stream == SNDRV_PCM_STREAM_PLAYBACK {
        format = &mut (*plugin).src_format;
    } else {
        format = &mut (*plugin).dst_format;
    }
    width = snd_pcm_format_physical_width((*format).format) as ssize_t;
    if width < 0 {
        return width as c_int;
    }
    size = array3_size(frames, (*format).channels as size_t, width as size_t);
    /* check for too large period size once again */
    if size > 1024 * 1024 {
        return -ENOMEM;
    }
    if snd_BUG_ON(size % 8 != 0) != 0 {
        return -ENXIO;
    }
    size /= 8;
    if (*plugin).buf_frames < frames {
        kvfree((*plugin).buf as *mut c_void);
        (*plugin).buf = kvzalloc(size, GFP_KERNEL);
        (*plugin).buf_frames = frames;
    }
    if (*plugin).buf.is_null() {
        (*plugin).buf_frames = 0;
        return -ENOMEM;
    }
    c = (*plugin).buf_channels;
    if (*plugin).access == SNDRV_PCM_ACCESS_RW_INTERLEAVED {
        channel = 0;
        while channel < (*format).channels {
            (*c).frames = frames;
            (*c).enabled = 1;
            (*c).wanted = 0;
            (*c).area.addr = (*plugin).buf;
            (*c).area.first = channel.wrapping_mul(width as c_uint);
            (*c).area.step = (*format).channels.wrapping_mul(width as c_uint);
            channel = channel.wrapping_add(1);
            c = c.add(1);
        }
    } else if (*plugin).access == SNDRV_PCM_ACCESS_RW_NONINTERLEAVED {
        if snd_BUG_ON(size % (*format).channels as size_t != 0) != 0 {
            return -EINVAL;
        }
        size /= (*format).channels as size_t;
        channel = 0;
        while channel < (*format).channels {
            (*c).frames = frames;
            (*c).enabled = 1;
            (*c).wanted = 0;
            (*c).area.addr = (*plugin).buf.add(channel as usize * size);
            (*c).area.first = 0;
            (*c).area.step = width as c_uint;
            channel = channel.wrapping_add(1);
            c = c.add(1);
        }
    } else {
        return -EINVAL;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_pcm_plug_alloc(
    plug: *mut snd_pcm_substream,
    mut frames: snd_pcm_uframes_t,
) -> c_int {
    let mut err: c_int;
    if snd_BUG_ON(snd_pcm_plug_first(plug).is_null()) != 0 {
        return -ENXIO;
    }
    if snd_pcm_plug_stream(plug) == SNDRV_PCM_STREAM_PLAYBACK {
        let mut plugin = snd_pcm_plug_first(plug);
        while !(*plugin).next.is_null() {
            if let Some(dst_frames) = (*plugin).dst_frames {
                frames = dst_frames(plugin, frames) as snd_pcm_uframes_t;
            }
            if (frames as snd_pcm_sframes_t) <= 0 {
                return -ENXIO;
            }
            plugin = (*plugin).next;
            err = snd_pcm_plugin_alloc(plugin, frames);
            if err < 0 {
                return err;
            }
        }
    } else {
        let mut plugin = snd_pcm_plug_last(plug);
        while !(*plugin).prev.is_null() {
            if let Some(src_frames) = (*plugin).src_frames {
                frames = src_frames(plugin, frames) as snd_pcm_uframes_t;
            }
            if (frames as snd_pcm_sframes_t) <= 0 {
                return -ENXIO;
            }
            plugin = (*plugin).prev;
            err = snd_pcm_plugin_alloc(plugin, frames);
            if err < 0 {
                return err;
            }
        }
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_pcm_plugin_client_channels(
    plugin: *mut snd_pcm_plugin,
    frames: snd_pcm_uframes_t,
    channels: *mut *mut snd_pcm_plugin_channel,
) -> snd_pcm_sframes_t {
    *channels = (*plugin).buf_channels;
    frames as snd_pcm_sframes_t
}

#[no_mangle]
pub unsafe extern "C" fn snd_pcm_plugin_build(
    plug: *mut snd_pcm_substream,
    name: *const c_char,
    src_format: *mut snd_pcm_plugin_format,
    dst_format: *mut snd_pcm_plugin_format,
    extra: size_t,
    ret: *mut *mut snd_pcm_plugin,
) -> c_int {
    let plugin: *mut snd_pcm_plugin;
    let channels: c_uint;

    if snd_BUG_ON(plug.is_null()) != 0 {
        return -ENXIO;
    }
    if snd_BUG_ON(src_format.is_null() || dst_format.is_null()) != 0 {
        return -ENXIO;
    }
    plugin = kzalloc_flex_plugin(extra);
    if plugin.is_null() {
        return -ENOMEM;
    }
    (*plugin).name = name;
    (*plugin).plug = plug;
    (*plugin).stream = snd_pcm_plug_stream(plug);
    (*plugin).access = SNDRV_PCM_ACCESS_RW_INTERLEAVED;
    (*plugin).src_format = *src_format;
    (*plugin).src_width = snd_pcm_format_physical_width((*src_format).format) as ssize_t;
    snd_BUG_ON((*plugin).src_width <= 0);
    (*plugin).dst_format = *dst_format;
    (*plugin).dst_width = snd_pcm_format_physical_width((*dst_format).format) as ssize_t;
    snd_BUG_ON((*plugin).dst_width <= 0);
    if (*plugin).stream == SNDRV_PCM_STREAM_PLAYBACK {
        channels = (*src_format).channels;
    } else {
        channels = (*dst_format).channels;
    }
    (*plugin).buf_channels = kzalloc_plugin_channels(channels);
    if (*plugin).buf_channels.is_null() {
        snd_pcm_plugin_free(plugin);
        return -ENOMEM;
    }
    (*plugin).client_channels = Some(snd_pcm_plugin_client_channels);
    *ret = plugin;
    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_pcm_plugin_free(plugin: *mut snd_pcm_plugin) -> c_int {
    if plugin.is_null() {
        return 0;
    }
    if let Some(private_free) = (*plugin).private_free {
        private_free(plugin);
    }
    kfree((*plugin).buf_channels as *mut c_void);
    kvfree((*plugin).buf as *mut c_void);
    kfree(plugin as *mut c_void);
    0
}

unsafe fn calc_dst_frames(
    plug: *mut snd_pcm_substream,
    mut frames: snd_pcm_sframes_t,
    check_size: bool,
) -> snd_pcm_sframes_t {
    let mut plugin: *mut snd_pcm_plugin;
    let mut plugin_next: *mut snd_pcm_plugin;

    plugin = snd_pcm_plug_first(plug);
    while !plugin.is_null() && frames > 0 {
        plugin_next = (*plugin).next;
        if check_size && (*plugin).buf_frames != 0 && frames > (*plugin).buf_frames as snd_pcm_sframes_t
        {
            frames = (*plugin).buf_frames as snd_pcm_sframes_t;
        }
        if let Some(dst_frames) = (*plugin).dst_frames {
            frames = dst_frames(plugin, frames as snd_pcm_uframes_t);
            if frames < 0 {
                return frames;
            }
        }
        plugin = plugin_next;
    }
    frames
}

unsafe fn calc_src_frames(
    plug: *mut snd_pcm_substream,
    mut frames: snd_pcm_sframes_t,
    check_size: bool,
) -> snd_pcm_sframes_t {
    let mut plugin: *mut snd_pcm_plugin;
    let mut plugin_prev: *mut snd_pcm_plugin;

    plugin = snd_pcm_plug_last(plug);
    while !plugin.is_null() && frames > 0 {
        plugin_prev = (*plugin).prev;
        if let Some(src_frames) = (*plugin).src_frames {
            frames = src_frames(plugin, frames as snd_pcm_uframes_t);
            if frames < 0 {
                return frames;
            }
        }
        if check_size && (*plugin).buf_frames != 0 && frames > (*plugin).buf_frames as snd_pcm_sframes_t
        {
            frames = (*plugin).buf_frames as snd_pcm_sframes_t;
        }
        plugin = plugin_prev;
    }
    frames
}

#[no_mangle]
pub unsafe extern "C" fn snd_pcm_plug_client_size(
    plug: *mut snd_pcm_substream,
    drv_frames: snd_pcm_uframes_t,
) -> snd_pcm_sframes_t {
    if snd_BUG_ON(plug.is_null()) != 0 {
        return -ENXIO as snd_pcm_sframes_t;
    }
    match snd_pcm_plug_stream(plug) {
        SNDRV_PCM_STREAM_PLAYBACK => calc_src_frames(plug, drv_frames as snd_pcm_sframes_t, false),
        SNDRV_PCM_STREAM_CAPTURE => calc_dst_frames(plug, drv_frames as snd_pcm_sframes_t, false),
        _ => {
            snd_BUG();
            -EINVAL as snd_pcm_sframes_t
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn snd_pcm_plug_slave_size(
    plug: *mut snd_pcm_substream,
    clt_frames: snd_pcm_uframes_t,
) -> snd_pcm_sframes_t {
    if snd_BUG_ON(plug.is_null()) != 0 {
        return -ENXIO as snd_pcm_sframes_t;
    }
    match snd_pcm_plug_stream(plug) {
        SNDRV_PCM_STREAM_PLAYBACK => calc_dst_frames(plug, clt_frames as snd_pcm_sframes_t, false),
        SNDRV_PCM_STREAM_CAPTURE => calc_src_frames(plug, clt_frames as snd_pcm_sframes_t, false),
        _ => {
            snd_BUG();
            -EINVAL as snd_pcm_sframes_t
        }
    }
}

unsafe fn snd_pcm_plug_formats(mask: *const snd_mask, format: snd_pcm_format_t) -> c_int {
    let mut formats: snd_mask = *mask;
    let linfmts: u64 = SNDRV_PCM_FMTBIT_U8
        | SNDRV_PCM_FMTBIT_S8
        | SNDRV_PCM_FMTBIT_U16_LE
        | SNDRV_PCM_FMTBIT_S16_LE
        | SNDRV_PCM_FMTBIT_U16_BE
        | SNDRV_PCM_FMTBIT_S16_BE
        | SNDRV_PCM_FMTBIT_U24_LE
        | SNDRV_PCM_FMTBIT_S24_LE
        | SNDRV_PCM_FMTBIT_U24_BE
        | SNDRV_PCM_FMTBIT_S24_BE
        | SNDRV_PCM_FMTBIT_U24_3LE
        | SNDRV_PCM_FMTBIT_S24_3LE
        | SNDRV_PCM_FMTBIT_U24_3BE
        | SNDRV_PCM_FMTBIT_S24_3BE
        | SNDRV_PCM_FMTBIT_U32_LE
        | SNDRV_PCM_FMTBIT_S32_LE
        | SNDRV_PCM_FMTBIT_U32_BE
        | SNDRV_PCM_FMTBIT_S32_BE;
    snd_mask_set(&mut formats, SNDRV_PCM_FORMAT_MU_LAW);

    if formats.bits[0] & lower_32_bits(linfmts) != 0 {
        formats.bits[0] |= lower_32_bits(linfmts);
    }
    if formats.bits[1] & upper_32_bits(linfmts) != 0 {
        formats.bits[1] |= upper_32_bits(linfmts);
    }
    snd_mask_test(&formats, format)
}

static mut preferred_formats: [snd_pcm_format_t; 18] = [
    unsafe { SNDRV_PCM_FORMAT_S16_LE },
    unsafe { SNDRV_PCM_FORMAT_S16_BE },
    unsafe { SNDRV_PCM_FORMAT_U16_LE },
    unsafe { SNDRV_PCM_FORMAT_U16_BE },
    unsafe { SNDRV_PCM_FORMAT_S24_3LE },
    unsafe { SNDRV_PCM_FORMAT_S24_3BE },
    unsafe { SNDRV_PCM_FORMAT_U24_3LE },
    unsafe { SNDRV_PCM_FORMAT_U24_3BE },
    unsafe { SNDRV_PCM_FORMAT_S24_LE },
    unsafe { SNDRV_PCM_FORMAT_S24_BE },
    unsafe { SNDRV_PCM_FORMAT_U24_LE },
    unsafe { SNDRV_PCM_FORMAT_U24_BE },
    unsafe { SNDRV_PCM_FORMAT_S32_LE },
    unsafe { SNDRV_PCM_FORMAT_S32_BE },
    unsafe { SNDRV_PCM_FORMAT_U32_LE },
    unsafe { SNDRV_PCM_FORMAT_U32_BE },
    unsafe { SNDRV_PCM_FORMAT_S8 },
    unsafe { SNDRV_PCM_FORMAT_U8 },
];

#[no_mangle]
pub unsafe extern "C" fn snd_pcm_plug_slave_format(
    format: snd_pcm_format_t,
    format_mask: *const snd_mask,
) -> snd_pcm_format_t {
    let mut i: c_int;

    if snd_mask_test(format_mask, format) != 0 {
        return format;
    }
    if snd_pcm_plug_formats(format_mask, format) == 0 {
        return -EINVAL;
    }
    if snd_pcm_format_linear(format) != 0 {
        let width: c_uint = snd_pcm_format_width(format);
        let unsignd: c_int = (snd_pcm_format_unsigned(format) > 0) as c_int;
        let big: c_int = (snd_pcm_format_big_endian(format) > 0) as c_int;
        let mut badness: c_uint;
        let mut best: c_uint = !0;
        let mut best_format: snd_pcm_format_t = -1;
        i = 0;
        while (i as usize) < preferred_formats.len() {
            let f: snd_pcm_format_t = preferred_formats[i as usize];
            let w: c_uint;
            if snd_mask_test(format_mask, f) == 0 {
                i += 1;
                continue;
            }
            w = snd_pcm_format_width(f);
            if w >= width {
                badness = w - width;
            } else {
                badness = width - w + 32;
            }
            badness = badness.wrapping_add((snd_pcm_format_unsigned(f) != unsignd) as c_uint);
            badness = badness.wrapping_add((snd_pcm_format_big_endian(f) != big) as c_uint);
            if badness < best {
                best_format = f;
                best = badness;
            }
            i += 1;
        }
        if best_format >= 0 {
            best_format
        } else {
            -EINVAL
        }
    } else {
        if format == SNDRV_PCM_FORMAT_MU_LAW {
            i = 0;
            while (i as usize) < preferred_formats.len() {
                let format1: snd_pcm_format_t = preferred_formats[i as usize];
                if snd_mask_test(format_mask, format1) != 0 {
                    return format1;
                }
                i += 1;
            }
        }
        -EINVAL
    }
}

#[no_mangle]
pub unsafe extern "C" fn snd_pcm_plug_format_plugins(
    plug: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    slave_params: *mut snd_pcm_hw_params,
) -> c_int {
    let mut tmpformat: snd_pcm_plugin_format;
    let mut dstformat = snd_pcm_plugin_format {
        format: 0,
        rate: 0,
        channels: 0,
    };
    let mut srcformat = dstformat;
    let mut src_access: snd_pcm_access_t;
    let mut dst_access: snd_pcm_access_t;
    let mut plugin: *mut snd_pcm_plugin = ptr::null_mut();
    let mut err: c_int;
    let stream: c_int = snd_pcm_plug_stream(plug);
    let slave_interleaved: c_int = (params_channels(slave_params) == 1
        || params_access(slave_params) == SNDRV_PCM_ACCESS_RW_INTERLEAVED)
        as c_int;

    match stream {
        SNDRV_PCM_STREAM_PLAYBACK => {
            dstformat.format = params_format(slave_params);
            dstformat.rate = params_rate(slave_params);
            dstformat.channels = params_channels(slave_params);
            srcformat.format = params_format(params);
            srcformat.rate = params_rate(params);
            srcformat.channels = params_channels(params);
            src_access = SNDRV_PCM_ACCESS_RW_INTERLEAVED;
            dst_access = if slave_interleaved != 0 {
                SNDRV_PCM_ACCESS_RW_INTERLEAVED
            } else {
                SNDRV_PCM_ACCESS_RW_NONINTERLEAVED
            };
        }
        SNDRV_PCM_STREAM_CAPTURE => {
            dstformat.format = params_format(params);
            dstformat.rate = params_rate(params);
            dstformat.channels = params_channels(params);
            srcformat.format = params_format(slave_params);
            srcformat.rate = params_rate(slave_params);
            srcformat.channels = params_channels(slave_params);
            src_access = if slave_interleaved != 0 {
                SNDRV_PCM_ACCESS_RW_INTERLEAVED
            } else {
                SNDRV_PCM_ACCESS_RW_NONINTERLEAVED
            };
            dst_access = SNDRV_PCM_ACCESS_RW_INTERLEAVED;
        }
        _ => {
            snd_BUG();
            return -EINVAL;
        }
    }
    tmpformat = srcformat;

    pdprintf(
        b"srcformat: format=%i, rate=%i, channels=%i\n\0".as_ptr() as *const c_char,
        srcformat.format,
        srcformat.rate,
        srcformat.channels,
    );
    pdprintf(
        b"dstformat: format=%i, rate=%i, channels=%i\n\0".as_ptr() as *const c_char,
        dstformat.format,
        dstformat.rate,
        dstformat.channels,
    );

    /* Format change (linearization) */
    if rate_match(srcformat.rate, dstformat.rate) == 0 && snd_pcm_format_linear(srcformat.format) == 0
    {
        if srcformat.format != SNDRV_PCM_FORMAT_MU_LAW {
            return -EINVAL;
        }
        tmpformat.format = SNDRV_PCM_FORMAT_S16;
        err = snd_pcm_plugin_build_mulaw(plug, &mut srcformat, &mut tmpformat, &mut plugin);
        if err < 0 {
            return err;
        }
        err = snd_pcm_plugin_append(plugin);
        if err < 0 {
            snd_pcm_plugin_free(plugin);
            return err;
        }
        srcformat = tmpformat;
        src_access = dst_access;
    }

    /* channels reduction */
    if srcformat.channels > dstformat.channels {
        tmpformat.channels = dstformat.channels;
        err = snd_pcm_plugin_build_route(plug, &mut srcformat, &mut tmpformat, &mut plugin);
        pdprintf(
            b"channels reduction: src=%i, dst=%i returns %i\n\0".as_ptr() as *const c_char,
            srcformat.channels,
            tmpformat.channels,
            err,
        );
        if err < 0 {
            return err;
        }
        err = snd_pcm_plugin_append(plugin);
        if err < 0 {
            snd_pcm_plugin_free(plugin);
            return err;
        }
        srcformat = tmpformat;
        src_access = dst_access;
    }

    /* rate resampling */
    if rate_match(srcformat.rate, dstformat.rate) == 0 {
        if srcformat.format != SNDRV_PCM_FORMAT_S16 {
            /* convert to S16 for resampling */
            tmpformat.format = SNDRV_PCM_FORMAT_S16;
            err = snd_pcm_plugin_build_linear(plug, &mut srcformat, &mut tmpformat, &mut plugin);
            if err < 0 {
                return err;
            }
            err = snd_pcm_plugin_append(plugin);
            if err < 0 {
                snd_pcm_plugin_free(plugin);
                return err;
            }
            srcformat = tmpformat;
            src_access = dst_access;
        }
        tmpformat.rate = dstformat.rate;
        err = snd_pcm_plugin_build_rate(plug, &mut srcformat, &mut tmpformat, &mut plugin);
        pdprintf(
            b"rate down resampling: src=%i, dst=%i returns %i\n\0".as_ptr() as *const c_char,
            srcformat.rate,
            tmpformat.rate,
            err,
        );
        if err < 0 {
            return err;
        }
        err = snd_pcm_plugin_append(plugin);
        if err < 0 {
            snd_pcm_plugin_free(plugin);
            return err;
        }
        srcformat = tmpformat;
        src_access = dst_access;
    }

    /* format change */
    if srcformat.format != dstformat.format {
        tmpformat.format = dstformat.format;
        if srcformat.format == SNDRV_PCM_FORMAT_MU_LAW || tmpformat.format == SNDRV_PCM_FORMAT_MU_LAW {
            err = snd_pcm_plugin_build_mulaw(plug, &mut srcformat, &mut tmpformat, &mut plugin);
        } else if snd_pcm_format_linear(srcformat.format) != 0
            && snd_pcm_format_linear(tmpformat.format) != 0
        {
            err = snd_pcm_plugin_build_linear(plug, &mut srcformat, &mut tmpformat, &mut plugin);
        } else {
            return -EINVAL;
        }
        pdprintf(
            b"format change: src=%i, dst=%i returns %i\n\0".as_ptr() as *const c_char,
            srcformat.format,
            tmpformat.format,
            err,
        );
        if err < 0 {
            return err;
        }
        err = snd_pcm_plugin_append(plugin);
        if err < 0 {
            snd_pcm_plugin_free(plugin);
            return err;
        }
        srcformat = tmpformat;
        src_access = dst_access;
    }

    /* channels extension */
    if srcformat.channels < dstformat.channels {
        tmpformat.channels = dstformat.channels;
        err = snd_pcm_plugin_build_route(plug, &mut srcformat, &mut tmpformat, &mut plugin);
        pdprintf(
            b"channels extension: src=%i, dst=%i returns %i\n\0".as_ptr() as *const c_char,
            srcformat.channels,
            tmpformat.channels,
            err,
        );
        if err < 0 {
            return err;
        }
        err = snd_pcm_plugin_append(plugin);
        if err < 0 {
            snd_pcm_plugin_free(plugin);
            return err;
        }
        srcformat = tmpformat;
        src_access = dst_access;
    }

    /* de-interleave */
    if src_access != dst_access {
        err = snd_pcm_plugin_build_copy(plug, &mut srcformat, &mut tmpformat, &mut plugin);
        pdprintf(
            b"interleave change (copy: returns %i)\n\0".as_ptr() as *const c_char,
            err,
        );
        if err < 0 {
            return err;
        }
        err = snd_pcm_plugin_append(plugin);
        if err < 0 {
            snd_pcm_plugin_free(plugin);
            return err;
        }
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_pcm_plug_client_channels_buf(
    plug: *mut snd_pcm_substream,
    buf: *mut c_char,
    count: snd_pcm_uframes_t,
    channels: *mut *mut snd_pcm_plugin_channel,
) -> snd_pcm_sframes_t {
    let plugin: *mut snd_pcm_plugin;
    let mut v: *mut snd_pcm_plugin_channel;
    let format: *mut snd_pcm_plugin_format;
    let width: c_int;
    let nchannels: c_int;
    let mut channel: c_int;
    let stream: c_int = snd_pcm_plug_stream(plug);

    if snd_BUG_ON(buf.is_null()) != 0 {
        return -ENXIO as snd_pcm_sframes_t;
    }
    if stream == SNDRV_PCM_STREAM_PLAYBACK {
        plugin = snd_pcm_plug_first(plug);
        format = &mut (*plugin).src_format;
    } else {
        plugin = snd_pcm_plug_last(plug);
        format = &mut (*plugin).dst_format;
    }
    v = (*plugin).buf_channels;
    *channels = v;
    width = snd_pcm_format_physical_width((*format).format);
    if width < 0 {
        return width as snd_pcm_sframes_t;
    }
    nchannels = (*format).channels as c_int;
    if snd_BUG_ON(
        (*plugin).access != SNDRV_PCM_ACCESS_RW_INTERLEAVED && (*format).channels > 1,
    ) != 0
    {
        return -ENXIO as snd_pcm_sframes_t;
    }
    channel = 0;
    while channel < nchannels {
        (*v).frames = count;
        (*v).enabled = 1;
        (*v).wanted = (stream == SNDRV_PCM_STREAM_CAPTURE) as c_int;
        (*v).area.addr = buf;
        (*v).area.first = (channel * width) as c_uint;
        (*v).area.step = (nchannels * width) as c_uint;
        channel += 1;
        v = v.add(1);
    }
    count as snd_pcm_sframes_t
}

#[no_mangle]
pub unsafe extern "C" fn snd_pcm_plug_write_transfer(
    plug: *mut snd_pcm_substream,
    mut src_channels: *mut snd_pcm_plugin_channel,
    size: snd_pcm_uframes_t,
) -> snd_pcm_sframes_t {
    let mut plugin: *mut snd_pcm_plugin;
    let mut next: *mut snd_pcm_plugin;
    let mut dst_channels: *mut snd_pcm_plugin_channel;
    let mut err: c_int;
    let mut frames: snd_pcm_sframes_t = size as snd_pcm_sframes_t;

    plugin = snd_pcm_plug_first(plug);
    while !plugin.is_null() {
        if frames <= 0 {
            return frames;
        }
        next = (*plugin).next;
        if !next.is_null() {
            let mut frames1: snd_pcm_sframes_t = frames;
            if let Some(dst_frames) = (*plugin).dst_frames {
                frames1 = dst_frames(plugin, frames as snd_pcm_uframes_t);
                if frames1 <= 0 {
                    return frames1;
                }
            }
            err = (*next).client_channels.unwrap()(next, frames1 as snd_pcm_uframes_t, &mut dst_channels)
                as c_int;
            if err < 0 {
                return err as snd_pcm_sframes_t;
            }
            if err as snd_pcm_sframes_t != frames1 {
                frames = err as snd_pcm_sframes_t;
                if let Some(src_frames) = (*plugin).src_frames {
                    frames = src_frames(plugin, frames1 as snd_pcm_uframes_t);
                    if frames <= 0 {
                        return frames;
                    }
                }
            }
        } else {
            dst_channels = ptr::null_mut();
        }
        pdprintf(
            b"write plugin: %s, %li\n\0".as_ptr() as *const c_char,
            (*plugin).name,
            frames,
        );
        frames = (*plugin).transfer.unwrap()(plugin, src_channels, dst_channels, frames as snd_pcm_uframes_t);
        if frames < 0 {
            return frames;
        }
        src_channels = dst_channels;
        plugin = next;
    }
    calc_src_frames(plug, frames, true)
}

#[no_mangle]
pub unsafe extern "C" fn snd_pcm_plug_read_transfer(
    plug: *mut snd_pcm_substream,
    dst_channels_final: *mut snd_pcm_plugin_channel,
    size: snd_pcm_uframes_t,
) -> snd_pcm_sframes_t {
    let mut plugin: *mut snd_pcm_plugin;
    let mut next: *mut snd_pcm_plugin;
    let mut src_channels: *mut snd_pcm_plugin_channel;
    let mut dst_channels: *mut snd_pcm_plugin_channel = ptr::null_mut();
    let mut frames: snd_pcm_sframes_t = size as snd_pcm_sframes_t;
    let mut err: c_int;

    frames = calc_src_frames(plug, frames, true);
    if frames < 0 {
        return frames;
    }

    src_channels = ptr::null_mut();
    plugin = snd_pcm_plug_first(plug);
    while !plugin.is_null() && frames > 0 {
        next = (*plugin).next;
        if !next.is_null() {
            err = (*plugin).client_channels.unwrap()(plugin, frames as snd_pcm_uframes_t, &mut dst_channels)
                as c_int;
            if err < 0 {
                return err as snd_pcm_sframes_t;
            }
            frames = err as snd_pcm_sframes_t;
        } else {
            dst_channels = dst_channels_final;
        }
        pdprintf(
            b"read plugin: %s, %li\n\0".as_ptr() as *const c_char,
            (*plugin).name,
            frames,
        );
        frames = (*plugin).transfer.unwrap()(plugin, src_channels, dst_channels, frames as snd_pcm_uframes_t);
        if frames < 0 {
            return frames;
        }
        plugin = next;
        src_channels = dst_channels;
    }
    frames
}

#[no_mangle]
pub unsafe extern "C" fn snd_pcm_area_silence(
    dst_area: *const snd_pcm_channel_area,
    dst_offset: size_t,
    mut samples: size_t,
    format: snd_pcm_format_t,
) -> c_int {
    /* FIXME: sub byte resolution and odd dst_offset */
    let mut dst: *mut c_uchar;
    let dst_step: c_uint;
    let mut width: c_int;
    let silence: *const c_uchar;
    if (*dst_area).addr.is_null() {
        return 0;
    }
    dst = (*dst_area).addr.add(((*dst_area).first as size_t + (*dst_area).step as size_t * dst_offset) / 8)
        as *mut c_uchar;
    width = snd_pcm_format_physical_width(format);
    if width <= 0 {
        return -EINVAL;
    }
    if (*dst_area).step == width as c_uint && width >= 8 {
        return snd_pcm_format_set_silence(format, dst as *mut c_void, samples);
    }
    silence = snd_pcm_format_silence_64(format);
    if silence.is_null() {
        return -EINVAL;
    }
    dst_step = (*dst_area).step / 8;
    if width == 4 {
        /* Ima ADPCM */
        let mut dstbit: c_int = ((*dst_area).first % 8) as c_int;
        let dstbit_step: c_int = ((*dst_area).step % 8) as c_int;
        while samples > 0 {
            samples -= 1;
            if dstbit != 0 {
                *dst &= 0xf0;
            } else {
                *dst &= 0x0f;
            }
            dst = dst.add(dst_step as usize);
            dstbit += dstbit_step;
            if dstbit == 8 {
                dst = dst.add(1);
                dstbit = 0;
            }
        }
    } else {
        width /= 8;
        while samples > 0 {
            samples -= 1;
            memcpy(dst as *mut c_void, silence as *const c_void, width as size_t);
            dst = dst.add(dst_step as usize);
        }
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_pcm_area_copy(
    src_area: *const snd_pcm_channel_area,
    src_offset: size_t,
    dst_area: *const snd_pcm_channel_area,
    dst_offset: size_t,
    mut samples: size_t,
    format: snd_pcm_format_t,
) -> c_int {
    /* FIXME: sub byte resolution and odd dst_offset */
    let mut src: *mut c_char;
    let mut dst: *mut c_char;
    let mut width: c_int;
    let src_step: c_int;
    let dst_step: c_int;
    src = (*src_area).addr.add(((*src_area).first as size_t + (*src_area).step as size_t * src_offset) / 8);
    if (*src_area).addr.is_null() {
        return snd_pcm_area_silence(dst_area, dst_offset, samples, format);
    }
    dst = (*dst_area).addr.add(((*dst_area).first as size_t + (*dst_area).step as size_t * dst_offset) / 8);
    if (*dst_area).addr.is_null() {
        return 0;
    }
    width = snd_pcm_format_physical_width(format);
    if width <= 0 {
        return -EINVAL;
    }
    if (*src_area).step == width as c_uint && (*dst_area).step == width as c_uint && width >= 8 {
        let bytes: size_t = samples * width as size_t / 8;
        memcpy(dst as *mut c_void, src as *const c_void, bytes);
        return 0;
    }
    src_step = ((*src_area).step / 8) as c_int;
    dst_step = ((*dst_area).step / 8) as c_int;
    if width == 4 {
        /* Ima ADPCM */
        let mut srcbit: c_int = ((*src_area).first % 8) as c_int;
        let srcbit_step: c_int = ((*src_area).step % 8) as c_int;
        let mut dstbit: c_int = ((*dst_area).first % 8) as c_int;
        let dstbit_step: c_int = ((*dst_area).step % 8) as c_int;
        while samples > 0 {
            samples -= 1;
            let srcval: c_uchar;
            if srcbit != 0 {
                srcval = *src as c_uchar & 0x0f;
            } else {
                srcval = ((*src as c_uchar & 0xf0) >> 4) as c_uchar;
            }
            if dstbit != 0 {
                *dst = ((*dst as c_uchar & 0xf0) | srcval) as c_char;
            } else {
                *dst = ((*dst as c_uchar & 0x0f) | (srcval << 4)) as c_char;
            }
            src = src.add(src_step as usize);
            srcbit += srcbit_step;
            if srcbit == 8 {
                src = src.add(1);
                srcbit = 0;
            }
            dst = dst.add(dst_step as usize);
            dstbit += dstbit_step;
            if dstbit == 8 {
                dst = dst.add(1);
                dstbit = 0;
            }
        }
    } else {
        width /= 8;
        while samples > 0 {
            samples -= 1;
            memcpy(dst as *mut c_void, src as *const c_void, width as size_t);
            src = src.add(src_step as usize);
            dst = dst.add(dst_step as usize);
        }
    }
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
