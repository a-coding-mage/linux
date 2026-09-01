// SPDX-License-Identifier: LGPL-2.0+
/*
 *  Route Plug-In
 *  Copyright (c) 2000 by Abramo Bagnara <abramo@alsa-project.org>
 */

// C dependencies: <linux/time.h>, <sound/core.h>, <sound/pcm.h>, "pcm_plugin.h"

use core::ptr;

unsafe fn zero_areas(
    mut dvp: *mut snd_pcm_plugin_channel,
    ndsts: ::core::ffi::c_int,
    frames: snd_pcm_uframes_t,
    format: snd_pcm_format_t,
) {
    let mut dst: ::core::ffi::c_int = 0;
    while dst < ndsts {
        if (*dvp).wanted != 0 {
            snd_pcm_area_silence(&mut (*dvp).area, 0, frames, format);
        }
        (*dvp).enabled = 0;
        dvp = dvp.add(1);
        dst += 1;
    }
}

#[inline]
unsafe fn copy_area(
    src_channel: *const snd_pcm_plugin_channel,
    dst_channel: *mut snd_pcm_plugin_channel,
    frames: snd_pcm_uframes_t,
    format: snd_pcm_format_t,
) {
    (*dst_channel).enabled = 1;
    snd_pcm_area_copy(
        &(*src_channel).area,
        0,
        &mut (*dst_channel).area,
        0,
        frames,
        format,
    );
}

unsafe extern "C" fn route_transfer(
    plugin: *mut snd_pcm_plugin,
    mut src_channels: *const snd_pcm_plugin_channel,
    dst_channels: *mut snd_pcm_plugin_channel,
    mut frames: snd_pcm_uframes_t,
) -> snd_pcm_sframes_t {
    let nsrcs: ::core::ffi::c_int;
    let ndsts: ::core::ffi::c_int;
    let mut dst: ::core::ffi::c_int;
    let mut dvp: *mut snd_pcm_plugin_channel;
    let format: snd_pcm_format_t;

    if snd_BUG_ON(plugin.is_null() || src_channels.is_null() || dst_channels.is_null()) {
        return -ENXIO;
    }
    if frames == 0 {
        return 0;
    }
    if frames > (*dst_channels.add(0)).frames {
        frames = (*dst_channels.add(0)).frames;
    }

    nsrcs = (*plugin).src_format.channels;
    ndsts = (*plugin).dst_format.channels;

    format = (*plugin).dst_format.format;
    dvp = dst_channels;
    if nsrcs <= 1 {
        /* expand to all channels */
        dst = 0;
        while dst < ndsts {
            copy_area(src_channels, dvp, frames, format);
            dvp = dvp.add(1);
            dst += 1;
        }
        return frames as snd_pcm_sframes_t;
    }

    dst = 0;
    while dst < ndsts && dst < nsrcs {
        copy_area(src_channels, dvp, frames, format);
        dvp = dvp.add(1);
        src_channels = src_channels.add(1);
        dst += 1;
    }
    if dst < ndsts {
        zero_areas(dvp, ndsts - dst, frames, format);
    }
    frames as snd_pcm_sframes_t
}

#[no_mangle]
pub unsafe extern "C" fn snd_pcm_plugin_build_route(
    plug: *mut snd_pcm_substream,
    src_format: *mut snd_pcm_plugin_format,
    dst_format: *mut snd_pcm_plugin_format,
    r_plugin: *mut *mut snd_pcm_plugin,
) -> ::core::ffi::c_int {
    let mut plugin: *mut snd_pcm_plugin = ptr::null_mut();
    let err: ::core::ffi::c_int;

    if snd_BUG_ON(r_plugin.is_null()) {
        return -ENXIO;
    }
    *r_plugin = ptr::null_mut();
    if snd_BUG_ON((*src_format).rate != (*dst_format).rate) {
        return -ENXIO;
    }
    if snd_BUG_ON((*src_format).format != (*dst_format).format) {
        return -ENXIO;
    }

    err = snd_pcm_plugin_build(
        plug,
        b"route conversion\0".as_ptr() as *const ::core::ffi::c_char,
        src_format,
        dst_format,
        0,
        &mut plugin,
    );
    if err < 0 {
        return err;
    }

    (*plugin).transfer = Some(route_transfer);
    *r_plugin = plugin;
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
