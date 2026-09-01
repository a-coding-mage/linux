// SPDX-License-Identifier: LGPL-2.0+
/*
 *  Linear conversion Plug-In
 *  Copyright (c) 2000 by Abramo Bagnara <abramo@alsa-project.org>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use core::ffi::{c_char, c_int};

pub type snd_pcm_sframes_t = isize;
pub type snd_pcm_uframes_t = usize;

pub const ENXIO: c_int = 6;

#[repr(C)]
pub struct snd_pcm_substream {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_pcm_plugin_format {
    pub format: c_int,
    pub rate: c_int,
    pub channels: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_pcm_channel_area {
    pub addr: *mut core::ffi::c_void,
    pub first: u32,
    pub step: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_pcm_plugin_channel {
    pub area: snd_pcm_channel_area,
    pub enabled: c_int,
    pub wanted: c_int,
}

pub type snd_pcm_plugin_transfer_t = Option<
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
    pub transfer: snd_pcm_plugin_transfer_t,
}

unsafe extern "C" {
    fn snd_BUG_ON(condition: bool) -> bool;
    fn snd_pcm_area_silence(
        area: *mut snd_pcm_channel_area,
        offset: snd_pcm_uframes_t,
        frames: snd_pcm_uframes_t,
        format: c_int,
    );
    fn snd_pcm_area_copy(
        src_area: *const snd_pcm_channel_area,
        src_offset: snd_pcm_uframes_t,
        dst_area: *mut snd_pcm_channel_area,
        dst_offset: snd_pcm_uframes_t,
        frames: snd_pcm_uframes_t,
        format: c_int,
    );
    fn snd_pcm_format_physical_width(format: c_int) -> c_int;
    fn snd_pcm_plugin_build(
        plug: *mut snd_pcm_substream,
        name: *const c_char,
        src_format: *mut snd_pcm_plugin_format,
        dst_format: *mut snd_pcm_plugin_format,
        extra: c_int,
        r_plugin: *mut *mut snd_pcm_plugin,
    ) -> c_int;
}

unsafe extern "C" fn copy_transfer(
    plugin: *mut snd_pcm_plugin,
    mut src_channels: *const snd_pcm_plugin_channel,
    mut dst_channels: *mut snd_pcm_plugin_channel,
    frames: snd_pcm_uframes_t,
) -> snd_pcm_sframes_t {
    let mut channel: u32;
    let nchannels: u32;

    if unsafe { snd_BUG_ON(plugin.is_null() || src_channels.is_null() || dst_channels.is_null()) } {
        return -(ENXIO as snd_pcm_sframes_t);
    }
    if frames == 0 {
        return 0;
    }
    nchannels = unsafe { (*plugin).src_format.channels };
    channel = 0;
    while channel < nchannels {
        if unsafe {
            snd_BUG_ON(
                (*src_channels).area.first % 8 != 0 || (*src_channels).area.step % 8 != 0,
            )
        } {
            return -(ENXIO as snd_pcm_sframes_t);
        }
        if unsafe {
            snd_BUG_ON((*dst_channels).area.first % 8 != 0 || (*dst_channels).area.step % 8 != 0)
        } {
            return -(ENXIO as snd_pcm_sframes_t);
        }
        if unsafe { (*src_channels).enabled == 0 } {
            if unsafe { (*dst_channels).wanted != 0 } {
                unsafe {
                    snd_pcm_area_silence(
                        &mut (*dst_channels).area,
                        0,
                        frames,
                        (*plugin).dst_format.format,
                    );
                }
            }
            unsafe {
                (*dst_channels).enabled = 0;
            }
            channel += 1;
            continue;
        }
        unsafe {
            (*dst_channels).enabled = 1;
            snd_pcm_area_copy(
                &(*src_channels).area,
                0,
                &mut (*dst_channels).area,
                0,
                frames,
                (*plugin).src_format.format,
            );
            src_channels = src_channels.add(1);
            dst_channels = dst_channels.add(1);
        }
        channel += 1;
    }
    frames as snd_pcm_sframes_t
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_pcm_plugin_build_copy(
    plug: *mut snd_pcm_substream,
    src_format: *mut snd_pcm_plugin_format,
    dst_format: *mut snd_pcm_plugin_format,
    r_plugin: *mut *mut snd_pcm_plugin,
) -> c_int {
    let err: c_int;
    let mut plugin: *mut snd_pcm_plugin = core::ptr::null_mut();
    let width: c_int;

    if unsafe { snd_BUG_ON(r_plugin.is_null()) } {
        return -ENXIO;
    }
    unsafe {
        *r_plugin = core::ptr::null_mut();
    }

    if unsafe { snd_BUG_ON((*src_format).format != (*dst_format).format) } {
        return -ENXIO;
    }
    if unsafe { snd_BUG_ON((*src_format).rate != (*dst_format).rate) } {
        return -ENXIO;
    }
    if unsafe { snd_BUG_ON((*src_format).channels != (*dst_format).channels) } {
        return -ENXIO;
    }

    width = unsafe { snd_pcm_format_physical_width((*src_format).format) };
    if unsafe { snd_BUG_ON(width <= 0) } {
        return -ENXIO;
    }

    err = unsafe {
        snd_pcm_plugin_build(
            plug,
            c"copy".as_ptr(),
            src_format,
            dst_format,
            0,
            &mut plugin,
        )
    };
    if err < 0 {
        return err;
    }
    unsafe {
        (*plugin).transfer = Some(copy_transfer);
        *r_plugin = plugin;
    }
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
