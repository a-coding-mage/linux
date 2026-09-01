// SPDX-License-Identifier: LGPL-2.0+
/*
 *  PCM I/O Plug-In Interface
 *  Copyright (c) 1999 by Jaroslav Kysela <perex@perex.cz>
 */

/* Dependencies from the original C includes:
 * <linux/time.h>, <sound/core.h>, <sound/pcm.h>, <sound/pcm_params.h>,
 * and "pcm_plugin.h".
 */

use core::ffi::{c_char, c_int, c_void};

pub type snd_pcm_sframes_t = isize;
pub type snd_pcm_uframes_t = usize;

pub const ENXIO: c_int = 6;

unsafe extern "C" {
    static SNDRV_PCM_ACCESS_RW_INTERLEAVED: c_int;
    static SNDRV_PCM_STREAM_PLAYBACK: c_int;

    fn snd_BUG_ON(cond: c_int) -> c_int;

    fn snd_pcm_oss_write3(
        plug: *mut snd_pcm_substream,
        buf: *mut c_void,
        count: snd_pcm_uframes_t,
        in_kernel: c_int,
    ) -> snd_pcm_sframes_t;
    fn snd_pcm_oss_writev3(
        plug: *mut snd_pcm_substream,
        vec: *mut *mut c_void,
        count: snd_pcm_uframes_t,
    ) -> snd_pcm_sframes_t;
    fn snd_pcm_oss_read3(
        plug: *mut snd_pcm_substream,
        buf: *mut c_void,
        count: snd_pcm_uframes_t,
        in_kernel: c_int,
    ) -> snd_pcm_sframes_t;
    fn snd_pcm_oss_readv3(
        plug: *mut snd_pcm_substream,
        vec: *mut *mut c_void,
        count: snd_pcm_uframes_t,
    ) -> snd_pcm_sframes_t;

    fn snd_pcm_plugin_client_channels(
        plugin: *mut snd_pcm_plugin,
        frames: snd_pcm_uframes_t,
        channels: *mut *mut snd_pcm_plugin_channel,
    ) -> c_int;
    fn snd_pcm_plugin_build(
        plug: *mut snd_pcm_substream,
        name: *const c_char,
        src_format: *const snd_pcm_plugin_format,
        dst_format: *const snd_pcm_plugin_format,
        extra: usize,
        r_plugin: *mut *mut snd_pcm_plugin,
    ) -> c_int;

    fn params_format(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_access(params: *mut snd_pcm_hw_params) -> c_int;
    fn snd_pcm_plug_stream(plug: *mut snd_pcm_substream) -> c_int;
}

pub type c_uint = u32;

#[repr(C)]
pub struct snd_pcm_substream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_pcm_channel_area {
    pub addr: *mut c_void,
}

#[repr(C)]
pub struct snd_pcm_plugin_channel {
    pub area: snd_pcm_channel_area,
    pub enabled: c_int,
    pub wanted: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_pcm_plugin_format {
    pub format: c_int,
    pub rate: c_int,
    pub channels: c_uint,
}

#[repr(C)]
pub struct snd_pcm_plugin {
    pub plug: *mut snd_pcm_substream,
    pub access: c_int,
    pub src_format: snd_pcm_plugin_format,
    pub dst_format: snd_pcm_plugin_format,
    pub extra_data: *mut c_void,
    pub transfer: Option<
        unsafe extern "C" fn(
            *mut snd_pcm_plugin,
            *const snd_pcm_plugin_channel,
            *mut snd_pcm_plugin_channel,
            snd_pcm_uframes_t,
        ) -> snd_pcm_sframes_t,
    >,
    pub client_channels: Option<
        unsafe extern "C" fn(
            *mut snd_pcm_plugin,
            snd_pcm_uframes_t,
            *mut *mut snd_pcm_plugin_channel,
        ) -> snd_pcm_sframes_t,
    >,
}

#[inline]
unsafe fn pcm_write(
    plug: *mut snd_pcm_substream,
    buf: *mut c_void,
    count: snd_pcm_uframes_t,
) -> snd_pcm_sframes_t {
    unsafe { snd_pcm_oss_write3(plug, buf, count, 1) }
}

#[inline]
unsafe fn pcm_writev(
    plug: *mut snd_pcm_substream,
    vec: *mut *mut c_void,
    count: snd_pcm_uframes_t,
) -> snd_pcm_sframes_t {
    unsafe { snd_pcm_oss_writev3(plug, vec, count) }
}

#[inline]
unsafe fn pcm_read(
    plug: *mut snd_pcm_substream,
    buf: *mut c_void,
    count: snd_pcm_uframes_t,
) -> snd_pcm_sframes_t {
    unsafe { snd_pcm_oss_read3(plug, buf, count, 1) }
}

#[inline]
unsafe fn pcm_readv(
    plug: *mut snd_pcm_substream,
    vec: *mut *mut c_void,
    count: snd_pcm_uframes_t,
) -> snd_pcm_sframes_t {
    unsafe { snd_pcm_oss_readv3(plug, vec, count) }
}

/*
 *  Basic io plugin
 */

unsafe extern "C" fn io_playback_transfer(
    plugin: *mut snd_pcm_plugin,
    src_channels: *const snd_pcm_plugin_channel,
    _dst_channels: *mut snd_pcm_plugin_channel,
    frames: snd_pcm_uframes_t,
) -> snd_pcm_sframes_t {
    unsafe {
        if snd_BUG_ON(plugin.is_null() as c_int) != 0 {
            return -(ENXIO as snd_pcm_sframes_t);
        }
        if snd_BUG_ON(src_channels.is_null() as c_int) != 0 {
            return -(ENXIO as snd_pcm_sframes_t);
        }
        if (*plugin).access == SNDRV_PCM_ACCESS_RW_INTERLEAVED {
            return pcm_write((*plugin).plug, (*src_channels).area.addr, frames);
        } else {
            let mut channel: c_int;
            let channels: c_int = (*plugin).dst_format.channels as c_int;
            let bufs: *mut *mut c_void = (*plugin).extra_data as *mut *mut c_void;
            if snd_BUG_ON(bufs.is_null() as c_int) != 0 {
                return -(ENXIO as snd_pcm_sframes_t);
            }
            channel = 0;
            while channel < channels {
                if (*src_channels.add(channel as usize)).enabled != 0 {
                    *bufs.add(channel as usize) = (*src_channels.add(channel as usize)).area.addr;
                } else {
                    *bufs.add(channel as usize) = core::ptr::null_mut();
                }
                channel += 1;
            }
            return pcm_writev((*plugin).plug, bufs, frames);
        }
    }
}

unsafe extern "C" fn io_capture_transfer(
    plugin: *mut snd_pcm_plugin,
    _src_channels: *const snd_pcm_plugin_channel,
    dst_channels: *mut snd_pcm_plugin_channel,
    frames: snd_pcm_uframes_t,
) -> snd_pcm_sframes_t {
    unsafe {
        if snd_BUG_ON(plugin.is_null() as c_int) != 0 {
            return -(ENXIO as snd_pcm_sframes_t);
        }
        if snd_BUG_ON(dst_channels.is_null() as c_int) != 0 {
            return -(ENXIO as snd_pcm_sframes_t);
        }
        if (*plugin).access == SNDRV_PCM_ACCESS_RW_INTERLEAVED {
            return pcm_read((*plugin).plug, (*dst_channels).area.addr, frames);
        } else {
            let mut channel: c_int;
            let channels: c_int = (*plugin).dst_format.channels as c_int;
            let bufs: *mut *mut c_void = (*plugin).extra_data as *mut *mut c_void;
            if snd_BUG_ON(bufs.is_null() as c_int) != 0 {
                return -(ENXIO as snd_pcm_sframes_t);
            }
            channel = 0;
            while channel < channels {
                if (*dst_channels.add(channel as usize)).enabled != 0 {
                    *bufs.add(channel as usize) = (*dst_channels.add(channel as usize)).area.addr;
                } else {
                    *bufs.add(channel as usize) = core::ptr::null_mut();
                }
                channel += 1;
            }
            return pcm_readv((*plugin).plug, bufs, frames);
        }
    }
}

unsafe extern "C" fn io_src_channels(
    plugin: *mut snd_pcm_plugin,
    frames: snd_pcm_uframes_t,
    channels: *mut *mut snd_pcm_plugin_channel,
) -> snd_pcm_sframes_t {
    unsafe {
        let mut err: c_int;
        let mut channel: c_uint;
        let mut v: *mut snd_pcm_plugin_channel = core::ptr::null_mut();
        err = snd_pcm_plugin_client_channels(plugin, frames, &mut v);
        if err < 0 {
            return err as snd_pcm_sframes_t;
        }
        *channels = v;
        if (*plugin).access == SNDRV_PCM_ACCESS_RW_INTERLEAVED {
            channel = 0;
            while channel < (*plugin).src_format.channels {
                (*v).wanted = 1;
                channel += 1;
                v = v.add(1);
            }
        }
        frames as snd_pcm_sframes_t
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_pcm_plugin_build_io(
    plug: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    r_plugin: *mut *mut snd_pcm_plugin,
) -> c_int {
    unsafe {
        let mut err: c_int;
        let mut format: snd_pcm_plugin_format = core::mem::zeroed();
        let mut plugin: *mut snd_pcm_plugin = core::ptr::null_mut();

        if snd_BUG_ON(r_plugin.is_null() as c_int) != 0 {
            return -ENXIO;
        }
        *r_plugin = core::ptr::null_mut();
        if snd_BUG_ON((plug.is_null() || params.is_null()) as c_int) != 0 {
            return -ENXIO;
        }
        format.format = params_format(params);
        format.rate = params_rate(params);
        format.channels = params_channels(params);
        err = snd_pcm_plugin_build(
            plug,
            b"I/O io\0".as_ptr() as *const c_char,
            &format,
            &format,
            core::mem::size_of::<*mut c_void>() * format.channels as usize,
            &mut plugin,
        );
        if err < 0 {
            return err;
        }
        (*plugin).access = params_access(params);
        if snd_pcm_plug_stream(plug) == SNDRV_PCM_STREAM_PLAYBACK {
            (*plugin).transfer = Some(io_playback_transfer);
            if (*plugin).access == SNDRV_PCM_ACCESS_RW_INTERLEAVED {
                (*plugin).client_channels = Some(io_src_channels);
            }
        } else {
            (*plugin).transfer = Some(io_capture_transfer);
        }

        *r_plugin = plugin;
        0
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
